//! Actor primitives for Sirius.
//!
//! Every stateful entity in Sirius (sessions, rooms, users, bots) is an actor.
//! An actor is a tokio task that owns its state exclusively and communicates
//! with the outside world only through typed message passing. No shared references,
//! no `Mutex` on hot paths, no reaching into another entity's internals.
//!
//! # Model
//!
//! The pattern is intentionally minimal:
//!
//! - [`Actor`] is the trait your stateful struct implements. It defines the command type
//! and the handler that processes each message.
//! - [`Handle`] is the only way to talk to a running actor from outside. It wraps an `mpsc`
//! sender and hides the channel entirely.
//! - [`ActorContext`] is passed to the handler on every message. It gives the actor a way to
//! send messages to itself and carries the actor's own handle for self-reference.
//!
//! # Example
//!
//! ```no_run
//! use sirius_actor::{Actor, ActorContext, Handle};
//! use sirius_error::SiriusError;
//!
//! enum CounterCommand {
//!     Increment,
//!     GetCount { reply: tokio::sync::oneshot::Sender<u64> },
//! }
//!
//! struct Counter {
//!     count: u64,
//! }
//!
//! impl Actor for Counter {
//!     type Command = CounterCommand;
//!
//!     async fn handle(
//!         &mut self,
//!         cmd: Self::Command,
//!         _ctx: &ActorContext<Self::Command>,
//!     ) -> Result<(), SiriusError> {
//!         match cmd {
//!             CounterCommand::Increment => self.count += 1,
//!             CounterCommand::GetCount { reply } => { let _ = reply.send(self.count); }
//!         }
//!         Ok(())
//!     }
//! }
//! ``````

mod actor;
mod context;
mod handle;
mod mailbox;

pub use actor::Actor;
pub use context::ActorContext;
pub use handle::Handle;
pub use mailbox::Mailbox;
