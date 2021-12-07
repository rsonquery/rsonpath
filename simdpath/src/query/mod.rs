//! Defines JSONPath query structure and parsing logic.
//!
//! # Examples
//! To create a query from a query string:
//! ```
//! # use simdpath::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let query_string = "$..person..phoneNumber";
//! let query = JsonPathQuery::parse(query_string)?;
//!
//! // Query structure is a linear sequence of nodes:
//! // Root '$', descendant '..', label 'person', descendant '..', label 'phoneNumber'.
//! let root_node = query.root();
//! let descendant_node1 = root_node.child().unwrap();
//! let label_node1 = descendant_node1.child().unwrap();
//! let descendant_node2 = label_node1.child().unwrap();
//! let label_node2 = descendant_node2.child().unwrap();
//!
//! assert!(root_node.is_root());
//! assert!(descendant_node1.is_descendant());
//! assert!(label_node1.is_label());
//! assert!(descendant_node2.is_descendant());
//! assert!(label_node2.is_label());
//! // Final node will have a None child.
//! assert!(label_node2.child().is_none());
//!
//! assert_eq!(label_node1.label().unwrap(), "person".as_bytes());
//! assert_eq!(label_node2.label().unwrap(), "phoneNumber".as_bytes());
//! # Ok(())
//! # }
//! ```
//!
mod parser;
use crate::bytes::align::{alignment, Aligned, AlignedBytes};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Label {
    label: AlignedBytes<alignment::Block>,
    label_with_quotes: AlignedBytes<alignment::Block>,
}

impl Label {
    pub fn new(label: &[u8]) -> Self {
        let without_quotes = AlignedBytes::<alignment::Block>::from(label);
        let mut with_quotes = AlignedBytes::<alignment::Block>::new(label.len() + 2);
        with_quotes[0] = b'"';
        with_quotes[1..label.len() + 1].copy_from_slice(label);
        with_quotes[label.len() + 1] = b'"';

        Self {
            label: without_quotes,
            label_with_quotes: with_quotes,
        }
    }

    pub fn bytes(&self) -> &AlignedBytes<alignment::Block> {
        &self.label
    }

    pub fn bytes_with_quotes(&self) -> &AlignedBytes<alignment::Block> {
        &self.label_with_quotes
    }
}

impl std::ops::Deref for Label {
    type Target = AlignedBytes<alignment::Block>;

    fn deref(&self) -> &Self::Target {
        self.bytes()
    }
}

impl PartialEq<Label> for Label {
    fn eq(&self, other: &Label) -> bool {
        self.label == other.label
    }
}

impl Eq for Label {}

impl PartialEq<Label> for [u8] {
    fn eq(&self, other: &Label) -> bool {
        self == other.label
    }
}

impl PartialEq<Label> for &[u8] {
    fn eq(&self, other: &Label) -> bool {
        *self == other.label
    }
}

impl PartialEq<[u8]> for Label {
    fn eq(&self, other: &[u8]) -> bool {
        self.label == other
    }
}

impl PartialEq<&[u8]> for Label {
    fn eq(&self, other: &&[u8]) -> bool {
        self.label == *other
    }
}

/// Linked list structure of a JSONPath query.
#[derive(Debug)]
pub enum JsonPathQueryNode {
    /// The first link in the list representing the root '`$`' character.
    Root(Option<Box<JsonPathQueryNode>>),
    /// Represents recursive descent ('`..`' token).
    Descendant(Box<JsonPathQueryNode>),
    /// Represents a label/key to be matched in the input JSON.
    Label(Label, Option<Box<JsonPathQueryNode>>),
}

use JsonPathQueryNode::*;

impl JsonPathQueryNode {
    /// Retrieve the child of the node or `None` if it is the last one
    /// on the list.
    pub fn child(&self) -> Option<&JsonPathQueryNode> {
        match self {
            Root(node) => node.as_deref(),
            Descendant(node) => Some(node),
            Label(_, node) => node.as_deref(),
        }
    }
}

/// JSONPath query structure represented by the root link of the
/// [`JsonPathQueryNode`] list.
#[derive(Debug)]
pub struct JsonPathQuery {
    root: Box<JsonPathQueryNode>,
}

impl JsonPathQuery {
    /// Retrieve reference to the root node.
    ///
    /// It is guaranteed that the root is the [`JsonPathQueryNode::Root`]
    /// variant and always exists.
    pub fn root(&self) -> &JsonPathQueryNode {
        self.root.as_ref()
    }

    /// Parse a query string into a [`JsonPathQuery`].
    pub fn parse(query_string: &str) -> Result<JsonPathQuery, String> {
        self::parser::parse_json_path_query(query_string)
    }

    /// Create a query from a root node.
    ///
    /// If node is not the [`JsonPathQueryNode::Root`] variant it will be
    /// automatically wrapped into a [`JsonPathQueryNode::Root`] node.
    pub fn new(node: Box<JsonPathQueryNode>) -> Result<JsonPathQuery, String> {
        let root = if node.is_root() {
            node
        } else {
            Box::new(Root(Some(node)))
        };

        match root.child() {
            None => Ok(Self { root }),
            Some(x) if x.is_descendant() => Self::validate(x).map(|_| Self { root }),
            Some(_) => Err("Root child expressions are not supported.".to_string()),
        }
    }

    fn validate(node: &JsonPathQueryNode) -> Result<(), String> {
        match node {
            Root(_) => Err(
                "The Root expression ('$') can appear only once at the start of the query."
                    .to_string(),
            ),
            Descendant(n) if n.is_descendant() => Err("Descendant expression ('..') cannot immediatelly follow another Descendant expression.".to_string()),
            Label(_, n) if n.is_label() => Err("Child Label expressions are not supported.".to_string()),
            _ => Ok(())
        }?;

        match node.child() {
            None => Ok(()),
            Some(x) => Self::validate(x),
        }
    }
}

impl Display for JsonPathQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root.as_ref())
    }
}

impl Display for JsonPathQueryNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = match self {
            Root(_) => "$",
            Descendant(_) => "..",
            Label(label, _) => std::str::from_utf8(label.bytes()).unwrap(),
        };
        write!(f, "{}", head)?;

        if let Some(child) = self.child() {
            write!(f, "{}", child)
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

    /// Returns `true` iff the type is [`JsonPathQueryNode::Label`].
    fn is_label(&self) -> bool;

    /// If the type is [`JsonPathQueryNode::Label`] returns the label it represents;
    /// otherwise, `None`.
    fn label(&self) -> Option<&Label>;
}

impl JsonPathQueryNodeType for JsonPathQueryNode {
    fn is_root(&self) -> bool {
        matches!(self, Root(_))
    }

    fn is_descendant(&self) -> bool {
        matches!(self, Descendant(_))
    }

    fn is_label(&self) -> bool {
        matches!(self, Label(_, _))
    }

    fn label(&self) -> Option<&Label> {
        match self {
            JsonPathQueryNode::Label(label, _) => Some(label),
            _ => None,
        }
    }
}

/// Utility blanket implementation for a [`JsonPathQueryNode`] wrapped in an [`Option`].
///
/// If the value is `None` automatically returns `false` or `None` on all calls in
/// the natural manner.
impl<T: std::ops::Deref<Target = JsonPathQueryNode>> JsonPathQueryNodeType for Option<T> {
    fn is_root(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_root())
    }

    fn is_descendant(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_descendant())
    }

    fn is_label(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_label())
    }

    fn label(&self) -> Option<&Label> {
        self.as_ref().and_then(|x| x.label())
    }
}
