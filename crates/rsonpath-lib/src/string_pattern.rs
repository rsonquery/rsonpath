//! JSONString unicode-aware pattern matching.
//!
//! A [`JsonString`] can be turned into a [`StringPattern`] that contains all data necessary
//! to match a candidate string against it in a unicode-aware manner. This is more involved than
//! just bytewise equality. For example, a logical string "ab" can be represented in four unique
//! but equivalent ways in a JSON:
//!   - `"ab"`
//!   - `"\u0097b"`
//!   - `"a\u0098"`
//!   - `"\u0097\u0098"`
//!
//! The [`StringPattern`] itself contains no matching logic. The functions [`cmpeq_forward`] and
//! [`cmpeq_backward`] allow matching a pattern against an input.
//!
pub(crate) mod matcher;
use crate::{BLOCK_SIZE, JSON_SPACE_BYTE};
use cfg_if::cfg_if;
use rsonpath_syntax::str::JsonString;
use std::fmt::Debug;

/// Compiled JSONString representation allowing pattern-matching JSON strings.
///
/// Any non-empty JSON string has multiple textual representations. For example,
/// `"a"` can also be written as `"\u0097"`. This structure precomputes the alternative
/// representations and allows efficient pattern-matching against JSON bytes.
///
/// A compiled pattern takes more space than a raw [`JsonString`], but is efficient to match.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct StringPattern {
    bytes: Vec<u8>,
    alternatives: Vec<AlternativeRepresentation>,
    len: usize,
    len_limit: usize,
    #[cfg(feature = "regex-matcher")]
    #[serde(with = "serde_regex")]
    regex_forward: regex::bytes::Regex,
    #[cfg(feature = "regex-matcher")]
    #[serde(with = "serde_regex")]
    regex_backward: regex::bytes::Regex,
}

impl std::hash::Hash for StringPattern {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl PartialOrd for StringPattern {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StringPattern {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl PartialEq for StringPattern {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for StringPattern {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq)]
enum AlternativeRepresentation {
    SlashUSingle(u32, u8),
    SlashUPair(u32, u32, u8),
    USingle(u32),
    SlashByteOrUSingle(u8, u32),
    None,
}

struct StringPatternBuilder {
    bytes: Vec<u8>,
    alternatives: Vec<AlternativeRepresentation>,
    len_limit: usize,
    #[cfg(feature = "regex-matcher")]
    regex_pattern: String,
}

impl StringPattern {
    /// Returns bytes of the canonical representation of the string pattern without the delimiting
    /// quotes.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath::StringPattern;
    /// # use rsonpath_syntax::str::JsonString;
    /// let simple_pattern = StringPattern::new(&JsonString::new("ab"));
    /// let complex_pattern = StringPattern::new(&JsonString::new("\n"));
    ///
    /// assert_eq!(simple_pattern.unquoted(), "ab".as_bytes());
    /// assert_eq!(complex_pattern.unquoted(), r"\n".as_bytes());
    /// ```
    #[inline]
    #[must_use]
    pub fn unquoted(&self) -> &[u8] {
        &self.quoted()[1..self.len - 1]
    }

    /// Returns bytes of the canonical representation of the string pattern including the delimiting
    /// quotes.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath::StringPattern;
    /// # use rsonpath_syntax::str::JsonString;
    /// let simple_pattern = StringPattern::new(&JsonString::new("ab"));
    /// let complex_pattern = StringPattern::new(&JsonString::new("\n"));
    ///
    /// assert_eq!(simple_pattern.quoted(), r#""ab""#.as_bytes());
    /// assert_eq!(complex_pattern.quoted(), r#""\n""#.as_bytes());
    /// ```
    #[inline]
    #[must_use]
    pub fn quoted(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    /// Returns the maximum length of JSON text (in bytes) that can possibly match this pattern.
    /// The length DOES include the delimiting quotes.
    ///
    /// In other words: if a JSON string contains more bytes than this, it definitely does not
    /// match this pattern.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath::StringPattern;
    /// # use rsonpath_syntax::str::JsonString;
    /// let pattern = StringPattern::new(&JsonString::new("ab"));
    /// // The pattern can be represented as: "\u0097\u0098", which is 14 bytes.
    /// assert_eq!(pattern.len_limit(), 14);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn len_limit(&self) -> usize {
        self.len_limit
    }

