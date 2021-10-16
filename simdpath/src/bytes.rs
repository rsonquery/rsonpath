//! Byte stream utilities for quick navigation through a JSON structure.
//!
//! This module comes in two flavours, SIMD and no-SIMD.
//!
//! SIMD is implemented for x86/x86_64 architectures and CPUs supporting AVX2.
//! On other architectures or older CPUs the module falls back to using no-SIMD operations.
//! The no-SIMD operations are always available through the [`nosimd`] submodule.
//!
//! This module publicly reexports the functions from either [`simd`] or [`nosimd`] submodule
//! and these reexports are what the JSONPath engine implementations use. The default is
//! SIMD mode, but this can be changed by enabling the feature `nosimd`.

#[cfg(feature = "nosimd")]
pub use nosimd::*;
#[cfg(not(feature = "nosimd"))]
#[doc(inline)]
pub use simd::*;

/// Find the first occurence of a byte in the slice that is not escaped, if it exists.
///
/// An escaped byte is one which is immediately preceeded by an unescaped backslash
/// character `\` U+005C. This applies recursively, so the quote character in `\"`
/// is escaped, in `\\"` it's unescaped, `\\\"` it's escaped, etc.
///
/// # Examples
/// ```
/// # use simdpath::bytes::find_unescaped_byte;
/// let bytes = "jsonpath".as_bytes();
/// let result = find_unescaped_byte(b'n', bytes);
///
/// assert_eq!(Some(3), result);
/// ```
///
/// ```
/// # use simdpath::bytes::find_unescaped_byte;
/// let bytes = "jsonpath".as_bytes();
/// let result = find_unescaped_byte(b'x', bytes);
///
/// assert_eq!(None, result);
/// ```
///
/// ```
/// # use simdpath::bytes::find_unescaped_byte;
/// let bytes = r#"jso\npath"#.as_bytes();
/// let result = find_unescaped_byte(b'n', bytes);
///
/// assert_eq!(None, result);
/// ```
///
/// ```
/// # use simdpath::bytes::find_unescaped_byte;
/// let bytes = r#"jso\\\\npath"#.as_bytes();
/// let result = find_unescaped_byte(b'n', bytes);
///
/// assert_eq!(Some(7), result);
/// ```
pub fn find_unescaped_byte(byte: u8, slice: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < slice.len() {
        let j = find_byte(byte, &slice[i..]);
        match j {
            None => return None,
            Some(j) if j == 0 => return Some(j + i),
            Some(j) => {
                if !is_escaped(j + i, slice) {
                    return Some(j + i);
                } else {
                    i = j + i + 1;
                }
            }
        }
    }
    None
}

/// Find the first occurence of either of two bites in the slice that is not escaped, if it exists.
///
/// An escaped byte is one which is immediately preceeded by an unescaped backslash
/// character `\` U+005C. This applies recursively, so the quote character in `\"`
/// is escaped, in `\\"` it's unescaped, `\\\"` it's escaped, etc.
///
/// # Examples
/// ```
/// # use simdpath::bytes::find_unescaped_byte2;
/// let bytes = "jsonpath".as_bytes();
/// let result = find_unescaped_byte2(b'n', b'o', bytes);
///
/// assert_eq!(Some(2), result);
/// ```
///
/// ```
/// # use simdpath::bytes::find_unescaped_byte2;
/// let bytes = "jsonpath".as_bytes();
/// let result = find_unescaped_byte2(b'n', b'p', bytes);
///
/// assert_eq!(Some(3), result);
/// ```
///
/// ```
/// # use simdpath::bytes::find_unescaped_byte2;
/// let bytes = r#"jso\npath"#.as_bytes();
/// let result = find_unescaped_byte2(b'n', b'p', bytes);
///
/// assert_eq!(Some(5), result);
/// ```
///
/// ```
/// # use simdpath::bytes::find_unescaped_byte2;
/// let bytes = r#"jso\\\\npath"#.as_bytes();
/// let result = find_unescaped_byte2(b'n', b'p', bytes);
///
/// assert_eq!(Some(7), result);
/// ```
///
/// ```
/// # use simdpath::bytes::find_unescaped_byte2;
/// let bytes = r#"jso\n\path"#.as_bytes();
/// let result = find_unescaped_byte2(b'n', b'p', bytes);
///
/// assert_eq!(None, result);
/// ```
pub fn find_unescaped_byte2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < slice.len() {
        let j = find_byte2(byte1, byte2, &slice[i..]);
        match j {
            None => return None,
            Some(j) if j == 0 => return Some(j + i),
            Some(j) => {
                if !is_escaped(j + i, slice) {
                    return Some(j + i);
                } else {
                    i = j + i + 1;
                }
            }
        }
    }
    None
}

