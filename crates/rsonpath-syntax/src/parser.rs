use crate::{
    error::{ParseErrorReport, ParserError},
    num::JsonUInt,
    str::JsonString,
    JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType,
};
use nom::{branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *};
use std::fmt::{self, Display};
#[derive(Debug, Clone)]
enum Token {
    Root,
    Child(JsonString),
    ArrayIndexChild(JsonUInt),
    WildcardChild(),
    Descendant(JsonString),
    ArrayIndexDescendant(JsonUInt),
    WildcardDescendant(),
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root => write!(f, "$"),
            Self::Child(member) => write!(f, "['{}']", member.unquoted()),
            Self::ArrayIndexChild(i) => write!(f, "[{i}]"),
            Self::WildcardChild() => write!(f, "[*]"),
            Self::Descendant(member) => write!(f, "..['{}']", member.unquoted()),
            Self::WildcardDescendant() => write!(f, "..[*]"),
            Self::ArrayIndexDescendant(i) => write!(f, "..[{i}]"),
        }
    }
}

pub(crate) fn parse_json_path_query(query_string: &str) -> Result<JsonPathQuery, ParserError> {
    let tokens_result = jsonpath()(query_string);
    let finished = tokens_result.finish();

    match finished {
        Ok(("", (_root_token, tokens))) => {
            let node = tokens_to_node(&mut tokens.into_iter())?;
            Ok(match node {
                None => JsonPathQuery::new(Box::new(JsonPathQueryNode::Root(None))),
                Some(node) if node.is_root() => JsonPathQuery::new(Box::new(node)),
                Some(node) => JsonPathQuery::new(Box::new(JsonPathQueryNode::Root(Some(Box::new(node))))),
            })
        }
        _ => {
            let mut parse_errors = ParseErrorReport::new();
            let mut continuation = finished.map(|x| x.0);
            loop {
                match continuation {
                    Ok("") => return Err(ParserError::SyntaxError { report: parse_errors }),
                    Ok(remaining) => {
                        let error_character_index = query_string.len() - remaining.len();
                        parse_errors.record_at(error_character_index);
                        let next_char_boundary = (1..=4)
                            .find(|x| remaining.is_char_boundary(*x))
                            .expect("longest UTF8 char is 4 bytes");
                        continuation = non_root()(&remaining[next_char_boundary..]).finish().map(|x| x.0);
                    }
                    Err(e) => return Err(nom::error::Error::new(query_string.to_owned(), e.code).into()),
                }
            }
        }
    }
}

fn tokens_to_node<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<Option<JsonPathQueryNode>, ParserError> {
    match tokens.next() {
        Some(token) => {
            let child_node = tokens_to_node(tokens)?.map(Box::new);
            match token {
                Token::Root => Ok(Some(JsonPathQueryNode::Root(child_node))),
                Token::Child(member) => Ok(Some(JsonPathQueryNode::Child(member, child_node))),
                Token::ArrayIndexChild(i) => Ok(Some(JsonPathQueryNode::ArrayIndexChild(i, child_node))),
                Token::WildcardChild() => Ok(Some(JsonPathQueryNode::AnyChild(child_node))),
                Token::Descendant(member) => Ok(Some(JsonPathQueryNode::Descendant(member, child_node))),
                Token::ArrayIndexDescendant(i) => Ok(Some(JsonPathQueryNode::ArrayIndexDescendant(i, child_node))),
                Token::WildcardDescendant() => Ok(Some(JsonPathQueryNode::AnyDescendant(child_node))),
            }
        }
        _ => Ok(None),
    }
}

pub(crate) trait Parser<'a, Out, Err = error::Error<&'a str>>:
    FnMut(&'a str) -> IResult<&'a str, Out, Err>
{
}

impl<'a, Out, Err, T: FnMut(&'a str) -> IResult<&'a str, Out, Err>> Parser<'a, Out, Err> for T {}

fn jsonpath<'a>() -> impl Parser<'a, (Option<Token>, Vec<Token>)> {
    pair(
        opt(map(char('$'), |_| Token::Root)), // root selector
        non_root(),
    )
}

fn non_root<'a>() -> impl Parser<'a, Vec<Token>> {
    many0(alt((
        wildcard_child_selector(),
        child_selector(),
        array_index_child_selector(),
        wildcard_descendant_selector(),
        descendant_selector(),
    )))
}

fn wildcard_child_selector<'a>() -> impl Parser<'a, Token> {
    map(alt((dot_wildcard_selector(), index_wildcard_selector())), |_| {
        Token::WildcardChild()
    })
}

fn child_selector<'a>() -> impl Parser<'a, Token> {
    map(alt((dot_selector(), index_selector())), Token::Child)
}

fn dot_selector<'a>() -> impl Parser<'a, JsonString> {
    preceded(char('.'), member())
}

fn dot_wildcard_selector<'a>() -> impl Parser<'a, char> {
    preceded(char('.'), char('*'))
}

fn descendant_selector<'a>() -> impl Parser<'a, Token> {
    preceded(
        tag(".."),
        alt((
            map(alt((member(), index_selector())), Token::Descendant),
            array_index_descendant_selector(),
        )),
    )
}

