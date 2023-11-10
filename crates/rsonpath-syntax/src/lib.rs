//! Complete, fast, and fully spec-compliant JSONPath query parser.
//!
//! The crate exposes the [`JsonPathQuery`] type and the [`parse`](`JsonPathQuery::parse`)
//! function that converts a query string into the AST representation. The parsing
//! complies with the proposed [JSONPath RFC specification](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html).
//!
//! A JSONPath query is a sequence of **segments**, each containing one or more
//! **selectors**. There are two types of segments, **child** and **descendant**,
//! and five different types of selectors: **name**, **wildcard**, **index**, **slice**, and **filter**.
//!
//! Descriptions of each segment and selector can be found in the documentation of the
//! relevant type in this crate, while the formal grammar is described in the RFC.
//!
//! # Examples
//! To create a query from a query string:
//! ```
//! # use rsonpath_syntax::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};
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
//! assert_eq!(descendant_node.member_name().unwrap(), "phoneNumbers".as_bytes());
//! assert_eq!(child_wildcard_node.member_name(), None);
//! assert_eq!(child_node.member_name().unwrap(), "number".as_bytes());
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]
// Generic pedantic lints.
#![warn(
    explicit_outlives_requirements,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes
)]
// Clippy pedantic lints.
#![warn(
    clippy::allow_attributes_without_reason,
    clippy::cargo_common_metadata,
    clippy::cast_lossless,
    clippy::cloned_instead_of_copied,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::fallible_impl_from,
    clippy::flat_map_option,
    clippy::if_then_some_else_none,
    clippy::inconsistent_struct_constructor,
    clippy::large_digit_groups,
    clippy::let_underscore_must_use,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_inline_in_public_items,
    clippy::mod_module_files,
    clippy::must_use_candidate,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::ptr_as_ptr,
    clippy::redundant_closure_for_method_calls,
    clippy::ref_binding_to_reference,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::undocumented_unsafe_blocks,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::use_self
)]
// Panic-free lint.
#![warn(clippy::exit)]
// Panic-free lints (disabled for tests).
#![cfg_attr(not(test), warn(clippy::panic, clippy::panic_in_result_fn, clippy::unwrap_used))]
// IO hygiene, only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(clippy::print_stderr, clippy::print_stdout, clippy::todo)
)]
// Documentation lints, enabled only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc,)
)]
#![cfg_attr(not(debug_assertions), warn(rustdoc::missing_crate_level_docs))]
// Docs.rs config.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_logo_url = "https://raw.githubusercontent.com/V0ldek/rsonpath/main/img/rsonquery-logo.svg")]

pub mod builder;
pub mod error;
pub mod number;
mod parser;
pub mod string;

use self::{error::ParserError, number::NonNegativeArrayIndex, string::JsonString};
use std::fmt::{self, Display};

/// Linked list structure of a JSONPath query.
#[derive(Debug, PartialEq, Eq)]
pub enum JsonPathQueryNode {
    /// The first link in the list representing the root '`$`' character.d
    Root(Option<Box<JsonPathQueryNode>>),
    /// Represents direct descendant with a given property name ('`.`' token).
    Child(JsonString, Option<Box<JsonPathQueryNode>>),
    /// Represents direct descendant with a wildcard ('`.*`' tokens).
    AnyChild(Option<Box<JsonPathQueryNode>>),
    /// Represents recursive descent ('`..`' token).
    Descendant(JsonString, Option<Box<JsonPathQueryNode>>),
    /// Represents recursive descendant with a wildcard ('`..*`' tokens).
    AnyDescendant(Option<Box<JsonPathQueryNode>>),
    /// Represents direct descendant list item with a positive index (numbers).
    ArrayIndexChild(NonNegativeArrayIndex, Option<Box<JsonPathQueryNode>>),
    /// Represents recursive descendant with an array index ('`..[n]`' tokens).
    ArrayIndexDescendant(NonNegativeArrayIndex, Option<Box<JsonPathQueryNode>>),
}

use JsonPathQueryNode::*;

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
            | ArrayIndexChild(_, node)
            | ArrayIndexDescendant(_, node) => node.as_deref(),
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
            Child(key, _) => write!(f, "['{}']", key.display()),
            AnyChild(_) => write!(f, "[*]"),
            Descendant(key, _) => write!(f, "..['{}']", key.display()),
            AnyDescendant(_) => write!(f, "..[*]"),
            ArrayIndexChild(i, _) => write!(f, "[{i}]"),
            ArrayIndexDescendant(i, _) => write!(f, "..[{i}]"),
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
    /// returns the member name it represents; otherwise, `None`.
    fn member_name(&self) -> Option<&JsonString>;

    /// If the type is [`JsonPathQueryNode::ArrayIndexDescendant`] or [`JsonPathQueryNode::ArrayIndexChild`]
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
    fn member_name(&self) -> Option<&JsonString> {
        match self {
            Child(name, _) | Descendant(name, _) => Some(name),
            Root(_) | AnyChild(_) | AnyDescendant(_) | ArrayIndexChild(_, _) | ArrayIndexDescendant(_, _) => None,
        }
    }

    #[inline(always)]
    fn array_index(&self) -> Option<&NonNegativeArrayIndex> {
        match self {
            ArrayIndexChild(i, _) | ArrayIndexDescendant(i, _) => Some(i),
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
    fn member_name(&self) -> Option<&JsonString> {
        self.as_ref().and_then(|x| x.member_name())
    }

    #[inline(always)]
    fn array_index(&self) -> Option<&NonNegativeArrayIndex> {
        self.as_ref().and_then(|x| x.array_index())
    }
}

// We don't implement Arbitrary for JsonPathQueryNode as it is pretty much meaningless.
// In particular, constructing a query as just a sequence of arbitrary nodes is invalid,
// because the Root note must be always be the first node in the query and can never occur later.
#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for JsonPathQuery {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        #[derive(arbitrary::Arbitrary)]
        enum RawNode {
            Child(JsonString),
            AnyChild,
            Descendant(JsonString),
            AnyDescendant,
            ArrayIndexChild(NonNegativeArrayIndex),
            ArrayIndexDescendant(NonNegativeArrayIndex),
        }

        let sequence = u.arbitrary_iter()?;
        let mut node = None;

        for raw in sequence {
            node = Some(Box::new(match raw? {
                RawNode::Child(s) => JsonPathQueryNode::Child(s, node),
                RawNode::AnyChild => JsonPathQueryNode::AnyChild(node),
                RawNode::Descendant(s) => JsonPathQueryNode::Descendant(s, node),
                RawNode::AnyDescendant => JsonPathQueryNode::AnyDescendant(node),
                RawNode::ArrayIndexChild(i) => JsonPathQueryNode::ArrayIndexChild(i, node),
                RawNode::ArrayIndexDescendant(i) => JsonPathQueryNode::ArrayIndexDescendant(i, node),
            }));
        }

        Ok(Self {
            root: Box::new(JsonPathQueryNode::Root(node)),
        })
    }
}
