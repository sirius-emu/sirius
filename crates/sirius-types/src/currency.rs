//! Currency types.
//!
//! Habbo has three standard currencies: credits, pixels and diamonds.
//! They're kept as separate variants rather than a bare integer so call sites
//! can't accidentally pass credits where pixels are expected.
//!
//! Amounts are `i64`. The sign is intentionally allowed here, negative amounts
//! represents deductions in transaction records. Enforcement of non-negative
//! balances is the responsibility of `sirius-currency`, not this crate.

use std::fmt;

pub enum Currency {
    Credits(i64),
    Pixels(i64),
    Diamonds(i64),
}

impl Currency {
    /// Returns the numeric amount, regardless of denomination.
    #[inline]
    pub const fn amount(self) -> i64 {
        match self {
            Self::Credits(n) | Self::Pixels(n) | Self::Diamonds(n) => n,
        }
    }

    /// Returns a new `Currency` of the same denomination with the given amount.
    #[inline]
    pub const fn with_amount(self, amount: i64) -> Self {
        match self {
            Self::Credits(_) => Self::Credits(amount),
            Self::Pixels(_) => Self::Pixels(amount),
            Self::Diamonds(_) => Self::Diamonds(amount),
        }
    }

    /// Returns the protocol type ID for this currency.
    ///
    /// Matches the values the Nitro client expects in wallet update packets.
    #[inline]
    pub const fn type_id(self) -> i32 {
        match self {
            Self::Credits(_) => 0,
            Self::Pixels(_) => 1,
            Self::Diamonds(_) => 5,
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Credits(n) => write!(f, "{} credits", n),
            Self::Pixels(n) => write!(f, "{} pixels", n),
            Self::Diamonds(n) => write!(f, "{} diamonds", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_accessor() {
        assert_eq!(Currency::Credits(100).amount(), 100);
        assert_eq!(Currency::Pixels(-50).amount(), -50);
    }

    #[test]
    fn with_amount_preserves_denomination() {
        let original = Currency::Credits(100);
        let updated = original.with_amount(200);
        assert!(matches!(updated, Currency::Credits(200)));
    }

    #[test]
    fn type_ids_are_distinct() {
        let ids = [
            Currency::Credits(0).type_id(),
            Currency::Pixels(0).type_id(),
            Currency::Diamonds(0).type_id(),
        ];
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), ids.len());
    }
}
