use std::fmt::Display;

/// String to search for in a JSON document, conforming to the
/// [RFC7159, section 7](https://www.rfc-editor.org/rfc/rfc7159#section-7)
///
/// Represents the bytes defining a label/key in a JSON object
/// that can be matched against when executing a query.
///
/// # Examples
///
/// ```
/// # use rsonpath_lib::query::JsonString;
///
/// let needle = JsonString::new("needle");
///
/// assert_eq!(needle.bytes(), "needle".as_bytes());
/// assert_eq!(needle.bytes_with_quotes(), "\"needle\"".as_bytes());
/// ```
#[derive(Clone)]
pub struct JsonString {
    string: Vec<u8>,
    string_with_quotes: Vec<u8>,
}

impl std::fmt::Debug for JsonString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{}"#,
            std::str::from_utf8(&self.string_with_quotes).unwrap_or("[invalid utf8]")
        )
    }
}

impl JsonString {
    /// Create a new label from UTF8 input.
    #[must_use]
    #[inline]
    pub fn new(string: &str) -> Self {
        let bytes = string.as_bytes();
        let without_quotes = Vec::from(bytes);

        let mut with_quotes = Vec::with_capacity(bytes.len() + 2);
        with_quotes.push(b'"');
        with_quotes.extend(bytes);
        with_quotes.push(b'"');

        Self {
            string: without_quotes,
            string_with_quotes: with_quotes,
        }
    }

    /// Return the raw bytes of the string, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        &self.string
    }

    /// Return the bytes representing the string with a leading and trailing
    /// double quote symbol `"`, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes_with_quotes(&self) -> &[u8] {
        &self.string_with_quotes
    }

    /// Return a display object with a UTF8 representation of this string.
    ///
    /// If the string contains invalid UTF8, the value will always be `"[invalid utf8]"`.
    #[must_use]
    #[inline(always)]
    pub fn display(&self) -> impl Display + '_ {
        std::str::from_utf8(&self.string).unwrap_or("[invalid utf8]")
    }
}

impl PartialEq<Self> for JsonString {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Eq for JsonString {}

impl PartialEq<JsonString> for [u8] {
    #[inline(always)]
    fn eq(&self, other: &JsonString) -> bool {
        self == other.string
    }
}

impl PartialEq<JsonString> for &[u8] {
    #[inline(always)]
    fn eq(&self, other: &JsonString) -> bool {
        *self == other.string
    }
}

impl PartialEq<[u8]> for JsonString {
    #[inline(always)]
    fn eq(&self, other: &[u8]) -> bool {
        self.string == other
    }
}

impl PartialEq<&[u8]> for JsonString {
    #[inline(always)]
    fn eq(&self, other: &&[u8]) -> bool {
        self.string == *other
    }
}

impl std::hash::Hash for JsonString {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let slice: &[u8] = &self.string;
        slice.hash(state);
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
