//! Strongly-typed identifiers.
//!
//! Using raw integers or UUIDs as IDs throughout the codebase is a trap.
//! It's trivially easy to pass a `UserId` where a `RoomId` is expected and
//! the compiler won't catch it. These newtypes cost nothing at runtime and
//! eliminate that class of mistakes entirely.
//!
//! All IDs are backed by `i32` to.
//! No conversion needed when reading from or writing to sqlx queries.

use serde::{Deserialize, Serialize};
use std::fmt;

macro_rules! define_id {
    (
        $(#[$attr:meta])*
        $name:ident
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub i32);

        impl $name {
            #[inline]
            pub const fn inner(self) -> i32 {
                self.0
            }
        }

        impl From<i32> for $name {
            #[inline]
            fn from(v: i32) -> Self {
                Self(v)
            }
        }

        impl From<$name> for i32 {
            #[inline]
            fn from(id: $name) -> Self {
                id.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}

define_id!(RoomId);

define_id!(UserId);

define_id!(ItemId);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtype_prevents_id_confusion() {
        let room: RoomId = RoomId::from(1);
        let user: UserId = UserId::from(1);

        assert_eq!(room.inner(), user.inner());
        assert_ne!(format!("{:?}", room), format!("{:?}", user));
    }

    #[test]
    fn roundtrip_i32() {
        let id = RoomId::from(42);
        let raw: i32 = id.into();
        assert_eq!(raw, 42);
    }

    #[test]
    fn serde_transparent() {
        let id = UserId::from(99);
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "99");

        let back: UserId = serde_json::from_str(&json).unwrap();
        assert_eq!(back, id);
    }
}
