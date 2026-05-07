//! Commands the session actor accepts.
//!
//! All interaction with a running session goes through one of those variants.

use sirius_codec::RawPacket;
use sirius_types::UserId;

#[derive(Debug)]
pub enum SessionCommand {
    /// An inbound packet arrived from the client.
    InboundPacket(RawPacket),

    /// Send a packet to the client.
    SendPacket(RawPacket),

    /// Forcefully closes this session.
    ///
    /// Used for bans, kicks and server shutdown.
    Close {
        reason: String,
    },

    /// Notifies the session that authentication completed successfully.
    AuthSuccess {
        user_id: UserId,
    },

    /// Notifies the session that authentication failed.
    AuthFailure {
        reason: String,
    },

    SendPing,
}
