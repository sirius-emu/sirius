//! Database-layer error type.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("failed to connect to database: {reason}")]
    ConnectionFailed { reason: String },

    #[error("connection pool timed out after {timeout_ms}ms")]
    PoolTimeout { timeout_ms: u64 },

    #[error("migration failed: {reason}")]
    MigrationFailed { reason: String },

    #[error("health check failed: {reason}")]
    HealthCheckFailed { reason: String },
}
