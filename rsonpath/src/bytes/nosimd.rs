//! Sequential byte utilities _not_ utilizing SIMD.
//!
//! These are the default operations used when the `simd` default feature is disabled.

/// Find the first occurrence of a byte in the slice, if it exists.
///
/// This is a sequential, no-SIMD version.
/// For big slices it is recommended to enable the default `simd` flag for better performance.
/// # Examples
/// ```
/// # use rsonpath::bytes::find_byte;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte(b'd', bytes);
///
/// assert_eq!(Some(3), result);
/// ```
///
/// ```
/// # use rsonpath::bytes::find_byte;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte(b'i', bytes);
///
/// assert_eq!(None, result);
/// ```
#[inline(always)]
pub fn find_byte(byte: u8, slice: &[u8]) -> Option<usize> {
    slice.iter().position(|&x| x == byte)
}

/// Find the first occurrence of either of two bytes in the slice, if it exists.
///
/// This is a sequential, no-SIMD version.
/// For big slices it is recommended to enable the default `simd` flag for better performance.
/// # Examples
/// ```
/// # use rsonpath::bytes::find_byte2;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte2(b'd', b'c', bytes);
///
/// assert_eq!(Some(2), result);
/// ```
///
/// ```
/// # use rsonpath::bytes::find_byte2;
/// let bytes = "abcdefgh".as_bytes();
/// let result = find_byte2(b'i', b'j', bytes);
///
/// assert_eq!(None, result);
/// ```
#[inline(always)]
pub fn find_byte2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
    slice.iter().position(|&x| x == byte1 || x == byte2)
}
