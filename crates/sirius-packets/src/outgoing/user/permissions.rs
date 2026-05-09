use crate::prelude::*;
use tracing::info;

#[derive(Debug, Clone)]
pub struct UserPermissionsComposer {
    pub club_level: i32,
    pub rank_level: i32,
    pub is_ambassador: bool,
}

impl UserPermissionsComposer {
    pub fn new(club_level: i32, rank_level: i32, is_ambassador: bool) -> Self {
        Self {
            club_level,
            rank_level,
            is_ambassador,
        }
    }
}
impl OutgoingPacket for UserPermissionsComposer {
    const HEADER_ID: u16 = 411;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::with_capacity(Self::HEADER_ID, 9);
        w.write_i32(self.club_level)
            .write_i32(self.rank_level)
            .write_bool(self.is_ambassador);
        w.finish_ok()
    }
}
