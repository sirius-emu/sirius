use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InfoRetrievePacket {}

impl IncomingPacket for InfoRetrievePacket {
    const HEADER_ID: u16 = 357;

    fn parse(_reader: &mut PacketReader) -> Result<Self, SiriusError> {
        Ok(Self {})
    }
}
