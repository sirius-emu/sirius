use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HotelWillCloseInMinutes {
    pub minutes: i32,
}

impl HotelWillCloseInMinutes {
    pub fn new(minutes: i32) -> Self {
        Self { minutes }
    }
}

impl OutgoingPacket for HotelWillCloseInMinutes {
    const HEADER_ID: u16 = 1050;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(self.minutes);

        w.finish_ok()
    }
}
