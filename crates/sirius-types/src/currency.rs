//! Currency types.
//!
//! Habbo has three standard currencies: credits, pixels and diamonds.
//! They're kept as separate variants rather than a bare integer so call sites
//! can't accidentally pass credits where pixels are expected.
//!
//! Amounts are `i64`. The sign is intentionally allowed here, negative amounts
//! represents deductions in transaction records. Enforcement of non-negative
//! balances is the responsibility of `sirius-currency`, not this crate.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CurrencyType {
    Credits,
    Pixels,
    Diamonds,
    Seasonal(i32),
}

impl CurrencyType {
    pub const fn type_id(self) -> i32 {
        match self {
            Self::Credits => -1,
            Self::Pixels => 0,
            Self::Diamonds => 5,
            Self::Seasonal(n) => n,
        }
    }
}

impl From<i32> for CurrencyType {
    fn from(val: i32) -> Self {
        match val {
            0 => Self::Pixels,
            5 => Self::Diamonds,
            other => Self::Seasonal(other),
        }
    }
}
