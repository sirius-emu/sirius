use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorMetaDataComposer;

impl NavigatorMetaDataComposer {
    pub fn new() -> Self {
        Self
    }
}

impl OutgoingPacket for NavigatorMetaDataComposer {
    const HEADER_ID: u16 = 3052;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(4)
            .write_string("official_view")
            .write_i32(0)
            .write_string("hotel_view")
            .write_i32(0)
            .write_string("roomads_view")
            .write_i32(0)
            .write_string("myworld_view")
            .write_i32(0);

        w.finish_ok()
    }
}
