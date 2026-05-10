use crate::prelude::*;
use sirius_repository::models::SearchResultBlock;

#[derive(Debug, Clone)]
pub struct NavigatorSearchResultsComposer {
    pub view: String,
    pub query: String,
    pub blocks: Vec<SearchResultBlock>,
}

impl NavigatorSearchResultsComposer {
    pub fn new(
        view: String,
        query: String,
        blocks: Vec<SearchResultBlock>,
    ) -> Self {
        Self {
            view,
            query,
            blocks,
        }
    }
}

impl OutgoingPacket for NavigatorSearchResultsComposer {
    const HEADER_ID: u16 = 2690;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_string(&self.view)
            .write_string(&self.query)
            .write_i32(self.blocks.len() as i32);

        for block in &self.blocks {
            w.write_string(&block.search_code)
                .write_string(&block.text)
                .write_i32(block.action)
                .write_bool(block.is_closed)
                .write_i32(block.view_mode)
                .write_i32(block.rooms.len() as i32);

            for room in &block.rooms {
                w.write_i32(room.id.into())
                    .write_string(&room.name)
                    .write_i32(room.owner_id.into())
                    .write_string(&room.owner_name)
                    .write_i32(room.lock_type as i32)
                    .write_i32(0)
                    .write_i32(room.max_users)
                    .write_string(&room.description)
                    .write_i32(0)
                    .write_i32(0)
                    .write_i32(0)
                    .write_i32(room.category.into())
                    .write_i32(0)
                    .write_i32(0);
            }
        }

        w.finish_ok()
    }
}
