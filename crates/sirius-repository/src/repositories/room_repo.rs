use crate::map_sqlx_error;
use crate::models::Room;
use sirius_database::DbPool;
use sirius_error::{DatabaseError, SiriusError};
use sirius_types::{RoomCategory, RoomId, RoomLockType, UserId};

#[derive(Debug, Clone)]
pub struct RoomRepository {
    pool: DbPool,
}

impl RoomRepository {
    pub(crate) fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_id(&self, id: RoomId) -> Result<Room, SiriusError> {
        let row = sqlx::query!(
            r#"
            SELECT
                r.id, r.name, r.owner_id, r.description, r.password,
                r.max_users, r.category, r.lock_type, u.username AS owner_name,
                r.created_at
            FROM rooms r
            INNER JOIN users u ON u.id = r.owner_id
            WHERE r.id = $1
            "#,
            id.inner()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(map_sqlx_error)?
        .ok_or_else(|| {
            SiriusError::Database(DatabaseError::NotFound {
                entity: "Room".into(),
            })
        })?;

        Ok(Room {
            id: RoomId::from(row.id),
            owner_id: UserId::from(row.owner_id),
            owner_name: row.owner_name,
            name: row.name,
            description: row.description,
            password: row.password,
            max_users: row.max_users,
            category: RoomCategory::from(row.category),
            lock_type: RoomLockType::try_from(row.lock_type as i32)
                .unwrap_or(RoomLockType::Open),
            created_at: row.created_at,
        })
    }

    pub async fn find_by_owner(
        &self,
        owner_id: UserId,
    ) -> Result<Vec<Room>, SiriusError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                r.id, r.name, r.owner_id, r.description, r.password,
                r.max_users, r.category, r.lock_type, u.username AS owner_name,
                r.created_at
            FROM rooms r
            INNER JOIN users u ON u.id = r.owner_id
            WHERE r.owner_id = $1
            ORDER BY r.id DESC
            "#,
            owner_id.inner()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        let mut rooms = Vec::with_capacity(rows.len());
        for row in rows {
            rooms.push(Room {
                id: RoomId::from(row.id),
                owner_id: UserId::from(row.owner_id),
                owner_name: row.owner_name,
                name: row.name,
                description: row.description,
                password: row.password,
                max_users: row.max_users,
                category: RoomCategory::from(row.category),
                lock_type: RoomLockType::try_from(row.lock_type as i32)
                    .unwrap_or(RoomLockType::Open),
                created_at: row.created_at,
            });
        }

        Ok(rooms)
    }
}
