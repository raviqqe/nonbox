//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers,
//! 61-bit payloads, and infinities and NaN.

use core::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

const ROTATION_COUNT: u32 = 3;

const MANTISSA_WIDTH: u32 = 52;
const EXPONENT_MASK: u64 = (1 << 11) - 1;
const MINIMUM_EXPONENT: u64 = 0x300;
const MAXIMUM_EXPONENT: u64 = 0x4ff;

const SPECIAL_TAG: u64 = 0b101;
const NAN: u64 = SPECIAL_TAG;
const POSITIVE_INFINITY: u64 = (1 << 3) | SPECIAL_TAG;
const NEGATIVE_INFINITY: u64 = (2 << 3) | SPECIAL_TAG;

/// Boxes a 63-bit signed integer.
#[inline]
pub const fn box_integer(integer: i64) -> u64 {
    (integer << 1) as _
}

/// Unboxes a 63-bit signed integer.
#[inline]
pub const fn unbox_integer(number: u64) -> Option<i64> {
    if is_integer(number) {
        Some(unbox_integer_unchecked(number))
    } else {
        None
    }
}

/// Unboxes a 63-bit signed integer without any type check.
#[inline]
pub const fn unbox_integer_unchecked(number: u64) -> i64 {
    number as i64 >> 1
}

/// Returns `true` if a number is an integer.
#[inline]
pub const fn is_integer(number: u64) -> bool {
    number & 1 == 0
}

/// Boxes a 61-bit payload.
#[inline]
pub const fn box_payload(payload: u64) -> u64 {
    (payload << 3) | 1
}

/// Unboxes a 61-bit payload.
#[inline]
pub const fn unbox_payload(number: u64) -> Option<u64> {
    if is_payload(number) {
        Some(unbox_payload_unchecked(number))
    } else {
        None
    }
}

/// Unboxes a 61-bit payload without any type check.
#[inline]
pub const fn unbox_payload_unchecked(number: u64) -> u64 {
    number >> 3
}

/// Returns `true` if a number is a payload.
#[inline]
pub const fn is_payload(number: u64) -> bool {
    number & 0b111 == 1
}

/// Boxes a 64-bit floating-point number.
#[inline]
pub const fn box_float(number: f64) -> u64 {
    if number == 0.0 {
        0
    } else if number.is_nan() {
        NAN
    } else if number == f64::INFINITY {
        POSITIVE_INFINITY
    } else if number == f64::NEG_INFINITY {
        NEGATIVE_INFINITY
    } else {
        let bits = number.to_bits();
        let exponent = bits >> MANTISSA_WIDTH & EXPONENT_MASK;

        if exponent < MINIMUM_EXPONENT {
            0
        } else if exponent > MAXIMUM_EXPONENT {
            if number < 0.0 {
                NEGATIVE_INFINITY
            } else {
                POSITIVE_INFINITY
            }
        } else {
            bits.rotate_left(ROTATION_COUNT) | 0b11
        }
    }
}

