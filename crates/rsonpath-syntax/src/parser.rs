use crate::{
    error::{InternalParseError, ParseErrorBuilder, SyntaxError, SyntaxErrorKind},
    num::{JsonInt, JsonUInt},
    str::{JsonString, JsonStringBuilder},
    Index, JsonPathQuery, Result, Segment, Selector, Selectors,
};
use nom::{branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *};
use std::{iter::Peekable, str::FromStr};

const WHITESPACE: [char; 4] = [' ', '\n', '\r', '\t'];

fn skip_whitespace(q: &str) -> &str {
    q.trim_start_matches(WHITESPACE)
}

fn skip_one(q: &str) -> &str {
    let mut chars = q.chars();
    chars.next();
    chars.as_str()
}

fn ignore_whitespace<'a, T, F, E>(mut inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, T, E>
where
    F: nom::Parser<&'a str, T, E>,
{
    move |q: &'a str| {
        inner
            .parse(skip_whitespace(q))
            .map(|(rest, res)| (skip_whitespace(rest), res))
    }
}

pub(crate) fn parse_json_path_query(q: &str) -> Result<JsonPathQuery> {
    let original_input = q;
    let mut parse_error = ParseErrorBuilder::new();
    let mut segments = vec![];
    let q = skip_whitespace(q);
    let q = match char::<_, nom::error::Error<_>>('$')(q).finish() {
        Ok((q, _)) => skip_whitespace(q),
        Err(e) => {
            parse_error.add(SyntaxError::new(
                SyntaxErrorKind::MissingRootIdentifier,
                e.input.len(),
                1,
            ));
            e.input
        }
    };

    let mut q = q;
    while !q.is_empty() {
        match segment(q).finish() {
            Ok((rest, segment)) => {
                segments.push(segment);
                q = skip_whitespace(rest)
            }
            Err(InternalParseError::SyntaxError(err, rest)) => {
                parse_error.add(err);
                q = rest;
            }
            Err(InternalParseError::SyntaxErrors(errs, rest)) => {
                parse_error.add_many(errs);
                q = rest;
            }
            Err(InternalParseError::NomError(err)) => panic!(
                "unexpected parser error; raw nom errors should never be produced; this is a bug\ncontext:\n{err}"
            ),
        }
    }

    if parse_error.is_empty() {
        Ok(JsonPathQuery { segments })
    } else {
        Err(parse_error.build(original_input.to_owned()))
    }
}

fn segment(q: &str) -> IResult<&str, Segment, InternalParseError> {
    // It's important to check descendant first, since we can always cut based on whether the prefix is ".." or not.
    alt((
        descendant_segment,
        child_segment,
        failed_segment(SyntaxErrorKind::InvalidSegmentAfterTwoPeriods),
    ))(q)
}

fn descendant_segment(q: &str) -> IResult<&str, Segment, InternalParseError> {
    map(
        preceded(
            tag(".."),
            cut(alt((
                bracketed_selection,
                map(wildcard_selector, Selectors::one),
                member_name_shorthand,
                failed_segment(SyntaxErrorKind::InvalidSegmentAfterTwoPeriods),
            ))),
        ),
        Segment::Descendant,
    )(q)
}

fn child_segment(q: &str) -> IResult<&str, Segment, InternalParseError> {
    map(
        alt((
            bracketed_selection,
            // This cut is only correct because we try parsing descendant_segment first.
            preceded(
                char('.'),
                cut(alt((
                    map(wildcard_selector, Selectors::one),
                    member_name_shorthand,
                    failed_segment(SyntaxErrorKind::InvalidNameShorthandAfterOnePeriod),
                ))),
            ),
        )),
        Segment::Child,
    )(q)
}

fn failed_segment<T>(kind: SyntaxErrorKind) -> impl FnMut(&str) -> IResult<&str, T, InternalParseError> {
    move |q: &str| {
        let rest = skip_one(q)
            .trim_start_matches('.')
            .trim_start_matches(|x| x != '.' && x != '[');
        Err(Err::Failure(InternalParseError::SyntaxError(
            SyntaxError::new(kind.clone(), q.len(), q.len() - rest.len()),
            rest,
        )))
    }
}

