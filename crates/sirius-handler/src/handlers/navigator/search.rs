use crate::prelude::*;
use sirius_packets::navigator::NavigatorSearchPacket;
use sirius_packets::outgoing::navigator::{
    NavigatorSearchResultsComposer, SearchResultBlock,
};

pub struct NavigatorSearchHandler;

impl PacketHandler for NavigatorSearchHandler {
    const HEADER_ID: u16 = NavigatorSearchPacket::HEADER_ID;

    async fn handle(
        &self,
        raw: RawPacket,
        ctx: HandlerContext,
    ) -> Result<(), SiriusError> {
        let packet = NavigatorSearchPacket::from_raw(raw)?;

        let nav_blocks = ctx
            .navigator
            .get_search_results(&packet.view, &packet.query);

        let packet_blocks: Vec<SearchResultBlock> = nav_blocks
            .into_iter()
            .map(|block| SearchResultBlock {
                search_code: block.search_code,
                text: block.text,
                action: block.action,
                is_closed: block.is_closed,
                view_mode: block.view_mode,
                rooms: block.rooms,
            })
            .collect();

        ctx.compose(&NavigatorSearchResultsComposer::new(
            packet.view,
            packet.query,
            packet_blocks,
        ))
        .await?;

        Ok(())
    }
}
