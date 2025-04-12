//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers
//! and 62-bit payloads.

/// Boxes a 63-bit unsigned integer.
pub const fn box_integer(integer: i64) -> u64 {
    (integer << 1) as _
}

/// Boxes a 63-bit unsigned integer.
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

/// Boxes a 62-bit payload.
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
}
