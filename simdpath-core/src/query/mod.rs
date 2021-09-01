pub use self::parser::parse_json_path_query;
mod parser;
use std::fmt::{self, Display};

type JsonPathQueryNodeBox<'a> = Box<JsonPathQueryNode<'a>>;
use std::ops::Deref;

#[derive(Debug)]
pub enum JsonPathQueryNode<'a> {
    Root(Option<JsonPathQueryNodeBox<'a>>),
    Descendant(JsonPathQueryNodeBox<'a>),
    Label(&'a [u8], Option<JsonPathQueryNodeBox<'a>>),
}

#[derive(Debug)]
pub struct JsonPathQuery<'a> {
    root: JsonPathQueryNodeBox<'a>,
}

impl<'a> JsonPathQuery<'a> {
    pub fn root(&self) -> &JsonPathQueryNode<'a> {
        self.root.as_ref()
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

use JsonPathQueryNode::*;

pub trait JsonPathQueryNodeType {
    fn is_root(&self) -> bool;
    fn is_descendant(&self) -> bool;
    fn is_label(&self) -> bool;
}

impl<'a> JsonPathQueryNodeType for JsonPathQueryNode<'a> {
    fn is_root(&self) -> bool {
        matches!(self, Root(_))
    }
    fn is_descendant(&self) -> bool {
        matches!(self, Descendant(_))
    }
    fn is_label(&self) -> bool {
        matches!(self, Label(_, _))
    }
}

impl<'a> JsonPathQueryNode<'a> {
    pub fn child(&self) -> Option<&JsonPathQueryNode<'a>> {
        match self {
            Root(node) => node.as_deref(),
            Descendant(node) => Some(node),
            Label(_, node) => node.as_deref(),
        }
    }
}

impl<T: JsonPathQueryNodeType, U: Deref<Target = T>> JsonPathQueryNodeType for Option<U> {
    fn is_root(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_root())
    }
    fn is_descendant(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_descendant())
    }
    fn is_label(&self) -> bool {
        self.as_ref().map_or(false, |x| x.is_label())
    }
}

impl<'a> JsonPathQuery<'a> {
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
