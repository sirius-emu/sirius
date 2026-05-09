use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorSavedSearches;

impl NavigatorSavedSearches {
    pub fn new() -> Self {
        Self
    }
}

impl OutgoingPacket for NavigatorSavedSearches {
    const HEADER_ID: u16 = 3984;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(0);

        w.finish_ok()
    }
}
