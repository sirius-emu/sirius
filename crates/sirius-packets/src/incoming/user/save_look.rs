pub use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct UserSaveLookPacket {
    pub gender: String,
    pub look: String,
}

impl IncomingPacket for UserSaveLookPacket {
    const HEADER_ID: u16 = 2730;

    fn parse(reader: &mut PacketReader) -> Result<Self, SiriusError> {
        let gender = reader.read_string()?;
        let look = reader.read_string()?;
        Ok(Self { gender, look })
    }
}
