//! Session registry.
//!
//! Tracks all **authenticated** sessions by [`UserId`]. User for server-side
//! operations.
//!
//! Unauthenticated sessions are not registered here, they only appear in `sirius-network`s
//! [`ConnectionManager`]. A session is registered once it transitions to [`AuthState::Authenticated`]
//! and unregistered when it spots.

use crate::SessionHandle;
use dashmap::DashMap;
use sirius_codec::RawPacket;
use sirius_types::UserId;
use std::sync::Arc;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct SessionManager {
    inner: Arc<SessionManagerInner>,
}

#[derive(Debug)]
struct SessionManagerInner {
    sessions: DashMap<UserId, SessionHandle>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SessionManagerInner {
                sessions: DashMap::new(),
            }),
        }
    }

    /// Registers an authenticated session.
    pub fn register(&self, user_id: UserId, handle: SessionHandle) {
        self.inner.sessions.insert(user_id, handle);
        debug!(%user_id, total = self.len(), "session registered");
    }

    /// Unregisters a session.
    pub fn unregister(&self, user_id: UserId) {
        if self.inner.sessions.remove(&user_id).is_some() {
            debug!(%user_id, total = self.len(), "session unregistered");
        }
    }

    /// Returns a clone of the handle for the given user, if online.
    pub fn get(&self, user_id: UserId) -> Option<SessionHandle> {
        self.inner.sessions.get(&user_id).map(|h| h.clone())
    }

    /// Returns `true` if the user is currently connected and authenticated.
    pub fn is_online(&self, user_id: UserId) -> bool {
        self.inner.sessions.contains_key(&user_id)
    }

    /// Returns the number of authenticated sessions.
    pub fn len(&self) -> usize {
        self.inner.sessions.len()
    }

    /// Returns `true` if there are no authenticated sessions.
    pub fn is_empty(&self) -> bool {
        self.inner.sessions.is_empty()
    }

    /// Sends a packet to a specific user.
    ///
    /// Returns `false` if the user is not online or if sending fails.
    pub async fn send_to(&self, user_id: UserId, packet: RawPacket) -> bool {
        if let Some(handle) = self.get(user_id) {
            handle
                .send(crate::SessionCommand::SendPacket(packet))
                .await
                .is_ok()
        } else {
            false
        }
    }

    /// Kicks a user by closing their session.
    ///
    /// No-op if the user is not online.
    pub async fn kick(&self, user_id: UserId, reason: impl Into<String>) {
        if let Some(handle) = self.get(user_id) {
            let _ = handle
                .send(crate::SessionCommand::Close {
                    reason: reason.into(),
                })
                .await;
        }
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
