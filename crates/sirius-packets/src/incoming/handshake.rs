//! Handshake-phase incoming packets.
//!
//! These are the packets the client sends before authentication is complete.
//! They are received in the `Unauthenticated` and `Authenticating` session
//! states. None of them require an active session.

use crate::{IncomingPacket, PacketReader};
use sirius_error::SiriusError;

/// First packet the Nitro client sends after establishing a TCP connection.
///
/// The client announces its release version string. The server does not need
/// to validate or act on this value, it is informational only.
///
/// Header ID: `4000`
///
/// Wire format:
/// ```text
/// [u16 len][utf-8 bytes]   ← release version string
/// ```
pub struct ReleaseVersionPacket {
    /// The client's self-reported release version string.
    pub release_version: String,
}

impl IncomingPacket for ReleaseVersionPacket {
    const HEADER_ID: u16 = 4000;

    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError> {
        let release_version = reader.read_string()?;
        Ok(Self { release_version })
    }
}

/// Carries the SSO authentication ticket issued by the hotel website.
///
/// After parsing, pass the `ticket` field to the auth subsystem for validation
#[derive(Debug, Clone)]
pub struct SsoTicketPacket {
    /// The raw SSO ticket string.
    pub ticket: String,
}

impl IncomingPacket for SsoTicketPacket {
    const HEADER_ID: u16 = 2419;

    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError> {
        let ticket = reader.read_string()?;
        Ok(Self { ticket })
    }
}

#[derive(Debug, Clone)]
pub struct PongPacket;

impl IncomingPacket for PongPacket {
    const HEADER_ID: u16 = 2596;

    fn parse(_reader: &mut PacketReader) -> Result<Self, SiriusError> {
        Ok(Self)
    }
}

#[derive(Debug, Clone)]
pub struct PingPacket {
    /// Arbitrary integer chosen by the client. Must be echoed back in `PongComposer`.
    pub id: i32,
}

impl IncomingPacket for PingPacket {
    const HEADER_ID: u16 = 295;

    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError> {
        let id = reader.read_i32()?;
        Ok(Self { id })
    }
}
