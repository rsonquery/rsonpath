//! Defines JSONPath query structure and parsing logic.
//!
//! # Examples
//! To create a query from a query string:
//! ```
//! # use rsonpath_lib::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let query_string = "$..phoneNumbers[*].number";
//! let query = JsonPathQuery::parse(query_string)?;
//!
//! // Query structure is a linear sequence of nodes:
//! // Root '$', descendant '..phoneNumbers', child wildcard, child 'number'.
//! let root_node = query.root();
//! let descendant_node = root_node.child().unwrap();
//! let child_wildcard_node = descendant_node.child().unwrap();
//! let child_node = child_wildcard_node.child().unwrap();
//!
//! assert!(root_node.is_root());
//! assert!(descendant_node.is_descendant());
//! assert!(child_wildcard_node.is_any_child());
//! assert!(child_node.is_child());
//! // Final node will have a None child.
//! assert!(child_node.child().is_none());
//!
//! assert_eq!(descendant_node.label().unwrap(), "phoneNumbers".as_bytes());
//! assert_eq!(child_wildcard_node.label(), None);
//! assert_eq!(child_node.label().unwrap(), "number".as_bytes());
//! # Ok(())
//! # }
//! ```
pub mod automaton;
pub mod builder;
pub mod error;
mod label;
mod parser;
pub use label::Label;

use log::*;
use std::fmt::{self, Display};

cfg_if! {
    if #[cfg(feature = "simd")] {
        /// Label byte alignment for SIMD.
        pub type LabelAlignment = alignment::SimdBlock;
    }
    else {
        /// Label byte alignment for `simd` feature disabled.
        pub type LabelAlignment = alignment::One;
    }
}

/// Provides the [IETF-conforming index value](https://www.rfc-editor.org/rfc/rfc7493.html#section-2).  Values are \[0, (2^53)-1].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NonNegativeArrayIndex(u64);

/// The upper inclusive bound on index values.
pub const ARRAY_INDEX_ULIMIT: u64 = (1 << 53) - 1;
impl TryFrom<u64> for NonNegativeArrayIndex {
    type Error = ArrayIndexError;

