//! Outgoing packet definitions.
//!
//! Each submodule owns a slice of the outgoing packet namespace.

pub mod handshake;

pub use handshake::AuthOkComposer;
