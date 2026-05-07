//! Connection registry.
//!
//! Tracks all live connections by [`ConnectionId`]. Used by the server to
//! enforce the connection limit, broadcast administrative disconnects and
//! get a count of active connections for metrics.

use crate::connection::ConnectionId;
use dashmap::DashMap;
use sirius_codec::RawPacket;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::debug;

/// A record stored for each live connection.
#[derive(Debug)]
struct ConnectionEntry {
    outbound_tx: mpsc::Sender<RawPacket>,
}

/// Registry of all live connections.
#[derive(Debug, Clone)]
pub struct ConnectionManager {
    inner: Arc<ConnectionManagerInner>,
}

#[derive(Debug)]
struct ConnectionManagerInner {
    connections: DashMap<ConnectionId, ConnectionEntry>,
    max_connections: usize,
    /// Counter used to assign unique [`ConnectionId`]s.
    next_id: std::sync::atomic::AtomicU64,
}

impl ConnectionManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            inner: Arc::new(ConnectionManagerInner {
                connections: DashMap::new(),
                max_connections,
                next_id: std::sync::atomic::AtomicU64::new(1),
            }),
        }
    }

    /// Allocates the next [`ConnectionId`].
    ///
    /// IDs are assigned sequentially and never reused within a server
    /// lifetime.
    pub fn next_id(&self) -> ConnectionId {
        let id = self
            .inner
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        ConnectionId::new(id.max(1))
    }

    /// Returns `true` if the server has capacity for another connection.
    pub fn has_capacity(&self) -> bool {
        self.inner.connections.len() < self.inner.max_connections
    }

    /// Returns the number of currently active connections.
    pub fn len(&self) -> usize {
        self.inner.connections.len()
    }

    /// Returns `true` if there are no active connections.
    pub fn is_empty(&self) -> bool {
        self.inner.connections.is_empty()
    }

    /// Registers a new connection.
    ///
    /// Called immediately after a connection is spawned. The `outbound_tx`
    /// is stored so the manager can push packets to the connection later.
    pub fn register(
        &self,
        id: ConnectionId,
        outbound_tx: mpsc::Sender<RawPacket>,
    ) {
        self.inner
            .connections
            .insert(id, ConnectionEntry { outbound_tx });
        debug!(%id, total = self.len(), "connection registered");
    }

    /// Removes a connection from the registry.
    ///
    /// Called when the connection task exits.
    pub fn unregister(&self, id: ConnectionId) {
        if self.inner.connections.remove(&id).is_some() {
            debug!(%id, total = self.len(), "connection unregistered");
        }
    }

    /// Forcefully closes a connection by dropping its outbound channel.
    ///
    /// The connection task will exit the next time it tries to read from
    /// the outbound channel. Use this for bans and administrative kicks.
    pub fn close(&self, id: ConnectionId) {
        if let Some((_, _entry)) = self.inner.connections.remove(&id) {
            // Dropping the entry drops the outbound_tx, which closes the
            // channel. The connection task detects this and shuts down.
            debug!(%id, "connection forcefully closed");
        }
    }
}

/// Starts a background task that drains the close notification channel
/// and unregisters connections as they exit.
///
/// `close_rx` is the receiving end of the channel that connection tasks
/// send their [`ConnectionId`] to when they exit. Pass the sending end
/// to each [`Connection::spawn`] call.
pub fn spawn_cleanup_task(
    manager: ConnectionManager,
    mut close_rx: mpsc::Receiver<ConnectionId>,
) {
    tokio::spawn(async move {
        while let Some(id) = close_rx.recv().await {
            manager.unregister(id);
        }
    });
}
