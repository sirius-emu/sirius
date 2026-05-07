//! Top-level tracing configuration.

use crate::ConfigError;
use serde::Deserialize;

/// Controls how the tracing subscriber is configured.
#[derive(Debug, Deserialize)]
pub struct TracingConfig {
    /// The minimum log level when `RUST_LOG` is not set.
    pub default_level: String,

    /// Output format.
    ///
    /// Default: "pretty" in development, "json" in production.
    pub format: String,

    /// Whether to include the source file and line number in log output.
    ///
    /// Useful in development. In production, the overhead is negligible but
    /// the output is noisier. Default: true in pretty mode, false in JSON.
    pub include_location: bool,

    /// Whether to include the target (module path) in log output.
    ///
    /// Default: true.
    pub include_target: bool,

    /// Service name included in structured log output.
    ///
    /// In JSON mode this is emitted as a `service` field on every log line,
    /// which makes it possible to filter and correlate logs across instances
    /// when shipping to a log aggregator.
    ///
    /// When OpenTelemetry is integrated this value will also be set as the
    /// `service.name` resource attribute on all exported spans.
    ///
    /// Default: `"sirius"`.
    pub service_name: String,
}

impl TracingConfig {
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        if self.default_level.trim().is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "tracing.default_level",
                reason: "default_level cannot be empty".into(),
            });
        }

        if self.service_name.trim().is_empty() {
            return Err(ConfigError::InvalidValue {
                field: "tracing.service_name",
                reason: "service_name cannot be empty".into(),
            });
        }

        match self.format.as_str() {
            "pretty" | "json" => {}
            _ => {
                return Err(ConfigError::InvalidValue {
                    field: "tracing.format",
                    reason: "must be either 'pretty' or 'json'".into(),
                });
            }
        }

        Ok(())
    }
}
