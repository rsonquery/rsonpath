use crate::{
    error::{InternalParseError, ParseErrorBuilder, SyntaxError, SyntaxErrorKind},
    num::{error::JsonIntParseError, JsonFloat, JsonInt, JsonNonZeroUInt, JsonNumber, JsonUInt},
    str::{JsonString, JsonStringBuilder},
    Comparable, ComparisonExpr, ComparisonOp, Index, JsonPathQuery, Literal, LogicalExpr, ParserOptions, Result,
    Segment, Selector, Selectors, Step, TestExpr, JSONPATH_WHITESPACE,
};
use nom::{branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*, *};
use std::{iter::Peekable, str::FromStr as _};

fn skip_whitespace(q: &str) -> &str {
    q.trim_start_matches(JSONPATH_WHITESPACE)
}

fn skip_one(q: &str) -> &str {
    let mut chars = q.chars();
    chars.next();
    chars.as_str()
}

fn ignore_whitespace<'a, T, F, E>(mut inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, T, E>
where
    F: nom::Parser<&'a str, Output = T, Error = E>,
{
    move |q: &'a str| {
        inner
            .parse(skip_whitespace(q))
            .map(|(rest, res)| (skip_whitespace(rest), res))
    }
}

// This context gets copied into parser functions that require access to the options.
// We also carry the current nesting depth to terminate recursion when excessive
// filter nesting is present.
#[derive(Debug, Clone, Copy)]
struct ParseCtx<'a> {
    options: &'a ParserOptions,
    current_nesting: usize,
}

impl<'a> ParseCtx<'a> {
    fn new(options: &'a ParserOptions) -> Self {
        Self {
            options,
            current_nesting: 0,
        }
    }

    fn increase_nesting(&self) -> Option<Self> {
        match self.options.recursion_limit {
            Some(limit) if limit <= self.current_nesting => None,
            _ => Some(Self {
                options: self.options,
                current_nesting: self.current_nesting + 1,
            }),
        }
    }
}

pub(crate) fn parse_with_options(q: &str, options: &ParserOptions) -> Result<JsonPathQuery> {
    parse_json_path_query(q, ParseCtx::new(options))
}

fn parse_json_path_query(q: &str, ctx: ParseCtx) -> Result<JsonPathQuery> {
    let original_input = q;
    let mut parse_error = ParseErrorBuilder::new();
    let mut segments = vec![];
    let q = skip_whitespace(q);
    let leading_whitespace_len = original_input.len() - q.len();
    if leading_whitespace_len > 0 && !ctx.options.is_leading_whitespace_allowed() {
        parse_error.add(SyntaxError::new(
            SyntaxErrorKind::DisallowedLeadingWhitespace,
            original_input.len(),
            leading_whitespace_len,
        ));
    }
    let q = match char::<_, nom::error::Error<_>>('$')(q).finish() {
        Ok((q, _)) => skip_whitespace(q),
        Err(e) => {
            parse_error.add(SyntaxError::new(
                SyntaxErrorKind::MissingRootIdentifier,
                e.input.len(),
                q.chars().next().map_or(1, char::len_utf8),
            ));
            e.input
        }
    };

    let mut q = q;
    while !q.is_empty() {
        q = match segment(q, ctx).finish() {
            Ok((rest, segment)) => {
                segments.push(segment);
                rest
            }
            Err(InternalParseError::SyntaxError(err, rest)) => {
                parse_error.add(err);
                rest
            }
            Err(InternalParseError::SyntaxErrors(errs, rest)) => {
                parse_error.add_many(errs);
                rest
            }
            Err(InternalParseError::RecursionLimitExceeded) => {
                return Err(ParseErrorBuilder::recursion_limit_exceeded(
                    original_input.to_owned(),
                    ctx.options
                        .recursion_limit
                        .expect("recursion limit should exists when exceeded"),
                ));
            }
            Err(InternalParseError::NomError(err)) => panic!(
                "unexpected parser error; raw nom errors should never be produced; this is a bug\ncontext:\n{err}"
            ),
        };
        q = skip_whitespace(q);
    }

    // For strict RFC compliance trailing whitespace has to be disallowed.
    // This is hard to organically obtain from the parsing above, so we insert this awkward direct check if needed.
    if !ctx.options.is_trailing_whitespace_allowed() {
        let trimmed = original_input.trim_end_matches(JSONPATH_WHITESPACE);
        let trailing_whitespace_len = original_input.len() - trimmed.len();
        if trailing_whitespace_len > 0 {
            parse_error.add(SyntaxError::new(
                SyntaxErrorKind::DisallowedTrailingWhitespace,
                trailing_whitespace_len,
                trailing_whitespace_len,
            ));
        }
    }

    if parse_error.is_empty() {
        Ok(JsonPathQuery { segments })
    } else {
        Err(parse_error.build(original_input.to_owned()))
    }
}

fn segment<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Segment, InternalParseError<'q>> {
    // It's important to check descendant first, since we can always cut based on whether the prefix is ".." or not.
    alt((
        |q| descendant_segment(q, ctx),
        |q| child_segment(q, ctx),
        failed_segment(SyntaxErrorKind::InvalidSegmentStart),
    ))
    .parse(q)
}

