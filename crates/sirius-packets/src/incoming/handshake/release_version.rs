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
