//! Database configuration.

use crate::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// Full PostgreSQL connection url.
    ///
    /// Format: `postgres://user:password@host:port/dbname`
    pub url: String,

    /// Maximum number of connections in the pool.
    ///
    /// Defaults to `10`.
    pub max_connections: u32,

    /// Minimum number of idle connections kept alive.
    ///
    /// Defaults to `1`.
    pub min_connections: u32,

    /// How long to wait for a connection from the pool before giving up.
    ///
    /// Defaults to `5`.
    pub acquire_timeout_secs: u64,

    /// Maximum lifetime of a connection regardless of activity.
    ///
    /// Defaults to 1800 (30 minutes). Prevents stale connections from accumulating
    /// on the PostgreSQL side.
    pub max_lifetime_secs: u64,

    /// Whether to run pending `sqlx` migrations on startup.
    ///
    /// Disable this in production if you manage migrations separately.
    pub run_migrations: bool,
}

impl DatabaseConfig {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        if self.url.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "database.url",
                reason: "database URL cannot be empty".into(),
            });
        }

        if self.max_connections == 0 {
            return Err(ConfigError::InvalidValue {
                field: "database.max_connections",
                reason: "must be at least 1".into(),
            });
        }

        if self.min_connections == 0 {
            return Err(ConfigError::InvalidValue {
                field: "database.min_connections",
                reason: "must be at least 1".into(),
            });
        }

        if self.min_connections > self.max_connections {
            return Err(ConfigError::InvalidValue {
                field: "database.min_connections",
                reason: "database.min_connections cannot be larger than database.max_connections".into(),
            });
        }

        if self.acquire_timeout_secs == 0 {
            return Err(ConfigError::InvalidValue {
                field: "database.acquire_timeout_secs",
                reason: "must be greater than 0".into(),
            });
        }

        Ok(())
    }
}
