use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UserCurrencyComposer {
    pub pixels: i32,
    pub diamonds: i32,
}

impl UserCurrencyComposer {
    pub fn new(pixels: i32, diamonds: i32) -> Self {
        Self { pixels, diamonds }
    }
}

impl OutgoingPacket for UserCurrencyComposer {
    const HEADER_ID: u16 = 2018;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(2)
            .write_i32(0)
            .write_i32(self.pixels)
            .write_i32(5)
            .write_i32(self.diamonds);

        w.finish_ok()
    }
}
