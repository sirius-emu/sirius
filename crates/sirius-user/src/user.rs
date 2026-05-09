//! The user actor.

use std::sync::Arc;

use sirius_actor::{Actor, ActorContext};
use sirius_codec::RawPacket;
use sirius_error::SiriusError;
use sirius_packets::OutgoingPacket;
use sirius_permissions::PermissionsManager;
use sirius_repository::models::User;
use sirius_types::CurrencyType;
use tokio::sync::mpsc;
use tracing::{info, warn};

use sirius_packets::outgoing::{
    handshake::UserInfoComposer,
    user::{
        UserCreditsComposer, UserCurrencyComposer, UserPermissionsComposer,
        UserSettingsComposer,
    },
};

use crate::UserCommand;

pub struct UserActor {
    user: User,
    outbound_tx: mpsc::Sender<RawPacket>,
    permissions: Arc<PermissionsManager>,
}

impl UserActor {
    pub fn new(
        user: User,
        outbound_tx: mpsc::Sender<RawPacket>,
        permissions: Arc<PermissionsManager>,
    ) -> Self {
        Self {
            user,
            outbound_tx,
            permissions,
        }
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
        self.compose(&UserInfoComposer::new(self.user.clone()))
            .await
    }

    pub async fn on_send_initial_data(&self) -> Result<(), SiriusError> {
        self.compose(&UserCreditsComposer::new(self.user.credits))
            .await?;

        self.compose(&UserSettingsComposer::new(self.user.settings.clone()))
            .await?;

        let rank = self.permissions.get_rank(self.user.rank);
        let is_ambassador =
            rank.as_ref().map(|r| r.is_ambassador()).unwrap_or(false);

        self.compose(&UserPermissionsComposer::new(
            0,
            self.user.rank,
            is_ambassador,
        ))
        .await?;

        Ok(())
    }

    async fn on_get_currency(&self) -> Result<(), SiriusError> {
        let pixels = self
            .user
            .currencies
            .get(&CurrencyType::Pixels)
            .copied()
            .unwrap_or(0);

        let diamonds = self
            .user
            .currencies
            .get(&CurrencyType::Diamonds)
            .copied()
            .unwrap_or(0);

        self.compose(&UserCurrencyComposer::new(pixels, diamonds))
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
            UserCommand::SendInitialData => self.on_send_initial_data().await?,
            UserCommand::GetCurrency => self.on_get_currency().await?,
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