#[inline(always)]
fn is_escaped(idx: usize, slice: &[u8]) -> bool {
    let mut k = 1;
    let mut parity = true;

    while idx >= k && slice[idx - k] == b'\\' {
        k += 1;
        parity = !parity;
    }

    !parity
}

/// Find the first occurence of a non-whitespace byte in the slice, if it exists.
///
/// This function is a stub. Currently we assume there is no whitespace between structural
/// characters, so the next non-whitespace byte is simply the next byte.
#[inline(always)]
pub fn find_non_whitespace(slice: &[u8]) -> Option<usize> {
    if slice.is_empty() {
        None
    } else {
        Some(0)
    }
}

/// Sequential byte utilities _not_ utlizing SIMD.
///
/// These are the default operations used when the `nosimd` feature is enabled,
/// or AVX2 is not supported on the target CPU.
pub mod nosimd {
    /// Find the first occurence of a byte in the slice, if it exists.
    ///
    /// This is a sequential, no-SIMD version. For big slices it is recommended to use
    /// the [`simd`](super::simd) module variant for better performance.
    /// # Examples
    /// ```
    /// # use simdpath::bytes::nosimd::find_byte;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte(b'd', bytes);
    ///
    /// assert_eq!(Some(3), result);
    /// ```
    ///
    /// ```
    /// # use simdpath::bytes::nosimd::find_byte;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte(b'i', bytes);
    ///
    /// assert_eq!(None, result);
    /// ```
    #[inline(always)]
    pub fn find_byte(byte: u8, slice: &[u8]) -> Option<usize> {
        slice.iter().position(|&x| x == byte)
    }

    /// Find the first occurence of either of two bytes in the slice, if it exists.
    ///
    /// This is a sequential, no-SIMD version. For big slices it is recommended to use
    /// the [`simd`](super::simd) module variant for better performance.
    /// # Examples
    /// ```
    /// # use simdpath::bytes::nosimd::find_byte2;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte2(b'd', b'c', bytes);
    ///
    /// assert_eq!(Some(2), result);
    /// ```
    ///
    /// ```
    /// # use simdpath::bytes::nosimd::find_byte2;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte2(b'i', b'j', bytes);
    ///
    /// assert_eq!(None, result);
    /// ```
    #[inline(always)]
    pub fn find_byte2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
        slice.iter().position(|&x| x == byte1 || x == byte2)
    }

    /// Find the first occurence of a two byte sequence in the slice, if it exists.
    ///
    /// This is a sequential, no-SIMD version. For big slices it is recommended to use
    /// the [`simd`](super::simd) module variant for better performance.
    /// # Examples
    /// ```
    /// # use simdpath::bytes::nosimd::find_byte_sequence2;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte_sequence2(b'd', b'e', bytes);
    ///
    /// assert_eq!(Some(3), result);
    /// ```
    ///
    /// ```
    /// # use simdpath::bytes::nosimd::find_byte_sequence2;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte_sequence2(b'e', b'd', bytes);
    ///
    /// assert_eq!(None, result);
    /// ```
    pub fn find_byte_sequence2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
        let needle = [byte1, byte2];
        slice.windows(2).position(|xs| xs == needle)
    }
}

