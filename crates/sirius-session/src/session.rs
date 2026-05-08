//! The session actor.

use crate::{AuthState, SessionCommand, SessionManager};
use sirius_actor::{Actor, ActorContext};
use sirius_codec::RawPacket;
use sirius_error::SiriusError;
use sirius_network::{Connection, ConnectionId};
use sirius_packets::{IncomingPacket, OutgoingPacket};
use sirius_repository::Repository;
use sirius_repository::models::User;
use std::net::SocketAddr;
use std::time::Instant;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

use sirius_packets::incoming::handshake::{
    ClientHelloPacket, PingPacket, PongPacket, SsoTicketPacket,
};
use sirius_packets::outgoing::availability::AvailabilityStatusComposer;
use sirius_packets::outgoing::handshake::{
    AuthenticatedComposer, PingComposer, PongComposer, UserInfoComposer,
};

/// The session actor state.
///
/// Owns the connection's outbound sender and drives the auth state machine.
/// All inbound packets arrive as [`SessionCommand::InboundPacket`] and are
/// dispatched here.
pub struct Session {
    pub id: ConnectionId,
    pub peer_addr: SocketAddr,
    pub auth_state: AuthState,
    outbound_tx: mpsc::Sender<RawPacket>,
    last_ping_at: Option<Instant>,
    manager: SessionManager,
    repo: Repository,
}

impl Session {
    /// Creates a session from an accepted [`Connection`].
    ///
    /// Consumes the connection. The session takes ownership of the outbound
    /// sender and spawns a separate task to pump inbound packets into the
    /// session's mailbox.
    pub fn from_connection(
        connection: Connection,
        manager: SessionManager,
        repo: Repository,
    ) -> (Self, mpsc::Receiver<RawPacket>) {
        let session = Self {
            id: connection.id,
            peer_addr: connection.peer_addr,
            auth_state: AuthState::Unauthenticated,
            outbound_tx: connection.outbound_tx,
            last_ping_at: None,
            manager,
            repo,
        };

        (session, connection.inbound_rx)
    }

    /// Sends a packet to the client.
    async fn send(&self, packet: RawPacket) -> Result<(), SiriusError> {
        self.outbound_tx.send(packet).await.map_err(|_| {
            warn!(id = %self.id, "outbound channel closed while sending packet");
            SiriusError::Network(sirius_error::NetworkError::ConnectionClosed)
        })
    }

    /// Sends an outgoing packet composer to the client.
    async fn compose<P: OutgoingPacket>(
        &self,
        composer: &P,
    ) -> Result<(), SiriusError> {
        let packet = composer.to_raw()?;
        self.send(packet).await?;
        Ok(())
    }

    async fn handle_inbound(
        &mut self,
        raw: RawPacket,
        ctx: &ActorContext<SessionCommand>,
    ) -> Result<(), SiriusError> {
        let header_id = raw.id();

        match header_id {
            ClientHelloPacket::HEADER_ID => self.on_client_hello(raw).await,
            PongPacket::HEADER_ID => self.on_pong().await,
            PingPacket::HEADER_ID => self.on_ping(raw).await,
            SsoTicketPacket::HEADER_ID => self.on_sso_ticket(raw, ctx).await,
            _ => {
                // Unknown or not-yet-implemented packet.
                debug!(
                    id = %self.id,
                    header_id,
                    state = %self.auth_state,
                    "unhandled packet"
                );
                Ok(())
            }
        }
    }

    async fn on_client_hello(
        &mut self,
        raw: RawPacket,
    ) -> Result<(), SiriusError> {
        let packet = ClientHelloPacket::from_raw(raw)?;
        info!(
            id = %self.id,
            release = %packet.release_version,
            "client hello"
        );

        Ok(())
    }

    async fn on_pong(&mut self) -> Result<(), SiriusError> {
        debug!(id = %self.id, "pong received");
        self.last_ping_at = None;
        Ok(())
    }

    async fn on_ping(&mut self, raw: RawPacket) -> Result<(), SiriusError> {
        let packet = PingPacket::from_raw(raw)?;

        debug!(id = %self.id, ping_id = packet.id, "ping received, sending pong");
        self.compose(&PongComposer::new(packet.id)).await
    }

    async fn on_send_ping(&mut self) -> Result<(), SiriusError> {
        self.compose(&PingComposer).await?;

        if self.last_ping_at.is_none() {
            self.last_ping_at = Some(Instant::now());
        }

        debug!(id = %self.id, "ping sent");
        Ok(())
    }

    fn check_ping_timeout(&self) -> bool {
        if let Some(sent_at) = self.last_ping_at {
            sent_at.elapsed() > std::time::Duration::from_secs(60)
        } else {
            false
        }
    }

