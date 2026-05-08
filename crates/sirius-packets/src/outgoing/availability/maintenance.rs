use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MaintenanceStatusComposer {
    pub is_in_maintenance: bool,

    pub minutes_until_maintenance: i32,

    pub duration: i32,
}

impl MaintenanceStatusComposer {
    pub fn new(
        is_in_maintenance: bool,
        minutes_until_maintenance: i32,
        duration: i32,
    ) -> Self {
        Self {
            is_in_maintenance,
            minutes_until_maintenance,
            duration,
        }
    }
}

impl OutgoingPacket for MaintenanceStatusComposer {
    const HEADER_ID: u16 = 1350;

    fn serialize(&self) -> Result<RawPacket, SiriusError> {
        let mut w = PacketWriter::new(Self::HEADER_ID);

        w.write_bool(self.is_in_maintenance)
            .write_i32(self.minutes_until_maintenance)
            .write_i32(self.duration);

        w.finish_ok()
    }
}