/// Vectorized byte utilities utilizing SIMD.
///
/// These are the default operations. Currently SIMD is only implemented
/// for x86/x86_64 architecture CPUs supporting AVX2. For all other targets
/// the functions in this module fall back to the [`nosimd`] module.
///
/// This module is not compiled if the `nosimd` feature is enabled.
#[cfg(not(feature = "nosimd"))]
pub mod simd {
    use super::nosimd;
    #[cfg(all(target_arch = "x86", target_feature = "avx2"))]
    use core::arch::x86::*;
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    use core::arch::x86_64::*;
    use memchr::*;

    #[allow(dead_code)]
    const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

    /// Find the first occurence of a byte in the slice, if it exists.
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

    /// Find the first occurence of either of two bytes in the slice, if it exists.
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

    /// Find the first occurence of a two byte sequence in the slice, if it exists.
    ///
    /// This is a SIMD version, if the target CPU is not x86/x86_64 or does not
    /// support AVX2 this will fallback to [`nosimd::find_byte_sequence2`].
    /// # Examples
    /// ```
    /// # use simdpath::bytes::simd::find_byte_sequence2;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte_sequence2(b'd', b'e', bytes);
    ///
    /// assert_eq!(Some(3), result);
    /// ```
    ///
    /// ```
    /// # use simdpath::bytes::simd::find_byte_sequence2;
    /// let bytes = "abcdefgh".as_bytes();
    /// let result = find_byte_sequence2(b'e', b'd', bytes);
    ///
    /// assert_eq!(None, result);
    /// ```
    #[inline(always)]
    pub fn find_byte_sequence2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "avx2"
        ))]
        unsafe {
            avx2_find_byte_sequence2(byte1, byte2, slice)
        }

        #[cfg(not(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "avx2"
        )))]
        nosimd::find_byte_sequence2(byte1, byte2, slice)
    }

    #[target_feature(enable = "avx2")]
    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ))]
    unsafe fn avx2_find_byte_sequence2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
        if slice.len() < BYTES_IN_AVX2_REGISTER * 2 {
            return nosimd::find_byte_sequence2(byte1, byte2, slice);
        }

        let mut slice = slice;
        let mut i: usize = 0;

        let mask1 = _mm256_set1_epi8(byte1 as i8);
        let mask2 = _mm256_set1_epi8(byte2 as i8);

        let first_block = _mm256_loadu_si256(slice.as_ptr() as *const __m256i);
        let cmp_mask1_first_block_vector = _mm256_cmpeq_epi8(first_block, mask1);
        let cmp_mask2_first_block_vector = _mm256_cmpeq_epi8(first_block, mask2);
        let mut cmp_mask1_first_block = _mm256_movemask_epi8(cmp_mask1_first_block_vector) as u32;
        let mut cmp_mask2_first_block = _mm256_movemask_epi8(cmp_mask2_first_block_vector) as u32;

        while slice.len() >= BYTES_IN_AVX2_REGISTER * 2 {
            let ptr = slice.as_ptr() as *const __m256i;

            let next_block = _mm256_loadu_si256(ptr.offset(1));

            let cmp_mask1_next_block_vector = _mm256_cmpeq_epi8(next_block, mask1);
            let cmp_mask2_next_block_vector = _mm256_cmpeq_epi8(next_block, mask2);

            let cmp_mask1_next_block = _mm256_movemask_epi8(cmp_mask1_next_block_vector) as u32;
            let cmp_mask2_next_block = _mm256_movemask_epi8(cmp_mask2_next_block_vector) as u32;

            let cmp_mask1 = (cmp_mask1_first_block as u64) | ((cmp_mask1_next_block as u64) << 32);
            let cmp_mask2 = (cmp_mask2_first_block as u64) | ((cmp_mask2_next_block as u64) << 32);

            let cmp = cmp_mask1 & (cmp_mask2 >> 1);

            if cmp != 0 {
                return Some(i + (cmp.trailing_zeros() as usize));
            }

            cmp_mask1_first_block = cmp_mask1_next_block;
            cmp_mask2_first_block = cmp_mask2_next_block;
            i += BYTES_IN_AVX2_REGISTER;
            slice = &slice[BYTES_IN_AVX2_REGISTER..];
        }

        nosimd::find_byte_sequence2(byte1, byte2, slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[inline(always)]
    fn find_non_whitespace(slice: &[u8]) -> Option<usize> {
        // Insignificant whitespace in JSON:
        // https://datatracker.ietf.org/doc/html/rfc4627#section-2
        const WHITESPACES: [u8; 4] = [b' ', b'\n', b'\t', b'\r'];
        let mut i = 0;
        while i < slice.len() {
            if !WHITESPACES.contains(&slice[i]) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    #[test]
    fn find_byte_when_there_are_no_bytes_returns_none() {
        let string = "";
        let bytes = string.as_bytes();

        let result = find_byte(b'{', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte_when_there_is_only_that_byte_returns_0() {
        let string = "{";
        let bytes = string.as_bytes();

        let result = find_byte(b'{', bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte_when_byte_exists_in_input_returns_first_occurence() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let bytes = string.as_bytes();

        let result = find_byte(b'"', bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_byte_when_byte_does_not_exist_in_input_returns_none() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let bytes = string.as_bytes();

        let result = find_byte(b'{', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte2_when_there_are_no_bytes_returns_none() {
        let string = "";
        let bytes = string.as_bytes();

        let result = find_byte2(b'{', b'}', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte2_when_there_is_only_first_byte_returns_0() {
        let string = "{";
        let bytes = string.as_bytes();

        let result = find_byte2(b'{', b'}', bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte2_when_there_is_only_second_byte_returns_0() {
        let string = "}";
        let bytes = string.as_bytes();

        let result = find_byte2(b'{', b'}', bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte2_when_byte_exists_in_input_returns_first_occurence_1() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let bytes = string.as_bytes();

        let result = find_byte2(b'"', b'L', bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_byte2_when_byte_exists_in_input_returns_first_occurence_2() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let bytes = string.as_bytes();

        let result = find_byte2(b'P', b'G', bytes);

        assert_eq!(Some(29), result);
    }

    #[test]
    fn find_byte2_when_neither_byte_does_not_exist_in_input_returns_none() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let bytes = string.as_bytes();

        let result = find_byte2(b'M', b'X', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_there_is_no_bytes_returns_none() {
        let string = "";
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'{', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_there_is_only_that_byte_returns_0() {
        let string = "{";
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'{', bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_unescaped_byte_when_all_matching_bytes_are_escaped_returns_none_1() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'"', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_all_matching_bytes_are_escaped_returns_none_2() {
        let string = r#"personne qui \"fait la plonge\" dans la restauration"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'"', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_some_matching_bytes_are_escaped_returns_first_unescaped_1() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches",
        {
            "label": 123
        }"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'"', bytes);

        assert_eq!(Some(55), result);
    }

    #[test]
    fn find_unescaped_byte_when_some_matching_bytes_are_escaped_returns_first_unescaped_2() {
        let string = r#"personne qui \"fait la plonge\" dans la restauration
        },
        "en""#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'"', bytes);

        assert_eq!(Some(72), result);
    }

    #[test]
    fn find_unescaped_byte_when_the_backslash_is_escaped_treats_as_unsescaped() {
        let string = r#"text \n xxx yyyy \\n was unescaped"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'n', bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_unescaped_byte_when_the_backslash_is_escaped_many_times_treats_as_unsescaped() {
        let string = r#"text \n xxx yyyy \\\\\\\\\\\\n was unescaped"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte(b'n', bytes);

        assert_eq!(Some(29), result);
    }

    #[test]
    fn find_unescaped_byte2_when_there_is_no_bytes_returns_none() {
        let string = "";
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'{', b'}', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte2_when_there_is_only_first_byte_returns_0() {
        let string = "{";
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'{', b'}', bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_unescaped_byte2_when_there_is_only_second_byte_returns_0() {
        let string = "}";
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'}', bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_unescaped_byte2_when_some_matching_bytes_are_escaped_returns_first_unescaped_1() {
        let string = r#"administrateur de \"La Libre Parole\", maire de Garches",
        {
            "label": 123
        }"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'}', b'{', bytes);

        assert_eq!(Some(66), result);
    }

    #[test]
    fn find_unescaped_byte2_when_some_matching_bytes_are_escaped_returns_first_unescaped_2() {
        let string = r#"personne qui \"fait la plonge\" dans la restauration
        },
        "en""#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'}', bytes);

        assert_eq!(Some(61), result);
    }

    #[test]
    fn find_unescaped_byte2_when_the_backslash_is_escaped_treats_as_unsescaped_1() {
        let string = r#"text \n xxx yyyy \\n was unescaped"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'n', b'}', bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_unescaped_byte2_when_the_backslash_is_escaped_treats_as_unsescaped_2() {
        let string = r#"text \n xxx yyyy \\n was unescaped"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'}', b'n', bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_unescaped_byte2_when_the_backslash_is_escaped_many_times_treats_as_unsescaped_1() {
        let string = r#"text \n xxx yyyy \\\\\\\\\\\\n was unescaped"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'n', b'}', bytes);

        assert_eq!(Some(29), result);
    }

    #[test]
    fn find_unescaped_byte2_when_the_backslash_is_escaped_many_times_treats_as_unsescaped_2() {
        let string = r#"text \n xxx yyyy \\\\\\\\\\\\n was unescaped"#;
        let bytes = string.as_bytes();

        let result = find_unescaped_byte2(b'}', b'n', bytes);

        assert_eq!(Some(29), result);
    }

    #[test]
    fn find_non_whitespace_byte_when_there_are_no_bytes_returns_none() {
        let string = "";
        let bytes = string.as_bytes();

        let result = find_non_whitespace(bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_non_whitespace_byte_when_there_is_only_one_non_whitespace_byte_returns_0() {
        let string = "x";
        let bytes = string.as_bytes();

        let result = find_non_whitespace(bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_when_there_is_leading_whitespace_returns_first_non_whitespace() {
        let string = " \t\n\r  \t  \n\t  \r \n\r  x";
        let bytes = string.as_bytes();

        let result = find_non_whitespace(bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_vertical_tab_as_whitespace() {
        let bytes = [11]; // U+000B - VERTICAL TAB

        let result = find_non_whitespace(&bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_form_feed_as_whitespace() {
        let bytes = [12]; // U+000C - FORM FEED

        let result = find_non_whitespace(&bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_next_line_as_whitespace() {
        let bytes = [133]; // U+0085 - NEXT LINE

        let result = find_non_whitespace(&bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_no_break_space_as_whitespace() {
        let bytes = [160]; // U+00A0 - NO-BREAK SPACE

        let result = find_non_whitespace(&bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte_sequence2_when_slice_is_empty_returns_none() {
        let bytes = [];

        let result = find_byte_sequence2(b'x', b'y', &bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte_sequence2_when_slice_is_only_the_sequence_returns_0() {
        let string = "xy";
        let bytes = string.as_bytes();

        let result = find_byte_sequence2(b'x', b'y', bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte_sequence2_when_the_sequence_is_reversed_returns_none() {
        let string = "xy";
        let bytes = string.as_bytes();

        let result = find_byte_sequence2(b'y', b'x', bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_only_first_byte_returns_none() {
        let bytes: Vec<_> = iter::repeat(b'x').take(256).collect();

        let result = find_byte_sequence2(b'x', b'y', &bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_only_second_byte_returns_none() {
        let bytes: Vec<_> = iter::repeat(b'y').take(256).collect();

        let result = find_byte_sequence2(b'x', b'y', &bytes);

        assert_eq!(None, result);
    }

    fn find_byte_sequence2_when_sequence_is_on_n_byte_boundary_returns_n_minus_1(n: usize) {
        let mut bytes: Vec<_> = iter::repeat(b'z').take(n * 2).collect();

        bytes[n - 1] = b'a';
        bytes[n] = b'b';

        let result = find_byte_sequence2(b'a', b'b', &bytes);

        assert_eq!(Some(n - 1), result);
    }

    fn find_byte_sequence2_when_sequence_is_before_n_byte_boundary_returns_n_minus_2(n: usize) {
        let mut bytes: Vec<_> = iter::repeat(b'z').take(n * 2).collect();

        bytes[n - 2] = b'a';
        bytes[n - 1] = b'b';

        let result = find_byte_sequence2(b'a', b'b', &bytes);

        assert_eq!(Some(n - 2), result);
    }

    fn find_byte_sequence2_when_sequence_is_after_n_byte_boundary_returns_n(n: usize) {
        let mut bytes: Vec<_> = iter::repeat(b'z').take(n * 2).collect();

        bytes[n] = b'a';
        bytes[n + 1] = b'b';

        let result = find_byte_sequence2(b'a', b'b', &bytes);

        assert_eq!(Some(n), result);
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_on_8_byte_boundary_returns_7() {
        find_byte_sequence2_when_sequence_is_on_n_byte_boundary_returns_n_minus_1(8)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_on_16_byte_boundary_returns_15() {
        find_byte_sequence2_when_sequence_is_on_n_byte_boundary_returns_n_minus_1(16)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_on_32_byte_boundary_returns_31() {
        find_byte_sequence2_when_sequence_is_on_n_byte_boundary_returns_n_minus_1(32)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_on_64_byte_boundary_returns_63() {
        find_byte_sequence2_when_sequence_is_on_n_byte_boundary_returns_n_minus_1(64)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_before_8_byte_boundary_returns_7() {
        find_byte_sequence2_when_sequence_is_before_n_byte_boundary_returns_n_minus_2(8)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_before_16_byte_boundary_returns_15() {
        find_byte_sequence2_when_sequence_is_before_n_byte_boundary_returns_n_minus_2(16)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_before_32_byte_boundary_returns_31() {
        find_byte_sequence2_when_sequence_is_before_n_byte_boundary_returns_n_minus_2(32)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_before_64_byte_boundary_returns_63() {
        find_byte_sequence2_when_sequence_is_before_n_byte_boundary_returns_n_minus_2(64)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_after_8_byte_boundary_returns_7() {
        find_byte_sequence2_when_sequence_is_after_n_byte_boundary_returns_n(8)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_after_16_byte_boundary_returns_15() {
        find_byte_sequence2_when_sequence_is_after_n_byte_boundary_returns_n(16)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_after_32_byte_boundary_returns_31() {
        find_byte_sequence2_when_sequence_is_after_n_byte_boundary_returns_n(32)
    }

    #[test]
    fn find_byte_sequence2_when_sequence_is_after_64_byte_boundary_returns_63() {
        find_byte_sequence2_when_sequence_is_after_n_byte_boundary_returns_n(64)
    }

    #[test]
    // This is the same data as used in the find_byte_sequence_benches bench,
    // however there is no clean way of sharing that code between tests and benches,
    // so it's duplicated here as a test.
    fn find_byte_sequence2_in_long_string() {
        const BYTE1: u8 = b'y';
        const BYTE2: u8 = b'x';
        const LENGTH: usize = 32 * 1024 * 1024;
        const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
        let mut contents = String::new();

        while contents.len() < LENGTH {
            contents += LETTERS;
        }

        contents += "y";
        contents += "x";
        contents += LETTERS;

        while contents.len() % 32 != 0 {
            contents += "x";
        }

        let bytes = contents.as_bytes();
        let result = find_byte_sequence2(BYTE1, BYTE2, bytes);

        assert_eq!(Some(33554456), result)
    }
}
