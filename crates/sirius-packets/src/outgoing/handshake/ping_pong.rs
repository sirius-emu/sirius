use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct PingComposer;

impl OutgoingPacket for PingComposer {
    const HEADER_ID: u16 = 3928;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        Ok(RawPacket::empty(Self::HEADER_ID))
    }
}

/// Sent in response to a client-initiated `PingEvent` (295).
///
/// The client sends a `PingEvent` with an `i32` value; the server echoes
/// that same value back in this composer.
#[derive(Debug, Clone)]
pub struct PongComposer {
    /// The value received in the client's `PingEvent`. Echoed back as-is.
    pub id: i32,
}

impl PongComposer {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

impl OutgoingPacket for PongComposer {
    const HEADER_ID: u16 = 10;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::with_capacity(Self::HEADER_ID, 4);
        w.write_i32(self.id);

        w.finish_ok()
    }
}
