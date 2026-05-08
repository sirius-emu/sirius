//! Tracing subscriber setup for Sirius.
//!
//! Call [`init`] once at the very start of `main`, before anything else.
//! That's the entire public API for most use cases.
//!
//! # Output formats
//!
//! - **Development** (`format = "pretty"`): human-readable, colorized output
//! - **Production** (`format = "json"`): one JSON object per log line
//!
//! # Log filtering
//!
//! The `RUST_LOG` environment variable controls which spans and events are emitted,
//! using the standard `tracing-subscriber` filter syntax:
//!
//! ```text
//! RUST_LOG=info
//! RUST_LOG=sirius=debug
//! RUST_LOG=sirius_room=trace,info
//! ```
//!
//! If `RUST_LOG` is not set, the level from [`TracingConfig`] is used as the default.

mod error;
mod subscriber;

pub use error::TracingError;
use sirius_config::TracingConfig;

/// Initializes the global tracing subscriber.
///
/// Must be called once, at the start of `main`, before spawning any tasks or making
/// any log calls. Calling it more than once will return an error on the second call.
///
/// # Errors
///
/// Returns [`TracingError`] if the subscriber cannot be installed.
pub fn init(config: &TracingConfig) -> Result<(), TracingError> {
    subscriber::install(config)
}
