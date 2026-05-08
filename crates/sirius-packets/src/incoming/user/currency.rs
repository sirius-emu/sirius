use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UserCurrencyPacket {}

impl IncomingPacket for UserCurrencyPacket {
    const HEADER_ID: u16 = 273;

    fn parse(_reader: &mut PacketReader) -> Result<Self, SiriusError> {
        Ok(Self {})
    }
}
