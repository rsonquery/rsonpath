//! Defines JSONPath query structure and parsing logic.
//!
//! # Examples
//! To create a query from a query string:
//! ```
//! # use rsonpath::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType};
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
pub mod automaton;
pub mod builder;
pub mod error;
