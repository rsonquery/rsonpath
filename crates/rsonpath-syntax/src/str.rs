//! JSON string types expressible in a JSONPath query.
pub mod error;

use self::error::JsonStringParseError;

/// String to search for in a JSON document, conforming to the
/// [RFC7159, section 7](https://www.rfc-editor.org/rfc/rfc7159#section-7)
///
/// Represents the bytes defining a string key or value in a JSON object
/// that can be matched against when executing a query.
///
/// # Examples
///
/// ```rust
/// # use rsonpath_syntax::str::JsonString;
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

impl JsonString {
    /// Create a new JSON string from UTF8 input.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath_syntax::str::JsonString;
    ///
    /// let needle = JsonString::new(r#"Stri\ng With \u00c9scapes \\n"#);
    ///
    /// assert_eq!(needle.unquoted(), r#"Stri\ng With \u00c9scapes \\n"#);
    /// ```
    #[inline]
    #[must_use]
    pub fn new(string: &str) -> Self {
        let mut quoted = String::with_capacity(string.len() + 2);
        quoted.push('"');
        quoted += string;
        quoted.push('"');
        Self { quoted }
    }

    /// Parse a JSON-encoded string.
    ///  
    /// The string is parsed and unescaped according to the JSONPath specification.
    /// Escape sequences are converted by replacing them with the equivalent Unicode character
    /// (see [2.3.1.2. of the spec](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html#name-semantics-3)).
    /// # Examples
    /// ```rust
    /// # use rsonpath_syntax::str::JsonString;
    ///
    /// let needle = JsonString::parse(r#"Stri\ng With \u00c9scapes \\n"#).unwrap();
    ///
    /// assert_eq!(needle.unquoted(), "Stri\ng With Éscapes \\n");
    /// ```
    #[inline]
    pub fn parse(string: &str) -> Result<Self, JsonStringParseError> {
        use nom::Finish;
        Self::parse_impl(string, StringParseMode::Unquoted, false)
            .finish()
            .map(|x| x.1)
    }

    #[inline]
    pub(crate) fn parse_single_quoted(string: &str) -> nom::IResult<&str, Self, JsonStringParseError> {
        Self::parse_impl(string, StringParseMode::SingleQuoted, true)
    }

