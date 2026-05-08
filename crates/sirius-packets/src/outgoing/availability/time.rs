use crate::prelude::*;

/// Sent to the client to indicate the hotel's current availability status.
#[derive(Debug, Clone)]
pub struct AvailabilityTimeComposer {
    /// Indicates if the hotel is currently open and accepting players.
    pub is_open: bool,

    /// The number of minutes until the current availability status flips.
    pub minutes_until_change: i32,
}

impl AvailabilityTimeComposer {
    pub fn new(is_open: bool, minutes_until_change: i32) -> Self {
        Self {
            is_open,
            minutes_until_change,
        }
    }
}

impl OutgoingPacket for AvailabilityTimeComposer {
    const HEADER_ID: u16 = 600;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_bool(self.is_open)
            .write_i32(self.minutes_until_change);

        w.finish_ok()
    }
}
