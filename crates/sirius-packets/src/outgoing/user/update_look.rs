use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UpdateUserLookComposer {
    pub gender: String,
    pub look: String,
}

impl UpdateUserLookComposer {
    pub fn new(gender: String, look: String) -> Self {
        Self { gender, look }
    }
}

impl OutgoingPacket for UpdateUserLookComposer {
    const HEADER_ID: u16 = 2429;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);
        w.write_string(&self.look).write_string(&self.gender);
        w.finish_ok()
    }
}
