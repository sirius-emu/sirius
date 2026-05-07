//! Database connectivity layer.
//!
//! This crate owns exactly one thing, the [`sqlx::PgPool`] and the
//! logic needed to create, configure and health-check it. No SQL queries
//! live here.

mod health;
mod pool;

pub use health::HealthStatus;
pub use pool::Database;

pub type DbPool = sqlx::PgPool;
