//! JSON string types expressible in a JSONPath query.
pub mod error;

use self::error::JsonStringParseError;

/// String to search for in a JSON document, conforming to the
/// [RFC7159, section 7](https://www.rfc-editor.org/rfc/rfc7159#section-7)
///
/// Represents the bytes defining a label/key in a JSON object
/// that can be matched against when executing a query.
///
/// # Examples
///
/// ```rust
/// # use rsonpath_syntax::string::JsonString;
/// let needle = JsonString::new("needle");
///
/// assert_eq!(needle.unquoted(), "needle");
/// assert_eq!(needle.quoted(), "\"needle\"");
/// ```
#[derive(Clone)]
pub struct JsonString {
    quoted: String,
}

impl std::fmt::Debug for JsonString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, r#"JsonString({})"#, self.quoted)
    }
}

fn unescape_into(src: &str, dest: &mut String) -> Result<(), JsonStringParseError> {
    use std::str::Chars;
    let mut stream = src.chars();

    while let Some(c) = stream.next() {
        if c == '\\' {
            let r = read_escape_sequence(&mut stream)?;
            dest.push(r);
        } else {
            dest.push(c);
        }
    }

    return Ok(());

    fn read_escape_sequence(chars: &mut Chars) -> Result<char, JsonStringParseError> {
        let ctrl = chars.next().ok_or(JsonStringParseError {})?;
        match ctrl {
            'u' => {
                let raw_c = read_hexadecimal_escape(chars)?;
                match raw_c {
                    // High surrogate, start of a UTF-16 pair.
                    0xD800..=0xDBFF => {
                        let next = chars.next().ok_or(JsonStringParseError {})?;
                        if next != '\\' {
                            return Err(JsonStringParseError {});
                        }
                        let next = chars.next().ok_or(JsonStringParseError {})?;
                        if next != 'u' {
                            return Err(JsonStringParseError {});
                        }
                        let low = read_hexadecimal_escape(chars)?;
                        match low {
                            0xDC00..=0xDFFF => {
                                let n = ((raw_c - 0xD800) << 10 | (low - 0xDC00)) + 0x10000;
                                char::from_u32(n).ok_or(JsonStringParseError {})
                            }
                            _ => Err(JsonStringParseError {}),
                        }
                    }
                    // Low surrogate, invalid escape sequence.
                    0xDC00..=0xDFFF => Err(JsonStringParseError {}),
                    _ => Ok(char::from_u32(raw_c).expect("invalid values are handled above")),
                }
            }
            'b' => Ok('\u{0008}'), // U+0008 BS backspace
            't' => Ok('\t'),       // U+0009 HT horizontal tab
            'n' => Ok('\n'),       // U+000A LF line feed
            'f' => Ok('\u{000C}'), // U+000C FF form feed
            'r' => Ok('\r'),       // U+000D CR carriage return
            x => Ok(x),            // remaining " ' / \ are passed as is
        }
    }

    fn read_hexadecimal_escape(chars: &mut Chars) -> Result<u32, JsonStringParseError> {
        let mut x = 0;
        for _ in 0..4 {
            let c = chars.next().ok_or(JsonStringParseError {})?;
            let v = match c {
                '0'..='9' => c as u32 - '0' as u32,
                // RFC8259.7-2 The hexadecimal letters A through F can be uppercase or lowercase.
                'a'..='f' => c as u32 - 'a' as u32 + 10,
                'A'..='F' => c as u32 - 'F' as u32 + 10,
                _ => return Err(JsonStringParseError {}),
            };
            x <<= 4;
            x += v;
        }
        Ok(x)
    }
}

impl JsonString {
    /// Create a new string from UTF8 input without quotes.
    ///
    /// The string is parsed and unescaped according to the JSONPath specification.
    /// Escape sequences are converted by replacing them with the equivalent Unicode character
    /// (see [2.3.1.2. of the spec](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#name-semantics-3)).
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath_syntax::string::JsonString;
    ///
    /// let needle = JsonString::new(r#"Stri\ng With\u00c9scapes \\n"#);
    ///
    /// assert_eq!(needle.unquoted(), "Stri\ng With Escapes \\n");
    /// ```
    #[inline]
    pub fn new(string: &str) -> Result<Self, JsonStringParseError> {
        let mut quoted = String::with_capacity(string.len() + 2);
        quoted.push('"');
        unescape_into(string, &mut quoted)?;
        quoted.push('"');
        Ok(Self { quoted })
    }

    pub(crate) fn new_unchecked(string: &str) -> Self {
        Self {
            quoted: format!(r#""{string}""#),
        }
    }

    /// Return the contents of the string.
    #[must_use]
    #[inline(always)]
    pub fn unquoted(&self) -> &str {
        let len = self.quoted.len();
        debug_assert!(len >= 2);
        &self.quoted[1..len - 1]
    }

    /// Return the bytes representing the string with the leading and trailing
    /// double quote symbol `"`.
    #[must_use]
    #[inline(always)]
    pub fn quoted(&self) -> &str {
        &self.quoted
    }
}

impl PartialEq<Self> for JsonString {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.unquoted() == other.unquoted()
    }
}

impl Eq for JsonString {}

impl std::hash::Hash for JsonString {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.unquoted().hash(state);
    }
}

#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for JsonString {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let chars = u.arbitrary_iter()?;
        let mut string = String::new();

        // RFC 7159: All Unicode characters may be placed [in the string],
        // except for characters that must be escaped: quotation mark,
        // reverse solidus, and the control characters (U+0000 through U+001F).
        for c in chars {
            let c = c?;
            match c {
                '\u{0000}'..='\u{001F}' | '\"' | '\\' => {
                    string.push('\\');
                    string.push(c);
                }
                _ => string.push(c),
            }
        }

        Ok(Self::new_unchecked(&string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    use test_case::test_case;

    #[test_case("dog", "dog"; "dog")]
    #[test_case("", ""; "empty")]
    fn equal_json_strings_are_equal(s1: &str, s2: &str) {
        let string1 = JsonString::new(s1).unwrap();
        let string2 = JsonString::new(s2).unwrap();

        assert_eq!(string1, string2);
    }

    #[test]
    fn different_json_strings_are_not_equal() {
        let string1 = JsonString::new("dog").unwrap();
        let string2 = JsonString::new("doc").unwrap();

        assert_ne!(string1, string2);
    }

    #[test_case("dog", "dog"; "dog")]
    #[test_case("", ""; "empty")]
    fn equal_json_strings_have_equal_hashes(s1: &str, s2: &str) {
        let string1 = JsonString::new(s1).unwrap();
        let string2 = JsonString::new(s2).unwrap();

        let mut hasher1 = DefaultHasher::new();
        string1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        string2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}
