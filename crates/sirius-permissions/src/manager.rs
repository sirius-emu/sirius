//! Lock-free permission manager.
//!
//! [`PermissionsManager`] holds an [`ArcSwap`]-wrapper [`RankTable`] that can
//! be hot-reloaded without blocking any reader. All permission checks are non-blocking
//! and require no `await`.
//!
//! # How it works
//!
//! On [`load`] the full rank and permission dataset is fetched from the database and stored
//! as an immutable [`RankTable`] behind an [`ArcSwap`]. Readers call [`load`] on the swap to
//! get a lightweight guard that keeps the current snapshot alive for the duration of the check.
//!
//! On [`reload`] a fresh [`RankTable`] is built and automatically swapped in. Readers holding
//! the old guard continue to see the previous snapshot until they drop it, at which point the
//! old table is freed.
//!
//! [`load`]: PermissionsManager::load
//! [`reload`]: PermissionsManager::reload

use crate::{Permission, PermissionSetting, Rank, RankTable};
use arc_swap::ArcSwap;
use sirius_database::DbPool;
use sirius_error::SiriusError;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Hot-reloadable permission manager.
///
/// Wrap in an [`Arc`] and share across subsystem. Cloning is cheap.
#[derive(Debug, Clone)]
pub struct PermissionsManager {
    table: Arc<ArcSwap<RankTable>>,
    pool: DbPool,
}

impl PermissionsManager {
    /// Loads all ranks and permissions from the database.
    ///
    /// Call once at startup and wrap the result in an [`Arc`].
    pub async fn load(pool: DbPool) -> Result<Self, SiriusError> {
        let table = Self::fetch_table(&pool).await?;

        info!(ranks = table.len(), "permissions loaded");

        Ok(Self {
            table: Arc::new(ArcSwap::from_pointee(table)),
            pool,
        })
    }

    /// Reloads ranks and permissions from the database and automatically
    /// swaps the internal table.
    ///
    /// Safe to call while the server is running.
    pub async fn reload(&self) -> Result<(), SiriusError> {
        let table = Self::fetch_table(&self.pool).await?;
        info!(ranks = table.len(), "permissions reloaded");
        self.table.store(Arc::new(table));
        Ok(())
    }

    /// Returns `true` if the given rank holds the requested permission.
    ///
    /// `is_room_owner` is only relevant for permissions with a
    /// [`PermissionSetting::RoomOwner`] setting.
    #[inline]
    pub fn has_permission(
        &self,
        rank_id: i32,
        permission: Permission,
        is_room_owner: bool,
    ) -> bool {
        self.table
            .load()
            .has_permission(rank_id, permission, is_room_owner)
    }

    /// Returns the [`Rank`] for the given ID, or `None` if it does not exist.
    #[inline]
    pub fn get_rank(&self, rank_id: i32) -> Option<Arc<Rank>> {
        self.table.load().get(rank_id).cloned()
    }

    /// Returns a guard over the current [`RankTable`] snapshot.
    ///
    /// Prefer this over multiple individual calls when you need to
    /// perform several permission checks. It guarantees a consistent view and
    /// performs only one atomic load.
    #[inline]
    pub fn snapshot(&self) -> arc_swap::Guard<Arc<RankTable>> {
        self.table.load()
    }

    /// Fetches ranks and permissions from the database and builds a
    /// [`RankTable`]. Ranks are loaded first, then all permissions are
    /// fetched in a single query and grouped by rank ID before being
    /// merged into each [`Rank`].
    async fn fetch_table(pool: &DbPool) -> Result<RankTable, SiriusError> {
        let rank_rows = sqlx::query!(
            r#"
                SELECT
                    id, name, level, badge, prefix, prefix_color,
                    room_effect, log_commands,
                    auto_credits_amount, auto_pixels_amount, auto_points_amount
                FROM  permissions_ranks
                ORDER BY id ASC
                "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            SiriusError::Database(sirius_error::DatabaseError::QueryFailed {
                reason: e.to_string(),
            })
        })?;

        let perm_rows = sqlx::query!(
            "SELECT rank_id, key, setting FROM permissions_rank_permissions"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            SiriusError::Database(sirius_error::DatabaseError::QueryFailed {
                reason: e.to_string(),
            })
        })?;

        let mut perm_map: HashMap<i32, HashMap<String, PermissionSetting>> =
            HashMap::new();

        for row in perm_rows {
            perm_map
                .entry(row.rank_id)
                .or_default()
                .insert(row.key, PermissionSetting::from(row.setting));
        }

        let ranks = rank_rows
            .into_iter()
            .map(|r| {
                let permissions = perm_map.remove(&r.id).unwrap_or_default();
                let rank = Arc::new(Rank::new(
                    r.id,
                    r.name,
                    r.level,
                    r.badge,
                    r.prefix,
                    r.prefix_color,
                    r.room_effect,
                    r.log_commands,
                    r.auto_credits_amount,
                    r.auto_pixels_amount,
                    r.auto_points_amount,
                    permissions,
                ));
                (r.id, rank)
            })
            .collect();

        Ok(RankTable::new(ranks))
    }
}
