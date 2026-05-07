//! Incoming packet decoder.
//!
//! Reads a stream of bytes and produces [`RawPacket`]s. Implements
//! [`tokio_util::codec::Decoder`] so it integrates with `Framed`.
//!
//! # Framing strategy
//!
//! The decoder is a two-phase length-delimited framer:
//!
//! 1. Wait until at least 4 bytes are available, then read the `length` field.
//! 2. Wait until `length` more bytes available, then emit `RawPacket`.
//!
//! This is stateful, the decoded length from step 1 is stored in `pending_length`
//! so the decoder doesn't re-read it on the next call.

use crate::header::PacketHeader;
use crate::packet::RawPacket;
use bytes::{Buf, BytesMut};
use sirius_error::{ProtocolError, SiriusError};
use tokio_util::codec::Decoder;
use tracing::trace;

/// Maximum body size the decoder will accept, in bytes.
const MAX_BODY_LEN: usize = 65_535;

/// Stateful decoder for the Nitro wire protocol.
#[derive(Debug, Default)]
pub struct NitroDecoder {
    /// The `length` field from the packet currently being assembled.
    ///
    /// `None` means we haven't read the length prefix yet.
    /// `Some(n)` means we read `n` and are waiting for `n` more bytes.
    pending_length: Option<u32>,
}

impl NitroDecoder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Decoder for NitroDecoder {
    type Item = RawPacket;
    type Error = SiriusError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        let length = if let Some(l) = self.pending_length {
            l
        } else {
            if src.len() < PacketHeader::LENGTH_FIELD_SIZE {
                src.reserve(PacketHeader::SIZE);
                return Ok(None);
            }

            let l = u32::from_be_bytes([src[0], src[1], src[2], src[3]]);
            self.pending_length = Some(l);
            l
        };

        if (length as usize) < PacketHeader::MIN_LENGTH as usize {
            return Err(SiriusError::Protocol(ProtocolError::LengthMismatch {
                declared: length,
                actual: src.len(),
            }));
        }

        let body_len = (length as usize) - PacketHeader::ID_FIELD_SIZE;

        if body_len > MAX_BODY_LEN {
            self.pending_length = None;
            return Err(SiriusError::Protocol(ProtocolError::PacketTooLarge {
                body_len,
                max: MAX_BODY_LEN,
            }));
        }

        let total = PacketHeader::LENGTH_FIELD_SIZE + length as usize;

        if src.len() < total {
            src.reserve(total - src.len());
            return Ok(None);
        }

        src.advance(PacketHeader::LENGTH_FIELD_SIZE);

        let id = u16::from_be_bytes([src[0], src[1]]);
        src.advance(PacketHeader::ID_FIELD_SIZE);

        let body = src.split_to(body_len).freeze();

        trace!(id, body_len, "decoded packet");

        self.pending_length = None;

        Ok(Some(RawPacket {
            header: PacketHeader { length, id },
            body,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BufMut;

    fn make_packet(id: u16, body: &[u8]) -> BytesMut {
        let length = (PacketHeader::ID_FIELD_SIZE + body.len()) as u32;
        let mut buf = BytesMut::new();
        buf.put_u32(length);
        buf.put_u16(id);
        buf.put_slice(body);
        buf
    }

    #[test]
    fn decodes_empty_body_packet() {
        let mut decoder = NitroDecoder::new();
        let mut buf = make_packet(4000, &[]);
        let result = decoder.decode(&mut buf).unwrap();
        let packet = result.expect("should produce a packet");
        assert_eq!(packet.id(), 4000);
        assert!(packet.body.is_empty());
        assert!(buf.is_empty());
    }

    #[test]
    fn decodes_packet_with_body() {
        let mut decoder = NitroDecoder::new();
        let mut buf = make_packet(1234, b"hello");
        let packet = decoder.decode(&mut buf).unwrap().unwrap();
        assert_eq!(packet.id(), 1234);
        assert_eq!(&packet.body[..], b"hello");
    }

    #[test]
    fn returns_none_when_incomplete_length_field() {
        let mut decoder = NitroDecoder::new();
        // Only 3 bytes — not enough to read the 4-byte length field.
        let mut buf = BytesMut::from(&[0u8, 0, 0][..]);
        assert!(decoder.decode(&mut buf).unwrap().is_none());
    }

    #[test]
    fn returns_none_when_length_field_present_but_body_incomplete() {
        let mut decoder = NitroDecoder::new();
        // Length field says 7 (2 id + 5 body), but we only have 4 bytes of body.
        let mut buf = BytesMut::new();
        buf.put_u32(7);
        buf.put_u16(999);
        buf.put_slice(b"hel"); // 3 of 5 expected body bytes
        assert!(decoder.decode(&mut buf).unwrap().is_none());
    }

    #[test]
    fn decodes_two_packets_from_one_buffer() {
        let mut decoder = NitroDecoder::new();
        let mut buf = BytesMut::new();
        buf.extend_from_slice(&make_packet(1, b"aa"));
        buf.extend_from_slice(&make_packet(2, b"bb"));

        let p1 = decoder.decode(&mut buf).unwrap().unwrap();
        let p2 = decoder.decode(&mut buf).unwrap().unwrap();

        assert_eq!(p1.id(), 1);
        assert_eq!(p2.id(), 2);
        assert_eq!(&p1.body[..], b"aa");
        assert_eq!(&p2.body[..], b"bb");
    }

    #[test]
    fn rejects_oversized_body() {
        let mut decoder = NitroDecoder::new();
        let mut buf = BytesMut::new();
        let length = (PacketHeader::ID_FIELD_SIZE + MAX_BODY_LEN + 1) as u32;
        buf.put_u32(length);
        assert!(decoder.decode(&mut buf).is_err());
    }
}