fn wildcard_descendant_selector<'a>() -> impl Parser<'a, Token> {
    map(preceded(tag(".."), alt((char('*'), index_wildcard_selector()))), |_| {
        Token::WildcardDescendant()
    })
}

fn index_selector<'a>() -> impl Parser<'a, JsonString> {
    delimited(char('['), quoted_member(), char(']'))
}

fn index_wildcard_selector<'a>() -> impl Parser<'a, char> {
    delimited(char('['), char('*'), char(']'))
}

fn member<'a>() -> impl Parser<'a, JsonString> {
    map(
        recognize(pair(member_first(), many0(member_character()))),
        JsonString::new,
    )
}

fn member_first<'a>() -> impl Parser<'a, char> {
    verify(anychar, |&x| x.is_alpha() || x == '_' || !x.is_ascii())
}

fn member_character<'a>() -> impl Parser<'a, char> {
    verify(anychar, |&x| x.is_alphanumeric() || x == '_' || !x.is_ascii())
}

fn array_index_child_selector<'a>() -> impl Parser<'a, Token> {
    map(array_index_selector(), Token::ArrayIndexChild)
}

fn array_index_descendant_selector<'a>() -> impl Parser<'a, Token> {
    map(array_index_selector(), Token::ArrayIndexDescendant)
}

fn array_index_selector<'a>() -> impl Parser<'a, JsonUInt> {
    delimited(char('['), nonnegative_array_index(), char(']'))
}

fn nonnegative_array_index<'a>() -> impl Parser<'a, JsonUInt> {
    map_res(parsed_array_index(), TryInto::try_into)
}

fn parsed_array_index<'a>() -> impl Parser<'a, JsonUInt> {
    map_res(digit1, str::parse)
}

fn quoted_member<'a>() -> impl Parser<'a, JsonString> {
    |s| {
        alt((
            delimited(char('"'), cut(JsonString::parse_double_quoted), char('"')),
            delimited(char('\''), cut(JsonString::parse_single_quoted), char('\'')),
        ))(s)
        .map_err(|x| match x {
            Err::Incomplete(_) => todo!(),
            Err::Error(e) => {
                //println!("wstawaj zesrałeś się {e}");
                Err::Error(nom::error::Error::new(s, error::ErrorKind::Alt))
            }
            Err::Failure(e) => {
                //println!("wstawaj zesrałeś się mocno {e}");
                Err::Error(nom::error::Error::new(s, error::ErrorKind::Alt))
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::parse_json_path_query;
    use crate::{str::JsonString, JsonPathQuery};
    use pretty_assertions::assert_eq;

    #[test]
    fn quoted_member() {
        let input = "'a'";

        let result = super::quoted_member()(input);

        assert_eq!(result, Ok(("", JsonString::new("a"))));
    }

    #[test]
    fn nonnegative_array_index() {
        let input = "[5]";

        let result = super::array_index_selector()(input);

        assert_eq!(result, Ok(("", 5_u64.try_into().unwrap())));
    }

    #[test]
    fn zero_array_index() {
        let input = "[0]";

        let result = super::array_index_selector()(input);

        assert_eq!(result, Ok(("", 0_u64.try_into().unwrap())));
    }

    #[test]
    fn negative_array_index() {
        let input = "[-5]";

        super::array_index_selector()(input).unwrap_err();
    }

    #[test]
    fn two_sixyfour_array_index() {
        let input = "[18446744073709551616]";

        super::array_index_selector()(input).unwrap_err();
    }

    #[test]
    fn two_sixyfour_plus_one_array_index() {
        let input = "[18446744073709551617]";

        super::array_index_selector()(input).unwrap_err();
    }

    #[test]
    fn two_pow_fiftythree_minus_one_array_index() {
        let input = "[9007199254740991]";

        let result = super::array_index_selector()(input);

        assert_eq!(result, Ok(("", 9_007_199_254_740_991_u64.try_into().unwrap())));
    }

    #[test]
    fn two_pow_fiftythree_index() {
        let input = "[9007199254740992]";

        super::array_index_selector()(input).unwrap_err();
    }

    #[test]
    fn should_infer_root_from_empty_string() {
        let input = "";
        let expected_query = JsonPathQuery::new(Box::new(crate::JsonPathQueryNode::Root(None)));

        let result = parse_json_path_query(input).expect("expected Ok");

        assert_eq!(result, expected_query);
    }

    #[test]
    fn root() {
        let input = "$";
        let expected_query = JsonPathQuery::new(Box::new(crate::JsonPathQueryNode::Root(None)));

        let result = parse_json_path_query(input).expect("expected Ok");

        assert_eq!(result, expected_query);
    }

    // This is a regression test. There was a bug where the error handling loop would try to resume
    // parsing at the next byte after an invalid character, which is invalid and causes a panic
    // if the character takes more than one byte - strings can be indexed only at char boundaries.
    #[test]
    fn error_handling_across_unicode_values() {
        // Ferris has 4 bytes of encoding.
        let input = "🦀.";

        let result = parse_json_path_query(input);

        assert!(result.is_err());
    }
}
