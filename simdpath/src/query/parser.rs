use crate::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};
use color_eyre::{
    eyre::{eyre, Result, WrapErr},
    section::Section,
};
use log::*;
use nom::{
    branch::*,
    bytes::complete::*,
    character::{complete::*, *},
    combinator::*,
    multi::*,
    sequence::*,
    *,
};
use std::fmt::{self, Display};

#[derive(Debug)]
enum Token<'a> {
    Root,
    Child,
    Descendant,
    Label(&'a [u8]),
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Root => write!(f, "$"),
            Token::Child => write!(f, "."),
            Token::Descendant => write!(f, ".."),
            Token::Label(label) => write!(f, "['{}']", std::str::from_utf8(label).unwrap()),
        }
    }
}

pub fn parse_json_path_query(query_string: &str) -> Result<JsonPathQuery> {
    let tokens_result = many0(json_path_query_node())(query_string.as_bytes());
    let finished = tokens_result.finish();
    match finished {
        Ok(([], tokens)) => {
            debug!(
                "Parsed tokens: {}",
                tokens
                    .iter()
                    .map(|x| format!("({:?})", x))
                    .collect::<String>()
            );
            let node = tokens_to_node(&mut tokens.into_iter())?;
            match node {
                None => JsonPathQuery::new(Box::new(JsonPathQueryNode::Root(None))),
                Some(node) => JsonPathQuery::new(Box::new(node)),
            }
        }
        Ok((remaining, tokens)) => {
            let remaining_characters = std::str::from_utf8(remaining).unwrap_or("[invalid utf8]");
            let parsed_characters = tokens
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>();
            let error_character_index = query_string.len() - remaining.len() + 1;

            Err(eyre!(
                "Unexpected tokens in the query string (position {}): `{}`.",
                error_character_index,
                remaining_characters
            ))
            .note(format!(
                "The preceding characters were successfully parsed as `{}`.",
                parsed_characters
            ))
            .suggestion(format!("Check the query syntax. If the error is caused by special characters in a label, use the explicit `['label']` syntax."))
        }
        Err(e) => Err(nom::error::Error {
            input: std::str::from_utf8(e.input)
                .unwrap_or("[invalid utf8]")
                .to_string(),
            code: e.code,
        })
        .wrap_err("Unexpected error parsing the query string."),
    }
}

fn tokens_to_node<'a, I: Iterator<Item = Token<'a>>>(
    tokens: &mut I,
) -> Result<Option<JsonPathQueryNode>> {
    let token = tokens.next();

    if token.is_none() {
        return Ok(None);
    }

    let child_node = tokens_to_node(tokens)?.map(Box::new);

    match token.unwrap() {
        Token::Root => Ok(Some(JsonPathQueryNode::Root(child_node))),
        Token::Child => {
            let child_node = child_node
                .ok_or_else(|| {
                    eyre!("Child expression ('.') must be followed by another expression.")
                })
                .note("The query was successfully parsed, but a trailing child expression is unexpected.")
                .suggestion(
                    "If the periods are part of a label, try using the explicit `['label']` syntax.",
                )?;
            Ok(Some(JsonPathQueryNode::Child(child_node)))
        }
        Token::Descendant => {
            let child_node = child_node
                .ok_or_else(|| {
                    eyre!("Descendant expression ('..') must be followed by another expression.")
                })
                .note("The query was successfully parsed, but a trailing descendant expression is unexpected.")
                .suggestion(
                    "If the periods are part of a label, try using the explicit `['label']` syntax.",
                )?;
            Ok(Some(JsonPathQueryNode::Descendant(child_node)))
        }
        Token::Label(label) => {
            let child_node = child_node.map(|x| {
                if x.is_label() {
                    Box::new(JsonPathQueryNode::Child(x))
                } else {
                    x
                }
            });

            Ok(Some(JsonPathQueryNode::Label(
                Label::new(label),
                child_node,
            )))
        }
    }
}

trait Parser<'a>: FnMut(&'a [u8]) -> IResult<&'a [u8], Token> {}

impl<'a, T: FnMut(&'a [u8]) -> IResult<&'a [u8], Token>> Parser<'a> for T {}

fn json_path_query_node<'a>() -> impl Parser<'a> {
    alt((
        complete(json_path_root()),
        complete(json_path_descendant()), // Must come before child to be unambiguous!
        complete(json_path_child()),
        complete(json_path_label()),
    ))
}

fn json_path_root<'a>() -> impl Parser<'a> {
    map(char('$'), |_| Token::Root)
}

fn json_path_child<'a>() -> impl Parser<'a> {
    map(tag("."), |_| Token::Child)
}

fn json_path_descendant<'a>() -> impl Parser<'a> {
    map(tag(".."), |_| Token::Descendant)
}

fn json_path_label<'a>() -> impl Parser<'a> {
    alt((
        complete(json_path_label_simple()),
        complete(json_path_label_bracketed()),
    ))
}

fn json_path_label_simple<'a>() -> impl Parser<'a> {
    map(
        take_while1(|x| is_alphanumeric(x) || x == b'_'),
        Token::Label,
    )
}

fn json_path_label_bracketed<'a>() -> impl Parser<'a> {
    map(
        preceded(tag("['"), terminated(take_until("']"), tag("']"))),
        Token::Label,
    )
}
