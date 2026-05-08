use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HotelClosedAndOpensComposer {
    pub open_hour: i32,

    pub open_minute: i32,
}

impl HotelClosedAndOpensComposer {
    pub fn new(open_hour: i32, open_minute: i32) -> Self {
        Self {
            open_hour,
            open_minute,
        }
    }
}

impl OutgoingPacket for HotelClosedAndOpensComposer {
    const HEADER_ID: u16 = 3728;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(self.open_hour).write_i32(self.open_minute);

        w.finish_ok()
    }
}
