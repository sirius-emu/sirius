use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorLiftedRoomsComposer;

impl NavigatorLiftedRoomsComposer {
    pub fn new() -> Self {
        Self
    }
}

impl OutgoingPacket for NavigatorLiftedRoomsComposer {
    const HEADER_ID: u16 = 3104;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(0);

        w.finish_ok()
    }
}
