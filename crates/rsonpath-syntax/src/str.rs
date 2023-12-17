//! JSON string types expressible in a JSONPath query.

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

#[derive(Debug)]
pub(crate) struct JsonStringBuilder {
    quoted: String,
}

impl std::fmt::Debug for JsonString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, r#"JsonString({})"#, self.quoted)
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EscapeMode {
    SingleQuoted,
    DoubleQuoted,
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

    #[inline]
    #[must_use]
    pub fn escape(str: &str, mode: EscapeMode) -> String {
        use std::fmt::Write;
        let mut result = String::new();
        for c in str.chars() {
            match c {
                '\'' if mode == EscapeMode::SingleQuoted => result.push_str(r"\'"),
                '\'' if mode == EscapeMode::DoubleQuoted => result.push('\''),
                '"' if mode == EscapeMode::SingleQuoted => result.push('"'),
                '"' if mode == EscapeMode::DoubleQuoted => result.push_str(r#"\""#),
                '\u{0008}' => result.push_str(r"\b"),
                '\u{000C}' => result.push_str(r"\f"),
                '\n' => result.push_str(r"\n"),
                '\r' => result.push_str(r"\r"),
                '\t' => result.push_str(r"\t"),
                '\\' => result.push_str(r"\\"),
                '\u{0000}'..='\u{001F}' => write!(result, "\\u{:0>4x}", c as u8).unwrap(),
                _ => result.push(c),
            }
        }

        result
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

        Ok(Self { quoted: string })
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
