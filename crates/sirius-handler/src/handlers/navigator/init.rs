use crate::prelude::*;
use sirius_packets::{
    incoming::navigator::NavigatorInitPacket,
    outgoing::navigator::NavigatorMetaDataComposer,
};

pub struct NavigatorInitHandler;

impl PacketHandler for NavigatorInitHandler {
    const HEADER_ID: u16 = NavigatorInitPacket::HEADER_ID;

    async fn handle(
        &self,
        raw: RawPacket,
        ctx: HandlerContext,
    ) -> Result<(), SiriusError> {
        ctx.compose(&NavigatorMetaDataComposer::new()).await?;

        Ok(())
    }
}
