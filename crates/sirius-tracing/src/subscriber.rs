//! Subscriber construction and installation.

use crate::error::TracingError;
use sirius_config::{TracingConfig, TracingFormat};
use tracing_subscriber::{
    EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
};

/// The output format for log events.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Format {
    /// Colorized, human-readable output. Use during local development.
    Pretty,
    /// One JSON object per log line. Use in production.
    Json,
}

pub(crate) fn install(config: &TracingConfig) -> Result<(), TracingError> {
    let filter = EnvFilter::try_from_default_env().or_else(|_| {
        EnvFilter::try_new(&config.default_level)
            .map_err(|e| TracingError::InvalidFilter(e.to_string()))
    })?;

    match config.format {
        TracingFormat::Pretty => install_pretty(config, filter),
        TracingFormat::Json => install_json(config, filter),
        TracingFormat::Unknown => unreachable!(),
    }
}

fn install_pretty(
    config: &TracingConfig,
    filter: EnvFilter,
) -> Result<(), TracingError> {
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
        .map_err(|_| TracingError::AlreadyInitialized)?;

    tracing::info!(service = %config.service_name, "tracing initialized");

    Ok(())
}

fn install_json(
    config: &TracingConfig,
    filter: EnvFilter,
) -> Result<(), TracingError> {
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
        .map_err(|_| TracingError::AlreadyInitialized)?;

    // Emit service name as a structured field so log aggregators can filter by it.
    // Every event after this point will be associated with this process; the
    // service name here acts as a process-level label in the log stream.
    // TODO: When OTel is integrated, set this as the `service.name` resource attribute.
    tracing::info!(service = %config.service_name, "tracing initialized");

    Ok(())
}
