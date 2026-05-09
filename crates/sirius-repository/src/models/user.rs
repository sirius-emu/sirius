use chrono::{DateTime, Utc};
use sirius_types::{CurrencyType, Gender, RoomId, UserId};
use std::collections::HashMap;

/// Respect-related statistics for a user.
///
/// Counters are reset daily by the housekeeping job.
#[derive(Debug, Clone)]
pub struct UserStats {
    pub respects_received: i32,
    pub respects_remaining: i32,
    pub respects_pet_remaining: i32,
}

/// Per-user client preferences stored server-side.
#[derive(Debug, Clone)]
pub struct UserSettings {
    /// Whether the user is allowed to change their username.
    pub can_change_name: bool,
    /// Whether the account safety lock is active.
    pub safety_locked: bool,
    /// System sound volume (0-100).
    pub volume_system: i32,
    /// Furniture sound volume (0-100).
    pub volume_furni: i32,
    /// Trax music volume (0-100).
    pub volume_trax: i32,
    /// Whether the user prefers the old chat style.
    pub old_chat: bool,
    /// Whether the user accepts room invites from friends.
    pub room_invites: bool,
    /// Whether the camera follows the user's avatar in a room.
    pub camera_follow: bool,
    /// Chat bubble style ID choosen by the user.
    pub chat_type: i32,
}

/// A fully loaded Habbo user.
///
/// Constructed by [`UserRepository::find_by_auth_ticket`] and held by
/// [`UserActor`] for the duration of the session.
#[derive(Debug, Clone)]
pub struct User {
    /// Unique numeric identifier.
    pub id: UserId,
    /// The user's display name.
    pub username: String,
    /// The user's profile motto.
    pub motto: String,
    /// Habbo figure string.
    pub look: String,
    /// The user's avatar gender.
    pub gender: Gender,
    /// Rank ID, used to look up permissions in [`PermissionsManager`].
    pub rank: i32,
    /// Credit balance.
    pub credits: i32,
    /// Alternative currency balances (pixels, diamonds, seasonal).
    /// Credits are not included here.
    pub currencies: HashMap<CurrencyType, i32>,
    /// The home room user is sent to on login, if set.
    pub home_room: Option<RoomId>,
    /// When the account was created.
    pub account_created: DateTime<Utc>,
    /// When the user was last seen online.
    pub last_online: DateTime<Utc>,
    /// The IP address of the most recent connection.
    pub current_ip: String,
    /// The machine identifier of the most recent connection.
    pub machine_id: String,
    /// Respect counters for this user.
    pub stats: UserStats,
    /// Client preferences for this user.
    pub settings: UserSettings,
}
