//! TCP accept loop.
//!
//! [`Listener`] binds to the configured address and runs a loop that
//! accepts incoming connections. Each accepted socket is passed to a
//! provided callback — the server binary uses this callback to spawn a
//! session and wire it to the connection.
//!
//! The listener enforces the connection limit and per-IP rate limiting
//! before the callback is invoked. Rejected connections are closed
//! immediately with no response.

use crate::connection::{Connection, ConnectionConfig, ConnectionId};
use crate::limiter::RateLimiter;
use crate::manager::ConnectionManager;
use sirius_config::NetworkConfig;
use sirius_error::{NetworkError, SiriusError};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::Duration;
use tracing::{error, info, warn};

/// The TCP listener and accept loop.
pub struct Listener {
    tcp: TcpListener,
    manager: ConnectionManager,
    limiter: RateLimiter,
    conn_config: Arc<ConnectionConfig>,
    close_tx: mpsc::Sender<ConnectionId>,
}

impl Listener {
    /// Binds to the given address and prepares to accept connections.
    ///
    /// Does not start accepting until [`run`] is called.
    ///
    /// [`run`]: Listener::run
    pub async fn bind(
        addr: SocketAddr,
        network_config: &NetworkConfig,
        manager: ConnectionManager,
        close_tx: mpsc::Sender<ConnectionId>,
    ) -> Result<Self, SiriusError> {
        let tcp = TcpListener::bind(addr)
            .await
            .map_err(|e| SiriusError::Network(NetworkError::Io(e)))?;

        let local_addr = tcp
            .local_addr()
            .map_err(|e| SiriusError::Network(NetworkError::Io(e)))?;

        info!(%local_addr, "TCP listener bound");

        let limiter = RateLimiter::new(network_config.rate_limit_per_ip);

        let conn_config = Arc::new(ConnectionConfig {
            read_timeout: Duration::from_secs(network_config.read_timeout_secs),
            write_timeout: Duration::from_secs(network_config.write_timeout_secs),
            websocket_enabled: network_config.websocket_enabled,
            websocket_path: network_config.websocket_path.clone(),
            websocket_ping_interval_secs: network_config.websocket_ping_interval_secs,
        });

        Ok(Self {
            tcp,
            manager,
            limiter,
            conn_config,
            close_tx,
        })
    }

    /// Runs the accept loop.
    ///
    /// Calls `on_accept` for each accepted connection. The callback receives
    /// a fully constructed [`Connection`], channels are live, the task is
    /// running. The callback is responsible for spawning the session and
    /// doing whatever wiring is needed.
    ///
    /// This method runs until `shutdown` is signalled or a fatal bind error
    /// occurs.
    pub async fn run<F, Fut>(&self, mut shutdown: tokio::sync::watch::Receiver<bool>, on_accept: F)
    where
        F: Fn(Connection) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        loop {
            tokio::select! {
                _ = shutdown.changed() => {
                    if *shutdown.borrow() {
                        info!("listener received shutdown signal");
                        break;
                    }
                }

                result = self.tcp.accept() => {
                    match result {
                        Err(e) => {
                            // Most accept errors are transient (e.g. too many
                            // open file descriptors). Log and continue rather
                            // than crashing the loop.
                            error!(error = %e, "accept error");
                            continue;
                        }
                        Ok((stream, peer_addr)) => {
                            self.handle_accept(stream, peer_addr, &on_accept).await;
                        }
                    }
                }
            }
        }
    }

    async fn handle_accept<F, Fut>(&self, stream: TcpStream, peer_addr: SocketAddr, on_accept: &F)
    where
        F: Fn(Connection) -> Fut,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let ip = peer_addr.ip();

        if !self.manager.has_capacity() {
            warn!(%peer_addr, "connection limit reached, rejecting");
            return;
        }

        if !self.limiter.check(ip) {
            warn!(%peer_addr, "rate limit exceeded, rejecting");
            return;
        }

        let id = self.manager.next_id();

        let connection = Connection::spawn(
            id,
            stream,
            peer_addr,
            ConnectionConfig {
                read_timeout: self.conn_config.read_timeout,
                write_timeout: self.conn_config.write_timeout,
                websocket_enabled: self.conn_config.websocket_enabled,
                websocket_path: self.conn_config.websocket_path.clone(),
                websocket_ping_interval_secs: self.conn_config.websocket_ping_interval_secs,
            },
            self.close_tx.clone(),
        );

        self.manager.register(id, connection.outbound_tx.clone());

        on_accept(connection).await;
    }
}
