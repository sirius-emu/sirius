use sirius_types::{RoomCategory, RoomId, RoomLockType, UserId};

/// A lightweight snapshot of a room used exclusively for display in the Navigator.
#[derive(Debug, Clone)]
pub struct RoomDisplayNode {
    pub room_id: RoomId,
    pub name: String,
    pub owner_id: UserId,
    pub owner_name: String,
    pub description: String,
    pub max_users: i32,
    pub category: RoomCategory,
    pub lock_type: RoomLockType,

    pub current_users: i32,
}

impl RoomDisplayNode {
    /// Determines if the room is currently full.
    #[inline]
    pub fn is_full(&self) -> bool {
        self.current_users >= self.max_users
    }
}
