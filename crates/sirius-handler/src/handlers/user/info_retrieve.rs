use crate::prelude::*;
use sirius_packets::incoming::UserInfoRetrievePacket;

pub struct UserInfoRetrieveHandler;

impl PacketHandler for UserInfoRetrieveHandler {
    const HEADER_ID: u16 = UserInfoRetrievePacket::HEADER_ID;

    async fn handle(
        &self,
        _raw: RawPacket,
        ctx: HandlerContext,
    ) -> Result<(), SiriusError> {
        ctx.user_actor.send(UserCommand::GetUserInfo).await?;

        Ok(())
    }
}