/// Unboxes a 64-bit floating-point number.
#[inline]
pub const fn unbox_float(number: u64) -> Option<f64> {
    if is_float(number) {
        Some(unbox_float_unchecked(number))
    } else if is_nan(number) {
        Some(f64::NAN)
    } else if number == POSITIVE_INFINITY {
        Some(f64::INFINITY)
    } else if number == NEGATIVE_INFINITY {
        Some(f64::NEG_INFINITY)
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

/// Returns `true` if a number is an infinity.
#[inline]
pub const fn is_infinite(number: u64) -> bool {
    number == POSITIVE_INFINITY || number == NEGATIVE_INFINITY
}

/// Returns `true` if a number is NaN.
#[inline]
pub const fn is_nan(number: u64) -> bool {
    number == NAN
}

/// A 62-bit floating-point number.
#[derive(Clone, Copy, Debug, Default)]
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

    /// Returns `true` if this number is an infinity.
    #[inline]
    pub const fn is_infinite(self) -> bool {
        is_infinite(self.0)
    }

    /// Returns `true` if this number is NaN.
    #[inline]
    pub const fn is_nan(self) -> bool {
        is_nan(self.0)
    }

    #[inline]
    const fn to_number(self) -> Result<i64, f64> {
        if let Some(integer) = self.to_integer() {
            Ok(integer)
        } else if let Some(float) = self.to_float() {
            Err(float)
        } else {
            Err(f64::NAN)
        }
    }
}

fn operate_float(lhs: Float62, rhs: Float62, operate: fn(f64, f64) -> f64) -> Float62 {
    Float62::from_float(match (lhs.to_number(), rhs.to_number()) {
        (Ok(_), Ok(_)) => unreachable!(),
        (Ok(x), Err(y)) => operate(x as f64, y),
        (Err(x), Ok(y)) => operate(x, y as f64),
        (Err(x), Err(y)) => operate(x, y),
    })
}

macro_rules! operate {
    ($lhs:ident, $rhs:ident, $operate:ident, $wrapping_operate:ident) => {{
        let (Some(x), Some(y)) = ($lhs.to_integer(), $rhs.to_integer()) else {
            return operate_float($lhs, $rhs, f64::$operate);
        };

        Self::from_integer(x.$wrapping_operate(y))
    }};
}

impl Add for Float62 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, add, wrapping_add)
    }
}

impl Sub for Float62 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, sub, wrapping_sub)
    }
}

impl Mul for Float62 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        operate!(self, rhs, mul, wrapping_mul)
    }
}

impl Div for Float62 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let (Some(x), Some(y)) = (self.to_integer(), rhs.to_integer()) else {
            return operate_float(self, rhs, f64::div);
        };

        if y == 0 {
            Self::from_float(f64::NAN)
        } else if x % y == 0 {
            Self::from_integer(x / y)
        } else {
            Self::from_float(x as f64 / y as f64)
        }
    }
}

impl Rem for Float62 {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        let (Some(x), Some(y)) = (self.to_integer(), rhs.to_integer()) else {
            return operate_float(self, rhs, f64::rem);
        };

        if y == 0 {
            Self::from_float(f64::NAN)
        } else {
            Self::from_integer(x % y)
        }
    }
}

