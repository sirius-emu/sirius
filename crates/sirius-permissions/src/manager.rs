use crate::{Permission, PermissionSetting, Rank, RankTable};
use arc_swap::ArcSwap;
use sirius_database::DbPool;
use sirius_error::SiriusError;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct PermissionsManager {
    table: Arc<ArcSwap<RankTable>>,
    pool: DbPool,
}

impl PermissionsManager {
    pub async fn load(pool: DbPool) -> Result<Self, SiriusError> {
        let table = Self::fetch_table(&pool).await?;

        info!(ranks = table.len(), "permissions loaded");

        Ok(Self {
            table: Arc::new(ArcSwap::from_pointee(table)),
            pool,
        })
    }

    pub async fn reload(&self) -> Result<(), SiriusError> {
        let table = Self::fetch_table(&self.pool).await?;
        info!(ranks = table.len(), "permissions reloaded");
        self.table.store(Arc::new(table));
        Ok(())
    }

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

    #[inline]
    pub fn get_rank(&self, rank_id: i32) -> Option<Arc<Rank>> {
        self.table.load().get(rank_id).cloned()
    }

    #[inline]
    pub fn snapshot(&self) -> arc_swap::Guard<Arc<RankTable>> {
        self.table.load()
    }

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
