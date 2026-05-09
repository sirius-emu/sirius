//! Shared domain primitives for the Sirius emultator.
//!
//! This crate has no internal dependencies. Everything else depends on it.
//! Keep it that way, the moment this crate starts importing from another
//! sirius-* crate, the dependency graph becomes a problem.
//!
//! What belongs here: newtypes, coordinates, enums that multiple crates need to
//! agree on. What does not belong here: logic, I/O, anything async.

pub mod coords;
pub mod currency;
pub mod enums;
pub mod ids;
pub mod room;

pub use coords::{Direction, Vec2, Vec3};
pub use currency::CurrencyType;
pub use enums::{Gender, RoomCategory, RoomLockType};
pub use ids::{ItemId, RoomId, UserId};
pub use room::RoomDisplayNode;
