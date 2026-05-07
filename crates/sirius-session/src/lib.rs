//! Session lifecycle and authentication state machine for Sirius.
//!
//! A session is the bridge between a raw [`Connection`] and the rest of the
//! game. It owns the auth state machine, dispatches incoming packets to the
//! correct handler based on auth state and exposes a [`SessionHandle`] that
//! other subsystems use to push packets to the client.
//!
//! # Lifecycle
//!
//! ```text
//! Connection accepted
//!     -> Session::spawn(connection)
//!         -> Session actor starts
//!         -> State: Unauthenticated
//!
//! Client sends ReleaseVersionEvent
//!     -> logged, no state change
//!
//! Client sends SsoTicket
//!     -> State: Authenticating
//!     -> Ticket validated (stub for now)
//!     -> State: Authenticated
//!     -> Sends AuthOk to client
//!     -> UserId bound to session
//!
//! Client sends Pong
//!     -> idle timer reset
//!
//! Connection closed / error
//!     -> Session actor stops
//!     -> SessionManager notified
//! ```
//!
//! # Actor model
//!
//! [`Session`] implements [`Actor`] with [`SessionCommand`] as its command
//! type. All interaction from outside goes through [`SessionHandle`], which
//! is just a [`Handle<SessionCommand>`].

mod command;
mod manager;
mod session;
mod state;

pub use command::SessionCommand;
pub use manager::SessionManager;
pub use session::{Session, spawn_session};
pub use state::AuthState;

use sirius_actor::Handle;

pub type SessionHandle = Handle<SessionCommand>;
