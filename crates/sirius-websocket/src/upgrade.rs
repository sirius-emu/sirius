//! HTTP upgrade handler for WebSockets.

use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tracing::{debug, warn};

/// Result of an upgrade attempt.
pub enum UpgradeResult {
    /// The HTTP upgrade succeeded.
    Success(WebSocketStream<TcpStream>),
    /// The client sent a valid HTTP request but it was rejected (e.g. wrong path).
    /// The connection should be dropped.
    Rejected,
    /// The handshake failed completely (invalid HTTP).
    Failed,
}

/// Attempts to upgrade a TCP stream to a WebSocket connection.
///
/// `expected_path` is the URI path the server expects (e.g. `"/”`).
/// If the client requests a different path, the upgrade is rejected with 404.
pub async fn accept_async(
    stream: TcpStream,
    expected_path: &str,
    peer_addr: SocketAddr,
) -> UpgradeResult {
    let mut path_matched = false;

    // The callback lets us inspect the HTTP request before accepting the upgrade.
    let callback = |req: &Request, mut response: Response| {
        debug!(%peer_addr, path = req.uri().path(), "websocket upgrade request");

        if req.uri().path() != expected_path {
            warn!(%peer_addr, requested = req.uri().path(), expected = expected_path, "rejecting websocket upgrade: wrong path");
            *response.status_mut() = http::StatusCode::NOT_FOUND;
        } else {
            path_matched = true;
        }
        Ok(response)
    };

    match tokio_tungstenite::accept_hdr_async(stream, callback).await {
        Ok(ws_stream) => UpgradeResult::Success(ws_stream),
        Err(e) => {
            if path_matched {
                // The path matched but the upgrade still failed (e.g. bad headers).
                warn!(%peer_addr, error = %e, "websocket handshake failed");
            }
            if let tokio_tungstenite::tungstenite::Error::Http(response) = &e {
                if response.status() == http::StatusCode::NOT_FOUND {
                    return UpgradeResult::Rejected;
                }
            }
            UpgradeResult::Failed
        }
    }
}