fn descendant_segment<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Segment, InternalParseError<'q>> {
    map(
        preceded(
            tag(".."),
            cut(alt((
                |q| bracketed_selection(q, ctx),
                map(wildcard_selector, Selectors::one),
                member_name_shorthand,
                failed_segment(SyntaxErrorKind::InvalidSegmentAfterTwoPeriods),
            ))),
        ),
        Segment::Descendant,
    )
    .parse(q)
}

fn child_segment<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Segment, InternalParseError<'q>> {
    map(
        alt((
            |q| bracketed_selection(q, ctx),
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
    )
    .parse(q)
}

fn failed_segment<T>(kind: SyntaxErrorKind) -> impl FnMut(&str) -> IResult<&str, T, InternalParseError<'_>> {
    move |q: &str| {
        // Special case when the error span starts with a bunch of periods.
        // The best error message to display is to skip all those periods and try to recover after.
        // This handles cases like e.g. `$....a`, where we will highlight `..a` as the error and
        // try to continue from there. We also handle this as a separate suggestion to remove extraneous periods.
        let rest = if q.starts_with('.') {
            q.trim_start_matches('.')
        } else {
            skip_one(q)
        }
        .trim_start_matches(|x| x != '.' && x != '[');
        // Don't highlight leading whitespace in an error.
        // This logic is duplicated with failed_selector, but I didn't find a way to extract this logic that wouldn't
        // make it actually *harder* to follow wtf is happening in the code, so...
        let error_len = q.len() - rest.len();
        let error_span = &q[..error_len];
        if error_span.chars().all(|x| [' ', '\n', '\r', '\t'].contains(&x)) {
            // Special case for a completely empty selector where we don't want to ignore whitespace.
            fail(SyntaxErrorKind::EmptySelector, q.len() + 1, error_len + 2, rest)
        } else {
            // Don't highlight leading whitespace in an error.
            let meaningful_span = skip_whitespace(error_span);
            let skipped_whitespace_len = error_span.len() - meaningful_span.len();
            let trimmed_span = meaningful_span.trim_end_matches(JSONPATH_WHITESPACE);
            fail(kind.clone(), q.len() - skipped_whitespace_len, trimmed_span.len(), rest)
        }
    }
}

fn bracketed_selection<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Selectors, InternalParseError<'q>> {
    let (mut q, _) = char('[')(q)?;
    let mut selectors = vec![];
    let mut syntax_errors = vec![];

    loop {
        match selector(q, ctx).finish() {
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
        q = skip_whitespace(q);

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

fn member_name_shorthand(q: &str) -> IResult<&str, Selectors, InternalParseError<'_>> {
    return map(
        preceded(
            peek(name_first),
            fold_many0(name_char, JsonStringBuilder::new, |mut acc, x| {
                acc.push(x);
                acc
            }),
        ),
        |x| Selectors::one(Selector::Name(x.into())),
    )
    .parse(q);

    fn name_first(q: &str) -> IResult<&str, char, InternalParseError<'_>> {
        satisfy(|x| x.is_ascii_alphabetic() || matches!(x, '_' | '\u{0080}'..='\u{D7FF}' | '\u{E000}'..='\u{10FFFF}'))(
            q,
        )
    }

    fn name_char(q: &str) -> IResult<&str, char, InternalParseError<'_>> {
        alt((name_first, satisfy(|x| x.is_ascii_digit()))).parse(q)
    }
}

fn selector<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Selector, InternalParseError<'q>> {
    alt((
        ignore_whitespace(name_selector),
        ignore_whitespace(wildcard_selector),
        ignore_whitespace(slice_selector),
        ignore_whitespace(index_selector),
        ignore_whitespace(|q| filter_selector(q, ctx)),
        failed_selector,
    ))
    .parse(q)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringParseMode {
    DoubleQuoted,
    SingleQuoted,
}

fn name_selector(q: &str) -> IResult<&str, Selector, InternalParseError<'_>> {
    map(string_literal, Selector::Name).parse(q)
}

fn string_literal(q: &str) -> IResult<&str, JsonString, InternalParseError<'_>> {
    alt((
        preceded(char('\''), string(StringParseMode::SingleQuoted)),
        preceded(char('"'), string(StringParseMode::DoubleQuoted)),
    ))
    .parse(q)
}

fn wildcard_selector(q: &str) -> IResult<&str, Selector, InternalParseError<'_>> {
    map(tag("*"), |_| Selector::Wildcard).parse(q)
}

