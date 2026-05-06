//! Outgoing packet encoder.
//!
//! Serializes [`RawPacket`]s into the Nitro wire format and writes them
//! into a [`BytesMut`] buffer. Implements [`tokio_util::codec::Encoder`]
//! so it integrates with `Framed`.

use crate::header::PacketHeader;
use crate::packet::RawPacket;
use bytes::{BufMut, BytesMut};
use sirius_error::{ProtocolError, SiriusError};
use tokio_util::codec::Encoder;
use tracing::trace;

/// Encoder for the Nitro wire protocol.
///
/// Stateless. All encoding is done in a single pass over the packet.
#[derive(Debug, Default)]
pub struct NitroEncoder;

impl NitroEncoder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Encoder<RawPacket> for NitroEncoder {
    type Error = SiriusError;

    fn encode(&mut self, packet: RawPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let body_len = packet.body.len();

        let length = PacketHeader::MIN_LENGTH
            .checked_add(body_len as u32)
            .ok_or_else(|| {
                SiriusError::Protocol(ProtocolError::EncodingFailed {
                    header_id: packet.header.id,
                    reason: format!("body length {} overflows u32", body_len),
                })
            })?;

        dst.reserve(PacketHeader::LENGTH_FIELD_SIZE + length as usize);

        dst.put_u32(length);
        dst.put_u16(packet.header.id);
        dst.put_slice(&packet.body);

        trace!(id = packet.header.id, body_len, "encoded packet");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn encodes_empty_packet() {
        let mut encoder = NitroEncoder::new();
        let mut dst = BytesMut::new();
        let packet = RawPacket::empty(4000);

        encoder.encode(packet, &mut dst).unwrap();

        // 4 bytes length (= 2), 2 bytes id (= 4000)
        assert_eq!(dst.len(), 6);
        assert_eq!(&dst[..4], &2u32.to_be_bytes());
        assert_eq!(&dst[4..6], &4000u16.to_be_bytes());
    }

    #[test]
    fn encodes_packet_with_body() {
        let mut encoder = NitroEncoder::new();
        let mut dst = BytesMut::new();
        let packet = RawPacket::new(1000, Bytes::from_static(b"hello"));

        encoder.encode(packet, &mut dst).unwrap();

        // length = 2 + 5 = 7
        assert_eq!(&dst[..4], &7u32.to_be_bytes());
        assert_eq!(&dst[4..6], &1000u16.to_be_bytes());
        assert_eq!(&dst[6..], b"hello");
    }

    #[test]
    fn roundtrip_with_decoder() {
        use crate::decoder::NitroDecoder;
        use tokio_util::codec::Decoder;

        let original = RawPacket::new(9999, Bytes::from_static(b"roundtrip"));

        let mut encoder = NitroEncoder::new();
        let mut buf = BytesMut::new();
        encoder.encode(original.clone(), &mut buf).unwrap();

        let mut decoder = NitroDecoder::new();
        let decoded = decoder.decode(&mut buf).unwrap().unwrap();

        assert_eq!(decoded.id(), original.id());
        assert_eq!(decoded.body, original.body);
    }
}
