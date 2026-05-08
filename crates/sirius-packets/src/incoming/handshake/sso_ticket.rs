use crate::prelude::*;

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
