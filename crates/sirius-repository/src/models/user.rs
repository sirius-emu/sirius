use chrono::{DateTime, Utc};
use sirius_types::{CurrencyType, Gender, RoomId, UserId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UserStats {
    pub respects_received: i32,
    pub respects_remaining: i32,
    pub respects_pet_remaining: i32,
}

#[derive(Debug, Clone)]
pub struct UserSettings {
    pub can_change_name: bool,
    pub safety_locked: bool,
    pub volume_system: i32,
    pub volume_furni: i32,
    pub volume_trax: i32,
    pub old_chat: bool,
    pub room_invites: bool,
    pub camera_follow: bool,
    pub chat_type: i32,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub motto: String,
    pub look: String,
    pub gender: Gender,
    pub rank: i32,
    pub credits: i32,
    pub currencies: HashMap<CurrencyType, i32>,
    pub home_room: Option<RoomId>,
    pub account_created: DateTime<Utc>,
    pub last_online: DateTime<Utc>,
    pub current_ip: String,
    pub machine_id: String,
    pub stats: UserStats,
    pub settings: UserSettings,
}
