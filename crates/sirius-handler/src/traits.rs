//! The `PacketHandler` trait.

use sirius_codec::RawPacket;
use sirius_error::SiriusError;

use crate::context::HandlerContext;

/// A handler for a single incoming packet type.
///
/// Implement this on a unit struct and register it with [`PacketRouter::register`].
///
/// Handlers are called with a shared reference to `self`. They must be
/// stateless or use interior mutability. All mutable state lives in
/// [`HandlerContext`] or the repositories it carries.
pub trait PacketHandler: Send + Sync + 'static {
    /// The header ID of the packet this handler processes.
    ///
    /// The router uses this to dispatch incoming packets. Two handlers with
    /// the same ID cannot be registered, the second registration panics in
    /// debug builds and silently overwrites in release.
    const HEADER_ID: u16;

    /// Processes the packet.
    ///
    /// `packet` is the raw incoming packet, parse it using the appropriate
    /// [`IncomingPacket::from_raw`] call at the top of this method.
    ///
    /// Returning an error does not close the session. The router logs the error
    /// and moves on. On truly fatal errors (lost DB connection, actor stopped) should
    /// bubble up, packet-level mistakes like a malformed body are logged at warn and dropped.
    ///
    /// [`IncomingPacket::from_raw`]: sirius_packets::IncomingPacket::from_raw
    fn handle(
        &self,
        raw: RawPacket,
        ctx: HandlerContext,
    ) -> impl Future<Output = Result<(), SiriusError>> + Send;
}
