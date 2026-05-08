use crate::prelude::*;

/// Sent to the client to indicate the hotel's current availability status.
#[derive(Debug, Clone)]
pub struct AvailabilityStatusComposer {
    /// Indicates if the hotel is currently open and accepting players.
    pub is_open: bool,

    /// Indicates if the server is in the process of a graceful shutdown.
    pub is_shutting_down: bool,

    /// Indicates if the server recognizes the client as an authentic Habbo client.
    pub is_authentic_habbo: bool,
}

impl AvailabilityStatusComposer {
    pub fn new(
        is_open: bool,
        is_shutting_down: bool,
        is_authentic_habbo: bool,
    ) -> Self {
        Self {
            is_open,
            is_shutting_down,
            is_authentic_habbo,
        }
    }
}

impl OutgoingPacket for AvailabilityStatusComposer {
    const HEADER_ID: u16 = 2033;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::with_capacity(Self::HEADER_ID, 3);

        w.write_bool(self.is_open)
            .write_bool(self.is_shutting_down)
            .write_bool(self.is_authentic_habbo);

        w.finish_ok()
    }
}
