use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorSettingsComposer;

impl NavigatorSettingsComposer {
    pub fn new() -> Self {
        Self
    }
}

impl OutgoingPacket for NavigatorSettingsComposer {
    const HEADER_ID: u16 = 518;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(100)
            .write_i32(100)
            .write_i32(425)
            .write_i32(535)
            .write_bool(false)
            .write_i32(0);

        w.finish_ok()
    }
}
