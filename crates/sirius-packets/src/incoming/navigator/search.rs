use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NavigatorSearchPacket {
    pub view: String,
    pub query: String,
}

impl IncomingPacket for NavigatorSearchPacket {
    const HEADER_ID: u16 = 249;

    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError> {
        let view = reader.read_string()?;
        let query = reader.read_string()?;

        tracing::debug!(view, query);

        Ok(Self { view, query })
    }
}
