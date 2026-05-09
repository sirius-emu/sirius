//! Domain enumerations shared across multiple crates.
//!
//! Only enums that two or more crates need to agree on live here.
//! If an enum is only used inside one crate, it belongs in that crate.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The gender of a Habbo figure.
///
/// The wire protocol represents this as a single ASCII character:
/// `M` for male, `F` for female. The `as_char` method provides that
/// encoding without requiring the packet layer to know about figure format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    /// Returns the single-character representation used in figure strings
    /// and the client protocol.
    #[must_use]
    #[inline]
    pub const fn as_char(self) -> char {
        match self {
            Self::Male => 'M',
            Self::Female => 'F',
        }
    }
}

impl TryFrom<char> for Gender {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c.to_ascii_uppercase() {
            'M' => Ok(Self::Male),
            'F' => Ok(Self::Female),
            _ => Err(c),
        }
    }
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl std::str::FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .next()
            .ok_or_else(|| format!("empty gender string"))
            .and_then(|c| {
                Gender::try_from(c).map_err(|c| format!("invalid gender: {c}"))
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum RoomLockType {
    /// Anyone can enter.
    Open = 0,
    /// Guests must knock, the owner must accept.
    Doorbell = 1,
    /// A password is required.
    Password = 2,
    /// Nobody can enter except the owner.
    Invisible = 3,
}

impl RoomLockType {
    #[inline]
    pub const fn as_i32(self) -> i32 {
        self as i32
    }
}

impl TryFrom<i32> for RoomLockType {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Open),
            1 => Ok(Self::Doorbell),
            2 => Ok(Self::Password),
            3 => Ok(Self::Invisible),
            _ => Err(v),
        }
    }
}

/// The category a room belongs to in the navigator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoomCategory(pub i32);

impl RoomCategory {
    #[inline]
    pub const fn id(self) -> i32 {
        self.0
    }
}

impl From<i32> for RoomCategory {
    #[inline]
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl From<RoomCategory> for i32 {
    #[inline]
    fn from(c: RoomCategory) -> Self {
        c.0
    }
}

impl fmt::Display for RoomCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
