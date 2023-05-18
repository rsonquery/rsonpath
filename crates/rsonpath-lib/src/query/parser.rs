use super::error::{ArrayIndexError, ParseErrorReport, ParserError};
use crate::debug;
use crate::query::{
    JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label, NonNegativeArrayIndex,
};
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *,
};
use std::borrow::Borrow;
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
enum Token<'a> {
    Root,
    Child(LabelString<'a>),
    ArrayIndexChild(NonNegativeArrayIndex),
    WildcardChild(),
    Descendant(LabelString<'a>),
    ArrayIndexDescendant(NonNegativeArrayIndex),
    WildcardDescendant(),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum LabelString<'a> {
    Borrowed(&'a str),
    Owned(String),
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Root => write!(f, "$"),
            Token::Child(label) => write!(f, "['{label}']"),
            Token::ArrayIndexChild(i) => write!(f, "[{i}]"),
            Token::WildcardChild() => write!(f, "[*]"),
            Token::Descendant(label) => write!(f, "..['{label}']"),
            Token::WildcardDescendant() => write!(f, "..[*]"),
            Token::ArrayIndexDescendant(i) => write!(f, "..[{i}]"),
        }
    }
}

impl Display for LabelString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LabelString::Borrowed(label) => write!(f, "{label}"),
            LabelString::Owned(label) => write!(f, "{label}"),
        }
    }
}

impl<'a> Borrow<str> for LabelString<'a> {
    fn borrow(&self) -> &str {
        match self {
            LabelString::Borrowed(label) => label,
            LabelString::Owned(label) => label,
        }
    }
}

impl<'a> From<Option<String>> for LabelString<'a> {
    #[inline]
    fn from(value: Option<String>) -> Self {
        match value {
            Some(label) => LabelString::Owned(label),
            None => LabelString::Borrowed(""),
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
                    Label::new(label.borrow()),
                    child_node,
                ))),
                Token::ArrayIndexChild(i) => {
                    Ok(Some(JsonPathQueryNode::ArrayIndexChild(i, child_node)))
                }
                Token::WildcardChild() => Ok(Some(JsonPathQueryNode::AnyChild(child_node))),
                Token::Descendant(label) => Ok(Some(JsonPathQueryNode::Descendant(
                    Label::new(label.borrow()),
                    child_node,
                ))),
                Token::ArrayIndexDescendant(i) => {
                    Ok(Some(JsonPathQueryNode::ArrayIndexDescendant(i, child_node)))
                }
                Token::WildcardDescendant() => {
                    Ok(Some(JsonPathQueryNode::AnyDescendant(child_node)))
                }
            }
        }
        _ => Ok(None),
    }
}

trait Parser<'a, Out>: FnMut(&'a str) -> IResult<&'a str, Out> {}

impl<'a, Out, T: FnMut(&'a str) -> IResult<&'a str, Out>> Parser<'a, Out> for T {}

/// Helper type for parsers that might return a character that must be escaped
/// when initialized in a [`Label`]. For example, an unescaped double quote
/// must always be escaped in a label.
enum MaybeEscapedChar {
    Char(char),
    Escaped(char),
}

/// Helper wrapper for a Vec, needed to implement traits for it.
struct MaybeEscapedCharVec(Vec<MaybeEscapedChar>);

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
        array_index_child_selector(),
        wildcard_descendant_selector(),
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

fn dot_selector<'a>() -> impl Parser<'a, LabelString<'a>> {
    preceded(char('.'), label())
}

fn dot_wildcard_selector<'a>() -> impl Parser<'a, char> {
    preceded(char('.'), char('*'))
}

fn descendant_selector<'a>() -> impl Parser<'a, Token<'a>> {
    preceded(
        tag(".."),
        alt((
            map(alt((label(), index_selector())), Token::Descendant),
            array_index_descendant_selector(),
        )),
    )
}

