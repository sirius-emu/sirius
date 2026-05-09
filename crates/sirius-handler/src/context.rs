//! Handle execution context.

use sirius_codec::RawPacket;
use sirius_error::SiriusError;
use sirius_navigator::NavigatorService;
use sirius_types::UserId;
use sirius_user::UserHandle;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Dependencies available to every packet handler.
#[derive(Clone)]
pub struct HandlerContext {
    pub outbound_tx: mpsc::Sender<RawPacket>,
    pub user: Arc<AuthenticatedUser>,
    pub user_actor: UserHandle,
    pub navigator: Arc<NavigatorService>,
}

/// The subset of user data that handlers commonly need.
///
/// A lightweight snapshot taken when the session transitions to
/// `Authenticated`. Avoids passing the full `User` struct everywhere.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: UserId,
    pub username: String,
    pub rank: i32,
}

impl HandlerContext {
    pub fn new(
        outbound_tx: mpsc::Sender<RawPacket>,
        user: Arc<AuthenticatedUser>,
        user_actor: UserHandle,
        navigator: Arc<NavigatorService>,
    ) -> Self {
        Self {
            outbound_tx,
            user,
            user_actor,
            navigator,
        }
    }

    /// Sends a packet to the client.
    ///
    /// Shorthand for `self.session.send(SessionCommand::SendPacket(packet))`.
    pub async fn send(&self, packet: RawPacket) -> Result<(), SiriusError> {
        self.outbound_tx.send(packet).await.map_err(|_| {
            SiriusError::Network(sirius_error::NetworkError::ConnectionClosed)
        })
    }

    /// Sends an outgoing packet composer to the client.
    pub async fn compose<P>(&self, composer: &P) -> Result<(), SiriusError>
    where
        P: sirius_packets::OutgoingPacket,
    {
        let packet = composer.to_raw()?;
        self.send(packet).await
    }
}
