use crate::prelude::*;
use sirius_packets::{
    incoming::navigator::NavigatorInitPacket,
    outgoing::navigator::{
        NavigatorMetaDataComposer, NavigatorSettingsComposer,
    },
};

pub struct NavigatorInitHandler;

impl PacketHandler for NavigatorInitHandler {
    const HEADER_ID: u16 = NavigatorInitPacket::HEADER_ID;

    async fn handle(
        &self,
        _raw: RawPacket,
        ctx: HandlerContext,
    ) -> Result<(), SiriusError> {
        ctx.compose(&NavigatorMetaDataComposer::new()).await?;

        ctx.compose(&NavigatorSettingsComposer::new()).await?;

        Ok(())
    }
}
