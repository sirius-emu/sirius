//! Authentication state machine.

use sirius_types::UserId;

/// The authentication state of a session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthState {
    /// TCP connection established, waiting for SSO ticket.
    Unauthenticated,

    /// SSO ticket received, validation in progress.
    ///
    /// The session stays in this state while ticket is being checked.
    Authenticating,

    /// Ticket validated, [`UserId`] is now bound to this session.
    Authenticated(UserId),

    /// The session is shutting down. No further packets will be processed.
    Closing,
}

impl AuthState {
    /// Returns `true` if the session has completed authentication.
    #[inline]
    pub fn is_authenticated(&self) -> bool {
        matches!(self, Self::Authenticated(_))
    }

    /// Returns the [`UserId`] if the session is authenticated.
    #[inline]
    pub fn user_id(&self) -> Option<UserId> {
        match self {
            Self::Authenticated(id) => Some(*id),
            _ => None,
        }
    }

    /// Returns `true` if the session is still able to process packets.
    #[inline]
    pub fn is_active(&self) -> bool {
        !matches!(self, Self::Closing)
    }
}

impl std::fmt::Display for AuthState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unauthenticated => write!(f, "Unauthenticated"),
            Self::Authenticating => write!(f, "Authenticating"),
            Self::Authenticated(id) => write!(f, "Authenticated({})", id),
            Self::Closing => write!(f, "Closing"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthenticated_is_not_authenticated() {
        assert!(!AuthState::Unauthenticated.is_authenticated());
        assert!(AuthState::Unauthenticated.user_id().is_none());
    }

    #[test]
    fn authenticated_state() {
        let uid = UserId::from(42);
        let state = AuthState::Authenticated(uid);
        assert!(state.is_authenticated());
        assert_eq!(state.user_id(), Some(uid));
        assert!(state.is_active());
    }

    #[test]
    fn closing_is_not_active() {
        assert!(!AuthState::Closing.is_active());
    }
}
