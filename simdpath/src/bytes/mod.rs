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

pub mod align;
mod classify;
pub(crate) mod debug;
mod depth;
mod sequences;

#[doc(inline)]
pub use classify::Structural;
#[doc(inline)]
pub use depth::DepthBlock;
#[cfg(feature = "nosimd")]
pub use nosimd::*;
#[cfg(not(feature = "nosimd"))]
#[doc(inline)]
pub use simd::*;

/// Find the first occurrence of a byte in the slice that is not escaped, if it exists.
///
/// An escaped byte is one which is immediately preceded by an unescaped backslash
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

/// Find the first occurrence of either of two bites in the slice that is not escaped, if it exists.
///
/// An escaped byte is one which is immediately preceded by an unescaped backslash
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
    if idx == 0 {
        return false;
    }
    slice[..idx]
        .iter()
        .rev()
        .take_while(|&&x| x == b'\\')
        .count()
        % 2
        != 0
}

/// Find the first occurrence of a non-whitespace byte in the slice, if it exists.
///
/// This function is a stub. Currently we assume there is no whitespace between structural
/// characters, so the next non-whitespace byte is simply the next byte.
#[inline(always)]
pub fn find_non_whitespace(slice: &[u8]) -> Option<usize> {
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

/// Sequential byte utilities _not_ utilizing SIMD.
///
/// These are the default operations used when the `nosimd` feature is enabled,
/// or AVX2 is not supported on the target CPU.
pub mod nosimd {
    #[doc(inline)]
    pub use super::classify::*;
    #[doc(inline)]
    pub use super::depth::nosimd as depth;
    #[doc(inline)]
    pub use super::sequences::nosimd::*;

    /// Find the first occurrence of a byte in the slice, if it exists.
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

    /// Find the first occurrence of either of two bytes in the slice, if it exists.
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
    pub use super::classify::*;
    #[doc(inline)]
    pub use super::depth::simd as depth;
    #[doc(inline)]
    pub use super::sequences::simd::*;
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
}

//cspell:disable - a lot of French words incoming.

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("", b'{' => None; "when there are no bytes")]
    #[test_case("{", b'{' => Some(0); "when there is only that byte")]
    #[test_case(r#"administrateur de \"La Libre Parole\", maire de Garches"#, b'"' => Some(19); "when byte exists")]
    #[test_case(r#"administrateur de \"La Libre Parole\", maire de Garches"#, b'{' => None; "when byte does not exists")]
    fn test_find_byte(string: &str, byte: u8) -> Option<usize> {
        find_byte(byte, string.as_bytes())
    }

    #[test_case("", b'{', b'}' => None; "when there are no bytes")]
    #[test_case("{", b'{', b'}' => Some(0); "when there is only the first byte")]
    #[test_case("}", b'{', b'}' => Some(0); "when there is only the second byte")]
    #[test_case(r#"administrateur de \"La Libre Parole\", maire de Garches"#, b'"', b'L' => Some(19); "when the first byte occurs first")]
    #[test_case(r#"administrateur de \"La Libre Parole\", maire de Garches"#, b'P', b'G' => Some(29); "when the second byte occurs first")]
    #[test_case(r#"administrateur de \"La Libre Parole\", maire de Garches"#, b'M', b'X' => None; "when none of the bytes occur")]
    fn test_find_byte2(string: &str, byte1: u8, byte2: u8) -> Option<usize> {
        find_byte2(byte1, byte2, string.as_bytes())
    }

    #[test_case("", b'{' => None; "when there are no bytes")]
    #[test_case("{", b'{' => Some(0); "when there is only that byte")]
    #[test_case(r#"administrateur de \"La Libre Parole\", maire de Garches"#, b'"' => None; "when all matching bytes are escaped 1")]
    #[test_case(r#"personne qui \"fait la plonge\" dans la restauration"#, b'"' => None; "when all matching bytes are escaped 2")]
    #[test_case(r#"administrateur de \"La Libre Parole\", maire de Garches"#, b'{' => None; "when byte does not exists")]
    #[test_case(
        r#"administrateur de \"La Libre Parole\", maire de Garches",
        {
            "label": 123
        }"#, 
        b'"' => Some(55); "when some matching bytes are escaped 1")]
    #[test_case(
        r#"personne qui \"fait la plonge\" dans la restauration
        },
        "en""#, 
        b'"' => Some(72); "when some matching bytes are escaped 2")]
    #[test_case(r#"text \n xxx yyyy \\n was unescaped"#, b'n' => Some(19); "when backslash is escaped")]
    #[test_case(r#"text \n xxx yyyy \\\\\\\\\\\\n was unescaped"#, b'n' => Some(29); "when backslash is escaped many times")]
    fn test_find_unescaped_byte(string: &str, byte: u8) -> Option<usize> {
        find_unescaped_byte(byte, string.as_bytes())
    }
    #[test_case("", b'{', b'}' => None; "when there are no bytes")]
    #[test_case("{", b'{', b'}' => Some(0); "when there is only the first byte")]
    #[test_case("}", b'{', b'}' => Some(0); "when there is only the second byte")]
    #[test_case(
        r#"administrateur de \"La Libre Parole\", maire de Garches",
        {
            "label": 123
        }"#, b'}', b'{' => Some(66); "when some matching bytes are escaped 1")]
    #[test_case(
        r#"personne qui \"fait la plonge\" dans la restauration
        },
        "en""#, b'"', b'}' => Some(61); "when some matching bytes are escaped 2")]
    #[test_case(r#"text \n xxx yyyy \\n was unescaped"#, b'n', b'}' => Some(19); "when the backslash is escaped 1")]
    #[test_case(r#"text \n xxx yyyy \\n was unescaped"#, b'}', b'n' => Some(19); "when the backslash is escaped 2")]
    #[test_case(r#"text \n xxx yyyy \\\\\\\\\\\\n was unescaped"#, b'}', b'n' => Some(29); "when the backslash is escaped many times 1")]
    #[test_case(r#"text \n xxx yyyy \\\\\\\\\\\\n was unescaped"#, b'}', b'n' => Some(29); "when the backslash is escaped many times 2")]
    fn test_find_unescaped_byte2(string: &str, byte1: u8, byte2: u8) -> Option<usize> {
        find_unescaped_byte2(byte1, byte2, string.as_bytes())
    }

    #[test_case("" => None; "when there are no bytes")]
    #[test_case("x" => Some(0); "when there is only one non whitespace byte")]
    #[test_case(" \t\n\r  \t  \n\t  \r \n\r  x" => Some(19); "when there is leading whitespace")]
    #[test_case("\u{000b}" => Some(0); "does not treat U+000B VERTICAL TAB as whitespace")]
    #[test_case("\u{000c}" => Some(0); "does not treat U+000C FORM FEED as whitespace")]
    #[test_case("\u{0085}" => Some(0); "does not treat U+0085 NEXT LINE as whitespace")]
    #[test_case("\u{00A0}" => Some(0); "does not treat U+00A0 NO-BREAK SPACE as whitespace")]
    fn test_find_non_whitespace(string: &str) -> Option<usize> {
        find_non_whitespace(string.as_bytes())
    }
}
