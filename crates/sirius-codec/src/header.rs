//! Packet header - the 6-byte prefix every Nitro packet starts with

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader {
    /// Total byte length of what follows this 4-byte length field.
    ///
    /// Includes the 2-byte header ID. A packet with an empty body has
    /// `length = 2`. The body is therefore `length - 2` bytes long.
    pub length: u32,

    /// The packet's header ID.
    ///
    /// Used by `sirius-packets` to dispatch the packet to the correct
    /// incoming handler. Unknown IDs should be logged at debug level
    /// and silently dropped.
    pub id: u16,
}

impl PacketHeader {
    pub const LENGTH_FIELD_SIZE: usize = 4;

    pub const ID_FIELD_SIZE: usize = 2;

    pub const SIZE: usize = Self::LENGTH_FIELD_SIZE + Self::ID_FIELD_SIZE;

    /// The minimum valid value for the `length` field.
    ///
    /// The length field counts the header ID and body together. A packet
    /// with no body still has an ID, so the minimum is 2.
    pub const MIN_LENGTH: u32 = 2;

    /// Returns the number of body bytes this packet contains.
    ///
    /// This is `length - 2`. Callers can rely on this being non-negative
    /// because the decoder rejects packets where `length < MIN_LENGTH`.
    #[must_use]
    pub fn body_len(self) -> usize {
        (self.length - Self::MIN_LENGTH) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_len_empty_packet() {
        let header = PacketHeader { length: 2, id: 0 };
        assert_eq!(header.body_len(), 0);
    }

    #[test]
    fn body_len_with_body() {
        // length = 2 (id) + 10 (body)
        let header = PacketHeader {
            length: 12,
            id: 4000,
        };
        assert_eq!(header.body_len(), 10);
    }

    #[test]
    fn header_size_is_six_bytes() {
        assert_eq!(PacketHeader::SIZE, 6);
    }
}
