use rsonpath_syntax::str::JsonString;

/// String pattern coming from a JSONPath query that can be matched against strings in a JSON.
///
/// Right now the only pattern is matching against a given [`JsonString`].
#[derive(Debug, Clone)]
pub struct StringPattern(JsonString);

impl std::hash::Hash for StringPattern {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialOrd for StringPattern {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.0.unquoted().cmp(other.0.unquoted()))
    }
}

impl Ord for StringPattern {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.unquoted().cmp(other.0.unquoted())
    }
}

impl PartialEq for StringPattern {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for StringPattern {}

impl StringPattern {
    /// Get the underlying [`JsonString`] as bytes, including the delimiting double quote symbols.
    #[inline]
    #[must_use]
    pub fn quoted(&self) -> &[u8] {
        self.0.quoted().as_bytes()
    }

    /// Get the underlying [`JsonString`] as bytes, without the delimiting quotes.
    #[inline]
    #[must_use]
    pub fn unquoted(&self) -> &[u8] {
        self.0.unquoted().as_bytes()
    }

    /// Create a new pattern from a given [`JsonString`].
    #[inline]
    #[must_use]
    pub fn new(string: &JsonString) -> Self {
        Self(string.clone())
    }
}

impl From<JsonString> for StringPattern {
    #[inline(always)]
    fn from(value: JsonString) -> Self {
        Self::new(&value)
    }
}

impl From<&JsonString> for StringPattern {
    #[inline(always)]
    fn from(value: &JsonString) -> Self {
        Self::new(value)
    }
}
