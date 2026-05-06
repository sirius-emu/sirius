use sirius_error::{ActorError, SiriusError};
use tokio::sync::mpsc;

/// A cloneable reference to a running actor's mailbox.
///
/// `Handle<C>` wraps an `mpsc::Sender<C>` and is the sole public interface
/// to an actor. Cloning a handle creates another reference to the same mailbox.
///
/// When all handles to an actor are dropped, the actor's mailbox closes and
/// the actor runs its `on_stop` hook before exiting.
///
/// # Sending messages
///
/// [`send`] is the standard path. It waits if the mailbox is full and returns
/// an error if the actor has stopped.
///
/// [`try_send`] returns immediately: `Ok(())` if the message was queued,
/// `Err` if the mailbox is full or the actor has stopped. Use this for non-critical
/// messages where dropping is acceptable.
///
/// [`send`]: Handle::send
/// [`try_send`]: Handle::try_send
#[derive(Debug)]
pub struct Handle<C> {
    tx: mpsc::Sender<C>,
}

impl<C> Handle<C> {
    pub(crate) fn new(tx: mpsc::Sender<C>) -> Self {
        Self { tx }
    }

    /// Sends a command to the actor, waiting if the mailbox is full.
    ///
    /// # Errors
    ///
    /// Returns [`ActorError::Stopped`] if the actor has stopped and its
    /// mailbox is closed.
    pub async fn send(&self, cmd: C) -> Result<(), SiriusError> {
        self.tx
            .send(cmd)
            .await
            .map_err(|_| SiriusError::Actor(ActorError::Stopped))
    }

    /// Sends a command without waiting.
    ///
    /// Returns an error if the mailbox is full or the actor has stopped.
    /// Unlike [`send`], this never yields.
    ///
    /// # Errors
    ///
    /// Returns [`ActorError::Stopped`] if the actor has stopped, or
    /// [`ActorError::Stopped`] if the mailbox is currently full.
    ///
    /// [`send`]: Handle::send
    pub fn try_send(&self, cmd: C) -> Result<(), SiriusError> {
        self.tx
            .try_send(cmd)
            .map_err(|_| SiriusError::Actor(ActorError::Stopped))
    }

    /// Returns `true` if the actor is still running.
    ///
    /// This is inherently racy. The actor could stop between this call and
    /// the next `send`. Use it only for cheap early-exit checks, not as a
    /// correctness guarantee.
    #[must_use]
    #[inline]
    pub fn is_alive(&self) -> bool {
        !self.tx.is_closed()
    }
}

impl<C> Clone for Handle<C> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
