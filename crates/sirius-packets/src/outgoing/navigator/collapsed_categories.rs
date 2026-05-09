use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorCollapsedCategoriesComposer;

impl NavigatorCollapsedCategoriesComposer {
    pub fn new() -> Self {
        Self
    }
}

impl OutgoingPacket for NavigatorCollapsedCategoriesComposer {
    const HEADER_ID: u16 = 1543;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(0);

        w.finish_ok()
    }
}
