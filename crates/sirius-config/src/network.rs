//! Network and connection layer configuration.

use crate::error::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NetworkConfig {
    /// How many seconds without data from a client before the connection is
    /// considered idle and closed.
    ///
    /// Default: 120.
    pub read_timeout_secs: u64,

    /// How many seconds to wait for a write to complete before giving up.
    ///
    /// Default: 30.
    pub write_timeout_secs: u64,

    /// Maximum number of new connections allowed per IP per second before
    /// rate limiting kicks in.
    ///
    /// Default: 10.
    pub rate_limit_per_ip: u32,

    /// Size of the per-connection read buffer in bytes.
    ///
    /// Packets larger than this cannot be received. The Nitro client doesn't
    /// send anything approaching this, so 8192 is safe upper bound.
    ///
    /// Default: 8192.
    pub read_buffer_size: usize,

    /// Whether to accept WebSocket connection in addition to raw TCP.
    ///
    /// Default: true.
    pub websocket_enabled: bool,

    /// The HTTP path the server expects for WebSocket upgrade requests.
    ///
    /// Default: `"/"`.
    pub websocket_path: String,

    /// Interval in seconds between WebSocket ping frames sent to clients.
    ///
    /// Default: 30.
    pub websocket_ping_interval_secs: u64,
}

impl NetworkConfig {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        if self.read_timeout_secs == 0 {
            return Err(ConfigError::InvalidValue {
                field: "network.read_timeout_secs",
                reason: "must be greater than 0".into(),
            });
        }

        if self.write_timeout_secs == 0 {
            return Err(ConfigError::InvalidValue {
                field: "network.write_timeout_secs",
                reason: "must be greater than 0".into(),
            });
        }

        if self.read_buffer_size == 0 {
            return Err(ConfigError::InvalidValue {
                field: "network.read_buffer_size",
                reason: "must be greater than 0".into(),
            });
        }

        if self.websocket_enabled && self.websocket_path.is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "network.websocket_path",
                reason: "must not be empty when WebSocket is enabled".into(),
            });
        }

        Ok(())
    }
}
