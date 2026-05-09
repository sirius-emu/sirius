use chrono::Utc;
use sirius_types::{RoomCategory, RoomId, RoomLockType, UserId};

#[derive(Debug, Clone)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub owner_id: UserId,
    pub owner_name: String,
    pub description: String,
    pub password: String,
    pub max_users: i32,
    pub category: RoomCategory,
    pub lock_type: RoomLockType,
    pub created_at: chrono::DateTime<Utc>,
}
