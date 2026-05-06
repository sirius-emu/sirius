//! Incoming packet definitions.
//!
//! Each submodule owns a slice of the incoming packet namespace. Add new
//! packets in the appropriate submodule, implement [`IncomingPacket`], then
//! register the header ID in the `HEADER_IDS` table below so the dispatcher
//! can route it.
