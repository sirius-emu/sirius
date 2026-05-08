//! The user actor.

use sirius_actor::{Actor, ActorContext};
use sirius_codec::RawPacket;
use sirius_error::{AuthError, SiriusError};
use sirius_packets::OutgoingPacket;
use sirius_packets::outgoing::handshake::UserInfoComposer;
use sirius_repository::models::User;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use crate::UserCommand;

pub struct UserActor {
    user: User,
    outbound_tx: mpsc::Sender<RawPacket>,
}

impl UserActor {
    pub fn new(user: User, outbound_tx: mpsc::Sender<RawPacket>) -> Self {
        Self { user, outbound_tx }
    }

    async fn send(&self, packet: RawPacket) -> Result<(), SiriusError> {
        self.outbound_tx.send(packet).await.map_err(|_| {
            warn!(user_id = %self.user.id, "outbound channel closed");
            SiriusError::Network(sirius_error::NetworkError::ConnectionClosed)
        })
    }

    async fn compose<P: OutgoingPacket>(
        &self,
        composer: &P,
    ) -> Result<(), SiriusError> {
        self.send(composer.to_raw()?).await
    }

    async fn on_get_user_info(&self) -> Result<(), SiriusError> {
        debug!("user info!!");
        self.compose(&UserInfoComposer::new(self.user.clone()))
            .await
    }
}

impl Actor for UserActor {
    type Command = UserCommand;

    async fn on_start(
        &mut self,
        _ctx: &ActorContext<Self::Command>,
    ) -> Result<(), SiriusError> {
        info!(user_id = %self.user.id, username = %self.user.username, "user actor started");

        Ok(())
    }

    async fn handle(
        &mut self,
        cmd: Self::Command,
        _ctx: &ActorContext<Self::Command>,
    ) -> Result<(), SiriusError> {
        match cmd {
            UserCommand::GetUserInfo => self.on_get_user_info().await?,
            UserCommand::Disconnect => {
                return Err(SiriusError::Auth(AuthError::NotAuthenticated));
            }
            _ => todo!(),
        }

        Ok(())
    }

    async fn on_stop(
        &mut self,
        _ctx: &ActorContext<Self::Command>,
    ) -> Result<(), SiriusError> {
        info!(user_id = %self.user.id, "user actor stopped");
        Ok(())
    }
}
