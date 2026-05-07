//! The `RawPacket` type. A fully framed but uninterpreted Nitro packet.

use crate::header::PacketHeader;
use bytes::Bytes;

/// A fully framed Nitro packet.
///
/// `RawPacket` holds a decoded header and the packet body as raw bytes.
///
/// It makes no attempt to interpret the body. That's `sirius-packets`' job.
/// This type is the boundary between the codec layer and the protocol layer.
///
/// The body is [`Bytes`], which is a reference-counted slice. Cloning it is cheap,
/// it does not copy the underlying memory.
#[derive(Debug, Clone)]
pub struct RawPacket {
    /// The decoded packet header.
    pub header: PacketHeader,

    /// The packet body, excluding the header.
    ///
    /// `body.len() == header.body_len()` is always true for packets produced
    /// by [`NitroDecoder`]. Packets constructed manually must uphold this invariant
    /// themsleves.
    ///
    /// [`NitroDecoder`]: crate::NitroDecoder
    pub body: Bytes,
}

impl RawPacket {
    /// Constructs a `RawPacket` from a header ID and a body.
    ///
    /// The `length` field in the header is computed automatically.
    ///
    /// # Panics
    ///
    /// Panics if `body.len()` exceeds `u32::MAX - 2`. In practice, the decoder
    /// enforces `MAX_BODY_LEN = 65_535`, so this can only be triggered by
    /// hand-constructed packets with unreasonably large bodies.
    pub fn new(id: u16, body: Bytes) -> Self {
        let body_len = u32::try_from(body.len()).unwrap_or_else(|_| {
            panic!("RawPacket body too large: {} bytes", body.len())
        });
        let length = PacketHeader::MIN_LENGTH + body_len;
        Self {
            header: PacketHeader { length, id },
            body,
        }
    }

    /// Constructs a `RawPacket` with an empty body.
    #[must_use]
    pub fn empty(id: u16) -> Self {
        Self::new(id, Bytes::new())
    }

    #[inline]
    pub fn id(&self) -> u16 {
        self.header.id
    }

    /// Returns the total wire length of this packet in bytes.
    ///
    /// This includes the 4-byte length prefix, the 2-byte header ID, and the body.
    #[inline]
    pub fn wire_len(&self) -> usize {
        PacketHeader::LENGTH_FIELD_SIZE + self.header.length as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_packet_wire_len() {
        let p = RawPacket::empty(4000);
        // 4 (length field) + 2 (id) + 0 (body) = 6
        assert_eq!(p.wire_len(), 6);
        assert_eq!(p.header.length, 2);
        assert!(p.body.is_empty());
    }

    #[test]
    fn packet_with_body_wire_len() {
        let body = Bytes::from_static(b"hello");
        let p = RawPacket::new(1000, body);
        // 4 + 2 + 5 = 11
        assert_eq!(p.wire_len(), 11);
        assert_eq!(p.header.length, 7);
    }

    #[test]
    fn id_shorthand() {
        let p = RawPacket::empty(42);
        assert_eq!(p.id(), 42);
    }
}
