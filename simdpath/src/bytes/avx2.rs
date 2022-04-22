/// Vectorized byte utilities utilizing SIMD.
///
/// These are the default operations. Currently SIMD is only implemented
/// for x86/x86_64 architecture CPUs supporting AVX2. For all other targets
/// the functions in this module fall back to the [`nosimd`] module.
///
/// This module is not compiled if the `nosimd` feature is enabled.
use memchr::*;

/// Size of a single SIMD block, i.e. number of bytes
/// stored in a single SIMD register.
pub const BLOCK_SIZE: usize = BYTES_IN_AVX2_REGISTER;

#[allow(dead_code)]
const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

/// Find the first occurrence of a byte in the slice, if it exists.
///
/// This is a SIMD version.
/// # Examples
/// ```
/// # use simdpath::bytes::simd::find_byte;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte(b'd', bytes);
///
/// assert_eq!(Some(3), result);
/// ```
///
/// ```
/// # use simdpath::bytes::simd::find_byte;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte(b'i', bytes);
///
/// assert_eq!(None, result);
/// ```
pub fn find_byte(byte: u8, slice: &[u8]) -> Option<usize> {
    memchr(byte, slice)
}

/// Find the first occurrence of either of two bytes in the slice, if it exists.
///
/// This is a SIMD version.
/// # Examples
/// ```
/// # use simdpath::bytes::simd::find_byte2;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte2(b'd', b'c', bytes);
///
/// assert_eq!(Some(2), result);
/// ```
///
/// ```
/// # use simdpath::bytes::simd::find_byte2;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte2(b'i', b'j', bytes);
///
/// assert_eq!(None, result);
/// ```
#[inline(always)]
pub fn find_byte2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
    memchr2(byte1, byte2, slice)
}
