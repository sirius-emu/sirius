//! TCP listener and connection management for Sirius.
//!
//! This crate owns everything from the moment a TCP connection is accepted
//! to the moment a framed [`RawPacket`] is handed off to the session layer.
//! It knows nothing about what's inside those packets.
//!
//! # Responsibilities
//!
//! - Bind and accept TCP connections.
//! - Enforce the server-wide connection limit.
//! - Rate-limit new connections per IP.
//! - Wrap each accepted socket in a [`NitroCodec`] framer.
//! - Spawn a [`Connection`] task per accepted socket.
//! - Track all live connections in a [`ConnectionManager`].
//!
//! # What this crate does NOT do
//!
//! - WebSocket upgrade - that's `sirius-websocket`.
//! - Session authentication - that's `sirius-session`.
//! - Packet interpretation - that's `sirius-packets`.

mod connection;
mod limiter;
mod listener;
mod manager;

pub use connection::{Connection, ConnectionId};
pub use limiter::RateLimiter;
pub use listener::Listener;
pub use manager::ConnectionManager;
