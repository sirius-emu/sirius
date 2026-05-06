//! Every `OutgoingPacket::serialize` implementation creates a `PacketWriter`,
//! calls typed write methods, then calls [`finish`] to get a [`RawPacket`].
//!
//! [`finish`]: PacketWriter::finish

use bytes::{BufMut, BytesMut};
use sirius_codec::RawPacket;
use sirius_error::SiriusError;

/// A buffer for building outgoing packet bodies.
///
/// Write method appends to an internal [`BytesMut`] buffer. Call [`finish`] to
/// freeze the buffer and wrap it in a [`RawPacket`] with the correct header.
///
/// # Example
///
/// ```
/// use sirius_packets::PacketWriter;
///
/// let mut w = PacketWriter::new(4000);
/// w.write_i32(42);
/// w.write_string("hello");
/// let packet = w.finish();
/// ```
#[derive(Debug)]
pub struct PacketWriter {
    header_id: u16,
    buf: BytesMut,
}

impl PacketWriter {
    /// Creates a new writer for a packet with the given header ID.
    pub fn new(header_id: u16) -> Self {
        Self {
            header_id,
            buf: BytesMut::new(),
        }
    }

    /// Creates a new writer with a pre-allocated capacity hint.
    ///
    /// Use this when you know roughly how large the packet will be to avoid
    /// repeated reallocations.
    pub fn with_capacity(header_id: u16, capacity: usize) -> Self {
        Self {
            header_id,
            buf: BytesMut::with_capacity(capacity),
        }
    }

    /// Writes a big-endian `i32`.
    pub fn write_i32(&mut self, v: i32) -> &mut Self {
        self.buf.put_i32(v);
        self
    }

    /// Writes a big-endian `u32`.
    pub fn write_u32(&mut self, v: u32) -> &mut Self {
        self.buf.put_u32(v);
        self
    }

    /// Writes a big-endian `i16`.
    pub fn write_i16(&mut self, v: i16) -> &mut Self {
        self.buf.put_i16(v);
        self
    }

    /// Writes a big-endian `u16`.
    pub fn write_u16(&mut self, v: u16) -> &mut Self {
        self.buf.put_u16(v);
        self
    }

    /// Writes a single `u8`.
    pub fn write_u8(&mut self, v: u8) -> &mut Self {
        self.buf.put_u8(v);
        self
    }

    /// Writes a `bool` as a single byte (`false = 0`, `true = 1`).
    pub fn write_bool(&mut self, v: bool) -> &mut Self {
        self.buf.put_u8(v as u8);
        self
    }

    /// Writes a length-prefixed UTF-8 string.
    ///
    /// The wire format is a big-endian `u16` byte length followed by the
    /// UTF-8 encoded string. Panics if the string is longer than 65535 bytes,
    /// no legitimate Habbo string comes close to that limit.
    pub fn write_string(&mut self, s: &str) -> &mut Self {
        let bytes = s.as_bytes();
        assert!(
            bytes.len() <= u16::MAX as usize,
            "string too long for packet: {} bytes",
            bytes.len()
        );
        self.buf.put_u16(bytes.len() as u16);
        self.buf.put_slice(bytes);
        self
    }

    /// Writes raw bytes without any length prefix.
    pub fn write_bytes(&mut self, data: &[u8]) -> &mut Self {
        self.buf.put_slice(data);
        self
    }

    /// Consumes the writer and returns a [`RawPacket`].
    ///
    /// This is infallible, the writer only accepts well-typed inputs, so
    /// there's nothing that can go wrong at finalization time.
    pub fn finish(self) -> RawPacket {
        RawPacket::new(self.header_id, self.buf.freeze())
    }

    /// Convenience method for `OutgoingPacket::serialize` implementations
    /// that want to return `Result<RawPacket, SiriusError>`.
    pub fn finish_ok(self) -> Result<RawPacket, SiriusError> {
        Ok(self.finish())
    }

    /// Returns the number of bytes written so far.
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if no bytes have been written yet.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_finish() {
        let mut w = PacketWriter::new(4000);
        w.write_i32(42).write_bool(true).write_string("hi");
        let packet = w.finish();

        assert_eq!(packet.id(), 4000);
        // 4 (i32) + 1 (bool) + 2 (str len) + 2 (str bytes) = 9
        assert_eq!(packet.body.len(), 9);
    }

    #[test]
    fn builder_is_chainable() {
        let packet = {
            let mut w = PacketWriter::new(1);
            w.write_u8(1).write_u8(2).write_u8(3);
            w.finish()
        };
        assert_eq!(packet.body.len(), 3);
    }

    #[test]
    fn roundtrip_string() {
        use crate::reader::PacketReader;

        let mut w = PacketWriter::new(0);
        w.write_string("Sirius");
        let packet = w.finish();

        let mut r = PacketReader::new(packet.body);
        assert_eq!(r.read_string().unwrap(), "Sirius");
    }

    #[test]
    fn wire_len_matches_header() {
        let mut w = PacketWriter::new(999);
        w.write_i32(0).write_i32(0); // 8 bytes body
        let packet = w.finish();

        // wire_len = 4 (length field) + 2 (id) + 8 (body) = 14
        assert_eq!(packet.wire_len(), 14);
        // length field = 2 (id) + 8 (body) = 10
        assert_eq!(packet.header.length, 10);
    }
}
