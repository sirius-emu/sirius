use chrono::{DateTime, Utc};
use sirius_types::{CurrencyType, Gender, RoomId, UserId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub motto: String,
    pub look: String,
    pub gender: Gender,

    pub rank: i32,

    pub credits: i32,

    pub currencies: HashMap<CurrencyType, i64>,

    pub home_room: Option<RoomId>,

    pub account_created: DateTime<Utc>,
    pub last_online: DateTime<Utc>,

    pub current_ip: String,

    pub machine_id: String,
}
