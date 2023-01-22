//! Utility for building a [`JsonPathQuery`](`crate::query::JsonPathQuery`)
//! programatically.
//!
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

    pub fn descendant(mut self, label: Label) -> Self {
        self.nodes.push(NodeTemplate::Descendant(label));
        self
    }

    pub fn build(self) -> JsonPathQuery {
        let mut last = None;

        for node in self.nodes.into_iter().rev() {
            last = match node {
                NodeTemplate::Child(label) => Some(Box::new(JsonPathQueryNode::Child(label, last))),
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
    Descendant(Label),
}