    #[inline]
    fn try_from(value: u64) -> Result<Self, ArrayIndexError> {
        if value > ARRAY_INDEX_ULIMIT {
            Err(ArrayIndexError::ExceedsUpperLimitError(value.to_string()))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<NonNegativeArrayIndex> for u64 {
    #[inline(always)]
    fn from(val: NonNegativeArrayIndex) -> Self {
        val.0
    }
}

impl Display for NonNegativeArrayIndex {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{index}", index = self.0)
    }
}

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
pub struct Label {
    label: AlignedBytes<LabelAlignment>,
    label_with_quotes: AlignedBytes<LabelAlignment>,
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

impl Clone for Label {
    #[inline]
    fn clone(&self) -> Self {
        let label_clone = AlignedBytes::from(self.label.as_ref());
        let quoted_clone = AlignedBytes::from(self.label_with_quotes.as_ref());
        Self {
            label: label_clone,
            label_with_quotes: quoted_clone,
        }
    }
}

impl Label {
    /// Create a new label from UTF8 input.
    #[must_use]
    #[inline]
    pub fn new(label: &str) -> Self {
        let bytes = label.as_bytes();
        let without_quotes = AlignedBytes::<LabelAlignment>::from(bytes);

        let mut with_quotes = AlignedBytes::<LabelAlignment>::new_zeroed(bytes.len() + 2);
        with_quotes[0] = b'"';
        with_quotes[1..bytes.len() + 1].copy_from_slice(bytes);
        with_quotes[bytes.len() + 1] = b'"';

        Self {
            label: without_quotes,
            label_with_quotes: with_quotes,
        }
    }

    /// Return the raw bytes of the label, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes(&self) -> &AlignedSlice<LabelAlignment> {
        &self.label
    }

    /// Return the bytes representing the label with a leading and trailing
    /// double quote symbol `"`, guaranteed to be block-aligned.
    #[must_use]
    #[inline(always)]
    pub fn bytes_with_quotes(&self) -> &AlignedSlice<LabelAlignment> {
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

impl std::ops::Deref for Label {
    type Target = AlignedSlice<LabelAlignment>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.bytes()
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
        self == &other.label
    }
}

impl PartialEq<Label> for &[u8] {
    #[inline(always)]
    fn eq(&self, other: &Label) -> bool {
        *self == &other.label
    }
}

impl PartialEq<[u8]> for Label {
    #[inline(always)]
    fn eq(&self, other: &[u8]) -> bool {
        &self.label == other
    }
}

impl PartialEq<&[u8]> for Label {
    #[inline(always)]
    fn eq(&self, other: &&[u8]) -> bool {
        &self.label == *other
    }
}

impl std::hash::Hash for Label {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let slice: &[u8] = &self.label;
        slice.hash(state);
    }
}

/// Linked list structure of a JSONPath query.
#[derive(Debug, PartialEq, Eq)]
pub enum JsonPathQueryNode {
    /// The first link in the list representing the root '`$`' character.
    Root(Option<Box<JsonPathQueryNode>>),
    /// Represents direct descendant with a label ('`.`' token).
    Child(Label, Option<Box<JsonPathQueryNode>>),
    /// Represents direct descendant with a wildcard ('`.*`' tokens).
    AnyChild(Option<Box<JsonPathQueryNode>>),
    /// Represents recursive descent ('`..`' token).
    Descendant(Label, Option<Box<JsonPathQueryNode>>),
    /// Represents recursive descendant with a wildcard ('`..*`' tokens).
    AnyDescendant(Option<Box<JsonPathQueryNode>>),
    /// Represents direct descendant list item with a positive index (numbers).
    ArrayIndex(NonNegativeArrayIndex, Option<Box<JsonPathQueryNode>>),
}

use JsonPathQueryNode::*;

use self::error::{ArrayIndexError, ParserError};

impl JsonPathQueryNode {
    /// Retrieve the child of the node or `None` if it is the last one
    /// on the list.
    #[must_use]
    #[inline(always)]
    pub fn child(&self) -> Option<&Self> {
        match self {
            Root(node)
            | Child(_, node)
            | AnyChild(node)
            | Descendant(_, node)
            | AnyDescendant(node)
            | ArrayIndex(_, node) => node.as_deref(),
        }
    }

    /// Create an iterator over nodes of the query in sequence,
    /// starting from the root.
    #[must_use]
    #[inline(always)]
    pub fn iter(&self) -> JsonPathQueryIterator {
        JsonPathQueryIterator { node: Some(self) }
    }
}

/// JSONPath query structure represented by the root link of the
/// [`JsonPathQueryNode`] list.
#[derive(Debug, PartialEq, Eq)]
pub struct JsonPathQuery {
    root: Box<JsonPathQueryNode>,
}

/// Iterator over query nodes traversing the parent-child relation.
pub struct JsonPathQueryIterator<'a> {
    node: Option<&'a JsonPathQueryNode>,
}

impl<'a> Iterator for JsonPathQueryIterator<'a> {
    type Item = &'a JsonPathQueryNode;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.node;

        if let Some(node) = result {
            self.node = node.child()
        }

        result
    }
}

impl JsonPathQuery {
    /// Retrieve reference to the root node.
    ///
    /// It is guaranteed that the root is the [`JsonPathQueryNode::Root`]
    /// variant and always exists.
    #[must_use]
    #[inline(always)]
    pub fn root(&self) -> &JsonPathQueryNode {
        self.root.as_ref()
    }

    /// Parse a query string into a [`JsonPathQuery`].
    ///
    /// # Errors
    ///
    /// Will return a [`ParserError`] if the `query_string` does
    /// not conform to the JSONPath grammar. See its documentation
    /// for details.
    #[inline(always)]
    pub fn parse(query_string: &str) -> Result<Self, ParserError> {
        self::parser::parse_json_path_query(query_string)
    }

    /// Create a query from a root node.
    ///
    /// If node is not the [`JsonPathQueryNode::Root`] variant it will be
    /// automatically wrapped into a [`JsonPathQueryNode::Root`] node.
    #[inline]
    #[must_use]
    pub fn new(node: Box<JsonPathQueryNode>) -> Self {
        let root = if node.is_root() {
            node
        } else {
            info!("Implicitly using the Root expression (`$`) at the start of the query.");
            Box::new(Root(Some(node)))
        };

        Self { root }
    }
}

impl Display for JsonPathQuery {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root.as_ref())
    }
}

