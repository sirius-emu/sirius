use crate::prelude::*;

use sirius_repository::models::UserSettings;

#[derive(Debug, Clone)]
pub struct UserSettingsComposer {
    pub settings: UserSettings,
}

impl UserSettingsComposer {
    pub fn new(settings: UserSettings) -> Self {
        Self { settings }
    }
}

impl OutgoingPacket for UserSettingsComposer {
    const HEADER_ID: u16 = 513;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_i32(self.settings.volume_system)
            .write_i32(self.settings.volume_furni)
            .write_i32(self.settings.volume_trax)
            .write_bool(self.settings.old_chat)
            .write_bool(self.settings.room_invites)
            .write_bool(self.settings.camera_follow)
            .write_i32(0) // TODO: flags
            .write_i32(self.settings.chat_type);

        w.finish_ok()
    }
}
