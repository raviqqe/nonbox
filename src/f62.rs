//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers
//! and 62-bit payloads.

use core::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

const ROTATION_COUNT: u32 = 3;

/// Boxes a 63-bit unsigned integer.
#[inline]
pub const fn box_integer(integer: i64) -> u64 {
    (integer << 1) as _
}

/// Unboxes a 63-bit unsigned integer.
#[inline]
pub const fn unbox_integer(number: u64) -> Option<i64> {
    if is_integer(number) {
        Some(unbox_integer_unchecked(number))
    } else {
        None
    }
}

/// Unboxes a 63-bit unsigned integer without any type check.
#[inline]
pub const fn unbox_integer_unchecked(number: u64) -> i64 {
    number as i64 >> 1
}

/// Returns `true` if a number is an integer.
#[inline]
pub const fn is_integer(number: u64) -> bool {
    number & 1 == 0
}

/// Boxes a 62-bit payload.
#[inline]
pub const fn box_payload(payload: u64) -> u64 {
    (payload << 2) | 1
}

/// Unboxes a 62-bit payload.
#[inline]
pub const fn unbox_payload(number: u64) -> Option<u64> {
    if is_payload(number) {
        Some(unbox_payload_unchecked(number))
    } else {
        None
    }
}

/// Unboxes a 62-bit payload without any type check.
#[inline]
pub const fn unbox_payload_unchecked(number: u64) -> u64 {
    number >> 2
}

/// Returns `true` if a number is a payload.
#[inline]
pub const fn is_payload(number: u64) -> bool {
    number & 0b11 == 1
}

/// Boxes a 64-bit floating-point number.
#[inline]
pub const fn box_float(number: f64) -> u64 {
    if number == 0.0 {
        number.to_bits()
    } else {
        number.to_bits().rotate_left(ROTATION_COUNT) | 0b11
    }
}

/// Unboxes a 64-bit floating-point number.
#[inline]
pub const fn unbox_float(number: u64) -> Option<f64> {
    if is_float(number) {
        Some(unbox_float_unchecked(number))
    } else {
        None
    }
}

/// Unboxes a 64-bit floating-point number without any type check.
#[inline]
pub const fn unbox_float_unchecked(number: u64) -> f64 {
    let exponent_tail = 2 - (number >> 63);

    f64::from_bits((number & !0b11 | exponent_tail).rotate_right(ROTATION_COUNT))
}

/// Returns `true` if a number is a 62-bit floating-point number.
#[inline]
pub const fn is_float(number: u64) -> bool {
    number & 0b11 == 0b11
}

/// A 62-bit floating-point number.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Float62(u64);

impl Float62 {
    /// Creates a 62-bit floating-point number from its raw representation.
    #[inline]
    pub const fn from_bits(number: u64) -> Self {
        Self(number)
    }

    /// Returns a raw representation.
    #[inline]
    pub const fn to_bits(self) -> u64 {
        self.0
    }

    /// Creates a 62-bit floating-point number from a payload.
    #[inline]
    pub const fn from_payload(payload: u64) -> Self {
        Self::from_bits(box_payload(payload))
    }

    /// Creates a 62-bit floating-point number from an integer.
    #[inline]
    pub const fn from_integer(integer: i64) -> Self {
        Self::from_bits(box_integer(integer))
    }

    /// Creates a 62-bit floating-point number from a 64-bit floating-point
    /// number.
    #[inline]
    pub const fn from_float(number: f64) -> Self {
        Self::from_bits(box_float(number))
    }

    /// Returns a payload.
    #[inline]
    pub const fn to_payload(self) -> Option<u64> {
        unbox_payload(self.0)
    }

    /// Returns a payload without any type check.
    #[inline]
    pub const fn to_payload_unchecked(self) -> u64 {
        unbox_payload_unchecked(self.0)
    }

    /// Returns an integer.
    #[inline]
    pub const fn to_integer(self) -> Option<i64> {
        unbox_integer(self.0)
    }

    /// Returns an integer without any type check.
    #[inline]
    pub const fn to_integer_unchecked(self) -> i64 {
        unbox_integer_unchecked(self.0)
    }

