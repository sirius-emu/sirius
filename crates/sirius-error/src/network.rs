//! Errors originating in the network and session layers.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NetworkError {
    /// The TCP or WebSocket connection was closed, either cleanly or abruptly.
    ///
    /// This is not always a problem, client disconnects all the time. Callers should
    /// decide whether to log, ignore or act based on context.
    #[error("connection closed")]
    ConnectionClosed,

    /// A read or write operation on the socket failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The WebSocket handshake failed before a valid connection was established.
    #[error("WebSocket handshake failed: {reason}")]
    HandshakeFailed { reason: String },

    /// The connection was rejected because the server has reached its connection limit.
    #[error("connection limit reached (max: {max})")]
    ConnectionLimitReached { max: usize },

    /// The connection was rejected because this IP address has exceeded the
    /// connection rate limit.
    #[error("rate limit exceeded for {ip}")]
    RateLimitExceeded { ip: std::net::IpAddr },

    /// The session actor's mailbox was dropped, meaning the task is gone.
    ///
    /// This typically means the session was already closed by the time
    /// the message was sent.
    #[error("session mailbox closed")]
    SessionClosed,

    /// A read or write timed out.
    #[error("operation timed out after {seconds}s")]
    Timeout { seconds: u64 },
}
