//! Errors originating in the authentication and handshake layers.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    /// The SSO ticket is syntactically invalid or has an unrecognized format.
    #[error("malformed SSO ticket")]
    MalformedTicket,

    /// The SSO ticket is syntactically valid but not found in the ticket store.
    ///
    /// This covers both genuinely unknown tickets and ones that were already consumed.
    /// Don't distinguish between the two in client-facing messages, doing so leaks information.
    #[error("SSO ticket is invalid or has already been used")]
    InvalidTicket,

    /// The SSO ticket exists but has passed its expiry time.
    #[error("SSO ticket expired")]
    ExpiredTicket,

    /// Authentication succeeded but the account is permanently banned.
    #[error("account is permanently banned: {reason}")]
    PermanentBan { reason: String },

    /// Authentication succeeded but the account is temporarily banned.
    #[error("account is banned until {expiry}")]
    TemporaryBan { expiry: String },

    /// The client's IP address is banned.
    #[error("IP address is banned")]
    IpBanned,

    /// The session attempted an operation that requires authentication, but
    /// the handshake has not completed yet.
    #[error("operation requires authentication")]
    NotAuthenticated,

    /// A second login attempt was made on a session that is already
    /// authenticated. This is either a client bug or an attempted exploit.
    #[error("session is already authenticated")]
    AlreadyAuthenticated,

    /// The account is already connected from another session. Behavior
    /// (kick old session vs. reject new one) is determined by configuration.
    #[error("account is already connected")]
    AlreadyConnected,
}
