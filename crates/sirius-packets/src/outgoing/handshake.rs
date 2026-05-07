//! Outgoing handshake packets.

use crate::OutgoingPacket;
use sirius_codec::RawPacket;
use sirius_error::SiriusError;

#[derive(Debug, Clone, Default)]
pub struct AuthOkComposer;

impl OutgoingPacket for AuthOkComposer {
    const HEADER_ID: u16 = 2491;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        Ok(RawPacket::empty(Self::HEADER_ID))
    }
}
