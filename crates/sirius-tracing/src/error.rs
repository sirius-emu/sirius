//! Tracing initialization errors.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TracingError {
    /// The global tracing subscriber has already been set.
    ///
    /// This happens if `init` is called more than once in the same process.
    /// It's always a programmer error.
    #[error("global tracing subscriber is already set")]
    AlreadyInitialized,

    /// The `RUST_LOG` directive string is syntactically invalid.
    #[error("invalid log filter directive: {0}")]
    InvalidFilter(String),

    #[error("invalid tracing format: {0}. expected 'pretty' or 'json'")]
    InvalidFormat(String),
}
