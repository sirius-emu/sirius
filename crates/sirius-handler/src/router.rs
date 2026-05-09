//! Packet dispatch router.
//!
//! [`PacketRouter`] maps header IDs to boxed [`PacketHandler`]s and dispatches
//! incoming packets to the correct handler.

use crate::{HandlerContext, PacketHandler};
use sirius_codec::RawPacket;
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tracing::{debug, warn};

type BoxedHandler = Box<
    dyn Fn(
            RawPacket,
            HandlerContext,
        ) -> Pin<Box<dyn Future<Output = ()> + Send>>
        + Send
        + Sync,
>;

pub struct PacketRouter {
    handlers: HashMap<u16, BoxedHandler>,
}

impl PacketRouter {
    pub fn builder() -> PacketRouterBuilder {
        PacketRouterBuilder::new()
    }

    pub async fn dispatch(&self, packet: RawPacket, ctx: HandlerContext) {
        let header_id = packet.id();

        match self.handlers.get(&header_id) {
            None => {
                debug!(header_id, "no handler registered for packet");
            }
            Some(handler) => {
                handler(packet, ctx).await;
            }
        }
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }
}

pub struct PacketRouterBuilder {
    handlers: HashMap<u16, BoxedHandler>,
}

impl PacketRouterBuilder {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register<H>(&mut self, handler: H) -> &mut Self
    where
        H: PacketHandler,
    {
        let header_id = H::HEADER_ID;

        if self.handlers.contains_key(&header_id) {
            panic!(
                "duplicate handler registration for header ID {header_id} ({})",
                std::any::type_name::<H>()
            );
        }

        let handler = Arc::new(handler);

        let boxed: BoxedHandler = Box::new(move |packet, ctx| {
            let handler = Arc::clone(&handler);
            Box::pin(async move {
                if let Err(e) = handler.handle(packet, ctx).await {
                    warn!(header_id, error = %e, "packet handler returned an error");
                }
            })
        });

        self.handlers.insert(header_id, boxed);
        self
    }

    pub fn build(self) -> PacketRouter {
        PacketRouter {
            handlers: self.handlers,
        }
    }
}

impl Default for PacketRouterBuilder {
    fn default() -> Self {
        Self::new()
    }
}
