use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorEventCategoriesComposer;

impl NavigatorEventCategoriesComposer {
    pub fn new() -> Self {
        Self
    }
}

impl OutgoingPacket for NavigatorEventCategoriesComposer {
    const HEADER_ID: u16 = 3244;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(2)
            .write_i32(1)
            .write_string("Parties")
            .write_bool(true)
            .write_i32(2)
            .write_string("Trade")
            .write_bool(true);

        w.finish_ok()
    }
}
