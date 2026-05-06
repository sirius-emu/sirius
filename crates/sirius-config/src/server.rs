//! Top-level server configuration.

use crate::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    /// The environment name. Used for logging and metrics labels.
    pub environment: String,

    /// Port the game server listens on.
    ///
    /// Default: 3000. Must be in range 1-65535.
    pub port: u16,

    /// Address the server binds to.
    ///
    /// Use `"0.0.0.0"` to accept connections on all interfaces, or
    /// `"127.0.0.1"` to restrict to localhost.
    pub bind_address: String,

    /// Maximum number of concurrent client connections.
    ///
    /// New connections are rejected with a "server full" response once this
    /// limit is reached. Default: 2000
    pub max_connections: usize,

    /// How long in seconds the server waits for in-flight work to finish
    /// after receiving SIGTERM before forcefully shutting down.
    ///
    /// Default: 30.
    pub shutdown_timeout_secs: u64,
}

impl ServerConfig {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        if self.environment.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "server.environment",
                reason: "must not be empty".into(),
            });
        }

        if self.port == 0 {
            return Err(ConfigError::InvalidValue {
                field: "server.port",
                reason: "must be between 1 and 65535".into(),
            });
        }

        if self.bind_address.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "server.bind_address",
                reason: "must not be empty".into(),
            });
        }

        if self.max_connections == 0 {
            return Err(ConfigError::InvalidValue {
                field: "server.max_connections",
                reason: "must be greater than 0".into(),
            });
        }

        Ok(())
    }
}