fn wildcard_descendant_selector<'a>() -> impl Parser<'a, Token<'a>> {
    map(
        preceded(tag(".."), alt((char('*'), index_wildcard_selector()))),
        |_| Token::WildcardDescendant(),
    )
}

fn index_selector<'a>() -> impl Parser<'a, LabelString<'a>> {
    delimited(char('['), quoted_label(), char(']'))
}

fn index_wildcard_selector<'a>() -> impl Parser<'a, char> {
    delimited(char('['), char('*'), char(']'))
}

fn label<'a>() -> impl Parser<'a, LabelString<'a>> {
    map(
        recognize(pair(label_first(), many0(label_character()))),
        LabelString::Borrowed,
    )
}

fn label_first<'a>() -> impl Parser<'a, char> {
    verify(anychar, |&x| x.is_alpha() || x == '_' || !x.is_ascii())
}

fn label_character<'a>() -> impl Parser<'a, char> {
    verify(anychar, |&x| {
        x.is_alphanumeric() || x == '_' || !x.is_ascii()
    })
}

fn array_index_child_selector<'a>() -> impl Parser<'a, Token<'a>> {
    map(array_index_selector(), Token::ArrayIndexChild)
}

fn array_index_descendant_selector<'a>() -> impl Parser<'a, Token<'a>> {
    map(array_index_selector(), Token::ArrayIndexDescendant)
}

fn array_index_selector<'a>() -> impl Parser<'a, NonNegativeArrayIndex> {
    delimited(char('['), nonnegative_array_index(), char(']'))
}

fn nonnegative_array_index<'a>() -> impl Parser<'a, NonNegativeArrayIndex> {
    map_res(parsed_array_index(), TryInto::try_into)
}

fn parsed_array_index<'a>() -> impl Parser<'a, u64> {
    map_res(length_limited_array_index(), str::parse)
}

const ARRAY_INDEX_ULIMIT_BASE_10_DIGIT_COUNT: usize = NonNegativeArrayIndex::MAX.get_index().ilog10() as usize;
fn length_limited_array_index<'a>() -> impl Parser<'a, &'a str> {
    map_res(digit1, |cs: &str| {
        if cs.len() > (ARRAY_INDEX_ULIMIT_BASE_10_DIGIT_COUNT + 1) {
            Err(ArrayIndexError::ExceedsUpperLimitError(cs.to_owned()))
        } else {
            Ok(cs)
        }
    })
}

fn quoted_label<'a>() -> impl Parser<'a, LabelString<'a>> {
    alt((
        delimited(
            char('\''),
            map(opt(single_quoted_label()), LabelString::from),
            char('\''),
        ),
        delimited(
            char('"'),
            map(opt(double_quoted_label()), LabelString::from),
            char('"'),
        ),
    ))
}

fn single_quoted_label<'a>() -> impl Parser<'a, String> {
    escaped_transform(
        // If ['"'] is parsed, we want the label to be \", not ", since
        // in a valid JSON document the only way to represent a double quote in a label is with an escape.
        map(
            many1(alt((
                map(unescaped(), MaybeEscapedChar::Char),
                map(char('"'), MaybeEscapedChar::Escaped),
            ))),
            MaybeEscapedCharVec,
        ),
        '\\',
        alt((escaped(), value("'", tag("'")))),
    )
}

fn double_quoted_label<'a>() -> impl Parser<'a, String> {
    escaped_transform(
        recognize(many1(alt((unescaped(), char('\''))))),
        '\\',
        // If ["\""] is parsed the label must be \". Same reason as in single_quoted_label.
        alt((escaped(), value("\\\"", tag("\"")))),
    )
}

fn escaped<'a>() -> impl Parser<'a, &'a str> {
    alt((
        value("\\b", tag("b")),
        value("\\f", tag("f")),
        value("\\n", tag("n")),
        value("\\r", tag("r")),
        value("\\t", tag("t")),
        value("\\\\", tag("\\")),
        value("/", tag("/")),
    ))
}

