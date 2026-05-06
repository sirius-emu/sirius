use crate::handle::Handle;
use tokio::sync::mpsc;

/// The receiving end of an actor's message channel.
///
/// Owned by the actor task. Not accessible from outside the crate.
pub struct Mailbox<C> {
    rx: mpsc::Receiver<C>,
}

impl<C> Mailbox<C> {
    /// Creates a new mailbox with the given capacity.
    ///
    /// Returns the `Mailbox` (receiver side) and a `Handle` (sender side).
    /// The actor task owns the mailbox; callers keep the handle.
    pub(crate) fn new(capacity: usize) -> (Self, Handle<C>) {
        let (tx, rx) = mpsc::channel(capacity);
        (Self { rx }, Handle::new(tx))
    }

    /// Converts the mailbox into the underlying `mpsc::Receiver`.
    ///
    /// Called by `Actor::spawn` to drive the recv loop.
    pub(crate) fn into_receiver(self) -> mpsc::Receiver<C> {
        self.rx
    }
}
