use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorInitPacket {}

impl IncomingPacket for NavigatorInitPacket {
    const HEADER_ID: u16 = 2110;

    fn parse(_reader: &mut PacketReader) -> Result<Self, SiriusError> {
        Ok(Self {})
    }
}
