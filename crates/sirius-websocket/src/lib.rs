//! WebSocket support for Sirius.
//!
//! Exposes HTTP upgrade handling and a WebSocket stream wrapper that understands
//! the Nitro packet framing format.

mod stream;
mod upgrade;

pub use stream::WsStream;
pub use upgrade::{UpgradeResult, accept_async};
