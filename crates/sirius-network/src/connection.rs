//! Per-connection task.
//!
//! Each accepted TCP socket gets its own [`Connection`], a tokio task that
//! owns that framed socket and drives the read/write loop. The connection
//! task has no knowledge of sessions, authentications or game state. It
//! reads [`RawPacket`]s from the wire and writes [`RawPacket`]s to the wire,
//! nothing more.
//!
//! The handoff to the session layer happens through channels: the connection
//! pushes inbound packets onto `inbound_tx`, and reads outbound packets from
//! `outbound_rx`. The session task owns the other ends of both channels.

use futures_util::{SinkExt, StreamExt};
use sirius_codec::{NitroCodec, RawPacket};
use sirius_error::{NetworkError, SiriusError};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use tokio_util::codec::Framed;
use tracing::{debug, trace, warn};

/// A unique identifier for a live connection.
///
/// Assigned at accept time and never reused within a server lifetime.
/// Used by the [`ConnectionManager`] to track and forcefully close
/// connections.
///
/// [`ConnectionManager`]: crate::ConnectionManager
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(u64);

impl ConnectionId {
    pub(crate) fn new(id: u64) -> Self {
        Self(id)
    }

    #[inline]
    pub fn inner(self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "conn:{}", self.0)
    }
}

/// Configuration for a single connection task.
#[derive(Clone)]
pub struct ConnectionConfig {
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub websocket_enabled: bool,
    pub websocket_path: String,
    pub websocket_ping_interval_secs: u64,
}

/// A live TCP connection, represented as a running tokio task.
///
/// Construct with [`Connection::spawn`]. The returned channels are the
/// only interface into the connection from outside — the task itself is
/// entirely self-contained.
pub struct Connection {
    pub id: ConnectionId,
    pub peer_addr: SocketAddr,
    /// Sender half — push packets here to write them to the client.
    pub outbound_tx: mpsc::Sender<RawPacket>,
    /// Receiver half — read packets the client sent.
    pub inbound_rx: mpsc::Receiver<RawPacket>,
}

impl Connection {
    /// Spawns a connection task for the given socket.
    ///
    /// Returns a [`Connection`] whose channels are connected to the spawned
    /// task. Drop the connection to close the underlying socket — the task
    /// will exit when the channel is closed.
    pub fn spawn(
        id: ConnectionId,
        stream: TcpStream,
        peer_addr: SocketAddr,
        config: ConnectionConfig,
        close_tx: mpsc::Sender<ConnectionId>,
    ) -> Self {
        let (inbound_tx, inbound_rx) = mpsc::channel(128);
        let (outbound_tx, outbound_rx) = mpsc::channel(128);

        tokio::spawn(run(
            id,
            stream,
            peer_addr,
            config,
            inbound_tx,
            outbound_rx,
            close_tx,
        ));

        Self {
            id,
            peer_addr,
            outbound_tx,
            inbound_rx,
        }
    }
}

async fn run(
    id: ConnectionId,
    stream: TcpStream,
    peer_addr: SocketAddr,
    config: ConnectionConfig,
    inbound_tx: mpsc::Sender<RawPacket>,
    outbound_rx: mpsc::Receiver<RawPacket>,
    close_tx: mpsc::Sender<ConnectionId>,
) {
    debug!(%id, %peer_addr, "connection accepted");

    let mut is_websocket = false;
    if config.websocket_enabled {
        let mut peek_buf = [0; 3];
        if let Ok(3) = stream.peek(&mut peek_buf).await {
            if &peek_buf == b"GET" {
                is_websocket = true;
            }
        }
    }

    if is_websocket {
        match sirius_websocket::accept_async(
            stream,
            &config.websocket_path,
            peer_addr,
        )
        .await
        {
            sirius_websocket::UpgradeResult::Success(ws_stream) => {
                let ws = sirius_websocket::WsStream::new(
                    ws_stream,
                    config.websocket_ping_interval_secs,
                );
                run_ws(id, ws, config, inbound_tx, outbound_rx).await;
            }
            sirius_websocket::UpgradeResult::Rejected
            | sirius_websocket::UpgradeResult::Failed => {
                debug!(%id, "websocket upgrade rejected or failed");
            }
        }
    } else {
        let framed = Framed::new(stream, NitroCodec::new());
        run_tcp(id, framed, config, inbound_tx, outbound_rx).await;
    }

    let _ = close_tx.send(id).await;
    debug!(%id, "connection task exited");
}

async fn run_tcp(
    id: ConnectionId,
    mut framed: Framed<TcpStream, NitroCodec>,
    config: ConnectionConfig,
    inbound_tx: mpsc::Sender<RawPacket>,
    mut outbound_rx: mpsc::Receiver<RawPacket>,
) {
    loop {
        tokio::select! {
            result = time::timeout(config.read_timeout, framed.next()) => {
                match result {
                    Err(_) => {
                        debug!(%id, "read timeout, closing connection");
                        break;
                    }
                    Ok(None) => {
                        debug!(%id, "connection closed by client");
                        break;
                    }
                    Ok(Some(Err(e))) => {
                        warn!(%id, error = %e, "codec error, closing connection");
                        break;
                    }
                    Ok(Some(Ok(packet))) => {
                        trace!(%id, header_id = packet.id(), "received packet");
                        if inbound_tx.send(packet).await.is_err() {
                            debug!(%id, "session dropped inbound channel, closing connection");
                            break;
                        }
                    }
                }
            }

            packet = outbound_rx.recv() => {
                match packet {
                    None => {
                        debug!(%id, "outbound channel closed, closing connection");
                        break;
                    }
                    Some(packet) => {
                        trace!(%id, header_id = packet.id(), "sending packet");
                        let send = time::timeout(config.write_timeout, framed.send(packet));
                        if let Err(e) = send.await
                            .map_err(|_| SiriusError::Network(NetworkError::Timeout {
                                seconds: config.write_timeout.as_secs(),
                            }))
                            .and_then(|r| r)
                        {
                            warn!(%id, error = %e, "write error, closing connection");
                            break;
                        }
                    }
                }
            }
        }
    }
}

async fn run_ws(
    id: ConnectionId,
    mut ws: sirius_websocket::WsStream,
    config: ConnectionConfig,
    inbound_tx: mpsc::Sender<RawPacket>,
    mut outbound_rx: mpsc::Receiver<RawPacket>,
) {
    loop {
        tokio::select! {
            result = ws.next(config.read_timeout) => {
                match result {
                    Err(e) => {
                        warn!(%id, error = %e, "websocket error, closing connection");
                        break;
                    }
                    Ok(None) => {
                        debug!(%id, "connection closed by client");
                        break;
                    }
                    Ok(Some(packet)) => {
                        trace!(%id, header_id = packet.id(), "received packet over ws");
                        if inbound_tx.send(packet).await.is_err() {
                            debug!(%id, "session dropped inbound channel, closing connection");
                            break;
                        }
                    }
                }
            }

            packet = outbound_rx.recv() => {
                match packet {
                    None => {
                        debug!(%id, "outbound channel closed, closing connection");
                        break;
                    }
                    Some(packet) => {
                        trace!(%id, header_id = packet.id(), "sending packet over ws");
                        let send = time::timeout(config.write_timeout, ws.send(packet));
                        if let Err(e) = send.await
                            .map_err(|_| SiriusError::Network(NetworkError::Timeout {
                                seconds: config.write_timeout.as_secs(),
                            }))
                            .and_then(|r| r)
                        {
                            warn!(%id, error = %e, "ws write error, closing connection");
                            break;
                        }
                    }
                }
            }
        }
    }
}
