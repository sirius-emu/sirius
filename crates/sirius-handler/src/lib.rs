//! Packet handler registry and dispatch for Sirius.
//!
//! This crate sits between session layer and the game layer. When a [`RawPacket`]
//! arrives at a session, the session calls [`PacketRouter::dispatch`] and never has to
//! know what the packet means or which subsystem handles it.
//!
//! # How it works
//!
//! Each game feature implements one or more [`PacketHandler`]s. Handlers are registered into
//! a [`PacketRouter`] at startup. When a packet arrives, the router looks up the header ID
//! and calls the matching handler with a [`HandlerContext`] that gives access to everything
//! the handler might need.
//!
//! # Adding a new handler
//!
//! 1. Create a file in appropriate `handlers/` subdirectory.
//! 2. Implement [`PacketHandler`] on a unit struct.
//! 3. Register it in [`PacketRouter::new`].
//!
//! The session never needs to be touched.

mod context;
mod prelude;
mod registry;
mod router;
mod traits;

pub mod handlers;

pub use context::{AuthenticatedUser, HandlerContext};
pub use registry::build_router;
pub use router::{PacketRouter, PacketRouterBuilder};
pub use traits::PacketHandler;
