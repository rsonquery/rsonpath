//! JSON string types expressible in a JSONPath query.
//!
//! This may refer to a member name when used in a name selector,
//! or a raw string value used for comparison or matching in a filter expression.

/// String value or JSON member name, conforming to the
/// [RFC7159, section 7](https://www.rfc-editor.org/rfc/rfc7159#section-7)
///
/// Represents the UTF-8 bytes defining a string key or value in a JSON object
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct JsonString {
    quoted: String,
}

#[derive(Debug)]
pub(crate) struct JsonStringBuilder {
    quoted: String,
}

impl std::fmt::Debug for JsonString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "JsonString({})", self.quoted)
    }
}

impl JsonStringBuilder {
    pub(crate) fn new() -> Self {
        Self {
            quoted: String::from('"'),
        }
    }

    pub(crate) fn push(&mut self, char: char) -> &mut Self {
        self.quoted.push(char);
        self
    }

    pub(crate) fn finish(mut self) -> JsonString {
        self.quoted.push('"');
        JsonString { quoted: self.quoted }
    }
}

impl From<JsonStringBuilder> for JsonString {
    #[inline(always)]
    fn from(value: JsonStringBuilder) -> Self {
        value.finish()
    }
}

impl From<&str> for JsonString {
    #[inline(always)]
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl FromIterator<char> for JsonString {
    #[inline]
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut quoted = String::new();
        quoted.push('"');
        for c in iter {
            quoted.push(c);
        }
        quoted.push('"');
        Self { quoted }
    }
}

/// Escape mode for the [`escape`] function.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EscapeMode {
    /// Treat the string as within single quotes `'`.
    SingleQuoted,
    /// Treat the string as within double quotes `"`.
    DoubleQuoted,
}

/// Escape a string according to JSONPath rules in a given quotation context.
///
/// ## Quotes
///
/// Processing quotes, `'` and `"`, depends on the `mode`:
/// - in [`EscapeMode::SingleQuoted`], the string is escaped as if written in a single-quoted
///   name selector `['<str>']`; single quotes are escaped as `\'`, double-quotes are copied as-is.
/// - in [`EscapeMode::DoubleQuoted`], the string is escaped as if written in double-quotes,
///   which is the same as a member name in a JSON document or a double-quoted name selector `["<str>"]`;
///   double quotes are escaped as `\"`, single quotes are copied as-is.
///
/// ### Examples
///
/// ```rust
/// # use rsonpath_syntax::str::{self, EscapeMode};
/// let result_single = str::escape(r#"'rust' or "rust"\n"#, EscapeMode::SingleQuoted);
/// let result_double = str::escape(r#"'rust' or "rust"\n"#, EscapeMode::DoubleQuoted);
/// assert_eq!(result_single, r#"\'rust\' or "rust"\\n"#);
/// assert_eq!(result_double, r#"'rust' or \"rust\"\\n"#);
/// ```
///
/// ## Control characters
///
/// Control characters (U+0000 to U+001F) are escaped as special sequences
/// where possible, e.g. Form Feed U+000C is escaped as `\f`.
/// Other control sequences are escaped as a Unicode sequence, e.g.
/// a null byte is escaped as `\u0000`.
///
/// ### Examples
///
/// ```rust
/// # use rsonpath_syntax::str::{self, EscapeMode};
/// let result = str::escape("\u{08}\u{09}\u{0A}\u{0B}\u{0C}\u{0D}", EscapeMode::DoubleQuoted);
/// assert_eq!(result, r"\b\t\n\u000b\f\r");
/// ```
///
/// ## Other
///
/// Characters that don't have to be escaped are not.
///
/// ### Examples
///
/// ```rust
/// # use rsonpath_syntax::str::{self, EscapeMode};
/// let result = str::escape("🦀", EscapeMode::DoubleQuoted);
/// assert_eq!(result, "🦀");
/// ```
///
/// Among other things, this means Unicode escapes are only produced
/// for control characters.
#[inline]
#[must_use]
pub fn escape(str: &str, mode: EscapeMode) -> String {
    use std::fmt::Write as _;
    let mut result = String::new();
    for c in str.chars() {
        match c {
            // # Mode-dependent quote escapes.
            '\'' if mode == EscapeMode::SingleQuoted => result.push_str(r"\'"),
            '\'' if mode == EscapeMode::DoubleQuoted => result.push('\''),
            '"' if mode == EscapeMode::SingleQuoted => result.push('"'),
            '"' if mode == EscapeMode::DoubleQuoted => result.push_str(r#"\""#),
            // # Mode-independent escapes.
            '\\' => result.push_str(r"\\"),
            // ## Special control sequences.
            '\u{0008}' => result.push_str(r"\b"),
            '\u{000C}' => result.push_str(r"\f"),
            '\n' => result.push_str(r"\n"),
            '\r' => result.push_str(r"\r"),
            '\t' => result.push_str(r"\t"),
            // ## Other control sequences escaped as Unicode escapes.
            '\u{0000}'..='\u{001F}' => write!(result, "\\u{:0>4x}", c as u8).expect("writing to string never fails"),
            // # Non-escapable characters.
            _ => result.push(c),
        }
    }

    result
}

impl JsonString {
    /// Create a new JSON string from UTF8 input.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath_syntax::str::JsonString;
    /// let str = JsonString::new(r#"Stri\ng With \u00c9scapes \\n"#);
    /// assert_eq!(str.unquoted(), r#"Stri\ng With \u00c9scapes \\n"#);
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

    /// Return the contents of the string.
    /// # Examples
    /// ```rust
    /// # use rsonpath_syntax::str::JsonString;
    /// let needle = JsonString::new(r#"Stri\ng With \u00c9scapes \\n"#);
    /// assert_eq!(needle.unquoted(), r#"Stri\ng With \u00c9scapes \\n"#);
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn unquoted(&self) -> &str {
        let len = self.quoted.len();
        debug_assert!(len >= 2, "self.quoted must contain at least the two quote characters");
        &self.quoted[1..len - 1]
    }

    /// Return the contents of the string with the leading and trailing
    /// double quote symbol `"`.
    /// # Examples
    /// ```rust
    /// # use rsonpath_syntax::str::JsonString;
    /// let needle = JsonString::new(r#"Stri\ng With \u00c9scapes \\n"#);
    /// assert_eq!(needle.quoted(), r#""Stri\ng With \u00c9scapes \\n""#);
    /// ```
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
