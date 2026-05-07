//! The session actor.

use crate::{AuthState, SessionCommand};
use sirius_actor::{Actor, ActorContext};
use sirius_codec::RawPacket;
use sirius_error::SiriusError;
use sirius_network::{Connection, ConnectionId};
use sirius_packets::IncomingPacket;
use sirius_packets::{OutgoingPacket, ReleaseVersionEvent};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

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
}

impl Session {
    /// Creates a session from an accepted [`Connection`].
    ///
    /// Consumes the connection. The session takes ownership of the outbound
    /// sender and spawns a separate task to pump inbound packets into the
    /// session's mailbox.
    pub fn from_connection(connection: Connection) -> (Self, mpsc::Receiver<RawPacket>) {
        let session = Self {
            id: connection.id,
            peer_addr: connection.peer_addr,
            auth_state: AuthState::Unauthenticated,
            outbound_tx: connection.outbound_tx,
        };

        (session, connection.inbound_rx)
    }

    /// Sends a packet to the client.
    async fn send(&self, packet: RawPacket) {
        if self.outbound_tx.send(packet).await.is_err() {
            warn!(id = %self.id, "outbound channel closed while sending packet");
        }
    }

    /// Sends an outgoing packet composer to the client.
    async fn compose<P: OutgoingPacket>(&self, composer: &P) -> Result<(), SiriusError> {
        let packet = composer.to_raw()?;
        self.send(packet).await;
        Ok(())
    }

    async fn handle_inbound(
        &mut self,
        raw: RawPacket,
        _ctx: &ActorContext<SessionCommand>,
    ) -> Result<(), SiriusError> {
        let header_id = raw.id();

        match header_id {
            ReleaseVersionEvent::HEADER_ID => self.on_release_version(raw).await,
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

    async fn on_release_version(&mut self, raw: RawPacket) -> Result<(), SiriusError> {
        let packet = ReleaseVersionEvent::from_raw(raw)?;
        info!(
            id = %self.id,
            release = %packet.release_version,
            "client release version"
        );

        Ok(())
    }
}

impl Actor for Session {
    type Command = SessionCommand;

    async fn on_start(&mut self, _ctx: &ActorContext<Self::Command>) -> Result<(), SiriusError> {
        info!(id = %self.id, peer = %self.peer_addr, "session started");

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
                self.send(packet).await;
            }

            SessionCommand::Close { reason } => {
                info!(id = %self.id, %reason, "session closing");
                self.auth_state = AuthState::Closing;
                return Err(SiriusError::Auth(sirius_error::AuthError::NotAuthenticated));
            }

            _ => {}
        }

        Ok(())
    }

    async fn on_stop(&mut self, _ctx: &ActorContext<Self::Command>) -> Result<(), SiriusError> {
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
pub fn spawn_session(connection: Connection) -> crate::SessionHandle {
    let (session, mut inbound_rx) = Session::from_connection(connection);
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
