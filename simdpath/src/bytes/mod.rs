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

mod depth;
mod sequences;

#[doc(inline)]
pub use depth::DepthBlock;
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
    #[doc(inline)]
    pub use super::depth::nosimd as depth;
    #[doc(inline)]
    pub use super::sequences::nosimd::*;

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
    #[doc(inline)]
    pub use super::depth::simd as depth;
    #[doc(inline)]
    pub use super::sequences::simd::*;
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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
