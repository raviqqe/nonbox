//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers
//! and 62-bit payloads.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

const ROTATION_COUNT: u32 = 3;

/// Boxes a 63-bit unsigned integer.
pub const fn box_integer(integer: i64) -> u64 {
    (integer << 1) as _
}

/// Unboxes a 63-bit unsigned integer.
pub const fn unbox_integer(number: u64) -> Option<i64> {
    if is_integer(number) {
        Some(number as i64 >> 1)
    } else {
        None
    }
}

/// Returns `true` if a number is an integer.
pub const fn is_integer(number: u64) -> bool {
    number & 1 == 0
}

/// Boxes a 62-bit payload.
pub const fn box_payload(payload: u64) -> u64 {
    (payload << 2) | 1
}

/// Unboxes a 62-bit payload.
pub const fn unbox_payload(number: u64) -> Option<u64> {
    if is_payload(number) {
        Some(number >> 2)
    } else {
        None
    }
}

/// Returns `true` if a number is a payload.
pub const fn is_payload(number: u64) -> bool {
    number & 0b11 == 1
}

/// Boxes a 64-bit floating-point number.
pub const fn box_float(number: f64) -> u64 {
    if number == 0.0 {
        number.to_bits()
    } else {
        number.to_bits().rotate_left(ROTATION_COUNT) | 0b11
    }
}

/// Unboxes a 64-bit floating-point number.
pub fn unbox_float(number: u64) -> Option<f64> {
    if is_f62(number) {
        Some(unbox_float_unchecked(number))
    } else {
        None
    }
}

fn unbox_float_unchecked(number: u64) -> f64 {
    let exponent_tail = 2 - (number >> 63);

    f64::from_bits((number & !0b11 | exponent_tail).rotate_right(ROTATION_COUNT))
}

/// Returns `true` if a number is a 62-bit floating-point number.
pub const fn is_f62(number: u64) -> bool {
    number & 0b11 == 0b11
}

/// A 62-bit floating-point number.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Float62(u64);

impl Float62 {
    /// Creates a 62-bit floating-point number from its binary representation.
    pub fn new(number: u64) -> Self {
        Self(number)
    }

    /// Creates a 62-bit floating-point number from a payload.
    pub fn from_payload(payload: u64) -> Self {
        Self::new(box_payload(payload))
    }

    /// Creates a 62-bit floating-point number from an integer.
    pub fn from_integer(integer: i64) -> Self {
        Self::new(box_integer(integer))
    }

    /// Creates a 62-bit floating-point number from a 64-bit floating-point number.
    pub fn from_float(number: f64) -> Self {
        Self::new(box_float(number))
    }

    /// Returns a payload.
    pub fn to_payload(self) -> Option<u64> {
        unbox_payload(self.0)
    }

    /// Returns an integer.
    pub fn to_integer(self) -> Option<i64> {
        unbox_integer(self.0)
    }

    /// Returns a 64-bit floating-point number.
    pub fn to_float(self) -> Option<f64> {
        unbox_float(self.0)
    }

    fn to_number(self) -> Result<i64, f64> {
        self.to_integer()
            .ok_or_else(|| unbox_float_unchecked(self.0))
    }

    #[inline(always)]
    fn operate(
        self,
        rhs: Self,
        operate_integer: fn(i64, i64) -> i64,
        operate_float: fn(f64, f64) -> f64,
    ) -> Self {
        match (self.to_number(), rhs.to_number()) {
            (Ok(x), Ok(y)) => Self::from_integer(operate_integer(x, y)),
            (Ok(x), Err(y)) => Self::from_float(operate_float(x as f64, y)),
            (Err(x), Ok(y)) => Self::from_float(operate_float(x, y as f64)),
            (Err(x), Err(y)) => Self::from_float(operate_float(x, y)),
        }
    }
}

impl Add for Float62 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.operate(rhs, Add::add, Add::add)
    }
}

impl Sub for Float62 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.operate(rhs, Sub::sub, Sub::sub)
    }
}

impl Mul for Float62 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.operate(rhs, Mul::mul, Mul::mul)
    }
}

impl Div for Float62 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.operate(rhs, Div::div, Div::div)
    }
}

impl AddAssign for Float62 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Float62 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Float62 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Float62 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Neg for Float62 {
    type Output = Self;

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
}
