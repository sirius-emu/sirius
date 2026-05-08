use crate::error::CurrencyError;
use sirius_database::DbPool;
use sirius_types::{CurrencyType, UserId};

pub struct CurrencyService {
    pool: DbPool,
}

impl CurrencyService {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn get_balance(
        &self,
        user_id: UserId,
        currency: CurrencyType,
    ) -> Result<i32, CurrencyError> {
        let balance = match currency {
            CurrencyType::Credits => {
                sqlx::query_scalar!(
                    "SELECT credits FROM users WHERE id = $1",
                    user_id.0
                )
                .fetch_one(&self.pool)
                .await?
            }
            _ => sqlx::query_scalar!(
                "SELECT amount FROM users_currency
                    WHERE user_id = $1 AND currency_type = $2",
                user_id.0,
                currency as i32,
            )
            .fetch_optional(&self.pool)
            .await?
            .unwrap_or(0),
        };

        Ok(balance)
    }
}