impl Float62 {
    /// Calculates the remainder of dividing this number by another number,
    /// returning `None` when both numbers are integers and the divisor is
    /// zero.
    #[inline]
    pub fn checked_rem(self, rhs: Self) -> Option<Self> {
        let (Some(x), Some(y)) = (self.to_integer(), rhs.to_integer()) else {
            return Some(self % rhs);
        };

        Some(Self::from_integer(x.checked_rem(y)?))
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

impl PartialEq for Float62 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Float62 {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            return (!self.is_nan()).then_some(Ordering::Equal);
        }

        match (self.to_number(), other.to_number()) {
            (Ok(x), Ok(y)) => x.partial_cmp(&y),
            (Ok(x), Err(y)) => (x as f64).partial_cmp(&y),
            (Err(x), Ok(y)) => x.partial_cmp(&(y as f64)),
            (Err(x), Err(y)) => x.partial_cmp(&y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    const INTEGER_LIMIT: i64 = 1 << 62;

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
    fn maximum_payload() {
        let maximum = (1 << 61) - 1;

        assert!(is_payload(box_payload(maximum)));
        assert_eq!(unbox_payload(box_payload(maximum)), Some(maximum));
        assert!(!is_infinite(box_payload(maximum)));
        assert!(!is_nan(box_payload(maximum)));
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

    #[test]
    fn keep_float_within_exponent_range() {
        let maximum = f64::from_bits(MAXIMUM_EXPONENT << MANTISSA_WIDTH);
        let minimum = f64::from_bits(MINIMUM_EXPONENT << MANTISSA_WIDTH);

        assert_eq!(unbox_float(box_float(maximum)), Some(maximum));
        assert_eq!(unbox_float(box_float(-maximum)), Some(-maximum));
        assert_eq!(unbox_float(box_float(minimum)), Some(minimum));
        assert_eq!(unbox_float(box_float(-minimum)), Some(-minimum));
    }

    #[test]
    fn saturate_to_infinity_on_overflow() {
        let overflow = f64::from_bits((MAXIMUM_EXPONENT + 1) << MANTISSA_WIDTH);

        assert_eq!(unbox_float(box_float(overflow)), Some(f64::INFINITY));
        assert_eq!(unbox_float(box_float(-overflow)), Some(f64::NEG_INFINITY));
        assert_eq!(unbox_float(box_float(f64::MAX)), Some(f64::INFINITY));
        assert_eq!(unbox_float(box_float(f64::MIN)), Some(f64::NEG_INFINITY));
    }

    #[test]
    fn flush_to_zero_on_underflow() {
        let underflow = f64::from_bits((MINIMUM_EXPONENT - 1) << MANTISSA_WIDTH);

        assert_eq!(box_float(underflow), 0);
        assert_eq!(box_float(-underflow), 0);
        assert_eq!(box_float(f64::MIN_POSITIVE), 0);
        assert_eq!(unbox_integer(box_float(underflow)), Some(0));
    }

    #[test]
    fn negative_zero() {
        assert_eq!(box_float(-0.0), box_float(0.0));
        assert_eq!(unbox_integer(box_float(-0.0)), Some(0));
        assert_eq!(unbox_float(box_float(-0.0)), None);
    }

    #[test]
    fn infinity() {
        assert!(is_infinite(box_float(f64::INFINITY)));
        assert!(is_infinite(box_float(f64::NEG_INFINITY)));
        assert_ne!(box_float(f64::INFINITY), box_float(f64::NEG_INFINITY));
        assert_eq!(unbox_float(box_float(f64::INFINITY)), Some(f64::INFINITY));
        assert_eq!(
            unbox_float(box_float(f64::NEG_INFINITY)),
            Some(f64::NEG_INFINITY)
        );

        for number in [box_float(f64::INFINITY), box_float(f64::NEG_INFINITY)] {
            assert!(!is_nan(number));
            assert!(!is_integer(number));
            assert!(!is_payload(number));
            assert!(!is_float(number));
        }
    }

    #[test]
    fn nan() {
        let number = box_float(f64::NAN);

        assert!(is_nan(number));
        assert!(unbox_float(number).unwrap().is_nan());
        assert_eq!(box_float(f64::NAN), box_float(-f64::NAN));
        assert!(!is_infinite(number));
        assert!(!is_integer(number));
        assert!(!is_payload(number));
        assert!(!is_float(number));
    }

    #[test]
    fn distinguish_representations() {
        let classify = |number| {
            (
                is_integer(number),
                is_payload(number),
                is_float(number),
                is_infinite(number),
                is_nan(number),
            )
        };

        assert_eq!(
            classify(box_integer(42)),
            (true, false, false, false, false)
        );
        assert_eq!(
            classify(box_payload(42)),
            (false, true, false, false, false)
        );
        assert_eq!(classify(box_float(4.2)), (false, false, true, false, false));
        assert_eq!(
            classify(box_float(f64::INFINITY)),
            (false, false, false, true, false)
        );
        assert_eq!(
            classify(box_float(f64::NEG_INFINITY)),
            (false, false, false, true, false)
        );
        assert_eq!(
            classify(box_float(f64::NAN)),
            (false, false, false, false, true)
        );
    }

    mod float62 {
        use super::*;

        #[test]
        fn default() {
            assert_eq!(Float62::default(), Float62::from_integer(0));
            assert_eq!(Float62::default(), Float62::from_float(0.0));
        }

        #[test]
        fn negative_zero() {
            assert_eq!(Float62::from_float(-0.0), Float62::from_integer(0));
            assert_eq!(Float62::from_float(-0.0), Float62::from_float(0.0));
            assert_eq!(Float62::from_float(-0.0).to_integer(), Some(0));
            assert_eq!(
                Float62::from_float(-1.0) * Float62::from_float(0.0),
                Float62::from_integer(0)
            );
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
                Float62::from_integer(1) / Float62::from_integer(2),
                Float62::from_float(0.5)
            );
            assert_eq!(
                Float62::from_integer(7) / Float62::from_integer(2),
                Float62::from_float(3.5)
            );
            assert_eq!(
                Float62::from_integer(-1) / Float62::from_integer(2),
                Float62::from_float(-0.5)
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
        fn div_by_zero() {
            assert!((Float62::from_integer(1) / Float62::from_integer(0)).is_nan());
            assert_eq!(
                Float62::from_float(6.0) / Float62::from_integer(0),
                Float62::from_float(f64::INFINITY)
            );
            assert!((Float62::from_float(6.0) / Float62::from_integer(0)).is_infinite());
            assert_eq!(
                Float62::from_float(6.0) / Float62::from_float(0.0),
                Float62::from_float(f64::INFINITY)
            );
        }

        #[test]
        fn infinity() {
            assert!(Float62::from_float(f64::INFINITY).is_infinite());
            assert!(Float62::from_float(f64::NEG_INFINITY).is_infinite());
            assert!(!Float62::from_float(f64::INFINITY).is_nan());
            assert_eq!(
                Float62::from_float(f64::INFINITY).to_float(),
                Some(f64::INFINITY)
            );
            assert_eq!(
                Float62::from_float(f64::NEG_INFINITY).to_float(),
                Some(f64::NEG_INFINITY)
            );
            assert_eq!(Float62::from_float(f64::INFINITY).to_integer(), None);
            assert_eq!(Float62::from_float(f64::INFINITY).to_payload(), None);
            assert_eq!(
                -Float62::from_float(f64::INFINITY),
                Float62::from_float(f64::NEG_INFINITY)
            );
            assert_eq!(
                Float62::from_float(f64::INFINITY) + Float62::from_integer(1),
                Float62::from_float(f64::INFINITY)
            );
            assert!(
                (Float62::from_float(f64::INFINITY) - Float62::from_float(f64::INFINITY)).is_nan()
            );
        }

        #[test]
        fn nan() {
            assert!(Float62::from_float(f64::NAN).is_nan());
            assert!(!Float62::from_float(f64::NAN).is_infinite());
            assert!(Float62::from_float(f64::NAN).to_float().unwrap().is_nan());
            assert_eq!(Float62::from_float(f64::NAN).to_integer(), None);
            assert_eq!(Float62::from_float(f64::NAN).to_payload(), None);
            assert!((-Float62::from_float(f64::NAN)).is_nan());
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
        fn rem_by_zero() {
            assert!((Float62::from_integer(6) % Float62::from_integer(0)).is_nan());
            assert!((Float62::from_integer(-6) % Float62::from_integer(0)).is_nan());
            assert!((Float62::from_float(6.0) % Float62::from_integer(0)).is_nan());
        }

        #[test]
        fn neg() {
            assert_eq!(-Float62::from_integer(42), Float62::from_integer(-42));
            assert_eq!(-Float62::from_integer(-42), Float62::from_integer(42));
            assert_eq!(-Float62::from_float(4.2), Float62::from_float(-4.2));
            assert_eq!(
                -Float62::from_integer(INTEGER_LIMIT - 1),
                Float62::from_integer(-INTEGER_LIMIT + 1)
            );
        }

        #[test]
        fn checked_rem() {
            assert_eq!(
                Float62::from_integer(5).checked_rem(Float62::from_integer(2)),
                Some(Float62::from_integer(1))
            );
            assert_eq!(
                Float62::from_integer(5).checked_rem(Float62::from_float(2.0)),
                Some(Float62::from_float(1.0))
            );
            assert_eq!(
                Float62::from_float(5.0).checked_rem(Float62::from_integer(2)),
                Some(Float62::from_float(1.0))
            );
            assert_eq!(
                Float62::from_float(5.0).checked_rem(Float62::from_float(2.0)),
                Some(Float62::from_float(1.0))
            );
            assert_eq!(
                Float62::from_integer(-7).checked_rem(Float62::from_integer(2)),
                Some(Float62::from_integer(-1))
            );
            assert_eq!(
                Float62::from_integer(7).checked_rem(Float62::from_integer(-2)),
                Some(Float62::from_integer(1))
            );
        }

        #[test]
        fn checked_rem_by_zero() {
            assert_eq!(
                Float62::from_integer(5).checked_rem(Float62::from_integer(0)),
                None
            );
            assert_eq!(
                Float62::from_integer(5).checked_rem(Float62::from_float(0.0)),
                None
            );
            assert!(
                Float62::from_float(5.0)
                    .checked_rem(Float62::from_integer(0))
                    .is_some()
            );
            assert!(
                Float62::from_float(5.0)
                    .checked_rem(Float62::from_float(0.0))
                    .is_some()
            );
        }

        #[test]
        fn keep_integer_within_range() {
            assert_eq!(
                Float62::from_integer(INTEGER_LIMIT - 2) + Float62::from_integer(1),
                Float62::from_integer(INTEGER_LIMIT - 1)
            );
            assert_eq!(
                Float62::from_integer(-INTEGER_LIMIT + 1) - Float62::from_integer(1),
                Float62::from_integer(-INTEGER_LIMIT)
            );
            assert_eq!(
                Float62::from_integer(INTEGER_LIMIT - 1) * Float62::from_integer(1),
                Float62::from_integer(INTEGER_LIMIT - 1)
            );
        }

        #[test]
        fn arithmetic_matches_reference() {
            let values = [
                0,
                1,
                -1,
                42,
                -42,
                1 << 26,
                -(1 << 26),
                1 << 40,
                -(1 << 40),
                INTEGER_LIMIT - 1,
                INTEGER_LIMIT - 2,
                -INTEGER_LIMIT,
                -INTEGER_LIMIT + 1,
            ];

            for &x in &values {
                for &y in &values {
                    assert_eq!(
                        (Float62::from_integer(x) + Float62::from_integer(y)).to_bits(),
                        Float62::from_integer(x.wrapping_add(y)).to_bits()
                    );
                    assert_eq!(
                        (Float62::from_integer(x) - Float62::from_integer(y)).to_bits(),
                        Float62::from_integer(x.wrapping_sub(y)).to_bits()
                    );
                    assert_eq!(
                        (Float62::from_integer(x) * Float62::from_integer(y)).to_bits(),
                        Float62::from_integer(x.wrapping_mul(y)).to_bits()
                    );
                }
            }
        }

        #[test]
        fn arithmetic_out_of_range_integers() {
            let big = Float62::from_integer(1 << 60);
            let huge = Float62::from_integer(INTEGER_LIMIT - 1);

            assert_eq!(big.to_integer(), Some(1 << 60));
            assert_eq!(
                (big + big).to_bits(),
                Float62::from_integer((1i64 << 60).wrapping_add(1 << 60)).to_bits()
            );
            assert_eq!((big - big).to_bits(), Float62::from_integer(0).to_bits());
            assert_eq!(
                (big * big).to_bits(),
                Float62::from_integer((1i64 << 60).wrapping_mul(1 << 60)).to_bits()
            );
            assert_eq!(
                (huge + huge).to_bits(),
                Float62::from_integer((INTEGER_LIMIT - 1) * 2).to_bits()
            );
            assert_eq!(
                (huge * huge).to_bits(),
                Float62::from_integer((INTEGER_LIMIT - 1).wrapping_mul(INTEGER_LIMIT - 1))
                    .to_bits()
            );
        }

        #[test]
        fn saturate_to_infinity_on_float_overflow() {
            assert_eq!(
                (Float62::from_float(1e70) * Float62::from_float(1e70)).to_float(),
                Some(f64::INFINITY)
            );
            assert_eq!(
                (Float62::from_float(-1e70) * Float62::from_float(1e70)).to_float(),
                Some(f64::NEG_INFINITY)
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
        fn compare_infinity() {
            assert_eq!(
                Float62::from_float(f64::INFINITY).partial_cmp(&Float62::from_integer(0)),
                Some(Ordering::Greater)
            );
            assert_eq!(
                Float62::from_float(f64::NEG_INFINITY).partial_cmp(&Float62::from_integer(0)),
                Some(Ordering::Less)
            );
            assert_eq!(
                Float62::from_float(f64::NEG_INFINITY)
                    .partial_cmp(&Float62::from_float(f64::INFINITY)),
                Some(Ordering::Less)
            );
        }

        #[test]
        fn compare_nan() {
            assert_eq!(
                Float62::from_float(f64::NAN).partial_cmp(&Float62::from_float(f64::NAN)),
                None
            );
            assert_eq!(
                Float62::from_float(f64::NAN).partial_cmp(&Float62::from_integer(0)),
                None
            );
        }

        #[test]
        fn equality() {
            assert_eq!(Float62::from_integer(4), Float62::from_float(4.0));
            assert_eq!(Float62::from_integer(0), Float62::from_float(0.0));
            assert_eq!(
                Float62::from_float(f64::INFINITY),
                Float62::from_float(f64::INFINITY)
            );
            assert_ne!(
                Float62::from_float(f64::INFINITY),
                Float62::from_float(f64::NEG_INFINITY)
            );
            assert_ne!(Float62::from_float(f64::NAN), Float62::from_float(f64::NAN));
            assert_eq!(Float62::from_payload(42), Float62::from_payload(42));
            assert_ne!(Float62::from_payload(42), Float62::from_payload(43));
            assert_ne!(Float62::from_payload(4), Float62::from_integer(4));
        }

        #[test]
        fn equality_matches_ordering() {
            let values = [
                Float62::from_integer(0),
                Float62::from_integer(4),
                Float62::from_integer(-4),
                Float62::from_float(4.0),
                Float62::from_float(4.5),
                Float62::from_float(f64::INFINITY),
                Float62::from_float(f64::NEG_INFINITY),
                Float62::from_float(f64::NAN),
                Float62::from_payload(1),
                Float62::from_payload(2),
            ];

            for &x in &values {
                for &y in &values {
                    assert_eq!(x == y, x.partial_cmp(&y) == Some(Ordering::Equal));
                    assert_eq!(x == y, y == x);
                }
            }
        }

        #[test]
        fn compare_integer_and_fractional_float() {
            assert_eq!(
                Float62::from_integer(1).partial_cmp(&Float62::from_float(1.5)),
                Some(Ordering::Less)
            );
            assert_eq!(
                Float62::from_integer(2).partial_cmp(&Float62::from_float(1.5)),
                Some(Ordering::Greater)
            );
        }

        #[test]
        fn equality_is_transitive() {
            let values = [
                Float62::from_integer(1 << 53),
                Float62::from_float((1u64 << 53) as f64),
                Float62::from_integer(0),
                Float62::from_float(0.0),
                Float62::from_integer(4),
                Float62::from_float(4.0),
            ];

            for &x in &values {
                for &y in &values {
                    for &z in &values {
                        if x == y && y == z {
                            assert_eq!(x, z);
                        }
                    }
                }
            }
        }

        #[test]
        fn format() {
            assert_eq!(Float62::from_integer(0).to_string(), "0");
            assert_eq!(Float62::from_integer(1).to_string(), "1");
            assert_eq!(Float62::from_float(0.0).to_string(), "0");
            assert_eq!(Float62::from_float(1.0).to_string(), "1");
            assert_eq!(Float62::from_integer(42).to_string(), "42");
            assert_eq!(Float62::from_float(4.2).to_string(), "4.2");
            assert_eq!(Float62::from_payload(42).to_string(), "0x2a");
        }

        #[test]
        fn format_special() {
            assert_eq!(Float62::from_float(f64::INFINITY).to_string(), "inf");
            assert_eq!(Float62::from_float(f64::NEG_INFINITY).to_string(), "-inf");
            assert_eq!(Float62::from_float(f64::NAN).to_string(), "NaN");
        }
    }
}