    #[inline]
    pub(crate) fn parse_double_quoted(string: &str) -> nom::IResult<&str, Self, JsonStringParseError> {
        Self::parse_impl(string, StringParseMode::DoubleQuoted, true)
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

    fn parse_impl(
        string: &str,
        mode: StringParseMode,
        stop_at_quote: bool,
    ) -> nom::IResult<&str, Self, JsonStringParseError> {
        let mut quoted = String::with_capacity(string.len() + 2);
        quoted.push('"');
        let (rest, _) = unescape_into(string, &mut quoted, mode, stop_at_quote)?;
        quoted.push('"');
        Ok((rest, Self { quoted }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringParseMode {
    DoubleQuoted,
    SingleQuoted,
    Unquoted,
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

        Ok(Self { quoted: string })
    }
}

fn unescape_into<'a>(
    src: &'a str,
    dest: &mut String,
    mode: StringParseMode,
    stop_at_quote: bool,
) -> nom::IResult<&'a str, (), JsonStringParseError<'a>> {
    use error::JsonStringParseErrorKind as ErrorKind;
    let mut stream = src.char_indices().enumerate();

    while let Some((i, (c_idx, c))) = stream.next() {
        match (c, mode) {
            ('\\', _) => {
                let r = read_escape_sequence(src, i, &mut stream, mode).map_err(nom::Err::Error)?;
                dest.push(r);
            }
            ('"', StringParseMode::DoubleQuoted) | ('\'', StringParseMode::SingleQuoted) if stop_at_quote => {
                return Ok((&src[c_idx..], ()))
            }
            ('"', StringParseMode::DoubleQuoted | StringParseMode::Unquoted)
            | ('\'', StringParseMode::SingleQuoted | StringParseMode::Unquoted)
            | (..='\u{0019}', _) => {
                return Err(nom::Err::Error(JsonStringParseError::err_at(
                    ErrorKind::InvalidUnescapedCharacter,
                    src,
                    i,
                )))
            }
            _ => {
                dest.push(c);
            }
        }
    }

    return Ok(("", ()));

    fn read_escape_sequence<'a, I>(
        src: &'a str,
        idx: usize,
        chars: &mut I,
        mode: StringParseMode,
    ) -> Result<char, JsonStringParseError<'a>>
    where
        I: Iterator<Item = (usize, (usize, char))>,
    {
        let (i, (_, ctrl)) = chars.next().ok_or(JsonStringParseError::err_at(
            ErrorKind::InvalidUnescapedCharacter,
            src,
            idx,
        ))?;
        match ctrl {
            'u' => {
                let raw_c = read_hexadecimal_escape(src, i, chars)?;
                match raw_c {
                    // High surrogate, start of a UTF-16 pair.
                    0xD800..=0xDBFF => {
                        let (_, (_, next)) = chars.next().ok_or(JsonStringParseError::err_at(
                            ErrorKind::UnpairedHighSurrogate,
                            src,
                            idx,
                        ))?;
                        if next != '\\' {
                            return Err(JsonStringParseError::err_at(ErrorKind::UnpairedHighSurrogate, src, idx));
                        }
                        let (i, (_, next)) = chars.next().ok_or(JsonStringParseError::err_at(
                            ErrorKind::UnpairedHighSurrogate,
                            src,
                            idx,
                        ))?;
                        if next != 'u' {
                            return Err(JsonStringParseError::err_at(ErrorKind::UnpairedHighSurrogate, src, idx));
                        }
                        let low = read_hexadecimal_escape(src, i, chars)?;
                        match low {
                            0xDC00..=0xDFFF => {
                                let n = ((raw_c - 0xD800) << 10 | (low - 0xDC00)) + 0x10000;
                                Ok(char::from_u32(n).expect("high and low surrogate pair is always a valid char"))
                            }
                            _ => Err(JsonStringParseError::err_at(ErrorKind::UnpairedHighSurrogate, src, idx)),
                        }
                    }
                    // Low surrogate, invalid escape sequence.
                    0xDC00..=0xDFFF => Err(JsonStringParseError::err_at(ErrorKind::UnpairedLowSurrogate, src, idx)),
                    _ => Ok(char::from_u32(raw_c).expect("invalid values are handled above")),
                }
            }
            'b' => Ok('\u{0008}'), // U+0008 BS backspace
            't' => Ok('\t'),       // U+0009 HT horizontal tab
            'n' => Ok('\n'),       // U+000A LF line feed
            'f' => Ok('\u{000C}'), // U+000C FF form feed
            'r' => Ok('\r'),       // U+000D CR carriage return
            '"' if mode == StringParseMode::DoubleQuoted || mode == StringParseMode::Unquoted => Ok(ctrl),
            '\'' if mode == StringParseMode::SingleQuoted || mode == StringParseMode::Unquoted => Ok(ctrl),
            '/' | '\\' => Ok(ctrl), // " ' / \ are passed as is
            _ => Err(JsonStringParseError::err_at(ErrorKind::InvalidEscapeSequence, src, idx)), // no other escape sequences are allowed
        }
    }

    fn read_hexadecimal_escape<'a, I>(src: &'a str, idx: usize, chars: &mut I) -> Result<u32, JsonStringParseError<'a>>
    where
        I: Iterator<Item = (usize, (usize, char))>,
    {
        let mut x = 0;
        for i in 0..4 {
            let (_, (_, c)) = chars.next().ok_or(JsonStringParseError::err_at_with_len(
                ErrorKind::InvalidEscapeSequence,
                src,
                idx - 1,
                2 + i,
            ))?;
            let v = match c {
                '0'..='9' => c as u32 - '0' as u32,
                // RFC8259.7-2 The hexadecimal letters A through F can be uppercase or lowercase.
                'a'..='f' => c as u32 - 'a' as u32 + 10,
                'A'..='F' => c as u32 - 'A' as u32 + 10,
                _ => {
                    return Err(JsonStringParseError::err_at(
                        ErrorKind::InvalidHexDigitInUnicodeEscape,
                        src,
                        idx + i + 1,
                    ))
                }
            };
            x <<= 4;
            x += v;
        }
        Ok(x)
    }
}

#[cfg(test)]
mod tests {
    use super::{error::JsonStringParseError, *};
    use pretty_assertions::{assert_eq, assert_ne};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    use test_case::test_case;

    fn unescape(str: &str) -> Result<String, JsonStringParseError> {
        JsonString::parse(str).map(|x| x.unquoted().to_string())
    }

