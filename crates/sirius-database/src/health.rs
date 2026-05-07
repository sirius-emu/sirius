//! Health status type, used by the `/health` endpoint in `sirius-api`.

/// The result of a [`Database::health_check`] call.
///
/// [`Database::health_check`]: crate::Database::health_check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    /// The database responded to `SELECT 1` successfully.
    Healthy,

    /// The database did not respond or returned an error.
    Unhealthy {
        /// Human-readable description of the failure.
        reason: String,
    },
}

impl HealthStatus {
    /// Returns `true` if the database is healthy.
    #[inline]
    pub fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Unhealthy { reason } => write!(f, "unhealthy: {reason}"),
        }
    }
}
