//! Incoming packet definitions.
//!
//! Each submodule owns a slice of the incoming packet namespace. Add new
//! packets in the appropriate submodule, implement [`IncomingPacket`], then
//! register it here so call sites can import from `sirius_packets::incoming`.

pub mod handshake;
pub mod user;
