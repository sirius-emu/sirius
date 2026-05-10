use crate::prelude::*;
use sirius_packets::incoming::UserCurrencyPacket;

pub struct UserCurrencyHandler;

impl PacketHandler for UserCurrencyHandler {
    const HEADER_ID: u16 = UserCurrencyPacket::HEADER_ID;

    async fn handle(
        &self,
        _raw: RawPacket,
        ctx: HandlerContext,
    ) -> Result<(), SiriusError> {
        ctx.user_actor.send(UserCommand::GetCurrency).await?;

        Ok(())
    }
}
