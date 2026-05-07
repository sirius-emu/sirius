//! Connection pool construction and lifecycle management.

use std::time::Duration;

use crate::{DbPool, HealthStatus};
use sirius_config::DatabaseConfig;
use sirius_error::{DatabaseError, SiriusError};
use sqlx::postgres::PgPoolOptions;
use tracing::{info, instrument, warn};

#[derive(Debug, Clone)]
pub struct Database {
    pool: DbPool,
}

/// A snapshot of pool utilization at a point in time.
#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    /// Total number of connections currently open (idle + in-use).
    pub size: u32,

    /// Number of connections sitting idle in the pool.
    pub idle: usize,
}

impl Database {
    #[instrument(skip(config), fields(url = redact_url(&config.url)))]
    pub async fn connect(config: &DatabaseConfig) -> Result<Self, SiriusError> {
        info!("connecting to database");

        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .acquire_timeout(Duration::from_secs(config.acquire_timeout_secs))
            .max_lifetime(Duration::from_secs(config.max_lifetime_secs))
            .connect(&config.url)
            .await
            .map_err(|e| {
                SiriusError::Database(DatabaseError::ConnectionFailed {
                    reason: e.to_string(),
                })
            })?;

        info!("database pool created, running health check");

        let db = Self { pool };
        db.ping().await?;

        info!("database ready");
        Ok(db)
    }

    #[inline]
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }

    pub async fn health_check(&self) -> HealthStatus {
        match self.ping().await {
            Ok(_) => HealthStatus::Healthy,
            Err(e) => {
                warn!(error = %e, "database health check failed");
                HealthStatus::Unhealthy {
                    reason: e.to_string(),
                }
            }
        }
    }

    /// Internal ping used during startup.
    async fn ping(&self) -> Result<(), SiriusError> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| {
                SiriusError::Database(DatabaseError::HealthCheckFailed {
                    reason: e.to_string(),
                })
            })
    }

    pub fn stats(&self) -> PoolStats {
        PoolStats {
            size: self.pool.size(),
            idle: self.pool.num_idle(),
        }
    }
}

/// Strips the password from a PostgreSQL URL for safe logging.
///
/// `postgres://user:secret@host/db` → `postgres://user:***@host/db`
fn redact_url(url: &str) -> String {
    if let Some(at) = url.find('@') {
        if let Some(colon) = url[..at].rfind(':') {
            return format!("{}:***@{}", &url[..colon], &url[at + 1..]);
        }
    }
    url.to_string()
}
