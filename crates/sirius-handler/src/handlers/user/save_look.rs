use crate::prelude::*;
use sirius_packets::user::UserSaveLookPacket;

pub struct UserSaveLookHandler;

impl PacketHandler for UserSaveLookHandler {
    const HEADER_ID: u16 = UserSaveLookPacket::HEADER_ID;

    async fn handle(
        &self,
        raw: RawPacket,
        ctx: HandlerContext,
    ) -> Result<(), SiriusError> {
        let packet = UserSaveLookPacket::from_raw(raw)?;

        ctx.user_actor
            .send(UserCommand::UpdateLook {
                gender: packet.gender,
                look: packet.look,
            })
            .await?;

        Ok(())
    }
}
