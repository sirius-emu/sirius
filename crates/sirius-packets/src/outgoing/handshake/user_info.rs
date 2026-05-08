use crate::prelude::*;

use sirius_repository::models::User;

#[derive(Debug, Clone)]
pub struct UserInfoComposer {
    pub user: User,
}

impl UserInfoComposer {
    #[allow(clippy::too_many_arguments)]
    pub fn new(user: User) -> Self {
        Self { user }
    }
}

impl OutgoingPacket for UserInfoComposer {
    const HEADER_ID: u16 = 2725;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(self.user.id.0)
            .write_string(&self.user.username)
            .write_string(&self.user.look)
            .write_string(&self.user.gender.to_string())
            .write_string(&self.user.motto)
            .write_string(&self.user.username)
            .write_bool(false) // direct mail
            .write_i32(self.user.stats.respects_received)
            .write_i32(self.user.stats.respects_remaining)
            .write_i32(self.user.stats.respects_pet_remaining)
            .write_bool(false)
            .write_string("01-01-1970 00:00:00")
            .write_bool(self.user.settings.can_change_name)
            .write_bool(self.user.settings.safety_locked);

        w.finish_ok()
    }
}