    /// Build a [`StringPattern`] for a given [`JsonString`].
    #[inline]
    #[must_use]
    pub fn new(string: &JsonString) -> Self {
        // A pattern to be matched consists of the bytes that should be matched in the "canonical"
        // representation of the string (the shortest possible valid representation), and possible
        // alternative escapes that should be considered if a mismatch occurs
        // at a given position relative to the canonical bytes.
        // We have the following cases:
        //   - The character is a control character or a special symbol that is canonically represented
        //     as backslash-itself. If it is mismatched at the backslash, there is no match alternative
        //     representation; on the second byte it can be replaced with uXXXX.
        //   - The character is a control character that can only be represented as a unicode escape;
        //     it has no alternative encodings.
        //   - The character is one of the two awfully designed JSON special cases:
        //     forward slash (/) or single quote ('). The canonical form of them is themselves, but they
        //     can also be present escaped (\/ or \'), or as a unicode escape.
        //   - The character is a "regular" character; it has only one alternative encoding - unicode
        //     escape, which is either a single sequence \uXXXX or a pair \uXXXX\uXXXX.
        let byte_length = string.quoted().len();
        let mut builder = StringPatternBuilder::new(byte_length);

        for char in string.unquoted().chars() {
            match char {
                '\u{0008}' => builder.short_escape(b'b', char),
                '\u{000C}' => builder.short_escape(b'f', char),
                '\n' => builder.short_escape(b'n', char),
                '\r' => builder.short_escape(b'r', char),
                '\t' => builder.short_escape(b't', char),
                '"' => builder.short_escape(b'"', char),
                '\\' => builder.short_escape(b'\\', char),
                '\u{0000}'..='\u{001F}' => builder.long_escape(char),
                '/' | '\'' => builder.special_escape(char),
                _ => builder.regular_escape(char),
            };
        }

        builder.into_pattern()
    }
}

impl StringPatternBuilder {
    fn new(byte_len: usize) -> Self {
        let mut this;

        cfg_if! {
            if #[cfg(feature = "regex-matcher")] {
                this = Self {
                    bytes: Vec::with_capacity(byte_len),
                    alternatives: Vec::with_capacity(byte_len),
                    len_limit: 0,
                    regex_pattern: String::new(),
                };
            }
            else {
                this = Self {
                    bytes: Vec::with_capacity(byte_len),
                    alternatives: Vec::with_capacity(byte_len),
                    len_limit: 0,
                };
            }
        }

        this.bytes.push(b'"');
        this.alternatives.push(AlternativeRepresentation::None);
        this.len_limit += 1;
        #[cfg(feature = "regex-matcher")]
        {
            this.regex_pattern.push('^');
            this.regex_pattern.push('"');
        }

        this
    }

    fn into_pattern(mut self) -> StringPattern {
        self.bytes.push(b'"');
        self.alternatives.push(AlternativeRepresentation::None);
        self.len_limit += 1;
        let len = self.bytes.len();
        for _ in 0..BLOCK_SIZE {
            self.bytes.push(JSON_SPACE_BYTE);
        }
        #[cfg(feature = "regex-matcher")]
        self.regex_pattern.push('"');

        cfg_if! {
            if #[cfg(feature = "regex-matcher")] {
                let regex_forward = regex::bytes::RegexBuilder::new(&self.regex_pattern).size_limit(16 * 1024 * 1024).build().expect("regex pattern must be constructed correctly");
                self.regex_pattern.push('$');
                let regex_backward = regex::bytes::RegexBuilder::new(&self.regex_pattern[1..]).size_limit(16 * 1024 * 1024).build().expect("regex pattern must be constructed correctly");
                StringPattern {
                    bytes: self.bytes,
                    alternatives: self.alternatives,
                    len_limit: self.len_limit,
                    len,
                    regex_forward,
                    regex_backward,
                }
            }
            else {
                StringPattern {
                    bytes: self.bytes,
                    alternatives: self.alternatives,
                    len_limit: self.len_limit,
                    len,
                }
            }
        }
    }