fn slice_selector(q: &str) -> IResult<&str, Selector, InternalParseError<'_>> {
    let (rest, opt_start) = terminated(opt(int), ignore_whitespace(char(':'))).parse(q)?;
    // We have parsed a ':', so this *must* be a slice selector. Any errors after here are fatal.
    let mut slice = crate::Slice::default();

    if let Some(start_str) = opt_start {
        match parse_directional_int(start_str) {
            DirectionalInt::Plus(int) => slice.start = Index::FromStart(int),
            DirectionalInt::Minus(int) => slice.start = Index::FromEnd(int),
            DirectionalInt::Error(err) => {
                return fail(
                    SyntaxErrorKind::SliceStartParseError(err),
                    q.len(),
                    start_str.len(),
                    rest,
                );
            }
        }
    }
    let q = rest;
    let (rest, opt_end) = opt(ignore_whitespace(int)).parse(q)?;

    if let Some(end_str) = opt_end {
        match parse_directional_int(end_str) {
            DirectionalInt::Plus(int) => slice.end = Some(Index::FromStart(int)),
            DirectionalInt::Minus(int) => slice.end = Some(Index::FromEnd(int)),
            DirectionalInt::Error(err) => {
                return fail(SyntaxErrorKind::SliceEndParseError(err), q.len(), end_str.len(), rest);
            }
        }
    }

    let q = rest;
    let (rest, opt_step) = opt(ignore_whitespace(preceded(char(':'), opt(ignore_whitespace(int))))).parse(q)?;

    if let Some(Some(step_str)) = opt_step {
        match parse_directional_int(step_str) {
            DirectionalInt::Plus(int) => slice.step = Step::Forward(int),
            DirectionalInt::Minus(int) => slice.step = Step::Backward(int),
            DirectionalInt::Error(err) => {
                return fail(SyntaxErrorKind::SliceStepParseError(err), q.len(), step_str.len(), rest);
            }
        }
    }

    // Fixup the bounds - if start was not given and step is negative, the default must be reversed.
    if slice.step.is_backward() && opt_start.is_none() {
        slice.start = crate::Slice::default_start_backwards();
    }

    Ok((rest, Selector::Slice(slice)))
}

fn index_selector(q: &str) -> IResult<&str, Selector, InternalParseError<'_>> {
    // This has to be called after the slice selector.
    // Thanks to that we can make a hard cut if we parsed an integer but it doesn't work as an index.
    let (rest, int) = int(q)?;
    match parse_directional_int(int) {
        DirectionalInt::Plus(int) => Ok((rest, Selector::Index(Index::FromStart(int)))),
        DirectionalInt::Minus(int) => Ok((rest, Selector::Index(Index::FromEnd(int)))),
        DirectionalInt::Error(err) => Err(Err::Failure(InternalParseError::SyntaxError(
            SyntaxError::new(SyntaxErrorKind::IndexParseError(err), q.len(), int.len()),
            rest,
        ))),
    }
}

fn failed_selector(q: &str) -> IResult<&str, Selector, InternalParseError<'_>> {
    let rest = q.trim_start_matches(|x| x != ',' && x != ']');
    let error_len = q.len() - rest.len();
    let error_span = &q[..error_len];

    if error_span.chars().all(|x| [' ', '\n', '\r', '\t'].contains(&x)) {
        // Special case for a completely empty selector where we don't want to ignore whitespace.
        fail(SyntaxErrorKind::EmptySelector, q.len() + 1, error_len + 2, rest)
    } else {
        // Don't highlight leading whitespace in an error.
        let meaningful_span = skip_whitespace(error_span);
        let skipped_whitespace_len = error_span.len() - meaningful_span.len();
        let trimmed_span = meaningful_span.trim_end_matches(JSONPATH_WHITESPACE);
        fail(
            SyntaxErrorKind::InvalidSelector,
            q.len() - skipped_whitespace_len,
            trimmed_span.len(),
            rest,
        )
    }
}

fn filter_selector<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Selector, InternalParseError<'q>> {
    into(preceded(char('?'), ignore_whitespace(|q| logical_expr(q, ctx)))).parse(q)
}

