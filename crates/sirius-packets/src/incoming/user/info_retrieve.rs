use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UserInfoRetrievePacket {}

impl IncomingPacket for UserInfoRetrievePacket {
    const HEADER_ID: u16 = 357;

    fn parse(_reader: &mut PacketReader) -> Result<Self, SiriusError> {
        Ok(Self {})
    }
}
