//! Utility for building a [`JsonPathQuery`](`crate::JsonPathQuery`)
//! programmatically.
use crate::{number::JsonUInt, string::JsonString, JsonPathQuery, JsonPathQueryNode};

/// Builder for [`JsonPathQuery`] instances.
///
/// # Examples
/// ```
/// # use rsonpath_syntax::{JsonPathQuery, builder::JsonPathQueryBuilder, string::JsonString};
/// let builder = JsonPathQueryBuilder::new()
///     .child(JsonString::new("a"))
///     .descendant(JsonString::new("b"))
///     .any_child()
///     .child(JsonString::new("c"))
///     .any_descendant();
///
/// // Can also use `builder.build()`.
/// let query: JsonPathQuery = builder.into();
///
/// assert_eq!(format!("{query}"), "$['a']..['b'][*]['c']..[*]");
/// ```
pub struct JsonPathQueryBuilder {
    nodes: Vec<NodeTemplate>,
}

impl JsonPathQueryBuilder {
    /// Initialize an empty builder.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{JsonPathQuery, JsonPathQueryNode, builder::JsonPathQueryBuilder};
    /// let builder = JsonPathQueryBuilder::new();
    /// let query: JsonPathQuery = builder.into();
    ///
    /// assert_eq!(*query.root(), JsonPathQueryNode::Root(None));
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn new() -> Self {
        Self { nodes: vec![] }
    }

    /// Add a child selector with a given member name.
    #[must_use]
    #[inline(always)]
    pub fn child(mut self, member_name: JsonString) -> Self {
        self.nodes.push(NodeTemplate::Child(member_name));
        self
    }

    /// Add a child selector with a given index.
    #[must_use]
    #[inline(always)]
    pub fn array_index_child(mut self, index: JsonUInt) -> Self {
        self.nodes.push(NodeTemplate::ArrayIndexChild(index));
        self
    }

    /// Add a descendant selector with a given index.
    #[must_use]
    #[inline(always)]
    pub fn array_index_descendant(mut self, index: JsonUInt) -> Self {
        self.nodes.push(NodeTemplate::ArrayIndexDescendant(index));
        self
    }

    /// Add a wildcard child selector.
    #[must_use]
    #[inline(always)]
    pub fn any_child(mut self) -> Self {
        self.nodes.push(NodeTemplate::AnyChild);
        self
    }

    /// Add a descendant selector with a given member_name.
    #[must_use]
    #[inline(always)]
    pub fn descendant(mut self, member_name: JsonString) -> Self {
        self.nodes.push(NodeTemplate::Descendant(member_name));
        self
    }

    /// Add a wildcard descendant selector.
    #[must_use]
    #[inline(always)]
    pub fn any_descendant(mut self) -> Self {
        self.nodes.push(NodeTemplate::AnyDescendant);
        self
    }

    /// Consume the builder and produce a [`JsonPathQuery`].
    #[must_use]
    #[inline]
    pub fn build(self) -> JsonPathQuery {
        let mut last = None;

        for node in self.nodes.into_iter().rev() {
            last = match node {
                NodeTemplate::ArrayIndexChild(i) => Some(Box::new(JsonPathQueryNode::ArrayIndexChild(i, last))),
                NodeTemplate::ArrayIndexDescendant(i) => {
                    Some(Box::new(JsonPathQueryNode::ArrayIndexDescendant(i, last)))
                }
                NodeTemplate::Child(name) => Some(Box::new(JsonPathQueryNode::Child(name, last))),
                NodeTemplate::AnyChild => Some(Box::new(JsonPathQueryNode::AnyChild(last))),
                NodeTemplate::Descendant(name) => Some(Box::new(JsonPathQueryNode::Descendant(name, last))),
                NodeTemplate::AnyDescendant => Some(Box::new(JsonPathQueryNode::AnyDescendant(last))),
            };
        }

        JsonPathQuery::new(Box::new(JsonPathQueryNode::Root(last)))
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
    Child(JsonString),
    ArrayIndexChild(JsonUInt),
    ArrayIndexDescendant(JsonUInt),
    AnyChild,
    AnyDescendant,
    Descendant(JsonString),
}
