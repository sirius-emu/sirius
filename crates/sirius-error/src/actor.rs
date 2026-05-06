//! Errors originating in the actor layer.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ActorError {
    /// The actor's mailbox is closed; the task has stopped.
    ///
    /// Returned by [`Handle::send`] and [`Handle::try_send`] when the actor
    /// task is no longer running. Callers that receive this should treat the
    /// actor as gone and stop interacting with it.
    ///
    /// [`Handle::send`]: sirius_actor::Handle::send
    /// [`Handle::try_send`]: sirius_actor::Handle::try_send
    #[error("actor stopped")]
    Stopped,
}
