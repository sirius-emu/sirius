//! Spatial primitives used across the room, pathfinder and furniture crates.
//!
//! The coordinate system follows Habbo convention: X increases east, Y increases south,
//! Z is the stack height of a tile (f64 to support fractional heights produced by stacked
//! furniture).

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Sub};

/// A position on the room tile grid.
///
/// X and Y are tile coordinates. They are signed to simplify arithmetic
/// during pathfinding. Intermediate values can go negative even if valid
/// positions don't.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Manhattan distance between two tile positions.
    ///
    /// Used by the pathfinder heuristic. Don't use Euclidean distance here,
    /// diagonal moves cost the same as cardinal ones on the Habbo grid.
    #[inline]
    #[must_use]
    pub fn manhattan_distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    /// Returns true if `other` is within the given tile radius (inclusive).
    #[inline]
    #[must_use]
    pub fn within_range(self, other: Self, radius: i32) -> bool {
        self.manhattan_distance(other) <= radius
    }

    /// The 8 neighboring positions (cardinal + diagonal).
    #[must_use]
    pub fn neighbors(self) -> [Self; 8] {
        [
            Self::new(self.x - 1, self.y - 1),
            Self::new(self.x, self.y - 1),
            Self::new(self.x + 1, self.y - 1),
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x - 1, self.y + 1),
            Self::new(self.x, self.y + 1),
            Self::new(self.x + 1, self.y + 1),
        ]
    }
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// A position on the room tile grid with a stack height.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0.0 };

    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Drops the Z component.
    #[inline]
    #[must_use]
    pub const fn to_vec2(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<Vec2> for Vec3 {
    #[inline]
    fn from(v: Vec2) -> Self {
        Self::new(v.x, v.y, 0.0)
    }
}

/// The 8 cardinal and diagonal directions. Encoded as the Habbo rotation value.
///
/// The numeric values match the wire protocol directly. No translation needed
/// when reading from or writing to packets.
///
/// ```text
///   7  0  1
///   6  *  2
///   5  4  3
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
}

impl Direction {
    /// Rotates 180 degrees. Used when an NPC turns to face a player.
    #[inline]
    #[must_use]
    pub fn opposite(self) -> Self {
        Self::from_u8((self as u8 + 4) % 8)
    }

    /// Returns the unit vector offset for this direction.
    #[inline]
    #[must_use]
    pub fn to_vec2_offset(self) -> Vec2 {
        match self {
            Self::North => Vec2::new(0, -1),
            Self::NorthEast => Vec2::new(1, -1),
            Self::East => Vec2::new(1, 0),
            Self::SouthEast => Vec2::new(1, 1),
            Self::South => Vec2::new(0, 1),
            Self::SouthWest => Vec2::new(-1, 1),
            Self::West => Vec2::new(-1, 0),
            Self::NorthWest => Vec2::new(-1, -1),
        }
    }

    /// Derives the direction from a source position to a target position.
    ///
    /// Returns `None` if source and target are the same tile.
    #[must_use]
    pub fn from_positions(from: Vec2, to: Vec2) -> Option<Self> {
        let dx = (to.x - from.x).signum();
        let dy = (to.y - from.y).signum();

        match (dx, dy) {
            (0, -1) => Some(Self::North),
            (1, -1) => Some(Self::NorthEast),
            (1, 0) => Some(Self::East),
            (1, 1) => Some(Self::SouthEast),
            (0, 1) => Some(Self::South),
            (-1, 1) => Some(Self::SouthWest),
            (-1, 0) => Some(Self::West),
            (-1, -1) => Some(Self::NorthWest),
            _ => None,
        }
    }

    /// Converts a raw u8 to a `Direction`, wrapping at 8.
    #[inline]
    #[must_use]
    pub fn from_u8(v: u8) -> Self {
        match v % 8 {
            0 => Self::North,
            1 => Self::NorthEast,
            2 => Self::East,
            3 => Self::SouthEast,
            4 => Self::South,
            5 => Self::SouthWest,
            6 => Self::West,
            7 => Self::NorthWest,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Direction {
    type Error = u8;

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        if v < 8 { Ok(Self::from_u8(v)) } else { Err(v) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manhattan_distance() {
        let a = Vec2::new(0, 0);
        let b = Vec2::new(3, 4);
        assert_eq!(a.manhattan_distance(b), 7);
    }

    #[test]
    fn neighbors_count() {
        assert_eq!(Vec2::ZERO.neighbors().len(), 8);
    }

    #[test]
    fn direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::SouthWest.opposite(), Direction::NorthEast);
    }

    #[test]
    fn direction_from_positions() {
        let from = Vec2::new(0, 0);
        assert_eq!(
            Direction::from_positions(from, Vec2::new(1, 0)),
            Some(Direction::East)
        );
        assert_eq!(
            Direction::from_positions(from, Vec2::new(0, 1)),
            Some(Direction::South)
        );
        assert_eq!(Direction::from_positions(from, from), None);
    }

    #[test]
    fn vec3_to_vec2() {
        let v = Vec3::new(3, 7, 1.5);
        assert_eq!(v.to_vec2(), Vec2::new(3, 7));
    }

    #[test]
    fn direction_try_from_out_of_range() {
        assert!(Direction::try_from(8).is_err());
        assert!(Direction::try_from(7).is_ok());
    }
}
