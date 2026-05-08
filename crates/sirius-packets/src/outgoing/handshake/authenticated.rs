use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct AuthenticatedComposer;

impl OutgoingPacket for AuthenticatedComposer {
    const HEADER_ID: u16 = 2491;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        Ok(RawPacket::empty(Self::HEADER_ID))
    }
}