    /// Returns a 64-bit floating-point number.
    #[inline]
    pub const fn to_float(self) -> Option<f64> {
        unbox_float(self.0)
    }

    /// Returns a 62-bit floating-point number without any type check.
    #[inline]
    pub const fn to_float_unchecked(self) -> f64 {
        unbox_float_unchecked(self.0)
    }

    #[inline]
    const fn to_number(self) -> Result<i64, f64> {
        if let Some(integer) = self.to_integer() {
            Ok(integer)
        } else {
            Err(unbox_float_unchecked(self.0))
        }
    }
}

macro_rules! operate {
    ($lhs:ident, $rhs:ident, $operate:ident) => {{
        fn calculate_float(lhs: Float62, rhs: Float62) -> Float62 {
            match (lhs.to_number(), rhs.to_number()) {
                (Ok(_), Ok(_)) => unreachable!(),
                (Ok(x), Err(y)) => Float62::from_float((x as f64).$operate(y)),
                (Err(x), Ok(y)) => Float62::from_float(x.$operate(y as f64)),
                (Err(x), Err(y)) => Float62::from_float(x.$operate(y)),
            }
        }

        let (Some(x), Some(y)) = ($lhs.to_integer(), $rhs.to_integer()) else {
            return calculate_float($lhs, $rhs);
        };

        Self::from_integer(x.$operate(y))
    }};
}

impl Add for Float62 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, add)
    }
}

impl Sub for Float62 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, sub)
    }
}

impl Mul for Float62 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, mul)
    }
}

impl Div for Float62 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, div)
    }
}

impl Rem for Float62 {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, rem)
    }
}

impl AddAssign for Float62 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Float62 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Float62 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Float62 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Float62 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        match self.to_number() {
            Ok(x) => Self::from_integer(-x),
            Err(x) => Self::from_float(-x),
        }
    }
}

impl Display for Float62 {
    #[inline]
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(integer) = self.to_integer() {
            write!(formatter, "{integer}")
        } else if let Some(float) = self.to_float() {
            write!(formatter, "{float}")
        } else {
            write!(formatter, "0x{:x}", self.to_payload_unchecked())
        }
    }
}

