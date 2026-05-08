use thiserror::Error;

#[derive(Debug, Error)]
pub enum CurrencyError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}
