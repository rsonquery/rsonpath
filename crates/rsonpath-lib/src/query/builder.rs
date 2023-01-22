//! Utility for building a [`JsonPathQuery`](`crate::query::JsonPathQuery`)
//! programatically.
//!
//! # Examples
//! ```
//! # use rsonpath_lib::query::{JsonPathQuery, Label, builder::JsonPathQueryBuilder};
//!
//! let builder = JsonPathQueryBuilder::new()
//!     .child(Label::new("a"))
//!     .descendant(Label::new("b"))
//!     .any_child()
//!     .child(Label::new("c"));
//!
//! // Can also use `builder.build()`.
//! let query: JsonPathQuery = builder.into();
//!
//! assert_eq!(format!("{query}"), "$['a']..['b'][*]['c']");
//! ```
use super::{JsonPathQuery, JsonPathQueryNode, Label};

pub struct JsonPathQueryBuilder {
    nodes: Vec<NodeTemplate>,
}

impl JsonPathQueryBuilder {
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    pub fn child(mut self, label: Label) -> Self {
        self.nodes.push(NodeTemplate::Child(label));
        self
    }

    pub fn any_child(mut self) -> Self {
        self.nodes.push(NodeTemplate::AnyChild);
        self
    }

    pub fn descendant(mut self, label: Label) -> Self {
        self.nodes.push(NodeTemplate::Descendant(label));
        self
    }

    pub fn build(self) -> JsonPathQuery {
        let mut last = None;

        for node in self.nodes.into_iter().rev() {
            last = match node {
                NodeTemplate::Child(label) => Some(Box::new(JsonPathQueryNode::Child(label, last))),
                NodeTemplate::AnyChild => Some(Box::new(JsonPathQueryNode::AnyChild(last))),
                NodeTemplate::Descendant(label) => {
                    Some(Box::new(JsonPathQueryNode::Descendant(label, last)))
                }
            };
        }

        JsonPathQuery {
            root: Box::new(JsonPathQueryNode::Root(last)),
        }
    }
}

impl From<JsonPathQueryBuilder> for JsonPathQuery {
    fn from(value: JsonPathQueryBuilder) -> Self {
        value.build()
    }
}

enum NodeTemplate {
    Child(Label),
    AnyChild,
    Descendant(Label),
}