    fn short_escape(&mut self, code_letter: u8, c: char) {
        self.bytes.push(b'\\');
        self.bytes.push(code_letter);

        let mut utf16_buf = [0; 1];
        let utf16 = c.encode_utf16(&mut utf16_buf);
        let code = Self::encode(utf16[0]);

        self.alternatives.push(AlternativeRepresentation::None);
        self.alternatives.push(AlternativeRepresentation::USingle(code));

        self.len_limit += 6;

        #[cfg(feature = "regex-matcher")]
        {
            let bs = code.to_ne_bytes();
            self.regex_pattern.push('(');
            self.regex_pattern.push('\\');
            self.regex_pattern.push('\\');
            self.regex_pattern.push('u');
            self.add_regex_case_insensitive_hex(bs[0]);
            self.add_regex_case_insensitive_hex(bs[1]);
            self.add_regex_case_insensitive_hex(bs[2]);
            self.add_regex_case_insensitive_hex(bs[3]);
            self.regex_pattern.push('|');
            self.regex_pattern.push('\\');
            self.regex_pattern.push('\\');
            if regex_syntax::is_meta_character(code_letter as char) {
                self.regex_pattern.push('\\');
            }
            self.regex_pattern.push(code_letter as char);
            self.regex_pattern.push(')');
        }
    }

    fn long_escape(&mut self, c: char) {
        let b3 = Self::encode_nibble((c as u8 & 0xF0) >> 4);
        let b4 = Self::encode_nibble((c as u8 & 0xF0) >> 4);
        self.bytes.push(b'\\');
        self.bytes.push(b'u');
        self.bytes.push(b'0');
        self.bytes.push(b'0');
        self.bytes.push(b3);
        self.bytes.push(b4);

        for _ in 0..6 {
            self.alternatives.push(AlternativeRepresentation::None);
        }

        self.len_limit += 6;

        #[cfg(feature = "regex-matcher")]
        {
            self.regex_pattern.push('\\');
            self.regex_pattern.push('\\');
            self.regex_pattern.push('u');
            self.regex_pattern.push('0');
            self.regex_pattern.push('0');
            self.add_regex_case_insensitive_hex(b3);
            self.add_regex_case_insensitive_hex(b4);
        }
    }

    fn special_escape(&mut self, c: char) {
        self.bytes.push(c as u8);

        let mut utf16_buf = [0; 1];
        let utf16 = c.encode_utf16(&mut utf16_buf);
        assert_eq!(utf16.len(), 1);
        let code = Self::encode(utf16[0]);

        self.alternatives
            .push(AlternativeRepresentation::SlashByteOrUSingle(c as u8, code));

        self.len_limit += 6;

        #[cfg(feature = "regex-matcher")]
        {
            let bs = code.to_ne_bytes();
            self.regex_pattern.push('(');
            self.regex_pattern.push('\\');
            self.regex_pattern.push('\\');
            self.regex_pattern.push('u');
            self.add_regex_case_insensitive_hex(bs[0]);
            self.add_regex_case_insensitive_hex(bs[1]);
            self.add_regex_case_insensitive_hex(bs[2]);
            self.add_regex_case_insensitive_hex(bs[3]);
            self.regex_pattern.push('|');
            if regex_syntax::is_meta_character(c) {
                self.regex_pattern.push('\\');
            }
            self.regex_pattern.push(c);
            self.regex_pattern.push('|');
            self.regex_pattern.push('\\');
            self.regex_pattern.push('\\');
            if regex_syntax::is_meta_character(c) {
                self.regex_pattern.push('\\');
            }
            self.regex_pattern.push(c);
            self.regex_pattern.push(')');
        }
    }