    #[test_case("", ""; "empty")]
    #[test_case("dog", "dog"; "ascii")]
    #[test_case(r"\\", r"\"; "backslash")]
    #[test_case("unescaped 🔥 fire emoji", "unescaped 🔥 fire emoji"; "unescaped emoji")]
    #[test_case(r"escape \b backspace", "escape \u{0008} backspace"; "BS escape")]
    #[test_case(r"escape \t tab", "escape \t tab"; "HT escape")]
    #[test_case(r"escape \n endln", "escape \n endln"; "LF escape")]
    #[test_case(r"escape \f formfeed", "escape \u{000C} formfeed"; "FF escape")]
    #[test_case(r"escape \r carriage", "escape \r carriage"; "CR escape")]
    #[test_case(r#"escape \" quote"#, r#"escape " quote"#; "quote escape")]
    #[test_case(r#"escape \' apost"#, r"escape ' apost"; "apostrophe escape")]
    #[test_case(r"escape \/ apost", r"escape / apost"; "slash escape")]
    #[test_case(r"escape \\ apost", r"escape \ apost"; "backslash escape")]
    #[test_case(r"escape \u2112 script L", "escape ℒ script L"; "U+2112 Script Capital L escape")]
    #[test_case(r"escape \u211269 script L", "escape ℒ69 script L"; "U+2112 Script Capital L escape followed by digits")]
    #[test_case(r"escape \u21a7 bar down arrow", "escape ↧ bar down arrow"; "U+21a7 Downwards Arrow From Bar (lowercase hex)")]
    #[test_case(r"escape \u21A7 bar down arrow", "escape ↧ bar down arrow"; "U+21A7 Downwards Arrow From Bar (uppercase hex)")]
    #[test_case(r"escape \ud83d\udd25 fire emoji", "escape 🔥 fire emoji"; "U+1F525 fire emoji escape (lowercase hex)")]
    #[test_case(r"escape \uD83D\uDD25 fire emoji", "escape 🔥 fire emoji"; "U+1F525 fire emoji escape (uppercase hex)")]
    fn unescape_on_correct_input(src: &str, expected: &str) {
        let res = unescape(src).expect("should successfully parse");
        assert_eq!(res, expected);
    }

    #[test_case("\u{0000}",
"this character must be escaped

  \u{0000}
  

at position 0"; "null byte")]
    #[test_case("\u{0019}",
"this character must be escaped

  \u{0019}
  

at position 0"; "U+0019 ctrl")]
    #[test_case(r#"unescaped " quote"#,
r#"this character must be escaped

  unescaped " quote
            ^

at position 10"#; "U+0020 quote")]
    fn unescape_on_input_with_chars_that_have_to_be_escaped(src: &str, err_msg: &str) {
        let err = unescape(src).expect_err("should fail to parse");
        assert_eq!(err.to_string(), err_msg);
    }

    #[test_case(r"escape \ a space",
r"not a valid escape sequence

  escape \ a space
         ^^

at positions 7-8"; "escaped whitespace")]
    #[test_case(r"\",
r"this character must be escaped

  \
  ^

at position 0";"just a backslash")]
    #[test_case(r"\U0012",
r"not a valid escape sequence

  \U0012
  ^^

at positions 0-1"; "uppercase U unicode escape")]
    fn unescape_on_input_with_invalid_escape_char(src: &str, err_msg: &str) {
        let err = unescape(src).expect_err("should fail to parse");
        assert_eq!(err.to_string(), err_msg);
    }

    #[test_case(r"escape \uD800 and that is it",
r"this high surrogate is unpaired

  escape \uD800 and that is it
         ^^^^^^

at positions 7-12"; "lone high surrogate")]
    #[test_case(r"escape \uD800\uD801 please",
r"this high surrogate is unpaired

  escape \uD800\uD801 please
         ^^^^^^

at positions 7-12"; "high surrogate twice")]
    #[test_case(r"escape \uD800\n please",
r"this high surrogate is unpaired

  escape \uD800\n please
         ^^^^^^

at positions 7-12"; "high surrogate followed by newline escape")]
    #[test_case(r"escape \uD800\uCC01 please",
r"this high surrogate is unpaired

  escape \uD800\uCC01 please
         ^^^^^^

at positions 7-12"; "high surrogate followed by non-surrogate")]
    #[test_case(r"escape \uDC01 please",
r"this low surrogate is unpaired

  escape \uDC01 please
         ^^^^^^

at positions 7-12"; "lone low surrogate")]
    fn unescape_on_input_with_surrogate_error(src: &str, err_msg: &str) {
        let err = unescape(src).expect_err("should fail to parse");
        assert_eq!(err.to_string(), err_msg);
    }
    #[test_case(r"\u",
r"not a valid escape sequence

