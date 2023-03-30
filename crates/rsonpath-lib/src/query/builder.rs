//! Utility for building a [`JsonPathQuery`](`crate::query::JsonPathQuery`)
//! programatically.
use super::{JsonPathQuery, JsonPathQueryNode, Label};
use crate::lib::{Box, Vec};

/// Builder for [`JsonPathQuery`] instances.
///
/// # Examples
/// ```
/// # use rsonpath_lib::query::{JsonPathQuery, Label, builder::JsonPathQueryBuilder};
/// let builder = JsonPathQueryBuilder::new()
///     .child(Label::new("a"))
///     .descendant(Label::new("b"))
///     .any_child()
///     .child(Label::new("c"));
///
/// // Can also use `builder.build()`.
/// let query: JsonPathQuery = builder.into();
///
/// assert_eq!(format!("{query}"), "$['a']..['b'][*]['c']");
/// ```
pub struct JsonPathQueryBuilder {
    nodes: Vec<NodeTemplate>,
}

impl JsonPathQueryBuilder {
    /// Initialize an empty builder.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_lib::query::{JsonPathQuery, JsonPathQueryNode, Label, builder::JsonPathQueryBuilder};
    /// let builder = JsonPathQueryBuilder::new();
    /// let query: JsonPathQuery = builder.into();
    ///
    /// assert_eq!(*query.root(), JsonPathQueryNode::Root(None));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Add a child selector with a given label.
    #[must_use]
    #[inline(always)]
    pub fn child(mut self, label: Label) -> Self {
        self.nodes.push(NodeTemplate::Child(label));
        self
    }

    /// Add a wildcard child selector.
    #[must_use]
    #[inline(always)]
    pub fn any_child(mut self) -> Self {
        self.nodes.push(NodeTemplate::AnyChild);
        self
    }

    /// Add a descendant selector with a given label.
    #[must_use]
    #[inline(always)]
    pub fn descendant(mut self, label: Label) -> Self {
        self.nodes.push(NodeTemplate::Descendant(label));
        self
    }

    /// Consume the builder and produce a [`JsonPathQuery`].
    #[must_use]
    #[inline]
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

impl Default for JsonPathQueryBuilder {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl From<JsonPathQueryBuilder> for JsonPathQuery {
    #[inline(always)]
    fn from(value: JsonPathQueryBuilder) -> Self {
        value.build()
    }
}

enum NodeTemplate {
    Child(Label),
    AnyChild,
    Descendant(Label),
}