fn logical_expr<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, LogicalExpr, InternalParseError<'q>> {
    // This is the most involved part of the parser, as it is inherently recursive.
    //
    // There are two sources of recursion here: parentheses introduce recursion,
    // since the rule is simply '(' filter_expression ')'; and the boolean combinations
    // require checking for an operator, and if any is present recursively parsing
    // another filter and wrapping the result in an appropriate node type.
    //
    // In total, we handle the negation operator at the start and then apply the rules:
    // - '(' |=> filter_expression, ')'
    // - literal |=> comp_op, comparable
    // - query, comp_op |=> comparable
    // - query
    // where |=> means a cut. We separately apply two additional restrictions:
    // - negation cannot immediately precede a comparison,
    // - query in a comparison must be singular.
    // It would be possible to directly disallow them by the rules, but if the parser understands
    // these two special cases it can give much clearer error messages about them.
    //
    // At the end, we check for `&&` and `||``, recurse and wrap if needed; if not, we end parsing
    // and leave the rest to the parsers higher up the stack. They might accept the next
    // character (e.g. it's `)` called from a recursive filter call, `,` chaining selectors,
    // `]` ending a segment...) and are responsible for error handling otherwise.
    #[derive(Debug, Clone, Copy)]
    enum BooleanOp {
        And,
        Or,
    }

    let Some(ctx) = ctx.increase_nesting() else {
        return Err(Err::Failure(InternalParseError::RecursionLimitExceeded));
    };

    let (rest, this_expr) = ignore_whitespace(|q| parse_single(q, ctx))(q)?;
    let mut loop_rest = skip_whitespace(rest);
    let mut final_expr = this_expr;

    loop {
        let (rest, mb_boolean_op) = opt(ignore_whitespace(alt((
            value(BooleanOp::And, tag("&&")),
            value(BooleanOp::Or, tag("||")),
        ))))
        .parse(loop_rest)?;
        loop_rest = rest;

        match mb_boolean_op {
            Some(BooleanOp::And) => {
                let (rest, rhs_expr) = ignore_whitespace(|q| parse_single(q, ctx))(loop_rest)?;
                loop_rest = rest;
                final_expr = LogicalExpr::And(Box::new(final_expr), Box::new(rhs_expr));
            }
            Some(BooleanOp::Or) => {
                let (rest, rhs_expr) = ignore_whitespace(|q| logical_expr(q, ctx))(loop_rest)?;
                loop_rest = rest;
                final_expr = LogicalExpr::Or(Box::new(final_expr), Box::new(rhs_expr));
            }
            None => break,
        }
    }

    return Ok((loop_rest, final_expr));

    fn parse_single<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, LogicalExpr, InternalParseError<'q>> {
        let (rest, opt_neg) = ignore_whitespace(opt(char('!')))(q)?;
        let negated = opt_neg.is_some();
        if let Ok((rest, _)) = char::<_, ()>('(')(rest) {
            let (rest, nested_filter) = cut(|q| logical_expr(q, ctx)).parse(skip_whitespace(rest))?;
            let rest = skip_whitespace(rest);
            let Ok((rest, _)) = char::<_, ()>(')')(rest) else {
                return failed_filter_expression(SyntaxErrorKind::MissingClosingParenthesis)(rest);
            };
            let selector = if negated {
                LogicalExpr::Not(Box::new(nested_filter))
            } else {
                nested_filter
            };
            return Ok((rest, selector));
        }

        match literal(rest) {
            Ok((rest, lhs)) => {
                let rest = skip_whitespace(rest);
                let (rest, comp_op) = match comparison_operator(rest) {
                    Ok((rest, comp_op)) => (rest, comp_op),
                    Err(Err::Failure(err)) => return Err(Err::Failure(err)),
                    _ => {
                        if peek(char::<_, ()>(']')).parse(rest).is_ok() {
                            return fail(SyntaxErrorKind::MissingComparisonOperator, rest.len(), 1, rest);
                        } else {
                            return failed_filter_expression(SyntaxErrorKind::InvalidComparisonOperator)(rest);
                        };
                    }
                };
                let rest = skip_whitespace(rest);
                let (rest, rhs) = comparable(rest, ctx)?;
                if negated {
                    return fail(SyntaxErrorKind::InvalidNegation, q.len(), 1, rest);
                } else {
                    return Ok((
                        rest,
                        LogicalExpr::Comparison(ComparisonExpr {
                            lhs: Comparable::Literal(lhs),
                            op: comp_op,
                            rhs,
                        }),
                    ));
                }
            }
            Err(Err::Failure(err)) => return Err(Err::Failure(err)),
            _ => (),
        }

        match filter_query(rest, ctx) {
            Ok((rest, query)) => {
                let query_len = q.len() - rest.len();
                let rest = skip_whitespace(rest);
                if let Ok((rest, comp_op)) = comparison_operator(rest) {
                    let rest = skip_whitespace(rest);
                    let (rest, rhs) = comparable(rest, ctx)?;
                    let Some(singular_query) = query.try_to_comparable() else {
                        return fail(SyntaxErrorKind::NonSingularQueryInComparison, q.len(), query_len, rest);
                    };
                    if negated {
                        fail(SyntaxErrorKind::InvalidNegation, q.len(), 1, rest)
                    } else {
                        Ok((
                            rest,
                            LogicalExpr::Comparison(ComparisonExpr {
                                lhs: singular_query,
                                rhs,
                                op: comp_op,
                            }),
                        ))
                    }
                } else {
                    let test_expr = LogicalExpr::Test(query.into_test_query());
                    let expr = if negated {
                        LogicalExpr::Not(Box::new(test_expr))
                    } else {
                        test_expr
                    };
                    Ok((rest, expr))
                }
            }
            Err(Err::Failure(err)) => Err(Err::Failure(err)),
            _ => failed_filter_expression(SyntaxErrorKind::InvalidFilter)(rest),
        }
    }
}

enum FilterQuery {
    Relative(JsonPathQuery),
    Absolute(JsonPathQuery),
}

#[derive(Clone, Copy)]
enum RootSelectorType {
    Relative,
    Absolute,
}

impl FilterQuery {
    fn into_test_query(self) -> TestExpr {
        match self {
            Self::Relative(q) => TestExpr::Relative(q),
            Self::Absolute(q) => TestExpr::Absolute(q),
        }
    }

    fn try_to_comparable(self) -> Option<Comparable> {
        match self {
            Self::Relative(q) => q.try_to_singular().ok().map(Comparable::RelativeSingularQuery),
            Self::Absolute(q) => q.try_to_singular().ok().map(Comparable::AbsoluteSingularQuery),
        }
    }
}

