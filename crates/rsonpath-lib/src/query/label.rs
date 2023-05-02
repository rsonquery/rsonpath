use std::fmt::Display;

/// Label to search for in a JSON document.
///
/// Represents the bytes defining a label/key in a JSON object
/// that can be matched against when executing a query.
///
/// # Examples
///
/// ```
/// # use rsonpath_lib::query::Label;
///
/// let label = Label::new("needle");
///
/// assert_eq!(label.bytes(), "needle".as_bytes());
/// assert_eq!(label.bytes_with_quotes(), "\"needle\"".as_bytes());
/// ```
#[derive(Clone)]
pub struct Label {
    label: Vec<u8>,
    label_with_quotes: Vec<u8>,
}

impl std::fmt::Debug for Label {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"{}"#,
            std::str::from_utf8(&self.label_with_quotes).unwrap_or("[invalid utf8]")
        )
    }
}

impl Label {
    /// Create a new label from UTF8 input.
    #[must_use]
    #[inline]
    pub fn new(label: &str) -> Self {
        let bytes = label.as_bytes();
        let without_quotes = Vec::from(bytes);

        let mut with_quotes = Vec::with_capacity(bytes.len() + 2);
        with_quotes.push(b'"');
        with_quotes.extend(bytes);
        with_quotes.push(b'"');

        Self {
            label: without_quotes,
            label_with_quotes: with_quotes,
        }
    }

    /// Return the raw bytes of the label, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        &self.label
    }

    /// Return the bytes representing the label with a leading and trailing
    /// double quote symbol `"`, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes_with_quotes(&self) -> &[u8] {
        &self.label_with_quotes
    }

    /// Return a display object with a UTF8 representation of this label.
    ///
    /// If the label contains invalid UTF8, the value will always be `"[invalid utf8]"`.
    #[must_use]
    #[inline(always)]
    pub fn display(&self) -> impl Display + '_ {
        std::str::from_utf8(&self.label).unwrap_or("[invalid utf8]")
    }
}

impl PartialEq<Self> for Label {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Label {}

impl PartialEq<Label> for [u8] {
    #[inline(always)]
    fn eq(&self, other: &Label) -> bool {
        self == other.label
    }
}

impl PartialEq<Label> for &[u8] {
    #[inline(always)]
    fn eq(&self, other: &Label) -> bool {
        *self == other.label
    }
}

impl PartialEq<[u8]> for Label {
    #[inline(always)]
    fn eq(&self, other: &[u8]) -> bool {
        self.label == other
    }
}

impl PartialEq<&[u8]> for Label {
    #[inline(always)]
    fn eq(&self, other: &&[u8]) -> bool {
        self.label == *other
    }
}

impl std::hash::Hash for Label {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let slice: &[u8] = &self.label;
        slice.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use super::*;

    #[test]
    fn label_equality() {
        let label1 = Label::new("dog");
        let label2 = Label::new("dog");

        assert_eq!(label1, label2);
    }

    #[test]
    fn label_inequality() {
        let label1 = Label::new("dog");
        let label2 = Label::new("doc");

        assert_ne!(label1, label2);
    }

    #[test]
    fn label_hash() {
        let label1 = Label::new("dog");
        let label2 = Label::new("dog");

        let mut s1 = DefaultHasher::new();
        label1.hash(&mut s1);
        let h1 = s1.finish();

        let mut s2 = DefaultHasher::new();
        label2.hash(&mut s2);
        let h2 = s2.finish();

        assert_eq!(h1, h2);
    }
}