impl Display for JsonPathQueryNode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Root(_) => write!(f, "$"),
            Child(label, _) => write!(f, "['{}']", label.display()),
            AnyChild(_) => write!(f, "[*]"),
            Descendant(label, _) => write!(f, "..['{}']", label.display()),
            AnyDescendant(_) => write!(f, "..[*]"),
            ArrayIndex(i, _) => write!(f, "[{i}]"),
        }?;

        if let Some(child) = self.child() {
            write!(f, "{child}")
        } else {
            Ok(())
        }
    }
}

/// Equips a struct with information on the type of [`JsonPathQueryNode`] it represents
/// and methods to extract query elements from it.
pub trait JsonPathQueryNodeType {
    /// Returns `true` iff the type is [`JsonPathQueryNode::Root`].
    fn is_root(&self) -> bool;

    /// Returns `true` iff the type is [`JsonPathQueryNode::Descendant`].
    fn is_descendant(&self) -> bool;

    /// Returns `true` iff the type is [`JsonPathQueryNode::AnyDescendant`].
    fn is_any_descendant(&self) -> bool;

    /// Returns `true` iff the type is [`JsonPathQueryNode::Child`].
    fn is_child(&self) -> bool;

    /// Returns `true` iff the type is [`JsonPathQueryNode::AnyChild`].
    fn is_any_child(&self) -> bool;

    /// If the type is [`JsonPathQueryNode::Descendant`] or [`JsonPathQueryNode::Child`]
    /// returns the label it represents; otherwise, `None`.
    fn label(&self) -> Option<&Label>;

    /// If the type is [`JsonPathQueryNode::ArrayIndex`]
    /// returns the index it represents; otherwise, `None`.
    fn array_index(&self) -> Option<&NonNegativeArrayIndex>;
}

impl JsonPathQueryNodeType for JsonPathQueryNode {
    #[inline(always)]
    fn is_root(&self) -> bool {
        matches!(self, Root(_))
    }

    #[inline(always)]
    fn is_descendant(&self) -> bool {
        matches!(self, Descendant(_, _))
    }

    #[inline(always)]
    fn is_any_descendant(&self) -> bool {
        matches!(self, AnyDescendant(_))
    }

    #[inline(always)]
    fn is_child(&self) -> bool {
        matches!(self, Child(_, _))
    }

    #[inline(always)]
    fn is_any_child(&self) -> bool {
        matches!(self, AnyChild(_))
    }

    #[inline(always)]
    fn label(&self) -> Option<&Label> {
        match self {
            Child(label, _) | Descendant(label, _) => Some(label),
            Root(_) | AnyChild(_) | AnyDescendant(_) | ArrayIndex(_, _) => None,
        }
    }

    #[inline(always)]
    fn array_index(&self) -> Option<&NonNegativeArrayIndex> {
        match self {
            ArrayIndex(i, _) => Some(i),
            Child(_, _) | Descendant(_, _) | Root(_) | AnyChild(_) | AnyDescendant(_) => None,
        }
    }
}

/// Utility blanket implementation for a [`JsonPathQueryNode`] wrapped in an [`Option`].
///
/// If the value is `None` automatically returns `false` or `None` on all calls in
/// the natural manner.
impl<T: std::ops::Deref<Target = JsonPathQueryNode>> JsonPathQueryNodeType for Option<T> {
    #[inline(always)]
    fn is_root(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_root())
    }

    #[inline(always)]
    fn is_descendant(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_descendant())
    }

    #[inline(always)]
    fn is_any_descendant(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_any_descendant())
    }

    #[inline(always)]
    fn is_child(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_child())
    }

    #[inline(always)]
    fn is_any_child(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_any_child())
    }

    #[inline(always)]
    fn label(&self) -> Option<&Label> {
        self.as_ref().and_then(|x| x.label())
    }

    #[inline(always)]
    fn array_index(&self) -> Option<&NonNegativeArrayIndex> {
        self.as_ref().and_then(|x| x.array_index())
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
    fn index_ulimit_sanity_check() {
        assert_eq!(9007199254740991, ARRAY_INDEX_ULIMIT);
    }

    #[test]
    fn index_ulimit_parse_check() {
        NonNegativeArrayIndex::try_from(ARRAY_INDEX_ULIMIT)
            .expect("Array index ulimit should be convertible.");

        NonNegativeArrayIndex::try_from(ARRAY_INDEX_ULIMIT + 1)
            .expect_err("Values in excess of array index ulimit should not be convertible.");
    }
}