fn bracketed_selection(q: &str) -> IResult<&str, Selectors, InternalParseError> {
    let (mut q, _) = char('[')(q)?;
    let mut selectors = vec![];
    let mut syntax_errors = vec![];

    loop {
        match selector(q).finish() {
            Ok((rest, selector)) => {
                selectors.push(selector);
                q = rest;
            }
            Err(InternalParseError::SyntaxError(err, rest)) => {
                syntax_errors.push(err);
                q = rest;
            }
            Err(InternalParseError::SyntaxErrors(mut errs, rest)) => {
                syntax_errors.append(&mut errs);
                q = rest;
            }
            Err(err) => return Err(Err::Failure(err)),
        }

        match char::<_, nom::error::Error<_>>(',')(q) {
            Ok((rest, _)) => q = rest,
            Err(_) => {
                if let Ok((rest, _)) = char::<_, nom::error::Error<_>>(']')(q) {
                    q = rest;
                    break;
                } else if q.is_empty() {
                    syntax_errors.push(SyntaxError::new(SyntaxErrorKind::MissingClosingBracket, 0, 1));
                    break;
                } else {
                    syntax_errors.push(SyntaxError::new(SyntaxErrorKind::MissingSelectorSeparator, q.len(), 1))
                }
            }
        }
    }

    if syntax_errors.is_empty() {
        Ok((q, Selectors::many(selectors)))
    } else {
        Err(Err::Failure(InternalParseError::SyntaxErrors(syntax_errors, q)))
    }
}

fn member_name_shorthand(q: &str) -> IResult<&str, Selectors, InternalParseError> {
    return map(
        preceded(
            peek(name_first),
            fold_many0(name_char, JsonStringBuilder::new, |mut acc, x| {
                acc.push(x);
                acc
            }),
        ),
        |x| Selectors::one(Selector::Name(x.into())),
    )(q);

    fn name_first(q: &str) -> IResult<&str, char, InternalParseError> {
        satisfy(|x| x.is_ascii_alphabetic() || matches!(x, '_' | '\u{0080}'..='\u{D7FF}' | '\u{E000}'..='\u{10FFFF}'))(
            q,
        )
    }

    fn name_char(q: &str) -> IResult<&str, char, InternalParseError> {
        alt((name_first, satisfy(|x| x.is_ascii_digit())))(q)
    }
}

