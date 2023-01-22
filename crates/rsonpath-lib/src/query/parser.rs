use super::error::{ParseErrorReport, ParserError};
use crate::debug;
use crate::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *,
};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
enum Token<'a> {
    Root,
    Child(&'a str),
    WildcardChild(),
    Descendant(&'a str),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Root => write!(f, "$"),
            Token::Child(label) => write!(f, "['{label}']"),
            Token::WildcardChild() => write!(f, "[*]"),
            Token::Descendant(label) => write!(f, "..['{label}']"),
        }
    }
}

pub(crate) fn parse_json_path_query(query_string: &str) -> Result<JsonPathQuery, ParserError> {
    let tokens_result = jsonpath()(query_string);
    let finished = tokens_result.finish();

    match finished {
        #[allow(unused_variables)]
        Ok(("", (root_token, tokens))) => {
            debug!(
                "Parsed tokens: {}",
                root_token.map_or(String::new(), |x| format!("{x}"))
                    + &tokens
                        .iter()
                        .map(|x| format!("({x:?})"))
                        .collect::<String>()
            );
            let node = tokens_to_node(&mut tokens.into_iter())?;
            Ok(match node {
                None => JsonPathQuery::new(Box::new(JsonPathQueryNode::Root(None))),
                Some(node) if node.is_root() => JsonPathQuery::new(Box::new(node)),
                Some(node) => {
                    JsonPathQuery::new(Box::new(JsonPathQueryNode::Root(Some(Box::new(node)))))
                }
            })
        }
        _ => {
            let mut parse_errors = ParseErrorReport::new();
            let mut continuation = finished.map(|x| x.0);
            loop {
                match continuation {
                    Ok("") => {
                        return Err(ParserError::SyntaxError {
                            report: parse_errors,
                        })
                    }
                    Ok(remaining) => {
                        let error_character_index = query_string.len() - remaining.len();
                        parse_errors.record_at(error_character_index);
                        continuation = non_root()(&remaining[1..]).finish().map(|x| x.0);
                    }
                    Err(e) => {
                        return Err(nom::error::Error::new(query_string.to_owned(), e.code).into())
                    }
                }
            }
        }
    }
}

fn tokens_to_node<'a, I: Iterator<Item = Token<'a>>>(
    tokens: &mut I,
) -> Result<Option<JsonPathQueryNode>, ParserError> {
    match tokens.next() {
        Some(token) => {
            let child_node = tokens_to_node(tokens)?.map(Box::new);
            match token {
                Token::Root => Ok(Some(JsonPathQueryNode::Root(child_node))),
                Token::Child(label) => Ok(Some(JsonPathQueryNode::Child(
                    Label::new(label),
                    child_node,
                ))),
                Token::WildcardChild() => Ok(Some(JsonPathQueryNode::AnyChild(child_node))),
                Token::Descendant(label) => Ok(Some(JsonPathQueryNode::Descendant(
                    Label::new(label),
                    child_node,
                ))),
            }
        }
        _ => Ok(None),
    }
}

trait Parser<'a, Out>: FnMut(&'a str) -> IResult<&'a str, Out> {}

impl<'a, Out, T: FnMut(&'a str) -> IResult<&'a str, Out>> Parser<'a, Out> for T {}

fn jsonpath<'a>() -> impl Parser<'a, (Option<Token<'a>>, Vec<Token<'a>>)> {
    pair(
        opt(map(char('$'), |_| Token::Root)), // root selector
        non_root(),
    )
}

fn non_root<'a>() -> impl Parser<'a, Vec<Token<'a>>> {
    many0(alt((
        wildcard_child_selector(),
        child_selector(),
        descendant_selector(),
    )))
}

fn wildcard_child_selector<'a>() -> impl Parser<'a, Token<'a>> {
    map(
        alt((dot_wildcard_selector(), index_wildcard_selector())),
        |_| Token::WildcardChild(),
    )
}

fn child_selector<'a>() -> impl Parser<'a, Token<'a>> {
    map(alt((dot_selector(), index_selector())), Token::Child)
}

fn dot_selector<'a>() -> impl Parser<'a, &'a str> {
    preceded(char('.'), label())
}

fn dot_wildcard_selector<'a>() -> impl Parser<'a, char> {
    preceded(char('.'), char('*'))
}

fn descendant_selector<'a>() -> impl Parser<'a, Token<'a>> {
    map(
        preceded(tag(".."), alt((label(), index_selector()))),
        Token::Descendant,
    )
}

fn index_selector<'a>() -> impl Parser<'a, &'a str> {
    delimited(char('['), quoted_label(), char(']'))
}

fn index_wildcard_selector<'a>() -> impl Parser<'a, char> {
    delimited(char('['), char('*'), char(']'))
}

fn label<'a>() -> impl Parser<'a, &'a str> {
    recognize(pair(label_first(), many0(label_character())))
}

fn label_first<'a>() -> impl Parser<'a, char> {
    verify(anychar, |&x| x.is_alpha() || x == '_' || !x.is_ascii())
}

fn label_character<'a>() -> impl Parser<'a, char> {
    verify(anychar, |&x| {
        x.is_alphanumeric() || x == '_' || !x.is_ascii()
    })
}

fn quoted_label<'a>() -> impl Parser<'a, &'a str> {
    alt((
        delimited(char('\''), single_quoted_label(), char('\'')),
        delimited(char('"'), double_quoted_label(), char('"')),
    ))
}

//cSpell: disable
fn single_quoted_label<'a>() -> impl Parser<'a, &'a str> {
    escaped(
        many0(alt((unescaped(), char('"')))),
        '\\',
        one_of(r#"'btnfru/\"#),
    )
}

fn double_quoted_label<'a>() -> impl Parser<'a, &'a str> {
    escaped(
        many0(alt((unescaped(), char('\'')))),
        '\\',
        one_of(r#""btnfru/\"#),
    )
}

fn unescaped<'a>() -> impl Parser<'a, char> {
    verify(anychar, |&x| x != '\'' && x != '"')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::builder::JsonPathQueryBuilder;

    #[test]
    fn single_quoted_label_test() {
        let input = "a";

        let result = single_quoted_label()(input);

        assert_eq!(result, Ok(("", "a")));
    }

    #[test]
    fn double_quoted_label_test() {
        let input = "a";

        let result = double_quoted_label()(input);

        assert_eq!(result, Ok(("", "a")));
    }

    #[test]
    fn quoted_label_test() {
        let input = "'a'";

        let result = quoted_label()(input);

        assert_eq!(result, Ok(("", "a")));
    }

    #[test]
    fn wildcard_child_selector_test() {
        let input = "$.*.a.*";
        let expected_query = JsonPathQueryBuilder::new()
            .any_child()
            .child(Label::new("a"))
            .any_child()
            .into();

        let result = parse_json_path_query(input).expect("expected Ok");

        assert_eq!(result, expected_query);
    }

    #[test]
    fn indexed_wildcard_child_selector_test() {
        let input = r#"$[*]['*']["*"]"#;
        let expected_query = JsonPathQueryBuilder::new()
            .any_child()
            .child(Label::new("*"))
            .child(Label::new("*"))
            .into();

        let result = parse_json_path_query(input).expect("expected Ok");

        assert_eq!(result, expected_query);
    }
}
