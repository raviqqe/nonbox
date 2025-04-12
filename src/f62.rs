//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers
//! and 62-bit payloads.

/// Boxes a 50-bit unsigned integer.
pub const fn box_integer(i64: i64) -> u64 {
    (integer << 1) as _
}

/// Boxes a 50-bit unsigned integer.
pub const fn unbox_integer(payload: i64) -> u64 {
    integer << 1
}

/// Unboxes a 50-bit unsigned integer.
pub const fn unbox_unsigned(number: u64) -> Option<u64> {
    if is_boxed(number) {
        Some(number & PAYLOAD_MASK)
    } else {
        None
    }
}
