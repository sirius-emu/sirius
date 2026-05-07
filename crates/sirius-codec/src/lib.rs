//! Nitro protocol codec for Sirius.
//!
//! This crate is purely about bytes. No game logic, no packet interpretation,
//! no string parsing beyond what the format requires.
//!
//! # Format
//!
//! Every Nitro packet has a fixed-size header followed by a body:
//!
//! length (4 bytes) -> id (2 bytes) -> body (length - 2 bytes)
//!
//! - **length**: total number of bytes that follow, including the 2-byte header ID.
//! - **id**: the packet header ID.
//! - **body**: `length - 2` bytes of packet-specific payload.
//!
//! Both fields are big-endian unsigned integers.
//!
//! # Integration
//!
//! [`NitroCodec`] implements [`tokio_util::codec::Decoder`] and
//! [`tokio_util::codec::Encoder`], so it plugs directly into a
//! `tokio_util::codec::Framed` wrapping any `AsyncRead + AsyncWrite`.
//!
//! ```no_run
//! use std::net::TcpStream;
//! use tokio_util::codec::Framed;
//! use sirius_codec::{NitroCodec, RawPacket};
//!
//! async fn handle(stream: TcpStream) {
//!     let mut framed = Framed::new(stream, NitroCodec::new());
//!     // framed is now a Stream<Item = Result<RawPacket, _>> + Sink<RawPacket>
//! }
//! ```

mod decoder;
mod encoder;
mod header;
mod packet;

pub use decoder::NitroDecoder;
pub use encoder::NitroEncoder;
pub use header::PacketHeader;
pub use packet::RawPacket;

use tokio_util::codec::{Decoder, Encoder};

/// The combined Nitro codec.
///
/// Implements both [`Decoder`] and [`Encoder`] so it can be passed directly to
/// [`tokio_util::codec::Framed`]. Use this unless you have a specific reason to use
/// the decoder and encoder separately.
#[derive(Debug, Default)]
pub struct NitroCodec {
    decoder: NitroDecoder,
    encoder: NitroEncoder,
}

impl NitroCodec {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Decoder for NitroCodec {
    type Item = RawPacket;
    type Error = sirius_error::SiriusError;

    fn decode(
        &mut self,
        src: &mut bytes::BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        self.decoder.decode(src)
    }
}

impl Encoder<RawPacket> for NitroCodec {
    type Error = sirius_error::SiriusError;

    fn encode(
        &mut self,
        item: RawPacket,
        dst: &mut bytes::BytesMut,
    ) -> Result<(), Self::Error> {
        self.encoder.encode(item, dst)
    }
}
