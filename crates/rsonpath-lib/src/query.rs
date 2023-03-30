//! Defines JSONPath query structure and parsing logic.
//!
//! # Examples
//! To create a query from a query string:
//! ```
//! # use rsonpath_lib::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let query_string = "$..person..phoneNumber";
//! let query = JsonPathQuery::parse(query_string)?;
//!
//! // Query structure is a linear sequence of nodes:
//! // Root '$', descendant '..person', descendant '..phoneNumber'.
//! let root_node = query.root();
//! let descendant_node1 = root_node.child().unwrap();
//! let descendant_node2 = descendant_node1.child().unwrap();
//!
//! assert!(root_node.is_root());
//! assert!(descendant_node1.is_descendant());
//! assert!(descendant_node2.is_descendant());
//! // Final node will have a None child.
//! assert!(descendant_node2.child().is_none());
//!
//! assert_eq!(descendant_node1.label().unwrap(), "person".as_bytes());
//! assert_eq!(descendant_node2.label().unwrap(), "phoneNumber".as_bytes());
//! # Ok(())
//! # }
//! ```
pub mod automaton;
pub mod builder;
pub mod error;
mod parser;

use crate::lib::{self, Box, fmt::{self, Display}};
use aligners::{alignment, AlignedBytes, AlignedSlice};
use cfg_if::cfg_if;
use log::*;

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

impl fmt::Debug for Label {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            r#"{}"#,
            str::from_utf8(&self.label_with_quotes).unwrap_or("[invalid utf8]")
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
        str::from_utf8(&self.label).unwrap_or("[invalid utf8]")
    }
}

impl crate::lib::ops::Deref for Label {
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

#[cfg(feature = "std")]
impl lib::hash::Hash for Label {
    #[inline(always)]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
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
}

use JsonPathQueryNode::*;

use self::error::ParserError;

impl JsonPathQueryNode {
    /// Retrieve the child of the node or `None` if it is the last one
    /// on the list.
    #[must_use]
    #[inline(always)]
    pub fn child(&self) -> Option<&Self> {
        match self {
            Root(node) | Child(_, node) | AnyChild(node) | Descendant(_, node) => node.as_deref(),
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

    /// Returns `true` iff the type is [`JsonPathQueryNode::Child`].
    fn is_child(&self) -> bool;

    /// If the type is [`JsonPathQueryNode::Descendant`] or [`JsonPathQueryNode::Child`]
    /// returns the label it represents; otherwise, `None`.
    fn label(&self) -> Option<&Label>;
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
    fn is_child(&self) -> bool {
        matches!(self, Child(_, _))
    }

    #[inline(always)]
    fn label(&self) -> Option<&Label> {
        match self {
            Child(label, _) | Descendant(label, _) => Some(label),
            Root(_) | AnyChild(_) => None,
        }
    }
}

/// Utility blanket implementation for a [`JsonPathQueryNode`] wrapped in an [`Option`].
///
/// If the value is `None` automatically returns `false` or `None` on all calls in
/// the natural manner.
impl<T: lib::ops::Deref<Target = JsonPathQueryNode>> JsonPathQueryNodeType for Option<T> {
    #[inline(always)]
    fn is_root(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_root())
    }

    #[inline(always)]
    fn is_descendant(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_descendant())
    }

    #[inline(always)]
    fn is_child(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_child())
    }

    #[inline(always)]
    fn label(&self) -> Option<&Label> {
        self.as_ref().and_then(|x| x.label())
    }
}

#[cfg(test)]
mod tests {
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
    #[cfg(feature = "std")]
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
