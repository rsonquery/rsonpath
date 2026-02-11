//! SIMD primitives specific to aarch64 NEON extensions.
use ::core::arch::aarch64::*;

/// Unpack a [`uint8x16_t`] vector into an `[i16]` bitmask with same semantics as x86 movemasks.
#[inline]
#[target_feature(enable = "neon")]
pub(crate) unsafe fn neon_movemask_epi8(cmp_vector: uint8x16_t) -> i16 {
    // movemask has to be implemented manually for neon
    // The idea is to first truncate the values to their leading bit, then shift it so that
    // every 8-bit lane within a 64-bit half of the vector has its bit on a unique position.
    // We can then fold-add all values from one 64-bit half with addv into a single byte.
    // This is effectively a movemask on one half of the vector, which we can apply independently
    // on both halvess and then combine the result.

    let shift_values: [i8; 16] = [-7, -6, -5, -4, -3, -2, -1, 0, -7, -6, -5, -4, -3, -2, -1, 0];
    let vshift = vld1q_s8(shift_values.as_ptr());

    // Mask the leading bit in each byte.
    let vmask = vandq_u8(cmp_vector, vdupq_n_u8(0x80));
    // Shift
    let vmask = vshlq_u8(vmask, vshift);
    // Extract halves independently.
    let low = i16::from(vaddv_u8(vget_low_u8(vmask)));
    let high = i16::from(vaddv_u8(vget_high_u8(vmask)));

    // Combine.
    low | (high << 8)
}
