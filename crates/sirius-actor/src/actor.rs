use crate::context::ActorContext;
use crate::handle::Handle;
use crate::mailbox::Mailbox;
use sirius_error::SiriusError;
use tracing::{debug, error};

/// A stateful entity that processes commands sequentially from a mailbox.
///
/// Implement this trait on your state struct. The runtime calls [`handle`] for each message in
/// arrival order. Because messages are processed one at a time, `handle` has exclusive mutable
/// access to `self`.
///
/// # Lifecycle hooks
///
/// [`on_start`] is called once before the first message is processed.
/// [`on_stop`] is called once after the mailbox is closed and all pending
/// messages have been drained. Both have default no-op implementations.
///
/// # Stopping
///
/// An actor stops when:
/// - Its mailbox is closed (all [`Handle`]s have been dropped)
/// - [`handle`] returns an error
///
/// [`handle`]: Actor::handle
/// [`on_start`]: Actor::on_start
/// [`on_stop`]: Actor::on_stop
pub trait Actor: Sized + Send + 'static {
    /// The command type this actor accepts.
    ///
    /// Use an enum. Each variant is a distinct message the actor can receive.
    type Command: Send + 'static;

    /// Processes a single command.
    ///
    /// Called with exclusive mutable access to `self`. Do not block here, spawn a task
    /// if you need to do async work that shouldn't hold up the next message.
    fn handle(
        &mut self,
        cmd: Self::Command,
        ctx: &ActorContext<Self::Command>,
    ) -> impl Future<Output = Result<(), SiriusError>> + Send;

    /// Called once before the actor begins processing messages.
    ///
    /// Use this to initialize resources that require async setup (e.g. loading
    /// state from the database). If this returns an error, the actor exits
    /// immediately without processing any messages.
    fn on_start(
        &mut self,
        _ctx: &ActorContext<Self::Command>,
    ) -> impl Future<Output = Result<(), SiriusError>> + Send {
        async { Ok(()) }
    }

    /// Called once after the mailbox is closed and drained.
    ///
    /// Use this to flush state, close connections or release resources.
    /// The return value is logged if it's an error, but does not propagate.
    fn on_stop(
        &mut self,
        _ctx: &ActorContext<Self::Command>,
    ) -> impl Future<Output = Result<(), SiriusError>> + Send {
        async { Ok(()) }
    }

    /// Spawns this actor as a tokio task and returns a [`Handle`] to it.
    ///
    /// This is the standard way to start an actor. The task runs until the mailbox
    /// is closed or `handle` returns an error.
    ///
    /// `mailbox_size` is the bound on the `mpsc` channel. Messages sent when the mailbox
    /// is full will wait until the space is available. Size this generously for actors that
    /// receive bursts.
    fn spawn(mut self, mailbox_size: usize) -> Handle<Self::Command> {
        let (mailbox, handle) = Mailbox::new(mailbox_size);
        let ctx = ActorContext::new(handle.clone());

        let task = tokio::spawn(async move {
            if let Err(e) = self.on_start(&ctx).await {
                error!(error = %e, "actor failed during on_start, shutting down");
                let mut rx = mailbox.into_receiver();
                rx.close();
                while rx.recv().await.is_some() {}
                return;
            }

            debug!("actor started");

            let mut rx = mailbox.into_receiver();

            while let Some(cmd) = rx.recv().await {
                if let Err(e) = self.handle(cmd, &ctx).await {
                    error!(error = %e, "actor encountered a fatal error, shutting down");
                    break;
                }
            }

            // Drain any remaining messages without processing them so senders
            // don't block waiting for capacity that will never open up.
            rx.close();
            while rx.recv().await.is_some() {}

            if let Err(e) = self.on_stop(&ctx).await {
                error!(error = %e, "actor encountered an error during on_stop");
            }

            debug!("actor stopped");
        });

        // Surface panics as errors instead of silently swallowing them.
        // A panicking actor task would otherwise disappear with no log output.
        tokio::spawn(async move {
            if let Err(e) = task.await
                && e.is_panic()
            {
                error!("actor task panicked");
            }
        });

        handle
    }
}
