use crate::query::{JsonPathQuery, JsonPathQueryNode};
use nom::{
    branch::*,
    bytes::complete::*,
    character::{complete::*, *},
    combinator::*,
    multi::*,
    sequence::*,
    *,
};

enum Token<'a> {
    Root,
    Descendant,
    Label(&'a [u8]),
}

pub fn parse_json_path_query(query_string: &str) -> Result<JsonPathQuery<'_>, String> {
    let tokens_result = many0(json_path_query_node())(query_string.as_bytes());
    let finished = tokens_result.finish();
    match finished {
        Ok(([], tokens)) => {
            let node = tokens_to_node(&mut tokens.into_iter())?;
            match node {
                None => JsonPathQuery::new(Box::new(JsonPathQueryNode::Root(None))),
                Some(node) => JsonPathQuery::new(Box::new(node)),
            }
        }
        Ok((i, _)) => Err(format!(
            "Unexpected tokens in the query string: '{}'.",
            std::str::from_utf8(i).unwrap_or("")
        )),
        Err(e) => Err(format!(
            "{}",
            nom::error::Error {
                input: std::str::from_utf8(e.input).unwrap_or(""),
                code: e.code
            }
        )),
    }
}

fn tokens_to_node<'a, I: Iterator<Item = Token<'a>>>(
    tokens: &mut I,
) -> Result<Option<JsonPathQueryNode<'a>>, String> {
    let token = tokens.next();

    if token.is_none() {
        return Ok(None);
    }

    let child_node = tokens_to_node(tokens)?.map(Box::new);

    match token.unwrap() {
        Token::Root => Ok(Some(JsonPathQueryNode::Root(child_node))),
        Token::Descendant => {
            let child_node = child_node.ok_or_else(|| {
                "Descendant expression ('..') must be followed by another expression.".to_string()
            })?;
            Ok(Some(JsonPathQueryNode::Descendant(child_node)))
        }
        Token::Label(label) => Ok(Some(JsonPathQueryNode::Label(label, child_node))),
    }
}

trait Parser<'a>: FnMut(&'a [u8]) -> IResult<&'a [u8], Token> {}

impl<'a, T: FnMut(&'a [u8]) -> IResult<&'a [u8], Token>> Parser<'a> for T {}

fn json_path_query_node<'a>() -> impl Parser<'a> {
    alt((
        complete(json_path_root()),
        complete(json_path_descendant()),
        complete(json_path_label()),
    ))
}

fn json_path_root<'a>() -> impl Parser<'a> {
    map(char('$'), |_| Token::Root)
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
