//! A 64-bit "number" representation.

use crate::f64::{box_signed, box_unsigned, is_boxed, unbox_signed, unbox_unsigned};
use core::ops::Add;

const INTEGER_FLAG: u64 = 0x0002 << 48;

/// A 64-bit "number" representation that embraces 49-bit payloads, 49-bit
/// integers, and 64-bit floating-point numbers.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct N64(u64);

impl N64 {
    /// Creates a number from a 64-bit floating-point number.
    pub const fn from_f64(payload: f64) -> Self {
        Self(payload.to_bits())
    }

    /// Returns a 64-bit floating-point number.
    pub const fn to_f64(self) -> Option<f64> {
        if self.is_boxed() {
            None
        } else {
            Some(f64::from_bits(self.0))
        }
    }

    /// Creates a number from a payload.
    pub const fn from_payload(payload: u64) -> Self {
        Self(box_unsigned(payload))
    }

    /// Returns a payload.
    pub const fn to_payload(self) -> Option<u64> {
        if self.is_boxed() && self.0 & INTEGER_FLAG == 0 {
            unbox_unsigned(self.0)
        } else {
            None
        }
    }

    /// Creates a number from an integer.
    pub const fn from_signed_integer(integer: i64) -> Self {
        Self(box_signed(integer) | INTEGER_FLAG)
    }

    /// Returns a signed integer.
    pub const fn to_signed_integer(self) -> Option<i64> {
        if self.is_boxed() && self.0 & INTEGER_FLAG == 0 {
            None
        } else {
            unbox_signed(self.0 & !INTEGER_FLAG)
        }
    }

    fn to_signed_integer_unchecked(self) -> i64 {
        self.to_signed_integer().unwrap_or_default()
    }

    /// Returns `true` if a payload or signed integer is boxed.
    pub const fn is_boxed(self) -> bool {
        is_boxed(self.0)
    }

    /// Returns `true` if it is a number.
    pub const fn is_number(self) -> bool {
        self.0 & INTEGER_FLAG == 0
    }
}

impl Add for N64 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        if self.is_boxed() && other.is_boxed() {
            Self::from_signed_integer(
                self.to_signed_integer_unchecked() + other.to_signed_integer_unchecked(),
            )
        } else {
            Self((f64::from_bits(self.0) + f64::from_bits(other.0)).to_bits())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64() {
        assert!(!N64::from_f64(42.0).is_boxed());
        assert_eq!(N64::from_f64(42.0).to_f64(), Some(42.0));
        assert_eq!(N64::from_f64(42.0).to_payload(), None);
        assert_eq!(N64::from_f64(42.0).to_signed_integer(), None);
    }

    #[test]
    fn payload() {
        assert!(N64::from_payload(42).is_boxed());
        assert_eq!(N64::from_payload(42).to_payload(), Some(42));
        assert_eq!(N64::from_payload(42).to_signed_integer(), None);
    }

    #[test]
    fn signed_integer() {
        assert!(N64::from_signed_integer(42).is_boxed());
        assert!(N64::from_signed_integer(-42).is_boxed());
        assert_eq!(N64::from_signed_integer(42).to_signed_integer(), Some(42));
        assert_eq!(N64::from_signed_integer(-42).to_signed_integer(), Some(-42));
        assert_eq!(N64::from_signed_integer(42).to_payload(), None);
        assert_eq!(N64::from_signed_integer(-42).to_payload(), None);
    }

    #[test]
    fn add() {
        assert_eq!(N64::from_f64(2.0) + N64::from_f64(3.0), N64::from_f64(5.0));
        assert_eq!(
            N64::from_signed_integer(2) + N64::from_signed_integer(3),
            N64::from_signed_integer(5)
        );
    }
}
