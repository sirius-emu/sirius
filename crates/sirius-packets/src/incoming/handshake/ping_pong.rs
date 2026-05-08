use crate::{IncomingPacket, PacketReader};
use sirius_error::SiriusError;

#[derive(Debug, Clone)]
pub struct PongPacket;

impl IncomingPacket for PongPacket {
    const HEADER_ID: u16 = 2596;

    fn parse(_reader: &mut PacketReader) -> Result<Self, SiriusError> {
        Ok(Self)
    }
}

#[derive(Debug, Clone)]
pub struct PingPacket {
    /// Arbitrary integer chosen by the client. Must be echoed back in `PongComposer`.
    pub id: i32,
}

impl IncomingPacket for PingPacket {
    const HEADER_ID: u16 = 295;

    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError> {
        let id = reader.read_i32()?;
        Ok(Self { id })
    }
}
