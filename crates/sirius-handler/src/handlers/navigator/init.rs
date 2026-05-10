use crate::prelude::*;
use sirius_packets::{
    incoming::NavigatorInitPacket,
    outgoing::{
        NavigatorCollapsedCategoriesComposer, NavigatorEventCategoriesComposer,
        NavigatorLiftedRoomsComposer, NavigatorMetaDataComposer,
        NavigatorSavedSearches, NavigatorSettingsComposer,
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
        ctx.compose(&NavigatorEventCategoriesComposer::new())
            .await?;
        ctx.compose(&NavigatorCollapsedCategoriesComposer::new())
            .await?;
        ctx.compose(&NavigatorSavedSearches::new()).await?;
        ctx.compose(&NavigatorLiftedRoomsComposer::new()).await?;

        Ok(())
    }
}
