use crate::{Permission, PermissionSetting, Rank};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct RankTable {
    ranks: HashMap<i32, Arc<Rank>>,
}

impl RankTable {
    pub(crate) fn new(ranks: HashMap<i32, Arc<Rank>>) -> Self {
        Self { ranks }
    }

    #[inline]
    pub fn get(&self, rank_id: i32) -> Option<&Arc<Rank>> {
        self.ranks.get(&rank_id)
    }

    #[inline]
    pub fn has_permission(
        &self,
        rank_id: i32,
        permission: Permission,
        is_room_owner: bool,
    ) -> bool {
        self.ranks
            .get(&rank_id)
            .map(|r| r.has(permission, is_room_owner))
            .unwrap_or(false)
    }

    #[inline]
    pub fn has_permission_str(
        &self,
        rank_id: i32,
        key: &str,
        is_room_owner: bool,
    ) -> bool {
        self.ranks
            .get(&rank_id)
            .map(|r| r.has_str(key, is_room_owner))
            .unwrap_or(false)
    }

    #[inline]
    pub fn setting(
        &self,
        rank_id: i32,
        permission: Permission,
    ) -> PermissionSetting {
        self.ranks
            .get(&rank_id)
            .map(|r| r.setting(permission))
            .unwrap_or(PermissionSetting::Disallowed)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ranks.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ranks.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<Rank>> {
        self.ranks.values()
    }
}