    fn regular_escape(&mut self, c: char) {
        let mut utf8_buf = [0; 4];
        let mut utf16_buf = [0; 2];
        let utf8 = c.encode_utf8(&mut utf8_buf);
        let utf16 = c.encode_utf16(&mut utf16_buf);

        self.bytes.extend_from_slice(utf8.as_bytes());
        let len = utf8.len();
        let repr;

        if utf16.len() == 1 {
            let code = Self::encode(utf16[0]);
            repr = AlternativeRepresentation::SlashUSingle(code, len as u8);
            self.alternatives.push(repr);
            self.len_limit += 6;
        } else {
            let code1 = Self::encode(utf16[0]);
            let code2 = Self::encode(utf16[1]);
            repr = AlternativeRepresentation::SlashUPair(code1, code2, len as u8);
            self.alternatives.push(repr);
            self.len_limit += 12;
        }

        for _ in 1..utf8.len() {
            self.alternatives.push(AlternativeRepresentation::None);
        }
        let last_idx = self.alternatives.len() - 1;
        self.alternatives[last_idx] = repr;

        #[cfg(feature = "regex-matcher")]
        {
            self.regex_pattern.push('(');
            if utf16.len() == 1 {
                let bs = Self::encode(utf16[0]).to_ne_bytes();
                self.regex_pattern.push('\\');
                self.regex_pattern.push('\\');
                self.regex_pattern.push('u');
                self.add_regex_case_insensitive_hex(bs[0]);
                self.add_regex_case_insensitive_hex(bs[1]);
                self.add_regex_case_insensitive_hex(bs[2]);
                self.add_regex_case_insensitive_hex(bs[3]);
            } else {
                let bs1 = Self::encode(utf16[0]).to_ne_bytes();
                let bs2 = Self::encode(utf16[1]).to_ne_bytes();
                self.regex_pattern.push('\\');
                self.regex_pattern.push('\\');
                self.regex_pattern.push('u');
                self.add_regex_case_insensitive_hex(bs1[0]);
                self.add_regex_case_insensitive_hex(bs1[1]);
                self.add_regex_case_insensitive_hex(bs1[2]);
                self.add_regex_case_insensitive_hex(bs1[3]);
                self.regex_pattern.push('\\');
                self.regex_pattern.push('\\');
                self.regex_pattern.push('u');
                self.add_regex_case_insensitive_hex(bs2[0]);
                self.add_regex_case_insensitive_hex(bs2[1]);
                self.add_regex_case_insensitive_hex(bs2[2]);
                self.add_regex_case_insensitive_hex(bs2[3]);
            }
            self.regex_pattern.push('|');
            if regex_syntax::is_meta_character(c) {
                self.regex_pattern.push('\\');
            }
            self.regex_pattern.push(c);
            self.regex_pattern.push(')');
        }
    }

    fn encode(utf16: u16) -> u32 {
        let bytes = utf16.to_be_bytes();
        let mut result = [0; 4];
        result[0] = Self::encode_nibble((bytes[0] & 0xF0) >> 4);
        result[1] = Self::encode_nibble(bytes[0] & 0x0F);
        result[2] = Self::encode_nibble((bytes[1] & 0xF0) >> 4);
        result[3] = Self::encode_nibble(bytes[1] & 0x0F);

        u32::from_ne_bytes(result)
    }

    fn encode_nibble(nibble: u8) -> u8 {
        match nibble {
            0x00..=0x09 => b'0' + nibble,
            0x0A..=0x0F => b'a' + nibble - 0x0A,
            _ => unreachable!(),
        }
    }

    fn add_regex_case_insensitive_hex(&mut self, b: u8) {
        if b'a' <= b && b <= b'f' {
            self.regex_pattern.push('[');
            self.regex_pattern.push(b as char);
            self.regex_pattern.push((b as char).to_ascii_uppercase());
            self.regex_pattern.push(']');
        } else {
            self.regex_pattern.push(b as char);
        }
    }
}

impl From<&JsonString> for StringPattern {
    #[inline(always)]
    fn from(value: &JsonString) -> Self {
        Self::new(value)
    }
}

impl Debug for StringPattern {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StringPattern")
            .field(
                "bytes",
                &self.quoted().iter().copied().map(DebugByte).collect::<Vec<_>>(),
            )
            .field("as_string", &String::from_utf8_lossy(self.quoted()))
            .field("alternatives", &self.alternatives)
            .finish()
    }
}

impl Debug for AlternativeRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SlashUSingle(arg0, arg1) => f
                .debug_tuple("SlashUSingle")
                .field(&DebugCode(*arg0))
                .field(arg1)
                .finish(),
            Self::SlashUPair(arg0, arg1, arg2) => f
                .debug_tuple("SlashUPair")
                .field(&DebugCode(*arg0))
                .field(&DebugCode(*arg1))
                .field(arg2)
                .finish(),
            Self::USingle(arg0) => f.debug_tuple("USingle").field(&DebugCode(*arg0)).finish(),
            Self::SlashByteOrUSingle(arg0, arg1) => f
                .debug_tuple("SlashByteOrUSingle")
                .field(&DebugByte(*arg0))
                .field(&DebugCode(*arg1))
                .finish(),
            Self::None => write!(f, "None"),
        }
    }
}

struct DebugByte(u8);
struct DebugCode(u32);

impl Debug for DebugByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0x20..=0x7F => write!(f, "b'{}'", self.0 as char),
            _ => write!(f, "0x{:0>2x}", self.0),
        }
    }
}

impl Debug for DebugCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:0>8x}", self.0)
    }
}