fn filter_query<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, FilterQuery, InternalParseError<'q>> {
    let (rest, root_type) = alt((
        value(RootSelectorType::Absolute, char('$')),
        value(RootSelectorType::Relative, char('@')),
    ))
    .parse(q)?;
    let rest = skip_whitespace(rest);
    let mut segments = vec![];
    let mut syntax_errors = vec![];

    let mut q = rest;

    loop {
        if peek(one_of::<_, _, ()>(".[")).parse(q).is_err() {
            break;
        }

        q = match alt((
            |q| descendant_segment(q, ctx),
            |q| child_segment(q, ctx),
            failed_segment_within_filter(SyntaxErrorKind::InvalidSegmentStart),
        ))
        .parse(q)
        .finish()
        {
            Ok((rest, segment)) => {
                segments.push(segment);
                rest
            }
            Err(InternalParseError::SyntaxError(err, rest)) => {
                syntax_errors.push(err);
                rest
            }
            Err(InternalParseError::SyntaxErrors(mut errs, rest)) => {
                syntax_errors.append(&mut errs);
                rest
            }
            Err(InternalParseError::RecursionLimitExceeded) => {
                return Err(Err::Failure(InternalParseError::RecursionLimitExceeded));
            }
            Err(InternalParseError::NomError(err)) => panic!(
                "unexpected parser error; raw nom errors should never be produced; this is a bug\ncontext:\n{err}"
            ),
        };
        q = skip_whitespace(q);
    }

    if !syntax_errors.is_empty() {
        Err(Err::Failure(InternalParseError::SyntaxErrors(syntax_errors, q)))
    } else {
        let query = JsonPathQuery { segments };
        let query = match root_type {
            RootSelectorType::Relative => FilterQuery::Relative(query),
            RootSelectorType::Absolute => FilterQuery::Absolute(query),
        };
        Ok((q, query))
    }
}

fn failed_segment_within_filter<T>(
    kind: SyntaxErrorKind,
) -> impl FnMut(&str) -> IResult<&str, T, InternalParseError<'_>> {
    move |q: &str| {
        // We want to find the next segment or close the filter.
        let rest = skip_one(q)
            .trim_start_matches('.')
            .trim_start_matches(|x| x != ',' && x != ']' && x != '.' && x != '[');
        fail(kind.clone(), q.len(), q.len() - rest.len(), rest)
    }
}

fn failed_filter_expression<T>(kind: SyntaxErrorKind) -> impl FnMut(&str) -> IResult<&str, T, InternalParseError<'_>> {
    move |q: &str| {
        // We want to close the filter, so just try to find the next ']' or ','
        let rest = skip_one(q).trim_start_matches(|x| x != ',' && x != ']');
        fail(kind.clone(), q.len(), q.len() - rest.len(), rest)
    }
}

fn comparison_operator(q: &str) -> IResult<&str, ComparisonOp, InternalParseError<'_>> {
    alt((
        value(ComparisonOp::EqualTo, tag("==")),
        value(ComparisonOp::NotEqualTo, tag("!=")),
        value(ComparisonOp::LesserOrEqualTo, tag("<=")),
        value(ComparisonOp::GreaterOrEqualTo, tag(">=")),
        value(ComparisonOp::LessThan, char('<')),
        value(ComparisonOp::GreaterThan, char('>')),
    ))
    .parse(q)
}

fn comparable<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Comparable, InternalParseError<'q>> {
    return alt((
        into(literal),
        |q| singular_query(q, ctx),
        failed_filter_expression(SyntaxErrorKind::InvalidComparable),
    ))
    .parse(q);

    fn singular_query<'q>(q: &'q str, ctx: ParseCtx) -> IResult<&'q str, Comparable, InternalParseError<'q>> {
        let (rest, query) = filter_query(q, ctx)?;
        let Some(cmp) = query.try_to_comparable() else {
            let query_len = q.len() - rest.len();
            return fail(SyntaxErrorKind::NonSingularQueryInComparison, q.len(), query_len, rest);
        };
        Ok((rest, cmp))
    }
}

fn literal(q: &str) -> IResult<&str, Literal, InternalParseError<'_>> {
    alt((
        into(number),
        into(string_literal),
        value(Literal::Bool(true), tag("true")),
        value(Literal::Bool(false), tag("false")),
        value(Literal::Null, tag("null")),
    ))
    .parse(q)
}

fn number(q: &str) -> IResult<&str, JsonNumber, InternalParseError<'_>> {
    map(float, |f| JsonNumber::from(f).normalize()).parse(q)
}

// Exported for JsonFloat::from_str
fn float(q: &str) -> IResult<&str, JsonFloat, InternalParseError<'_>> {
    // Look ahead to verify that this has a chance to be a number.
    let (rest, valid_str) = recognize(alt((preceded(char('-'), base_float), base_float))).parse(q)?;

    // It is a number, so after here we can hard cut.
    return match JsonFloat::from_str(valid_str) {
        Ok(n) => Ok((rest, n)),
        Err(e) => fail(SyntaxErrorKind::NumberParseError(e), rest.len(), valid_str.len(), q),
    };

    fn base_float(q: &str) -> IResult<&str, &str, InternalParseError<'_>> {
        recognize((
            digit1,
            opt(preceded(char('.'), digit1)),
            opt(preceded(
                tag_no_case("e"),
                preceded(opt(alt((char('+'), char('-')))), digit1),
            )),
        ))
        .parse(q)
    }
}

