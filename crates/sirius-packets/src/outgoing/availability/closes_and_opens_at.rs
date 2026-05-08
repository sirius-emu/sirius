use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HotelClosesAndOpensAtComposer {
    pub open_hour: i32,

    pub open_minute: i32,

    pub user_thrown_out_at_close: bool,
}

impl HotelClosesAndOpensAtComposer {
    pub fn new(
        open_hour: i32,
        open_minute: i32,
        user_thrown_out_at_close: bool,
    ) -> Self {
        Self {
            open_hour,
            open_minute,
            user_thrown_out_at_close,
        }
    }
}

impl OutgoingPacket for HotelClosesAndOpensAtComposer {
    const HEADER_ID: u16 = 2771;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(self.open_hour)
            .write_i32(self.open_minute)
            .write_bool(self.user_thrown_out_at_close);

        w.finish_ok()
    }
}
