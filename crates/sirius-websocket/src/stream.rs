//! Wrapper around WebSocketStream that speaks `RawPacket`.

use bytes::{BufMut, BytesMut};
use futures_util::{SinkExt, StreamExt};
use sirius_codec::{NitroCodec, RawPacket};
use sirius_error::{NetworkError, SiriusError};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::codec::{Decoder, Encoder};
use tracing::{trace, warn};

/// A WebSocket connection that yields and accepts [`RawPacket`]s.
pub struct WsStream {
    inner: WebSocketStream<TcpStream>,
    codec: NitroCodec,
    read_buf: BytesMut,
    write_buf: BytesMut,
    ping_interval: time::Interval,
}

impl WsStream {
    /// Wraps an upgraded WebSocket stream.
    ///
    /// `ping_interval_secs` controls how often a Ping frame is sent to keep
    /// the connection alive.
    pub fn new(
        inner: WebSocketStream<TcpStream>,
        ping_interval_secs: u64,
    ) -> Self {
        let mut ping_interval =
            time::interval(Duration::from_secs(ping_interval_secs));
        // The first tick fires immediately, we don't want to ping instantly.
        ping_interval.reset();

        Self {
            inner,
            codec: NitroCodec::new(),
            read_buf: BytesMut::with_capacity(4096),
            write_buf: BytesMut::with_capacity(4096),
            ping_interval,
        }
    }

    /// Reads the next [`RawPacket`] from the WebSocket.
    ///
    /// This method also handles Ping/Pong and Close frames internally. It will
    /// wait up to `timeout` before failing.
    pub async fn next(
        &mut self,
        timeout: Duration,
    ) -> Result<Option<RawPacket>, SiriusError> {
        loop {
            // If we already have a full packet in the buffer from a previous frame,
            // yield it immediately before reading more.
            if let Some(packet) = self.codec.decode(&mut self.read_buf)? {
                return Ok(Some(packet));
            }

            tokio::select! {
                _ = self.ping_interval.tick() => {
                    trace!("sending websocket ping");
                    self.send_ping().await?;
                    continue;
                }

                result = time::timeout(timeout, self.inner.next()) => {
                    match result {
                        Err(_) => {
                            // Timeout
                            return Err(SiriusError::Network(NetworkError::Timeout {
                                seconds: timeout.as_secs(),
                            }));
                        }
                        Ok(None) => {
                            // EOF
                            return Ok(None);
                        }
                        Ok(Some(Err(e))) => {
                            warn!(error = %e, "websocket read error");
                            return Err(SiriusError::Network(NetworkError::Io(std::io::Error::new(
                                std::io::ErrorKind::ConnectionReset,
                                e.to_string(),
                            ))));
                        }
                        Ok(Some(Ok(msg))) => {
                            match msg {
                                Message::Binary(data) => {
                                    self.read_buf.put_slice(&data);
                                }
                                Message::Text(_) => {
                                    warn!("received text frame from client, ignoring");
                                }
                                Message::Ping(data) => {
                                    trace!("received websocket ping, sending pong");
                                    self.inner.send(Message::Pong(data)).await.map_err(|e| {
                                        SiriusError::Network(NetworkError::Io(std::io::Error::new(
                                            std::io::ErrorKind::BrokenPipe,
                                            e.to_string(),
                                        )))
                                    })?;
                                }
                                Message::Pong(_) => {
                                    trace!("received websocket pong");
                                }
                                Message::Close(_) => {
                                    return Ok(None);
                                }
                                Message::Frame(_) => unreachable!(),
                            }
                        }
                    }
                }
            }
        }
    }

    /// Sends a [`RawPacket`] to the client as a Binary frame.
    pub async fn send(&mut self, packet: RawPacket) -> Result<(), SiriusError> {
        self.write_buf.clear();
        self.codec.encode(packet, &mut self.write_buf)?;

        let msg = Message::Binary(self.write_buf.split().freeze());
        self.inner.send(msg).await.map_err(|e| {
            SiriusError::Network(NetworkError::Io(std::io::Error::new(
                std::io::ErrorKind::BrokenPipe,
                e.to_string(),
            )))
        })
    }

    async fn send_ping(&mut self) -> Result<(), SiriusError> {
        self.inner
            .send(Message::Ping(bytes::Bytes::new()))
            .await
            .map_err(|e| {
                SiriusError::Network(NetworkError::Io(std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    e.to_string(),
                )))
            })
    }
}