enum DirectionalInt {
    Plus(JsonUInt),
    Minus(JsonNonZeroUInt),
    Error(JsonIntParseError),
}

fn parse_directional_int(int_str: &str) -> DirectionalInt {
    match JsonInt::from_str(int_str) {
        Ok(int) => {
            if let Ok(uint) = JsonUInt::try_from(int) {
                DirectionalInt::Plus(uint)
            } else {
                DirectionalInt::Minus(int.abs().try_into().expect("zero would convert to JsonUInt above"))
            }
        }
        Err(err) => DirectionalInt::Error(err),
    }
}

fn int(q: &str) -> IResult<&str, &str, InternalParseError<'_>> {
    let (rest, int) = recognize(alt((preceded(char('-'), digit1), digit1))).parse(q)?;

    if int != "0" {
        if int == "-0" {
            return fail(SyntaxErrorKind::NegativeZeroInteger, q.len(), int.len(), rest);
        }
        let without_minus = int.strip_prefix('-').unwrap_or(int);
        if without_minus.strip_prefix(['0']).is_some() {
            return fail(SyntaxErrorKind::LeadingZeros, q.len(), int.len(), rest);
        }
    }

    Ok((rest, int))
}

fn string(mode: StringParseMode) -> impl FnMut(&str) -> IResult<&str, JsonString, InternalParseError<'_>> {
    move |q: &str| {
        let mut builder = JsonStringBuilder::new();
        let mut syntax_errors = vec![];
        let mut stream = q.char_indices().peekable();

        while let Some((c_idx, c)) = stream.next() {
            match (c, mode) {
                ('\\', _) => match read_escape_sequence(q.len(), c_idx, &mut stream, mode) {
                    Ok(r) => {
                        builder.push(r);
                    }
                    Err(err) => {
                        syntax_errors.push(err);
                    }
                },
                ('"', StringParseMode::DoubleQuoted) | ('\'', StringParseMode::SingleQuoted) => {
                    let rest = stream.next().map_or("", |(i, _)| &q[i..]);
                    return if syntax_errors.is_empty() {
                        Ok((rest, builder.finish()))
                    } else {
                        Err(nom::Err::Failure(InternalParseError::SyntaxErrors(syntax_errors, rest)))
                    };
                }
                (..='\u{001F}', _) => {
                    let rest = stream.peek().map_or("", |(i, _)| &q[*i..]);
                    syntax_errors.push(SyntaxError::new(
                        SyntaxErrorKind::InvalidUnescapedCharacter,
                        rest.len() + 1,
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
                                    let n = (((raw_c - 0xD800) << 10) | (low - 0xDC00)) + 0x10000;
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

fn fail<T>(
    kind: SyntaxErrorKind,
    rev_idx: usize,
    err_len: usize,
    rest: &str,
) -> IResult<&str, T, InternalParseError<'_>> {
    Err(Err::Failure(InternalParseError::SyntaxError(
        SyntaxError::new(kind, rev_idx, err_len),
        rest,
    )))
}

#[cfg(test)]
mod tests {
    use crate::{
        num::{JsonFloat, JsonInt, JsonNumber},
        str::JsonString,
        Comparable, ComparisonExpr, ComparisonOp, Index, Literal, LogicalExpr, ParserOptions, Selector,
        SingularJsonPathQuery, SingularSegment, Step,
    };
    use test_case::test_case;

    #[test]
    fn name_selector() {
        let input = "'a'";
        let result = super::name_selector(input).expect("should parse");

        assert_eq!(result, ("", Selector::Name(JsonString::new("a"))));
    }

    #[test]
    fn wildcard_selector() {
        let input = "*";
        let result = super::wildcard_selector(input).expect("should parse");

        assert_eq!(result, ("", Selector::Wildcard));
    }

    #[test]
    fn nonnegative_array_index() {
        let input = "5";
        let result = super::index_selector(input).expect("should parse");

        assert_eq!(result, ("", Selector::Index(Index::FromStart(5.into()))));
    }

    #[test]
    fn negative_array_index() {
        let input = "-5";
        let result = super::index_selector(input).expect("should parse");

        assert_eq!(result, ("", Selector::Index(Index::FromEnd(5.try_into().unwrap()))));
    }

    #[test]
    fn zero_array_index() {
        let input = "0";
        let result = super::index_selector(input).expect("should parse");

        assert_eq!(result, ("", Selector::Index(Index::FromStart(0.into()))));
    }

    #[test]
    fn two_sixtyfour_array_index() {
        let input = "18446744073709551616";
        super::index_selector(input).expect_err("should not parse");
    }

    #[test]
    fn two_sixtyfour_plus_one_array_index() {
        let input = "18446744073709551617";
        super::index_selector(input).expect_err("should not parse");
    }

    #[test]
    fn two_pow_fiftythree_minus_one_array_index() {
        let input = "9007199254740991";
        let result = super::index_selector(input).expect("should parse");

        assert_eq!(
            result,
            (
                "",
                Selector::Index(Index::FromStart(9_007_199_254_740_991_u64.try_into().unwrap()))
            )
        );
    }

    #[test]
    fn minus_two_pow_fiftythree_minus_one_array_index() {
        let input = "-9007199254740991";
        let result = super::index_selector(input).expect("should parse");

        assert_eq!(
            result,
            (
                "",
                Selector::Index(Index::FromEnd(9_007_199_254_740_991_u64.try_into().unwrap()))
            )
        );
    }

    #[test]
    fn minus_two_pow_fiftythree_index() {
        let input = "-9007199254740992";
        super::index_selector(input).expect_err("should not parse");
    }

    #[test_case("3:4:5", Index::FromStart(3.into()), Some(Index::FromStart(4.into())), Step::Forward(5.into()); "test 3c4c5")]
    #[test_case("-3:4:5", Index::FromEnd(3.try_into().unwrap()), Some(Index::FromStart(4.into())), Step::Forward(5.into()); "test m3c4c5")]
    #[test_case("3:-4:5", Index::FromStart(3.into()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Forward(5.into()); "test 3cm4c5")]
    #[test_case("3:4:-5", Index::FromStart(3.into()), Some(Index::FromStart(4.into())), Step::Backward(5.try_into().unwrap()); "test 3c4cm5")]
    #[test_case("-3:-4:5", Index::FromEnd(3.try_into().unwrap()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Forward(5.into()); "test m3cm4c5")]
    #[test_case("-3:4:-5", Index::FromEnd(3.try_into().unwrap()), Some(Index::FromStart(4.into())), Step::Backward(5.try_into().unwrap()); "test m3c4cm5")]
    #[test_case("3:-4:-5", Index::FromStart(3.into()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Backward(5.try_into().unwrap()); "test 3cm4cm5")]
    #[test_case("-3:-4:-5", Index::FromEnd(3.try_into().unwrap()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Backward(5.try_into().unwrap()); "test m3cm4cm5")]
    #[test_case(":4:5", Index::FromStart(0.into()), Some(Index::FromStart(4.into())), Step::Forward(5.into()); "test c4c5")]
    #[test_case(":-4:5", Index::FromStart(0.into()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Forward(5.into()); "test cm4c5")]
    #[test_case(":4:-5", Index::FromEnd(1.try_into().unwrap()), Some(Index::FromStart(4.into())), Step::Backward(5.try_into().unwrap()); "test c4cm5")]
    #[test_case(":-4:-5", Index::FromEnd(1.try_into().unwrap()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Backward(5.try_into().unwrap()); "test cm4cm5")]
    #[test_case("3::5", Index::FromStart(3.into()), None, Step::Forward(5.into()); "test 3cc5")]
    #[test_case("-3::5", Index::FromEnd(3.try_into().unwrap()), None, Step::Forward(5.into()); "test m3cc5")]
    #[test_case("3::-5", Index::FromStart(3.into()), None, Step::Backward(5.try_into().unwrap()); "test 3ccm5")]
    #[test_case("-3::-5", Index::FromEnd(3.try_into().unwrap()), None, Step::Backward(5.try_into().unwrap()); "test m3ccm5")]
    #[test_case("3:4:", Index::FromStart(3.into()), Some(Index::FromStart(4.into())), Step::Forward(1.into()); "test 3c4c")]
    #[test_case("-3:4:", Index::FromEnd(3.try_into().unwrap()), Some(Index::FromStart(4.into())), Step::Forward(1.into()); "test m3c4c")]
    #[test_case("3:-4:", Index::FromStart(3.into()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Forward(1.into()); "test 3cm4c")]
    #[test_case("-3:-4:", Index::FromEnd(3.try_into().unwrap()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Forward(1.into()); "test m3cm4c")]
    #[test_case("3::", Index::FromStart(3.into()), None, Step::Forward(1.into()); "test 3cc")]
    #[test_case("-3::", Index::FromEnd(3.try_into().unwrap()), None, Step::Forward(1.into()); "test m3cc")]
    #[test_case("3:", Index::FromStart(3.into()), None, Step::Forward(1.into()); "test 3c")]
    #[test_case("-3:", Index::FromEnd(3.try_into().unwrap()), None, Step::Forward(1.into()); "test m3c")]
    #[test_case(":4:", Index::FromStart(0.into()), Some(Index::FromStart(4.into())), Step::Forward(1.into()); "test c4c")]
    #[test_case(":-4:", Index::FromStart(0.into()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Forward(1.into()); "test cm4c")]
    #[test_case(":4", Index::FromStart(0.into()), Some(Index::FromStart(4.into())), Step::Forward(1.into()); "test c4")]
    #[test_case(":-4", Index::FromStart(0.into()), Some(Index::FromEnd(4.try_into().unwrap())), Step::Forward(1.into()); "test cm4")]
    #[test_case("::5", Index::FromStart(0.into()), None, Step::Forward(5.into()); "test cc5")]
    #[test_case("::-5", Index::FromEnd(1.try_into().unwrap()), None, Step::Backward(5.try_into().unwrap()); "test ccm5")]
    #[test_case("::", Index::FromStart(0.into()), None, Step::Forward(1.into()); "test cc")]
    #[test_case("::-1", Index::FromEnd(1.try_into().unwrap()), None, Step::Backward(1.try_into().unwrap()); "test ccm1")]
    #[test_case("0::-1", Index::FromStart(0.into()), None, Step::Backward(1.try_into().unwrap()); "test 0ccm1")]
    #[test_case("0:0:-1", Index::FromStart(0.into()), Some(Index::FromStart(0.into())), Step::Backward(1.try_into().unwrap()); "test 0c0cm1")]
    fn slice(input: &str, exp_start: Index, exp_end: Option<Index>, exp_step: Step) {
        let (rest, selector) = super::slice_selector(input).expect("should parse");
        assert_eq!("", rest);
        match selector {
            Selector::Slice(slice) => {
                assert_eq!(slice.start, exp_start);
                assert_eq!(slice.end, exp_end);
                assert_eq!(slice.step, exp_step);
            }
            _ => unreachable!(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[test_case("true", Literal::Bool(true); "true literal")]
    #[test_case("false", Literal::Bool(false); "false literal")]
    #[test_case("null", Literal::Null; "null literal")]
    #[test_case("42", Literal::Number(JsonNumber::Int(JsonInt::from(42))); "42 int")]
    #[test_case("42.37", Literal::Number(JsonNumber::Float(JsonFloat::try_from(42.37).unwrap())); "42d37 float")]
    #[test_case("-42.37", Literal::Number(JsonNumber::Float(JsonFloat::try_from(-42.37).unwrap())); "m42d37 float")]
    #[test_case("0", Literal::Number(JsonNumber::Int(JsonInt::ZERO)); "0")]
    #[test_case("0.0", Literal::Number(JsonNumber::Int(JsonInt::ZERO)); "m0")]
    #[test_case("-0", Literal::Number(JsonNumber::Int(JsonInt::ZERO)); "0d0")]
    #[test_case("-0.0", Literal::Number(JsonNumber::Int(JsonInt::ZERO)); "m0d0")]
    #[test_case("1e15", Literal::Number(JsonNumber::Int(JsonInt::try_from(1_000_000_000_000_000_i64).unwrap())); "1e15")]
    #[test_case("1E15", Literal::Number(JsonNumber::Int(JsonInt::try_from(1_000_000_000_000_000_i64).unwrap())); "1ue15")]
    #[test_case("1e16", Literal::Number(JsonNumber::Float(JsonFloat::try_from(1e16).unwrap())); "1e16")]
    #[test_case("-1e15", Literal::Number(JsonNumber::Int(JsonInt::try_from(-1_000_000_000_000_000_i64).unwrap())); "m1e15")]
    #[test_case("-1e16", Literal::Number(JsonNumber::Float(JsonFloat::try_from(-1e16).unwrap())); "m1e16")]
    #[test_case("1.04e15", Literal::Number(JsonNumber::Int(JsonInt::try_from(1_040_000_000_000_000_i64).unwrap())); "1d04e15")]
    #[test_case("1.04e16", Literal::Number(JsonNumber::Float(JsonFloat::try_from(1.04e16).unwrap())); "1d04e16")]
    #[test_case("-1.04e15", Literal::Number(JsonNumber::Int(JsonInt::try_from(-1_040_000_000_000_000_i64).unwrap())); "m1d04e15")]
    #[test_case("-1.04e16", Literal::Number(JsonNumber::Float(JsonFloat::try_from(-1.04e16).unwrap())); "m1d04e16")]
    #[test_case("1e-15", Literal::Number(JsonNumber::Float(JsonFloat::try_from(1e-15).unwrap())); "1em15")]
    #[test_case("-1e-15", Literal::Number(JsonNumber::Float(JsonFloat::try_from(-1e-15).unwrap())); "m1em15")]
    #[test_case("1.04e-15", Literal::Number(JsonNumber::Float(JsonFloat::try_from(1.04e-15).unwrap())); "1d04em15")]
    #[test_case("-1.04e-15", Literal::Number(JsonNumber::Float(JsonFloat::try_from(-1.04e-15).unwrap())); "m1d04em15")]
    #[test_case("-1.04E-15", Literal::Number(JsonNumber::Float(JsonFloat::try_from(-1.04e-15).unwrap())); "m1d04uem15")]
    #[test_case(r#""abc""#, Literal::String(JsonString::new("abc")))]
    fn valid_literal(input: &str, exp: Literal) {
        let (rest, lit) = super::literal(input).expect("should parse");
        assert_eq!("", rest);
        assert_eq!(lit, exp);
    }

    #[test_case("?@.b == 'kilo'", LogicalExpr::Comparison(ComparisonExpr {
        lhs: Comparable::RelativeSingularQuery(SingularJsonPathQuery {
            segments: vec![SingularSegment::Name(JsonString::from("b"))],
        }),
        rhs: Comparable::Literal(Literal::String(JsonString::from("kilo"))),
        op: ComparisonOp::EqualTo,
    }))]
    fn valid_filter(input: &str, exp: LogicalExpr) {
        let no_limit_opts = ParserOptions {
            recursion_limit: None,
            ..Default::default()
        };
        let (rest, lit) = super::filter_selector(input, super::ParseCtx::new(&no_limit_opts)).expect("should parse");
        assert_eq!("", rest);
        assert_eq!(lit, Selector::Filter(exp));
    }
}
