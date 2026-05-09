use std::collections::HashMap;

use crate::{Permission, PermissionSetting};

#[derive(Debug, Clone)]
pub struct Rank {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub badge: String,
    pub prefix: String,
    pub prefix_color: String,
    pub room_effect: i32,
    pub log_commands: bool,
    pub auto_credits_amount: i32,
    pub auto_pixels_amount: i32,
    pub auto_points_amount: i32,
    permissions: HashMap<String, PermissionSetting>,
}

impl Rank {
    pub(crate) fn new(
        id: i32,
        name: String,
        level: i32,
        badge: String,
        prefix: String,
        prefix_color: String,
        room_effect: i32,
        log_commands: bool,
        auto_credits_amount: i32,
        auto_pixels_amount: i32,
        auto_points_amount: i32,
        permissions: HashMap<String, PermissionSetting>,
    ) -> Self {
        Self {
            id,
            name,
            level,
            badge,
            prefix,
            prefix_color,
            room_effect,
            log_commands,
            auto_credits_amount,
            auto_pixels_amount,
            auto_points_amount,
            permissions,
        }
    }

    #[inline]
    pub fn has(&self, permission: Permission, is_room_owner: bool) -> bool {
        self.has_str(permission.as_str(), is_room_owner)
    }

    #[inline]
    pub fn has_str(&self, key: &str, is_room_owner: bool) -> bool {
        self.permissions
            .get(key)
            .copied()
            .unwrap_or(PermissionSetting::Disallowed)
            .is_granted(is_room_owner)
    }

    #[inline]
    pub fn setting(&self, permission: Permission) -> PermissionSetting {
        self.permissions
            .get(permission.as_str())
            .copied()
            .unwrap_or(PermissionSetting::Disallowed)
    }

    #[inline]
    pub fn is_ambassador(&self) -> bool {
        self.has(Permission::Ambassador, false)
    }

    #[inline]
    pub fn has_prefix(&self) -> bool {
        !self.prefix.is_empty()
    }
}
