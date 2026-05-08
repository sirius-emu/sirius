use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UserCreditsComposer {
    pub credits: i32,
}

impl UserCreditsComposer {
    pub fn new(credits: i32) -> Self {
        Self { credits }
    }
}

impl OutgoingPacket for UserCreditsComposer {
    const HEADER_ID: u16 = 3475;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_string(&self.credits.to_string());

        w.finish_ok()
    }
}
