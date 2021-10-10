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
//! assert_eq!(label_node1.label(), Some("person".as_bytes()));
//! assert_eq!(label_node2.label(), Some("phoneNumber".as_bytes()));
//! # Ok(())
//! # }
//! ```
//!
mod parser;
use std::fmt::{self, Display};

type JsonPathQueryNodeBox<'a> = Box<JsonPathQueryNode<'a>>;
use std::ops::Deref;

/// Linked list structure of a JSONPath query.
#[derive(Debug)]
pub enum JsonPathQueryNode<'a> {
    /// The first link in the list representing the root '`$`' character.
    Root(Option<JsonPathQueryNodeBox<'a>>),
    /// Represents recursive descent ('`..`' token).
    Descendant(JsonPathQueryNodeBox<'a>),
    /// Represents a label/key to be matched in the input JSON.
    Label(&'a [u8], Option<JsonPathQueryNodeBox<'a>>),
}

use JsonPathQueryNode::*;

impl<'a> JsonPathQueryNode<'a> {
    /// Retrieve the child of the node or `None` if it is the last one
    /// on the list.
    pub fn child(&self) -> Option<&JsonPathQueryNode<'a>> {
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
pub struct JsonPathQuery<'a> {
    root: JsonPathQueryNodeBox<'a>,
}

impl<'a> JsonPathQuery<'a> {
    /// Retrieve reference to the root node.
    ///
    /// It is guaranteed that the root is the [`JsonPathQueryNode::Root`]
    /// variant and always exists.
    pub fn root(&self) -> &JsonPathQueryNode<'a> {
        self.root.as_ref()
    }

    /// Parse a query string into a [`JsonPathQuery`].
    pub fn parse(query_string: &str) -> Result<JsonPathQuery<'_>, String> {
        self::parser::parse_json_path_query(query_string)
    }

    /// Create a query from a root node.
    ///
    /// If node is not the [`JsonPathQueryNode::Root`] variant it will be
    /// automatically wrapped into a [`JsonPathQueryNode::Root`] node.
    pub fn new(node: JsonPathQueryNodeBox<'a>) -> Result<JsonPathQuery<'a>, String> {
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

    fn validate(node: &JsonPathQueryNode<'a>) -> Result<(), String> {
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

impl<'a> Display for JsonPathQuery<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.root.as_ref())
    }
}

impl<'a> Display for JsonPathQueryNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = match self {
            Root(_) => "$",
            Descendant(_) => "..",
            Label(label, _) => std::str::from_utf8(label).unwrap(),
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
pub trait JsonPathQueryNodeType<'a> {
    /// Returns `true` iff the type is [`JsonPathQueryNode::Root`].
    fn is_root(&self) -> bool;

    /// Returns `true` iff the type is [`JsonPathQueryNode::Descendant`].
    fn is_descendant(&self) -> bool;

    /// Returns `true` iff the type is [`JsonPathQueryNode::Label`].
    fn is_label(&self) -> bool;

    /// If the type is [`JsonPathQueryNode::Label`] returns the label it represents;
    /// otherwise, `None`.
    fn label(&self) -> Option<&'a [u8]>;
}

impl<'a> JsonPathQueryNodeType<'a> for JsonPathQueryNode<'a> {
    fn is_root(&self) -> bool {
        matches!(self, Root(_))
    }

    fn is_descendant(&self) -> bool {
        matches!(self, Descendant(_))
    }

    fn is_label(&self) -> bool {
        matches!(self, Label(_, _))
    }

    fn label(&self) -> Option<&'a [u8]> {
        match self {
            JsonPathQueryNode::Label(label, _) => Some(label),
            _ => None,
        }
    }
}

/// Utility blanket implementation for a [`JsonPathQueryNodeType`] wrapped in an [`Option`].
///
/// If the value is `None` automatically returns `false` or `None` on all calls in
/// the natural manner.
impl<'a, T: JsonPathQueryNodeType<'a>, U: Deref<Target = T>> JsonPathQueryNodeType<'a>
    for Option<U>
{
    fn is_root(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_root())
    }

    fn is_descendant(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_descendant())
    }

    fn is_label(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_label())
    }

    fn label(&self) -> Option<&'a [u8]> {
        self.as_ref().and_then(|x| x.label())
    }
}
