mod command;
mod user;

pub use command::UserCommand;
pub use user::UserActor;

use sirius_actor::Handle;
pub type UserHandle = Handle<UserCommand>;
