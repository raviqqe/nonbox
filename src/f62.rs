//! NaN boxing for 62-bit floating-pointer numbers encompassing 63-bit integers
//! and 62-bit payloads.

const EXPONENT_MASK_OFFSET: usize = 48;
const SIGN_MASK: u64 = 1 << 63;
const EXPONENT_MASK: u64 = 0x7ffc << EXPONENT_MASK_OFFSET;
const PAYLOAD_MASK: u64 = !(0xfffc << EXPONENT_MASK_OFFSET);

/// Boxes a 50-bit unsigned integer.
pub const fn box_unsigned(payload: u64) -> u64 {
    EXPONENT_MASK | payload
}

/// Unboxes a 50-bit unsigned integer.
pub const fn unbox_unsigned(number: u64) -> Option<u64> {
    if is_boxed(number) {
        Some(number & PAYLOAD_MASK)
    } else {
        None
    }
}

/// Returns `true` if a payload is boxed in a given number.
pub const fn is_boxed(number: u64) -> bool {
    number & EXPONENT_MASK == EXPONENT_MASK
}
