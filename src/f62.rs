//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers
//! and 62-bit payloads.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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
        Some(number as i64 >> 1)
    } else {
        None
    }
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
        Some(number >> 2)
    } else {
        None
    }
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
    if is_f62(number) {
        Some(unbox_float_unchecked(number))
    } else {
        None
    }
}

#[inline]
const fn unbox_float_unchecked(number: u64) -> f64 {
    let exponent_tail = 2 - (number >> 63);

    f64::from_bits((number & !0b11 | exponent_tail).rotate_right(ROTATION_COUNT))
}

/// Returns `true` if a number is a 62-bit floating-point number.
#[inline]
pub const fn is_f62(number: u64) -> bool {
    number & 0b11 == 0b11
}

/// A 62-bit floating-point number.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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

    /// Creates a 62-bit floating-point number from a 64-bit floating-point number.
    #[inline]
    pub const fn from_float(number: f64) -> Self {
        Self::from_bits(box_float(number))
    }

    /// Returns a payload.
    #[inline]
    pub const fn to_payload(self) -> Option<u64> {
        unbox_payload(self.0)
    }

    /// Returns an integer.
    #[inline]
    pub const fn to_integer(self) -> Option<i64> {
        unbox_integer(self.0)
    }

    /// Returns a 64-bit floating-point number.
    #[inline]
    pub const fn to_float(self) -> Option<f64> {
        unbox_float(self.0)
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(is_f62(box_float(1.0)));
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
    }
}
