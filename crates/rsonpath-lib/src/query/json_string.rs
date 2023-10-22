/// String to search for in a JSON document, conforming to the
/// [RFC7159, section 7](https://www.rfc-editor.org/rfc/rfc7159#section-7)
///
/// Represents the bytes defining a label/key in a JSON object
/// that can be matched against when executing a query.
///
/// # Examples
///
/// ```rust
/// # use rsonpath::query::JsonString;
/// let needle = JsonString::new("needle");
///
/// assert_eq!(needle.bytes(), "needle".as_bytes());
/// assert_eq!(needle.bytes_with_quotes(), "\"needle\"".as_bytes());
/// ```
#[derive(Debug, Clone)]
pub struct JsonString {
    string: String,
    string_with_quotes: String,
}

impl std::fmt::Display for JsonString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.string)
    }
}

impl JsonString {
    /// Create a new label from UTF8 input.
    #[must_use]
    #[inline]
    pub fn new(string: &str) -> Self {
        let without_quotes = string.to_owned();

        let mut with_quotes = String::with_capacity(string.len() + 2);
        with_quotes.push('"');
        with_quotes += string;
        with_quotes.push('"');

        Self {
            string: without_quotes,
            string_with_quotes: with_quotes,
        }
    }

    /// Return the raw bytes of the string, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    /// Return the bytes representing the string with a leading and trailing
    /// double quote symbol `"`, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes_with_quotes(&self) -> &[u8] {
        self.string_with_quotes.as_bytes()
    }
}

impl<S: AsRef<str>> From<S> for JsonString {
    #[inline(always)]
    fn from(value: S) -> Self {
        Self::new(value.as_ref())
    }
}

impl From<JsonString> for String {
    fn from(value: JsonString) -> Self {
        value.to_string()
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
        self == other.bytes()
    }
}

impl PartialEq<JsonString> for &[u8] {
    #[inline(always)]
    fn eq(&self, other: &JsonString) -> bool {
        *self == other.bytes()
    }
}

impl PartialEq<[u8]> for JsonString {
    #[inline(always)]
    fn eq(&self, other: &[u8]) -> bool {
        self.bytes() == other
    }
}

impl PartialEq<&[u8]> for JsonString {
    #[inline(always)]
    fn eq(&self, other: &&[u8]) -> bool {
        self.bytes() == *other
    }
}

impl std::hash::Hash for JsonString {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes().hash(state);
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

        Ok(Self::new(&string))
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