impl PartialOrd for Float62 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        fn compare_float(lhs: Float62, rhs: Float62) -> Option<Ordering> {
            match (lhs.to_number(), rhs.to_number()) {
                (Ok(_), Ok(_)) => unreachable!(),
                (Ok(x), Err(y)) => (x as f64).partial_cmp(&y),
                (Err(x), Ok(y)) => x.partial_cmp(&(y as f64)),
                (Err(x), Err(y)) => x.partial_cmp(&y),
            }
        }

        let (Some(x), Some(y)) = (self.to_integer(), other.to_integer()) else {
            return compare_float(*self, *other);
        };

        x.partial_cmp(&y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn integer() {
        assert!(is_integer(box_integer(0)));
        assert_eq!(unbox_integer(box_integer(0)), Some(0));
        assert_eq!(unbox_integer(box_integer(1)), Some(1));
        assert_eq!(unbox_integer(box_integer(-1)), Some(-1));
        assert_eq!(unbox_integer(box_integer(42)), Some(42));
        assert_eq!(unbox_integer(box_integer(-42)), Some(-42));
    }

    #[test]
    fn payload() {
        assert!(is_payload(box_payload(0)));
        assert_eq!(unbox_payload(box_payload(0)), Some(0));
        assert_eq!(unbox_payload(box_payload(1)), Some(1));
        assert_eq!(unbox_payload(box_payload(42)), Some(42));
    }

    #[test]
    fn f62() {
        assert!(is_float(box_float(1.0)));
        assert_eq!(unbox_float(box_float(0.0)), None);
        assert_eq!(unbox_float(box_float(1.0)), Some(1.0));
        assert_eq!(unbox_float(box_float(-1.0)), Some(-1.0));
        assert_eq!(unbox_float(box_float(42.0)), Some(42.0));
        assert_eq!(unbox_float(box_float(-42.0)), Some(-42.0));
    }

    mod float62 {
        use super::*;

        #[test]
        fn default() {
            assert_eq!(Float62::default(), Float62::from_integer(0));
            assert_eq!(Float62::default(), Float62::from_float(0.0));
        }

        #[test]
        fn add() {
            assert_eq!(
                Float62::from_integer(2) + Float62::from_integer(3),
                Float62::from_integer(5)
            );
            assert_eq!(
                Float62::from_integer(2) + Float62::from_float(3.0),
                Float62::from_float(5.0)
            );
            assert_eq!(
                Float62::from_float(2.0) + Float62::from_integer(3),
                Float62::from_float(5.0)
            );
            assert_eq!(
                Float62::from_float(2.0) + Float62::from_float(3.0),
                Float62::from_float(5.0)
            );
        }

        #[test]
        fn sub() {
            assert_eq!(
                Float62::from_integer(2) - Float62::from_integer(3),
                Float62::from_integer(-1)
            );
            assert_eq!(
                Float62::from_integer(2) - Float62::from_float(3.0),
                Float62::from_float(-1.0)
            );
            assert_eq!(
                Float62::from_float(2.0) - Float62::from_integer(3),
                Float62::from_float(-1.0)
            );
            assert_eq!(
                Float62::from_float(2.0) - Float62::from_float(3.0),
                Float62::from_float(-1.0)
            );
        }

        #[test]
        fn mul() {
            assert_eq!(
                Float62::from_integer(2) * Float62::from_integer(3),
                Float62::from_integer(6)
            );
            assert_eq!(
                Float62::from_integer(2) * Float62::from_float(3.0),
                Float62::from_float(6.0)
            );
            assert_eq!(
                Float62::from_float(2.0) * Float62::from_integer(3),
                Float62::from_float(6.0)
            );
            assert_eq!(
                Float62::from_float(2.0) * Float62::from_float(3.0),
                Float62::from_float(6.0)
            );
        }

        #[test]
        fn div() {
            assert_eq!(
                Float62::from_integer(6) / Float62::from_integer(2),
                Float62::from_integer(3)
            );
            assert_eq!(
                Float62::from_integer(6) / Float62::from_float(2.0),
                Float62::from_float(3.0)
            );
            assert_eq!(
                Float62::from_float(6.0) / Float62::from_integer(2),
                Float62::from_float(3.0)
            );
            assert_eq!(
                Float62::from_float(6.0) / Float62::from_float(2.0),
                Float62::from_float(3.0)
            );
        }

        #[test]
        fn rem() {
            assert_eq!(
                Float62::from_integer(5) % Float62::from_integer(2),
                Float62::from_integer(1)
            );
            assert_eq!(
                Float62::from_integer(5) % Float62::from_float(2.0),
                Float62::from_float(1.0)
            );
            assert_eq!(
                Float62::from_float(5.0) % Float62::from_integer(2),
                Float62::from_float(1.0)
            );
            assert_eq!(
                Float62::from_float(5.0) % Float62::from_float(2.0),
                Float62::from_float(1.0)
            );
        }

        #[test]
        fn cmp() {
            assert_eq!(
                Float62::from_integer(0).partial_cmp(&Float62::from_integer(1)),
                Some(Ordering::Less)
            );
            assert_eq!(
                Float62::from_integer(0).partial_cmp(&Float62::from_float(1.0)),
                Some(Ordering::Less)
            );
            assert_eq!(
                Float62::from_integer(0).partial_cmp(&Float62::from_integer(1)),
                Some(Ordering::Less)
            );
            assert_eq!(
                Float62::from_float(0.0).partial_cmp(&Float62::from_integer(1)),
                Some(Ordering::Less)
            );

            assert_eq!(
                Float62::from_integer(42).partial_cmp(&Float62::from_float(42.0)),
                Some(Ordering::Equal)
            );
            assert_eq!(
                Float62::from_integer(1).partial_cmp(&Float62::from_float(0.0)),
                Some(Ordering::Greater)
            );
        }

        #[test]
        fn format() {
            assert_eq!(Float62::from_integer(42).to_string(), "42");
            assert_eq!(Float62::from_float(4.2).to_string(), "4.2");
            assert_eq!(Float62::from_payload(42).to_string(), "0x2a");
        }
    }
}
