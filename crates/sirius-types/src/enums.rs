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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gender_roundtrip() {
        assert_eq!(Gender::try_from('M'), Ok(Gender::Male));
        assert_eq!(Gender::try_from('f'), Ok(Gender::Female));
        assert!(Gender::try_from('X').is_err());
    }
}
