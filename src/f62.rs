//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers
//! and 62-bit payloads.

/// Boxes a 50-bit unsigned integer.
pub const fn box_integer(integer: i64) -> u64 {
    (integer << 1) as _
}

/// Boxes a 50-bit unsigned integer.
pub const fn unbox_integer(number: u64) -> i64 {
    number as i64 >> 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        assert_eq!(unbox_integer(box_integer(0)), 0);
        assert_eq!(unbox_integer(box_integer(1)), 1);
        assert_eq!(unbox_integer(box_integer(-1)), -1);
    }
}