fn unescaped<'a>() -> impl Parser<'a, char> {
    verify(none_of(r#"'"\"#), |&c| u32::from(c) >= 0x20)
}

// This impl is needed for nom `escaped_transform` to work with our `MaybeEscapedChar`.
// Logic is simple, we can extend a `String` with `MaybeEscapedChar` by appending
// either the raw char, or a backslash followed by the should-be-escaped char.
impl nom::ExtendInto for MaybeEscapedCharVec {
    type Item = char;

    type Extender = String;

    fn new_builder(&self) -> Self::Extender {
        String::new()
    }

    fn extend_into(&self, acc: &mut Self::Extender) {
        for maybe_escaped in &self.0 {
            match maybe_escaped {
                MaybeEscapedChar::Char(c) => acc.push(*c),
                MaybeEscapedChar::Escaped(c) => {
                    acc.push('\\');
                    acc.push(*c);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parse_json_path_query;
    use crate::query::{parser::LabelString, JsonPathQuery};
    use pretty_assertions::assert_eq;

    #[test]
    fn single_quoted_label() {
        let input = "a";

        let result = super::single_quoted_label()(input);

        assert_eq!(result, Ok(("", "a".to_owned())));
    }

    #[test]
    fn double_quoted_label() {
        let input = "a";

        let result = super::double_quoted_label()(input);

        assert_eq!(result, Ok(("", "a".to_owned())));
    }

    #[test]
    fn single_quoted_label_should_not_unescape_backslashes() {
        let input = r#"\\x"#;

        let result = super::single_quoted_label()(input);

        assert_eq!(result, Ok(("", r#"\\x"#.to_owned())));
    }

    #[test]
    fn double_quoted_label_should_not_unescape_backslashes() {
        let input = r#"\\x"#;

        let result = super::double_quoted_label()(input);

        assert_eq!(result, Ok(("", r#"\\x"#.to_owned())));
    }

    #[test]
    fn single_quoted_label_should_escape_double_quotes() {
        let input = r#"""#;

        let result = super::single_quoted_label()(input);

        assert_eq!(result, Ok(("", r#"\""#.to_owned())));
    }

    #[test]
    fn double_quoted_label_should_not_unescape_double_quotes() {
        let input = r#"\""#;

        let result = super::double_quoted_label()(input);

        assert_eq!(result, Ok(("", r#"\""#.to_owned())));
    }

    #[test]
    fn quoted_label() {
        let input = "'a'";

        let result = super::quoted_label()(input);

        assert_eq!(result, Ok(("", LabelString::Owned("a".to_string()))));
    }

    #[test]
    fn nonnegative_array_index() {
        let input = "[5]";

        let result = super::array_index_selector()(input);

        assert_eq!(result, Ok(("", 5.try_into().unwrap())));
    }

    #[test]
    fn zero_array_index() {
        let input = "[0]";

        let result = super::array_index_selector()(input);

        assert_eq!(result, Ok(("", 0.try_into().unwrap())));
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

        assert_eq!(result, Ok(("", 9_007_199_254_740_991.try_into().unwrap())));
    }

    #[test]
    fn two_pow_fiftythree_index() {
        let input = "[9007199254740992]";

        super::array_index_selector()(input).unwrap_err();
    }

    #[test]
    fn should_infer_root_from_empty_string() {
        let input = "";
        let expected_query =
            JsonPathQuery::new(Box::new(crate::query::JsonPathQueryNode::Root(None)));

        let result = parse_json_path_query(input).expect("expected Ok");

        assert_eq!(result, expected_query);
    }

    #[test]
    fn root() {
        let input = "$";
        let expected_query =
            JsonPathQuery::new(Box::new(crate::query::JsonPathQueryNode::Root(None)));

        let result = parse_json_path_query(input).expect("expected Ok");

        assert_eq!(result, expected_query);
    }
}
