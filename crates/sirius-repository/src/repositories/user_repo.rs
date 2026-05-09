//! User database operations.

use crate::map_sqlx_error;
use crate::models::{User, UserSettings, UserStats};
use sirius_database::DbPool;
use sirius_error::{DatabaseError, SiriusError};
use sirius_types::{CurrencyType, Gender, RoomId, UserId};
use std::collections::HashMap;

/// Cheaply cloneable.
#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub(crate) fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Loads a fully hydrated [`User`] by their SSO auth ticket.
    ///
    /// Returns [`DatabaseError::NotFound`] if no user has that ticket.
    pub async fn find_by_auth_ticket(
        &self,
        ticket: &str,
    ) -> Result<User, SiriusError> {
        let row = sqlx::query!(
            r#"
            SELECT
                u.id, u.username, u.motto, u.look, u.gender, u.rank,
                u.credits, u.home_room, u.account_created, u.last_online,
                u.current_ip, u.machine_id,
                st.respects_received, st.daily_respects, st.daily_pet_respects,
                se.can_change_name, se.safety_locked, se.volume_system, se.volume_furni,
                se.volume_trax, se.old_chat, se.room_invites, se.camera_follow,
                se.chat_type
            FROM users u
            INNER JOIN users_stats st ON st.user_id = u.id
            INNER JOIN users_settings se ON se.user_id = u.id
            WHERE u.auth_ticket = $1
            "#,
            ticket
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            SiriusError::Database(DatabaseError::QueryFailed {
                reason: e.to_string(),
            })
        })?;

        let row =
            row.ok_or(SiriusError::Database(DatabaseError::NotFound {
                entity: "User".into(),
            }))?;

        let user_id = UserId::from(row.id);
        let currencies = self.fetch_currencies(user_id).await?;
        let home_room = row.home_room.map(RoomId::from);

        let gender_char = row.gender.chars().next().unwrap_or('M');
        let gender = Gender::try_from(gender_char).unwrap_or(Gender::Male);

        let stats = UserStats {
            respects_received: row.respects_received,
            respects_remaining: row.daily_respects,
            respects_pet_remaining: row.daily_pet_respects,
        };

        let settings = UserSettings {
            can_change_name: row.can_change_name,
            safety_locked: row.safety_locked,
            volume_system: row.volume_system,
            volume_furni: row.volume_furni,
            volume_trax: row.volume_trax,
            old_chat: row.old_chat,
            room_invites: row.room_invites,
            camera_follow: row.camera_follow,
            chat_type: row.chat_type,
        };

        Ok(User {
            id: user_id,
            username: row.username,
            motto: row.motto,
            look: row.look,
            gender: gender,
            rank: row.rank,
            credits: row.credits,
            home_room,
            account_created: row.account_created,
            last_online: row.last_online,
            current_ip: row.current_ip,
            machine_id: row.machine_id,
            currencies,
            stats,
            settings,
        })
    }

    /// Clears the auth ticket after a successful login.
    pub async fn consume_auth_ticket(
        &self,
        user_id: UserId,
    ) -> Result<(), SiriusError> {
        sqlx::query!(
            r#"UPDATE users SET auth_ticket = NULL WHERE id = $1"#,
            user_id.0
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }

    /// Fetches all alternative currency balances for a user. Credits are not
    /// included.
    async fn fetch_currencies(
        &self,
        user_id: UserId,
    ) -> Result<HashMap<CurrencyType, i32>, SiriusError> {
        let rows = sqlx::query!(
            r#"SELECT currency_type, amount FROM users_currency WHERE user_id = $1"#,
            user_id.0
        ).fetch_all(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(rows
            .into_iter()
            .map(|r| (CurrencyType::from(r.currency_type), r.amount))
            .collect())
    }

    /// Updates the user's figure string and gender in the database.
    pub async fn update_look(
        &self,
        user_id: UserId,
        look: &str,
        gender: &str,
    ) -> Result<(), SiriusError> {
        sqlx::query!(
            "UPDATE users SET look = $1, gender = $2 WHERE id = $3",
            look,
            gender,
            user_id.0,
        )
        .execute(&self.pool)
        .await
        .map_err(map_sqlx_error)?;

        Ok(())
    }
}
