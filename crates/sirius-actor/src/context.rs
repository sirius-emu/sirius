use crate::handle::Handle;

/// Passed to [`Actor::handle`], [`Actor::on_start`], and [`Actor::on_stop`].
///
/// Gives the actor a reference to its own handle, which enables two things:
/// 1. **Self-messaging:** the actor can send commands to itself. This is the standard way to
/// schedule deferred work or implement internal ticks without an external timer.
/// 2. **Handle sharing:** the actor can hand its own handle to callbacks, futures or
/// spawned subtasks that need to send it messages later.
///
/// # Example
/// ```no_run
/// use sirius_actor::{Actor, ActorContext};
/// use sirius_error::SiriusError;
/// enum RoomCommand { Tick }
/// struct Room;
/// impl Actor for Room {
///     type Command = RoomCommand;
///
///     async fn handle(
///         &mut self,
///         cmd: RoomCommand,
///         ctx: &ActorContext<RoomCommand>,
///     ) -> Result<(), SiriusError> {
///         match cmd {
///             RoomCommand::Tick => {
///                 // Schedule the next tick.
///                 let handle = ctx.handle().clone();
///                 tokio::spawn(async move {
///                     tokio::time::sleep(std::time::Duration::from_millis(500)).await;
///                     let _ = handle.try_send(RoomCommand::Tick);
///                 });
///             }
///         }
///         Ok(())
///     }
/// }
/// ```
///
/// [`Actor::handle`]: crate::Actor::handle
/// [`Actor::on_start`]: crate::Actor::on_start
/// [`Actor::on_stop`]: crate::Actor::on_stop
/// ```
#[derive(Debug)]
pub struct ActorContext<C> {
    handle: Handle<C>,
}

impl<C> ActorContext<C> {
    pub(crate) fn new(handle: Handle<C>) -> Self {
        Self { handle }
    }

    /// Returns a reference to the actor's own handle.
    ///
    /// Clone this if you need to move it into a spawned task or a closure.
    #[inline]
    pub fn handle(&self) -> &Handle<C> {
        &self.handle
    }
}