    async fn on_sso_ticket(
        &mut self,
        raw: RawPacket,
        ctx: &ActorContext<SessionCommand>,
    ) -> Result<(), SiriusError> {
        if !matches!(self.auth_state, AuthState::Unauthenticated) {
            warn!(id = %self.id, "received SsoTicket in wrong state, ignoring");
            return Ok(());
        }

        let packet = SsoTicketPacket::from_raw(raw)?;
        debug!(id = %self.id, ticket = %packet.ticket, "received SSO ticket, validating");

        self.auth_state = AuthState::Authenticating;

        let handle = ctx.handle().clone();
        let repo = self.repo.clone();
        let ticket = packet.ticket;

        tokio::spawn(async move {
            match sirius_handshake::authenticate(ticket, &repo).await {
                Ok(user) => {
                    let _ =
                        handle.send(SessionCommand::AuthSuccess { user }).await;
                }
                Err(e) => {
                    let _ = handle
                        .send(SessionCommand::AuthFailure {
                            reason: e.to_string(),
                        })
                        .await;
                }
            }
        });

        Ok(())
    }

    async fn on_auth_success(
        &mut self,
        user: User,
        ctx: &ActorContext<SessionCommand>,
    ) -> Result<(), SiriusError> {
        let user_id = user.id;
        self.auth_state = AuthState::Authenticated(user_id);

        info!(
            id = %self.id,
            %user_id,
            username = %user.username,
            peer = %self.peer_addr,
            "session authenticated"
        );

        self.manager.register(user_id, ctx.handle().clone());

        self.compose(&AuthenticatedComposer).await?;
        self.compose(&AvailabilityStatusComposer::new(false, false, false))
            .await?;
        self.compose(&UserInfoComposer::new(user)).await?;

        Ok(())
    }

    async fn on_auth_failure(
        &mut self,
        reason: &str,
    ) -> Result<(), SiriusError> {
        warn!(id = %self.id, %reason, "authentication failed, closing session");
        self.auth_state = AuthState::Closing;
        Ok(())
    }
}

impl Actor for Session {
    type Command = SessionCommand;

    async fn on_start(
        &mut self,
        ctx: &ActorContext<Self::Command>,
    ) -> Result<(), SiriusError> {
        info!(id = %self.id, peer = %self.peer_addr, "session started");

        let handle = ctx.handle().clone();
        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(30));

            interval.tick().await;
            loop {
                interval.tick().await;
                if handle.send(SessionCommand::SendPing).await.is_err() {
                    break;
                }
            }
        });

        Ok(())
    }

    async fn handle(
        &mut self,
        cmd: Self::Command,
        ctx: &ActorContext<Self::Command>,
    ) -> Result<(), SiriusError> {
        if !self.auth_state.is_active() {
            return Ok(());
        }

        match cmd {
            SessionCommand::InboundPacket(packet) => {
                self.handle_inbound(packet, ctx).await?;
            }
            SessionCommand::SendPacket(packet) => {
                self.send(packet).await?;
            }
            SessionCommand::AuthSuccess { user } => {
                self.on_auth_success(user, ctx).await?;
            }
            SessionCommand::AuthFailure { reason } => {
                self.on_auth_failure(&reason).await?;
            }
            SessionCommand::Close { reason } => {
                info!(id = %self.id, %reason, "session closing");
                self.auth_state = AuthState::Closing;
                return Err(SiriusError::Auth(
                    sirius_error::AuthError::NotAuthenticated,
                ));
            }
            SessionCommand::SendPing => {
                if self.check_ping_timeout() {
                    warn!(id = %self.id, "ping timeout, closing session");
                    self.auth_state = AuthState::Closing;
                    return Err(SiriusError::Auth(
                        sirius_error::AuthError::NotAuthenticated,
                    ));
                }
                self.on_send_ping().await?;
            }
        }

        Ok(())
    }

    async fn on_stop(
        &mut self,
        _ctx: &ActorContext<Self::Command>,
    ) -> Result<(), SiriusError> {
        if let Some(user_id) = self.auth_state.user_id() {
            self.manager.unregister(user_id);
        }

        info!(id = %self.id, state = %self.auth_state, "session stopped");
        Ok(())
    }
}

/// Spawns a session from a [`Connection`] and returns its handle.
///
/// Internally this:
/// 1. Constructs the [`Session`] actor from the connection.
/// 2. Spawns the actor (which starts the `on_start` hook and message loop).
/// 3. Spawns a pump task that reads from `inbound_rx` and forwards each packet
///    as a [`SessionCommand::InboundPacket`].
pub fn spawn_session(
    connection: Connection,
    manager: SessionManager,
    repo: Repository,
) -> crate::SessionHandle {
    let (session, mut inbound_rx) =
        Session::from_connection(connection, manager, repo);
    let handle = session.spawn(256);
    let pump_handle = handle.clone();

    tokio::spawn(async move {
        while let Some(packet) = inbound_rx.recv().await {
            if pump_handle
                .send(SessionCommand::InboundPacket(packet))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    handle
}