fn selector(q: &str) -> IResult<&str, Selector, InternalParseError> {
    alt((
        ignore_whitespace(name_selector),
        ignore_whitespace(wildcard_selector),
        ignore_whitespace(index_selector),
        failed_selector,
    ))(q)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringParseMode {
    DoubleQuoted,
    SingleQuoted,
}

fn name_selector(q: &str) -> IResult<&str, Selector, InternalParseError> {
    return map(
        alt((
            preceded(char('\''), string(StringParseMode::SingleQuoted)),
            preceded(char('"'), string(StringParseMode::DoubleQuoted)),
        )),
        Selector::Name,
    )(q);
}

fn wildcard_selector(q: &str) -> IResult<&str, Selector, InternalParseError> {
    map(tag("*"), |_| Selector::Wildcard)(q)
}

fn index_selector(q: &str) -> IResult<&str, Selector, InternalParseError> {
    let (rest, int) = int(q)?;
    match JsonInt::from_str(int) {
        Ok(int) => {
            if let Ok(uint) = JsonUInt::try_from(int) {
                Ok((rest, Selector::Index(Index::FromStart(uint))))
            } else {
                Ok((rest, Selector::Index(Index::FromEnd(int.abs()))))
            }
        }
        Err(err) => Err(Err::Failure(InternalParseError::SyntaxError(
            SyntaxError::new(SyntaxErrorKind::IndexParseError(err), q.len(), int.len()),
            rest,
        ))),
    }
}

fn failed_selector(q: &str) -> IResult<&str, Selector, InternalParseError> {
    let rest = q.trim_start_matches(|x| x != ',' && x != ']');
    let error_len = q.len() - rest.len();
    let error_span = &q[..error_len];

    Err(Err::Failure(InternalParseError::SyntaxError(
        if error_span.chars().all(|x| [' ', '\n', '\r', '\t'].contains(&x)) {
            SyntaxError::new(SyntaxErrorKind::EmptySelector, q.len() + 1, error_len + 2)
        } else {
            let meaningful_span = skip_whitespace(error_span);
            let skipped_whitespace_len = error_span.len() - meaningful_span.len();
            let trimmed_span = meaningful_span.trim_end_matches(WHITESPACE);

            SyntaxError::new(
                SyntaxErrorKind::InvalidSelector,
                q.len() - skipped_whitespace_len,
                trimmed_span.len(),
            )
        },
        rest,
    )))
}

fn int(q: &str) -> IResult<&str, &str, InternalParseError> {
    let (rest, int) = recognize(alt((preceded(char('-'), cut(digit1)), digit1)))(q)?;

    if int != "0" {
        if int == "-0" {
            return Err(Err::Failure(InternalParseError::SyntaxError(
                SyntaxError::new(SyntaxErrorKind::NegativeZeroInteger, q.len(), int.len()),
                rest,
            )));
        }
        let without_minus = int.strip_prefix('-').unwrap_or(int);
        if without_minus.strip_prefix(['0']).is_some() {
            return Err(Err::Failure(InternalParseError::SyntaxError(
                SyntaxError::new(SyntaxErrorKind::LeadingZeros, q.len(), int.len()),
                rest,
            )));
        }
    }

    Ok((rest, int))
}

fn string<'a>(mode: StringParseMode) -> impl FnMut(&'a str) -> IResult<&'a str, JsonString, InternalParseError> {
    move |q: &'a str| {
        let mut builder = JsonStringBuilder::new();
        let mut syntax_errors = vec![];
        let mut stream = q.char_indices().peekable();

        while let Some((c_idx, c)) = stream.next() {
            match (c, mode) {
                ('\\', _) => {
                    match read_escape_sequence(q.len(), c_idx, &mut stream, mode) {
                        Ok(r) => {
                            builder.push(r);
                        }
                        Err(err) => {
                            syntax_errors.push(err);
                        }
                    };
                }
                ('"', StringParseMode::DoubleQuoted) | ('\'', StringParseMode::SingleQuoted) => {
                    let rest = stream.next().map_or("", |(i, _)| &q[i..]);
                    return if syntax_errors.is_empty() {
                        Ok((rest, builder.finish()))
                    } else {
                        Err(nom::Err::Failure(InternalParseError::SyntaxErrors(syntax_errors, rest)))
                    };
                }
                (..='\u{0019}', _) => {
                    let rest = stream.peek().map_or("", |(i, _)| &q[*i..]);
                    syntax_errors.push(SyntaxError::new(
                        SyntaxErrorKind::InvalidUnescapedCharacter,
                        rest.len(),
                        1,
                    ))
                }
                _ => {
                    builder.push(c);
                }
            }
        }

        let err_kind = if mode == StringParseMode::SingleQuoted {
            SyntaxErrorKind::MissingClosingSingleQuote
        } else {
            SyntaxErrorKind::MissingClosingDoubleQuote
        };
        syntax_errors.push(SyntaxError::new(err_kind, 0, 1));
        return Err(nom::Err::Failure(InternalParseError::SyntaxErrors(syntax_errors, "")));

        fn read_escape_sequence<I>(
            q_len: usize,
            c_idx: usize,
            chars: &mut Peekable<I>,
            mode: StringParseMode,
        ) -> std::result::Result<char, SyntaxError>
        where
            I: Iterator<Item = (usize, char)>,
        {
            let (i, ctrl) = chars.next().ok_or(SyntaxError::new(
                SyntaxErrorKind::InvalidUnescapedCharacter,
                q_len - c_idx,
                1,
            ))?;
            match ctrl {
                'u' => {
                    let raw_c = read_hexadecimal_escape(q_len, i, chars)?;
                    match raw_c {
                        // High surrogate, start of a UTF-16 pair.
                        0xD800..=0xDBFF => {
                            let &(_, next) = chars.peek().ok_or(SyntaxError::new(
                                SyntaxErrorKind::UnpairedHighSurrogate,
                                q_len - c_idx,
                                6,
                            ))?;
                            if next != '\\' {
                                return Err(SyntaxError::new(
                                    SyntaxErrorKind::UnpairedHighSurrogate,
                                    q_len - c_idx,
                                    6,
                                ));
                            }
                            chars.next();
                            let (i, next) = chars.next().ok_or(SyntaxError::new(
                                SyntaxErrorKind::UnpairedHighSurrogate,
                                q_len - c_idx,
                                6,
                            ))?;
                            if next != 'u' {
                                return Err(SyntaxError::new(
                                    SyntaxErrorKind::UnpairedHighSurrogate,
                                    q_len - c_idx,
                                    6,
                                ));
                            }
                            let low = read_hexadecimal_escape(q_len, i, chars)?;
                            match low {
                                0xDC00..=0xDFFF => {
                                    let n = ((raw_c - 0xD800) << 10 | (low - 0xDC00)) + 0x10000;
                                    Ok(char::from_u32(n).expect("high and low surrogate pair is always a valid char"))
                                }
                                _ => Err(SyntaxError::new(
                                    SyntaxErrorKind::UnpairedHighSurrogate,
                                    q_len - c_idx,
                                    6,
                                )),
                            }
                        }
                        // Low surrogate, invalid escape sequence.
                        0xDC00..=0xDFFF => Err(SyntaxError::new(
                            SyntaxErrorKind::UnpairedLowSurrogate,
                            q_len - c_idx,
                            6,
                        )),
                        _ => Ok(char::from_u32(raw_c).expect("invalid values are handled above")),
                    }
                }
                'b' => Ok('\u{0008}'), // U+0008 BS backspace
                't' => Ok('\t'),       // U+0009 HT horizontal tab
                'n' => Ok('\n'),       // U+000A LF line feed
                'f' => Ok('\u{000C}'), // U+000C FF form feed
                'r' => Ok('\r'),       // U+000D CR carriage return
                '"' if mode == StringParseMode::DoubleQuoted => Ok(ctrl),
                '\'' if mode == StringParseMode::SingleQuoted => Ok(ctrl),
                '/' | '\\' => Ok(ctrl), // " ' / \ are passed as is
                _ => Err(SyntaxError::new(
                    SyntaxErrorKind::InvalidEscapeSequence,
                    q_len - c_idx,
                    2,
                )), // no other escape sequences are allowed
            }
        }

        fn read_hexadecimal_escape<I>(
            q_len: usize,
            c_idx: usize,
            chars: &mut Peekable<I>,
        ) -> std::result::Result<u32, SyntaxError>
        where
            I: Iterator<Item = (usize, char)>,
        {
            let mut x = 0;
            for i in 0..4 {
                let &(_, c) = chars.peek().ok_or(SyntaxError::new(
                    SyntaxErrorKind::InvalidEscapeSequence,
                    q_len - c_idx + 1,
                    2 + i,
                ))?;
                let v = match c {
                    '0'..='9' => c as u32 - '0' as u32,
                    // RFC8259.7-2 The hexadecimal letters A through F can be uppercase or lowercase.
                    'a'..='f' => c as u32 - 'a' as u32 + 10,
                    'A'..='F' => c as u32 - 'A' as u32 + 10,
                    _ => {
                        return Err(SyntaxError::new(
                            SyntaxErrorKind::InvalidHexDigitInUnicodeEscape,
                            q_len - c_idx - i - 1,
                            1,
                        ))
                    }
                };
                x <<= 4;
                x += v;
                chars.next();
            }
            Ok(x)
        }
    }
}

/*
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
        let expected_query = todo!();

        let result = parse_json_path_query(input).expect("expected Ok");

        assert_eq!(result, expected_query);
    }

    #[test]
    fn root() {
        let input = "$";
        let expected_query = todo!();

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
*/
