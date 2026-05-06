//! Subscriber construction and installation.

use crate::error::TracingError;
use tracing::Level;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Controls how the tracing subscriber is configured.
#[derive(Debug, Clone)]
pub struct TracingConfig {
    /// The minimum log level when `RUST_LOG` is not set.
    ///
    /// Default: [`Level::INFO`].
    pub default_level: Level,

    /// Output format.
    ///
    /// Default: [`Format::Pretty`] in development, [`Format::Json`] in
    /// production. Set this based on your `server.environment` config field.
    pub format: Format,

    /// Whether to include the source file and line number in log output.
    ///
    /// Useful in development. In production, the overhead is negligible but
    /// the output is noisier. Default: true in pretty mode, false in JSON.
    pub include_location: bool,

    /// Whether to include the target (module path) in log output.
    ///
    /// Default: true.
    pub include_target: bool,

    /// Service name included in structured log output and OTel spans.
    ///
    /// Default: `"sirius"`.
    pub service_name: String,
}

/// The output format for log events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    /// Colorized, human-readable output. Use during local development.
    Pretty,
    /// One JSON object per log line. Use in production.
    Json,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            default_level: Level::INFO,
            format: Format::Pretty,
            include_location: true,
            include_target: true,
            service_name: "sirius".into(),
        }
    }
}

impl TracingConfig {
    pub fn production(service_name: impl Into<String>) -> Self {
        Self {
            default_level: Level::INFO,
            format: Format::Json,
            include_location: false,
            include_target: true,
            service_name: service_name.into(),
        }
    }

    pub fn development() -> Self {
        Self {
            default_level: Level::DEBUG,
            format: Format::Pretty,
            include_location: true,
            include_target: true,
            service_name: "sirius".into(),
        }
    }
}

pub(crate) fn install(config: TracingConfig) -> Result<(), TracingError> {
    let filter = build_filter(&config)?;

    match config.format {
        Format::Pretty => install_pretty(config, filter),
        Format::Json => install_json(config, filter),
    }
}

fn build_filter(config: &TracingConfig) -> Result<EnvFilter, TracingError> {
    // RUST_LOG takes precedence. Fall back to the configured default level.
    EnvFilter::try_from_default_env().or_else(|_| {
        EnvFilter::try_new(config.default_level.as_str())
            .map_err(|e| TracingError::InvalidFilter(e.to_string()))
    })
}

fn install_pretty(config: TracingConfig, filter: EnvFilter) -> Result<(), TracingError> {
    let fmt_layer = fmt::layer()
        .with_target(config.include_target)
        .with_file(config.include_location)
        .with_line_number(config.include_location)
        .with_thread_ids(false)
        .with_thread_names(false);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|_| TracingError::AlreadyInitialized)
}

fn install_json(config: TracingConfig, filter: EnvFilter) -> Result<(), TracingError> {
    let fmt_layer = fmt::layer()
        .json()
        .with_target(config.include_target)
        .with_file(config.include_location)
        .with_line_number(config.include_location)
        .with_current_span(true)
        .with_span_list(false);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|_| TracingError::AlreadyInitialized)
}
