//! NaN boxing for `f64`.

const EXPONENT_MASK_OFFSET: usize = 48;
const SIGN_MASK: u64 = 1 << 63;
const EXPONENT_MASK: u64 = 0x7ffc << EXPONENT_MASK_OFFSET;
const PAYLOAD_MASK: u64 = !(0xfffc << EXPONENT_MASK_OFFSET);

/// Boxes a 50-bit unsigned integer.
#[inline]
pub const fn box_unsigned(payload: u64) -> u64 {
    EXPONENT_MASK | payload
}

/// Unboxes a 50-bit unsigned integer.
#[inline]
pub const fn unbox_unsigned(number: u64) -> Option<u64> {
    if is_boxed(number) {
        Some(unbox_unsigned_unchecked(number))
    } else {
        None
    }
}

/// Unboxes a 50-bit unsigned integer without any type check.
#[inline]
pub const fn unbox_unsigned_unchecked(number: u64) -> u64 {
    number & PAYLOAD_MASK
}

/// Boxes a 51-bit signed integer.
#[inline]
pub const fn box_signed(payload: i64) -> u64 {
    (if payload < 0 { SIGN_MASK } else { 0 }) | box_unsigned(payload.unsigned_abs())
}

/// Unboxes a 51-bit signed integer.
#[inline]
pub const fn unbox_signed(number: u64) -> Option<i64> {
    if let Some(value) = unbox_unsigned(number) {
        Some((if number & SIGN_MASK == 0 { 1 } else { -1 }) * value as i64)
    } else {
        None
    }
}

/// Returns `true` if a payload is boxed in a given number.
#[inline]
pub const fn is_boxed(number: u64) -> bool {
    number & EXPONENT_MASK == EXPONENT_MASK
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAXIMUM: i64 = 1 << 50;

    #[test]
    fn check_mask() {
        assert_ne!(EXPONENT_MASK, f64::NAN.to_bits());
        assert!(f64::from_bits(EXPONENT_MASK).is_nan());
    }

    #[test]
    fn unbox_nan() {
        assert_eq!(unbox_signed(f64::NAN.to_bits()), None);
        assert_eq!(unbox_signed(f64::INFINITY.to_bits()), None);
        assert_eq!(unbox_signed(f64::NEG_INFINITY.to_bits()), None);
    }

    #[test]
    fn box_unsigned_value() {
        fn box_to_f64(payload: u64) -> f64 {
            f64::from_bits(box_unsigned(payload))
        }

        assert!(box_to_f64(0).is_nan());
        assert!(box_to_f64(1).is_nan());
        assert!(box_to_f64(7).is_nan());
        assert!(box_to_f64(42).is_nan());
    }

    #[test]
    fn unbox_unsigned_value() {
        assert_eq!(unbox_unsigned(42.0f64.to_bits()), None);
        assert_eq!(unbox_unsigned(box_unsigned(0)), Some(0));
        assert_eq!(unbox_unsigned(box_unsigned(1)), Some(1));
        assert_eq!(unbox_unsigned(box_unsigned(7)), Some(7));
        assert_eq!(unbox_unsigned(box_unsigned(42)), Some(42));
    }

    #[test]
    fn unsigned_maximum() {
        let x = MAXIMUM as _;

        assert_eq!(unbox_unsigned(box_unsigned(x - 1)), Some(x - 1));
        assert_eq!(unbox_unsigned(box_unsigned(x)), Some(0));
    }

    #[test]
    fn box_signed_value() {
        fn box_to_f64(payload: i64) -> f64 {
            f64::from_bits(box_signed(payload))
        }

        assert!(box_to_f64(0).is_nan());
        assert!(box_to_f64(1).is_nan());
        assert!(box_to_f64(7).is_nan());
        assert!(box_to_f64(42).is_nan());
        assert!(box_to_f64(-1).is_nan());
        assert!(box_to_f64(-7).is_nan());
        assert!(box_to_f64(-42).is_nan());
    }

    #[test]
    fn unbox_signed_value() {
        assert_eq!(unbox_signed(42.0f64.to_bits()), None);
        assert_eq!(unbox_signed(box_signed(0)), Some(0));
        assert_eq!(unbox_signed(box_signed(1)), Some(1));
        assert_eq!(unbox_signed(box_signed(7)), Some(7));
        assert_eq!(unbox_signed(box_signed(42)), Some(42));
        assert_eq!(unbox_signed(box_signed(-1)), Some(-1));
        assert_eq!(unbox_signed(box_signed(-7)), Some(-7));
        assert_eq!(unbox_signed(box_signed(-42)), Some(-42));
    }

    #[test]
    fn signed_maximum() {
        assert_eq!(unbox_signed(box_signed(MAXIMUM - 1)), Some(MAXIMUM - 1));
        assert_eq!(unbox_signed(box_signed(MAXIMUM)), Some(0));
    }

    #[test]
    fn signed_minimum() {
        assert_eq!(unbox_signed(box_signed(1 - MAXIMUM)), Some(1 - MAXIMUM));
        assert_eq!(unbox_signed(box_signed(-MAXIMUM)), Some(0));
    }

    #[test]
    fn unbox_f64_value() {
        fn unbox_from_f64(number: f64) -> Option<u64> {
            unbox_unsigned(number.to_bits())
        }

        assert_eq!(unbox_from_f64(0.0), None);
        assert_eq!(unbox_from_f64(-1.0), None);
        assert_eq!(unbox_from_f64(1.0), None);
        assert_eq!(unbox_from_f64(42.0), None);
    }
}
