//! Unified error hierarchy for the Sirius emulator.
//!
//! Every subsystem defines its own error type in a dedicated module.
//! The top-level `SiriusError` enum wraps them all, so any function that
//! can fail across subsystem boundaries can return `Result<T, SiriusError>`
//! without losing context about where the failure originated.
//!
//! # Design
//!
//! - Subsystem errors are defined in their own modules and re-exported at
//! the crate root for convenience.
//! - `SiriusError` is the boundary type. It's what crosses crate lines.
//! Within a single crate, prefer the specific subystem error directly.
//! - All conversions are via `#[from]`, so `?` works everywhere without
//! manual `map_err` calls.

mod auth;
mod network;
mod protocol;

pub use auth::AuthError;
pub use network::NetworkError;
pub use protocol::ProtocolError;

use thiserror::Error;

/// The top-level error type for Sirius emulator.
///
/// Use this as the return type for functions that will fail across subystem boundaries.
/// For functions that are entirely within one subsystem, return that subsystem's error type
/// directly. It carries more context and avoids an unnecessary wrapping layer.
#[derive(Debug, Error)]
pub enum SiriusError {
    #[error(transparent)]
    Network(#[from] NetworkError),

    #[error(transparent)]
    Protocol(#[from] ProtocolError),

    #[error(transparent)]
    Auth(#[from] AuthError),
}