  \u
  ^^

at positions 0-1"; "alone in the string with no digits")]
    #[test_case(r"escape \u and that is it",
r"not a hex digit

  escape \u and that is it
           ^

at position 9"; "with no digits")]
    #[test_case(r"escape \u1 and that is it",
r"not a hex digit

  escape \u1 and that is it
            ^

at position 10"; "with one digit")]
    #[test_case(r"escape \u12 and that is it",
r"not a hex digit

  escape \u12 and that is it
             ^

at position 11"; "with two digits")]
    #[test_case(r"escape \u123 and that is it",
r"not a hex digit

  escape \u123 and that is it
              ^

at position 12"; "with three digits")]
    #[test_case(r"escape \uGFFF please",
r"not a hex digit

  escape \uGFFF please
           ^

at position 9"; "with invalid hex digit G at first position")]
    #[test_case(r"escape \uFGFF please",
r"not a hex digit

  escape \uFGFF please
            ^

at position 10"; "with invalid hex digit G at second position")]
    #[test_case(r"escape \uFFGF please",
r"not a hex digit

  escape \uFFGF please
             ^

at position 11"; "with invalid hex digit G at third position")]
    #[test_case(r"escape \uFFFG please",
r"not a hex digit

  escape \uFFFG please
              ^

at position 12"; "with invalid hex digit G at fourth position")]
    #[test_case(r"escape \uD800\u please",
r"not a hex digit

  escape \uD800\u please
                 ^

at position 15"; "high surrogate followed by unicode escape with no digits")]
    #[test_case(r"escape \uD800\uD please",
r"not a hex digit

  escape \uD800\uD please
                  ^

at position 16"; "high surrogate followed by unicode escape with one digit")]
    #[test_case(r"escape \uD800\uDC please",
r"not a hex digit

  escape \uD800\uDC please
                   ^

at position 17"; "high surrogate followed by unicode escape with two digits")]
    #[test_case(r"escape \uD800\uDC0 please",
r"not a hex digit

  escape \uD800\uDC0 please
                    ^

at position 18"; "high surrogate followed by unicode escape with three digits")]
    #[test_case(r"escape \uD800\uDC0X please",
r"not a hex digit

  escape \uD800\uDC0X please
                    ^

at position 18"; "high surrogate followed by invalid hex escape")]
    fn unescape_on_input_with_malformed_unicode_escape(src: &str, err_msg: &str) {
        let err = unescape(src).expect_err("should fail to parse");
        assert_eq!(err.to_string(), err_msg);
    }

    #[test_case(r"Ｈｅｌｌｏ, ｗｏｒｌｄ!\u222X",
r"not a hex digit

  Ｈｅｌｌｏ, ｗｏｒｌｄ!\u222X
                              ^

at position 18"; "wide letters")] // This may render incorrectly in the editor, but is actually aligned in a terminal.
    #[test_case(r"क्\u12G4",
r"not a hex digit

  क्\u12G4
       ^

at position 6"; "grapheme cluster")]
    #[test_case(r"👩‍🔬\u222X",
r"not a hex digit

  👩‍🔬\u222X
           ^

at position 8"; "ligature emoji")] // This may render incorrectly in the editor, but is actually aligned in a terminal.
    fn unescape_error_on_input_with_varying_length_unicode_characters(src: &str, err_msg: &str) {
        let err = unescape(src).expect_err("should fail to parse");
        assert_eq!(err.to_string(), err_msg);
    }

    #[test_case("dog", "dog"; "dog")]
    #[test_case("", ""; "empty")]
    fn equal_json_strings_are_equal(s1: &str, s2: &str) {
        let string1 = JsonString::new(s1);
        let string2 = JsonString::new(s2);

        assert_eq!(string1, string2);
    }

    #[test]
    fn different_json_strings_are_not_equal() {
        let string1 = JsonString::new("dog");
        let string2 = JsonString::new("doc");

        assert_ne!(string1, string2);
    }

    #[test_case("dog", "dog"; "dog")]
    #[test_case("", ""; "empty")]
    fn equal_json_strings_have_equal_hashes(s1: &str, s2: &str) {
        let string1 = JsonString::new(s1);
        let string2 = JsonString::new(s2);

        let mut hasher1 = DefaultHasher::new();
        string1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        string2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}
