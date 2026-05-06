//! Every `IncomingPacket::parse` implementation receives a `&mut PacketReader`
//! and calls typed read methods on it. The reader tracks the current position
//! and returns a [`ProtocolError`] if there aren't enough bytes left.

use bytes::{Buf, Bytes};
use sirius_error::{ProtocolError, SiriusError};

/// A cursor over the raw bytes of a packet body.
///
/// Methods advance the internal position on each successful read. If fewer
/// bytes remain than the method requires, a [`ProtocolError::PacketTooLong`]
/// is returned.
#[derive(Debug)]
pub struct PacketReader {
    /// The packet body. Uses `Bytes` so slices are zero-copy.
    buf: Bytes,

    /// Header ID of the packet being read.
    header_id: u16,
}

impl PacketReader {
    /// Creates a new reader over the given body bytes.
    ///
    /// `header_id` is only used in error messages.
    pub fn new(body: Bytes) -> Self {
        Self::with_header_id(body, 0)
    }

    /// Crates a new reader and records the header ID for error context.
    pub fn with_header_id(body: Bytes, header_id: u16) -> Self {
        Self {
            buf: body,
            header_id,
        }
    }

    /// Returns the number of bytes remaining.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.buf.remaining()
    }

    /// Returns `true` if there are no bytes left.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Reads a big-endian `i32`.
    pub fn read_i32(&mut self) -> Result<i32, SiriusError> {
        self.need(4)?;
        Ok(self.buf.get_i32())
    }

    /// Reads a big-endian `u32`.
    pub fn read_u32(&mut self) -> Result<u32, SiriusError> {
        self.need(4)?;
        Ok(self.buf.get_u32())
    }

    /// Reads a big-endian `i16`.
    pub fn read_i16(&mut self) -> Result<i16, SiriusError> {
        self.need(2)?;
        Ok(self.buf.get_i16())
    }

    /// Reads a big-endian `u16`.
    pub fn read_u16(&mut self) -> Result<u16, SiriusError> {
        self.need(2)?;
        Ok(self.buf.get_u16())
    }

    /// Reads a single `u8`.
    pub fn read_u8(&mut self) -> Result<u8, SiriusError> {
        self.need(1)?;
        Ok(self.buf.get_u8())
    }

    /// Reads a single `i8`.
    pub fn read_i8(&mut self) -> Result<i8, SiriusError> {
        self.need(1)?;
        Ok(self.buf.get_i8())
    }

    /// Reads a `bool` encoded as a single byte (`0 = false`, non-zero = true).
    pub fn read_bool(&mut self) -> Result<bool, SiriusError> {
        Ok(self.read_u8()? != 0)
    }

    /// Reads a length-prefixed UTF-8 string.
    ///
    /// The wire format is a big-endian `u16` length followed by that many
    /// UTF-8 bytes. Returns a [`ProtocolError::InvalidString`] if the bytes
    /// are not valid UTF-8.
    pub fn read_string(&mut self) -> Result<String, SiriusError> {
        let len = self.read_u16()? as usize;
        self.need(len)?;

        let offset = self.buf.len() - self.remaining() + 2;
        let slice = self.buf.split_to(len);

        String::from_utf8(slice.to_vec()).map_err(|_| {
            SiriusError::Protocol(ProtocolError::InvalidString {
                header_id: self.header_id,
                offset,
            })
        })
    }

    /// Reads exactly `n` raw bytes as a [`Bytes`] slice.
    ///
    /// Zero-copy! The returned `Bytes` shares the same underlying allocation
    /// as the reader's buffer.
    pub fn read_bytes(&mut self, n: usize) -> Result<Bytes, SiriusError> {
        self.need(n)?;
        Ok(self.buf.split_to(n))
    }

    /// Returns an error if fewer than `n` bytes remain.
    #[inline]
    fn need(&self, n: usize) -> Result<(), SiriusError> {
        if self.buf.remaining() < n {
            Err(SiriusError::Protocol(ProtocolError::PacketTooShort {
                header_id: self.header_id,
                expected: n,
                got: self.buf.remaining(),
            }))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reader(data: &[u8]) -> PacketReader {
        PacketReader::new(Bytes::copy_from_slice(data))
    }

    #[test]
    fn read_i32() {
        let mut r = reader(&42i32.to_be_bytes());
        assert_eq!(r.read_i32().unwrap(), 42);
        assert!(r.is_empty());
    }

    #[test]
    fn read_bool() {
        let mut r = reader(&[0, 1, 255]);
        assert!(!r.read_bool().unwrap());
        assert!(r.read_bool().unwrap());
        assert!(r.read_bool().unwrap());
    }

    #[test]
    fn read_string() {
        // "hi" encoded as u16 length (2) + UTF-8 bytes
        let mut data = vec![0u8, 2, b'h', b'i'];
        let mut r = reader(&data);
        assert_eq!(r.read_string().unwrap(), "hi");

        // Empty string
        data = vec![0, 0];
        let mut r = reader(&data);
        assert_eq!(r.read_string().unwrap(), "");
    }

    #[test]
    fn read_past_end_returns_error() {
        let mut r = reader(&[0u8, 0, 0]); // only 3 bytes
        assert!(r.read_i32().is_err());
    }

    #[test]
    fn read_bytes_zero_copy() {
        let original = Bytes::from_static(b"abcdef");
        let mut r = PacketReader::new(original.clone());
        let slice = r.read_bytes(3).unwrap();
        assert_eq!(&slice[..], b"abc");
        assert_eq!(r.remaining(), 3);
    }
}
