//! Errors originating in the codec, packet, and composer layers.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    /// The packet header ID is not mapped to any known incoming packet.
    ///
    /// This is normal when a client sends packets for features the server
    /// doesn't implement yet. Log at debug level and move on.
    #[error("unknown packet header: {header_id}")]
    UnknownHeader { header_id: u16 },

    /// The packet body is shorter than the structure requires.
    ///
    /// Either the client sent a malformed packet or there's a mismatch between
    /// the codec and the packet definition.
    #[error("packet {header_id} is too short: expected at least {expected} bytes, got {got}")]
    PacketTooShort {
        header_id: u16,
        expected: usize,
        got: usize,
    },

    /// A string field in the packet is not valid UTF-8.
    #[error("packet {header_id} contains invalid UTF-8 at byte offset {offset}")]
    InvalidString { header_id: u16, offset: usize },

    /// A field value is outside the range of valid values.
    ///
    /// For example, a direction byte of 12 when only 0–7 are valid.
    #[error("packet {header_id} field '{field}' has invalid value: {value}")]
    InvalidFieldValue {
        header_id: u16,
        field: &'static str,
        value: String,
    },

    /// The declared packet length in the header does not match the actual
    /// number of bytes available in the frame.
    #[error("packet length mismatch: header declares {declared}, frame contains {actual}")]
    LengthMismatch { declared: u32, actual: usize },

    /// A packet could not be encoded for sending.
    #[error("failed to encode packet {header_id}: {reason}")]
    EncodingFailed { header_id: u16, reason: String },
}
