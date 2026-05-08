use crate::prelude::*;

/// First packet the Nitro client sends after establishing a TCP connection.
///
/// The client announces its release version string. The server does not need
/// to validate or act on this value, it is informational only.
pub struct ReleaseVersionPacket {
    pub release_version: String,
}

impl IncomingPacket for ReleaseVersionPacket {
    const HEADER_ID: u16 = 4000;

    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError> {
        let release_version = reader.read_string()?;
        Ok(Self { release_version })
    }
}
