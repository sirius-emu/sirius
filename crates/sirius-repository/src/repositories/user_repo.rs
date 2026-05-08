//! User database operations.

use crate::models::User;
use sirius_database::DbPool;
use sirius_error::{DatabaseError, SiriusError};
use sirius_types::{CurrencyType, Gender, RoomId, UserId};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    pub(crate) fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_auth_ticket(
        &self,
        ticket: &str,
    ) -> Result<User, SiriusError> {
        let row = sqlx::query!(
            r#"
            SELECT
                id, username, motto, look, gender, rank,
                credits, home_room, account_created, last_online,
                current_ip, machine_id
            FROM users
            WHERE auth_ticket = $1
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
        })
    }

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

    async fn fetch_currencies(
        &self,
        user_id: UserId,
    ) -> Result<HashMap<CurrencyType, i64>, SiriusError> {
        let rows = sqlx::query!(
            r#"SELECT currency_type, amount FROM users_currency WHERE user_id = $1"#,
            user_id.0
        ).fetch_all(&self.pool)
        .await
        .map_err(|e| SiriusError::Database(DatabaseError::QueryFailed { reason: e.to_string() }))?;

        let mut currencies = HashMap::with_capacity(rows.len());
        for row in rows {
            let c_type = CurrencyType::from(row.currency_type);

            currencies.insert(c_type, row.amount);
        }

        Ok(currencies)
    }
}

fn map_sqlx_error(err: sqlx::Error) -> SiriusError {
    let db_err = match err {
        sqlx::Error::RowNotFound => {
            DatabaseError::NotFound { entity: "Unknown" }
        }
        sqlx::Error::Database(db_err) => {
            if let Some(code) = db_err.code() {
                if code == "23505" {
                    return SiriusError::Database(
                        DatabaseError::UniqueViolation {
                            field: "unknown".into(),
                        },
                    );
                }
            }
            DatabaseError::QueryFailed {
                reason: db_err.message().to_string(),
            }
        }
        _ => DatabaseError::QueryFailed {
            reason: err.to_string(),
        },
    };

    SiriusError::Database(db_err)
}
