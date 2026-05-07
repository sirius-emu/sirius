//! Typed packet definitions for the Nitro protocol.
//!
//! This crate sits directly above `sirius-codec`. The codec layer gives us
//! [`RawPacket`]s. This crate gives those bytes meaning.
//!
//! # Incoming packets
//!
//! Incoming packets are parsed from a [`RawPacket`] using [`PacketReader`].
//! Each packet struct implements [`IncomingPacket`], which carries its header ID
//! as an associated constant so the dispatcher can route by ID at compile time.
//!
//! # Outgoing packets
//!
//! Outgoing packets implement [`OutgoingPacket`], which serializes the
//! struct into a [`RawPacket`] using [`PacketWriter`]. The result is handed
//! to `sirius-composer` for dispatch.
//!
//! # Adding a new packet
//!
//! **Incoming:**
//! 1. Add a struct in the appropriate `incoming/` submodule.
//! 2. Implement [`IncomingPacket`] with the correct `HEADER_ID`.
//! 3. Register it in `incoming/mod.rs` dipatch table.
//!
//! **Outgoing:**
//! 1. Add a struct in the appropriate `outgoing/` submodule.
//! 2. Implement [`OutgoingPacket`] with the correct `HEADER_ID`.
//!
//! [`RawPacket`]: sirius_codec::RawPacket

pub mod incoming;
pub mod outgoing;
pub mod reader;
pub mod writer;

pub use incoming::*;
pub use reader::PacketReader;
pub use writer::PacketWriter;

use sirius_codec::RawPacket;
use sirius_error::SiriusError;

/// A packet type that can be parsed from a [`RawPacket`].
pub trait IncomingPacket: Sized {
    /// The header ID that identifies this packet on the wire.
    const HEADER_ID: u16;

    /// Parses the packet body from the given reader.
    ///
    /// The reader's position is at the start of the body, the header has
    /// already been consumed by the codec. Read exactly as many bytes as
    /// the packet requires; leftover bytes are not an error but are a sign
    /// of a definition mismatch worth investigating.
    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError>;

    /// Convenience method, parses directly from a [`RawPacket`].
    ///
    /// Verifies that the header ID matches before attempting to parse.
    fn from_raw(packet: RawPacket) -> Result<Self, SiriusError> {
        use sirius_error::ProtocolError;

        if packet.header.id != Self::HEADER_ID {
            return Err(SiriusError::Protocol(ProtocolError::UnknownHeader {
                header_id: packet.header.id,
            }));
        }

        let mut reader = PacketReader::new(packet.body);
        Self::parse(&mut reader)
    }
}

/// A packet type that can be serialized into a [`RawPacket`].
pub trait OutgoingPacket {
    /// The header ID that identifies this packet on the wire.
    const HEADER_ID: u16;

    /// Serializes this packet into a [`RawPacket`].
    fn serialize(&self) -> Result<RawPacket, SiriusError>;

    /// Convenience method — writes through a [`PacketWriter`] and returns
    /// the finished [`RawPacket`].
    fn to_raw(&self) -> Result<RawPacket, SiriusError> {
        self.serialize()
    }
}
