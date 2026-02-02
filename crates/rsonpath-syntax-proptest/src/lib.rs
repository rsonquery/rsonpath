//! Utilities for property testing with types in [`rsonpath-syntax`](https://docs.rs/rsonpath-syntax/latest/rsonpath_syntax/).
//!
//! Implementation of [`proptest::arbitrary::Arbitrary`]
//! for JSONPath queries via the [`ArbitraryJsonPathQuery`] struct.
//!
//! # Examples
//!
//! ```rust,no_run
//! use proptest::prelude::*;
//! use rsonpath_syntax_proptest::ArbitraryJsonPathQuery;
//!
//! proptest! {
//!     #[test]
//!     fn example(ArbitraryJsonPathQuery { parsed, string } in prop::arbitrary::any::<ArbitraryJsonPathQuery>()) {
//!         assert_eq!(parsed, rsonpath_syntax::parse(&string)?);
//!     }
//! }
//! ```

use proptest::{option, prelude::*, strategy};
use rsonpath_syntax::{
    builder::SliceBuilder, num::JsonInt, str::JsonString, JsonPathQuery, LogicalExpr, Segment, Selector, Selectors,
};
use std::fmt::Debug;

/// A valid JSONPath string and the [`JsonPathQuery`] object parsed from it.
///
/// This is the struct through which an [`proptest::arbitrary::Arbitrary`] implementation
/// for [`JsonPathQuery`] is provided.
pub struct ArbitraryJsonPathQuery {
    /// The JSONPath query string.
    pub string: String,
    /// The parsed JSONPath query.
    pub parsed: JsonPathQuery,
}

impl Debug for ArbitraryJsonPathQuery {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArbitraryJsonPathQuery")
            .field("string", &self.string)
            .field("parsed", &self.parsed)
            .field("string_raw", &self.string.as_bytes())
            .finish()
    }
}

/// Parameters of the [`ArbitraryJsonPathQuery`] [`Arbitrary`](`proptest::arbitrary::Arbitrary`) implementation.
#[derive(Debug)]
pub struct ArbitraryJsonPathQueryParams {
    /// Depth limit for recursion for generated JSONPath queries. Default value: 3.
    ///
    /// JSONPath queries are recursive since a filter selector can contain an arbitrary JSONPath query.
    /// This limits the nesting level.
    /// See [proptest::strategy::Strategy::prop_recursive] for details of how this affects the recursive generation.
    pub recursive_depth: u32,
    /// Desired size in terms of tree nodes of a generated JSONPath query. Default value: 10.
    ///
    /// JSONPath queries are recursive since a filter selector can contain an arbitrary JSONPath query.
    /// This limits the nesting level.
    /// See [proptest::strategy::Strategy::prop_recursive] for details of how this affects the recursive generation.
    pub desired_size: u32,
    /// Limit on the number of segments in the generated query, not including the initial root `$` selector.
    /// Default value: 10.
    pub max_segments: usize,
    /// Minimum number of selectors in each of the generated segments. Default value: 1.
    ///
    /// Must be non-zero.
    pub min_selectors: usize,
    /// Maximum number of selectors in each of the generated segments. Default value: 5.
    ///
    /// Must be at least `min_segments`.
    pub max_selectors: usize,
    /// Only generate query elements that are supported by the [`rsonpath`](https://docs.rs/rsonpath-lib/latest/rsonpath/) crate.
    ///
    /// Consult rsonpath's documentation for details on what this entails.
    pub only_rsonpath_supported_subset: bool,
}

impl ArbitraryJsonPathQuery {
    #[inline]
    #[must_use]
    pub fn new(string: String, parsed: JsonPathQuery) -> Self {
        Self { string, parsed }
    }
}

impl Default for ArbitraryJsonPathQueryParams {
    #[inline]
    fn default() -> Self {
        Self {
            only_rsonpath_supported_subset: false,
            recursive_depth: 3,
            desired_size: 10,
            max_segments: 10,
            min_selectors: 1,
            max_selectors: 5,
        }
    }
}

impl proptest::arbitrary::Arbitrary for ArbitraryJsonPathQuery {
    type Parameters = ArbitraryJsonPathQueryParams;
    type Strategy = BoxedStrategy<Self>;

    #[inline]
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
        assert!(args.min_selectors > 0);
        assert!(args.max_selectors >= args.min_selectors);

        if args.only_rsonpath_supported_subset {
            rsonpath_valid_query(&args).prop_map(|x| Self::new(x.0, x.1)).boxed()
        } else {
            any_valid_query(&args).prop_map(|x| Self::new(x.0, x.1)).boxed()
        }
    }
}

/* Approach: we generate the query string bit by bit, each time attaching what the expected
 * typed element is. At the end we have the input string all ready, and the expected
 * parser result can be easily obtained by a 1-1 translation.
 */
#[derive(Debug, Clone)]
enum PropSegment {
    // .*
    ShortChildWildcard,
    // .name
    ShortChildName(JsonString),
    // ..*
    ShortDescendantWildcard,
    // ..name
    ShortDescendantName(JsonString),
    // [<vec>]
    BracketedChild(Vec<PropSelector>),
    // ..[<vec>]
    BracketedDescendant(Vec<PropSelector>),
}

#[derive(Debug, Clone)]
enum PropSelector {
    Wildcard,
    Name(JsonString),
    Index(JsonInt),
    Slice(Option<JsonInt>, Option<JsonInt>, Option<JsonInt>),
    Filter(LogicalExpr),
}

fn any_valid_query(props: &ArbitraryJsonPathQueryParams) -> impl Strategy<Value = (String, JsonPathQuery)> {
    let ArbitraryJsonPathQueryParams {
        min_selectors,
        max_selectors,
        max_segments,
        recursive_depth,
        desired_size,
        ..
    } = *props;

    prop::collection::vec(any_segment(None, min_selectors, max_selectors), 0..max_segments)
        .prop_map(map_prop_segments)
        .prop_recursive(recursive_depth, desired_size, 5, move |query_strategy| {
            prop::collection::vec(
                any_segment(Some(query_strategy), min_selectors, max_selectors),
                0..max_segments,
            )
            .prop_map(map_prop_segments)
        })
}

fn rsonpath_valid_query(props: &ArbitraryJsonPathQueryParams) -> impl Strategy<Value = (String, JsonPathQuery)> {
    let ArbitraryJsonPathQueryParams { max_segments, .. } = *props;
    prop::collection::vec(rsonpath_valid_segment(), 0..max_segments).prop_map(map_prop_segments)
}

fn map_prop_segments(segments: Vec<(String, PropSegment)>) -> (String, JsonPathQuery) {
    let mut s = "$".to_string();
    let mut v = vec![];

    for (segment_s, segment) in segments {
        s.push_str(&segment_s);
        match segment {
            PropSegment::ShortChildWildcard => v.push(Segment::Child(Selectors::one(Selector::Wildcard))),
            PropSegment::ShortChildName(n) => v.push(Segment::Child(Selectors::one(Selector::Name(n)))),
            PropSegment::ShortDescendantWildcard => v.push(Segment::Descendant(Selectors::one(Selector::Wildcard))),
            PropSegment::ShortDescendantName(n) => v.push(Segment::Descendant(Selectors::one(Selector::Name(n)))),
            PropSegment::BracketedChild(ss) => v.push(Segment::Child(Selectors::many(
                ss.into_iter().map(map_prop_selector).collect(),
            ))),
            PropSegment::BracketedDescendant(ss) => v.push(Segment::Descendant(Selectors::many(
                ss.into_iter().map(map_prop_selector).collect(),
            ))),
        }
    }

    (s, JsonPathQuery::from_iter(v))
}

fn map_prop_selector(s: PropSelector) -> Selector {
    match s {
        PropSelector::Wildcard => Selector::Wildcard,
        PropSelector::Name(n) => Selector::Name(n),
        PropSelector::Index(i) => Selector::Index(i.into()),
        PropSelector::Slice(start, end, step) => Selector::Slice({
            let mut builder = SliceBuilder::new();
            if let Some(start) = start {
                builder.with_start(start);
            }
            if let Some(step) = step {
                builder.with_step(step);
            }
            if let Some(end) = end {
                builder.with_end(end);
            }
            builder.into()
        }),
        PropSelector::Filter(logical) => Selector::Filter(logical),
    }
}

fn any_segment(
    recursive_query_strategy: Option<BoxedStrategy<(String, JsonPathQuery)>>,
    min_selectors: usize,
    max_selectors: usize,
) -> impl Strategy<Value = (String, PropSegment)> {
    return prop_oneof![
        strategy::Just((".*".to_string(), PropSegment::ShortChildWildcard)),
        strategy::Just(("..*".to_string(), PropSegment::ShortDescendantWildcard)),
        any_short_name().prop_map(|name| (format!(".{name}"), PropSegment::ShortChildName(JsonString::new(&name)))),
        any_short_name().prop_map(|name| (
            format!("..{name}"),
            PropSegment::ShortDescendantName(JsonString::new(&name))
        )),
        prop::collection::vec(
            any_selector(recursive_query_strategy.clone()),
            min_selectors..max_selectors
        )
        .prop_map(|reprs| {
            let mut s = "[".to_string();
            let v = collect_reprs(reprs, &mut s);
            s.push(']');
            (s, PropSegment::BracketedChild(v))
        }),
        prop::collection::vec(any_selector(recursive_query_strategy), min_selectors..max_selectors).prop_map(|reprs| {
            let mut s = "..[".to_string();
            let v = collect_reprs(reprs, &mut s);
            s.push(']');
            (s, PropSegment::BracketedDescendant(v))
        }),
    ];

    fn collect_reprs(reprs: Vec<(String, PropSelector)>, s: &mut String) -> Vec<PropSelector> {
        let mut result = Vec::with_capacity(reprs.len());
        let mut first = true;
        for (repr_s, prop_selector) in reprs {
            if !first {
                s.push(',');
            }
            first = false;
            s.push_str(&repr_s);
            result.push(prop_selector);
        }
        result
    }
}

fn rsonpath_valid_segment() -> impl Strategy<Value = (String, PropSegment)> {
    prop_oneof![
        strategy::Just((".*".to_string(), PropSegment::ShortChildWildcard)),
        strategy::Just(("..*".to_string(), PropSegment::ShortDescendantWildcard)),
        any_short_name().prop_map(|name| (format!(".{name}"), PropSegment::ShortChildName(JsonString::new(&name)))),
        any_short_name().prop_map(|name| (
            format!("..{name}"),
            PropSegment::ShortDescendantName(JsonString::new(&name))
        )),
        rsonpath_valid_selector().prop_map(|repr| {
            let mut s = "[".to_string();
            s.push_str(&repr.0);
            s.push(']');
            (s, PropSegment::BracketedChild(vec![repr.1]))
        }),
        rsonpath_valid_selector().prop_map(|repr| {
            let mut s = "..[".to_string();
            s.push_str(&repr.0);
            s.push(']');
            (s, PropSegment::BracketedDescendant(vec![repr.1]))
        }),
    ]
}

fn any_selector(
    recursive_query_strategy: Option<BoxedStrategy<(String, JsonPathQuery)>>,
) -> impl Strategy<Value = (String, PropSelector)> {
    prop_oneof![
        strategy::Just(("*".to_string(), PropSelector::Wildcard)),
        strings::any_json_string().prop_map(|(raw, s)| (raw, PropSelector::Name(s))),
        any_json_int().prop_map(|(raw, i)| (raw, PropSelector::Index(i))),
        any_slice().prop_map(|(raw, a, b, c)| (raw, PropSelector::Slice(a, b, c))),
        filters::any_logical_expr(recursive_query_strategy)
            .prop_map(|(raw, expr)| (format!("?{raw}"), PropSelector::Filter(expr)))
    ]
}

fn rsonpath_valid_selector() -> impl Strategy<Value = (String, PropSelector)> {
    prop_oneof![
        strategy::Just(("*".to_string(), PropSelector::Wildcard)),
        strings::any_json_string().prop_map(|(raw, s)| (raw, PropSelector::Name(s))),
        rsonpath_valid_json_int().prop_map(|(raw, i)| (raw, PropSelector::Index(i))),
        rsonpath_valid_slice().prop_map(|(raw, a, b, c)| (raw, PropSelector::Slice(a, b, c))),
    ]
}

fn any_json_int() -> impl Strategy<Value = (String, JsonInt)> {
    (-((1_i64 << 53) + 1)..((1_i64 << 53) - 1)).prop_map(|i| (i.to_string(), JsonInt::try_from(i).unwrap()))
}

fn rsonpath_valid_json_int() -> impl Strategy<Value = (String, JsonInt)> {
    (0..((1_i64 << 53) - 1)).prop_map(|i| (i.to_string(), JsonInt::try_from(i).unwrap()))
}

fn any_slice() -> impl Strategy<Value = (String, Option<JsonInt>, Option<JsonInt>, Option<JsonInt>)> {
    (
        option::of(any_json_int()),
        option::of(any_json_int()),
        option::of(any_json_int()),
    )
        .prop_map(|(a, b, c)| {
            let mut s = String::new();
            let a = a.map(|(a_s, a_i)| {
                s.push_str(&a_s);
                a_i
            });
            s.push(':');
            let b = b.map(|(b_s, b_i)| {
                s.push_str(&b_s);
                b_i
            });
            s.push(':');
            let c = c.map(|(c_s, c_i)| {
                s.push_str(&c_s);
                c_i
            });
            (s, a, b, c)
        })
}

fn rsonpath_valid_slice() -> impl Strategy<Value = (String, Option<JsonInt>, Option<JsonInt>, Option<JsonInt>)> {
    (
        option::of(rsonpath_valid_json_int()),
        option::of(rsonpath_valid_json_int()),
        option::of(rsonpath_valid_json_int()),
    )
        .prop_map(|(a, b, c)| {
            let mut s = String::new();
            let a = a.map(|(a_s, a_i)| {
                s.push_str(&a_s);
                a_i
            });
            s.push(':');
            let b = b.map(|(b_s, b_i)| {
                s.push_str(&b_s);
                b_i
            });
            s.push(':');
            let c = c.map(|(c_s, c_i)| {
                s.push_str(&c_s);
                c_i
            });
            (s, a, b, c)
        })
}

fn any_short_name() -> impl Strategy<Value = String> {
    r"([A-Za-z]|_|[^\u0000-\u007F])([A-Za-z0-9]|_|[^\u0000-\u007F])*"
}

mod strings {
    use proptest::{prelude::*, sample::SizeRange};
    use rsonpath_syntax::str::JsonString;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum JsonStringToken {
        EncodeNormally(char),
        ForceUnicodeEscape(char),
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum JsonStringTokenEncodingMode {
        SingleQuoted,
        DoubleQuoted,
    }

    impl JsonStringToken {
        fn raw(self) -> char {
            match self {
                Self::EncodeNormally(x) | Self::ForceUnicodeEscape(x) => x,
            }
        }

        fn encode(self, mode: JsonStringTokenEncodingMode) -> String {
            return match self {
                Self::EncodeNormally('\u{0008}') => r"\b".to_owned(),
                Self::EncodeNormally('\t') => r"\t".to_owned(),
                Self::EncodeNormally('\n') => r"\n".to_owned(),
                Self::EncodeNormally('\u{000C}') => r"\f".to_owned(),
                Self::EncodeNormally('\r') => r"\r".to_owned(),
                Self::EncodeNormally('"') => match mode {
                    JsonStringTokenEncodingMode::DoubleQuoted => r#"\""#.to_owned(),
                    JsonStringTokenEncodingMode::SingleQuoted => r#"""#.to_owned(),
                },
                Self::EncodeNormally('\'') => match mode {
                    JsonStringTokenEncodingMode::DoubleQuoted => r#"'"#.to_owned(),
                    JsonStringTokenEncodingMode::SingleQuoted => r#"\'"#.to_owned(),
                },
                Self::EncodeNormally('/') => r"\/".to_owned(),
                Self::EncodeNormally('\\') => r"\\".to_owned(),
                Self::EncodeNormally(c @ ..='\u{001F}') | Self::ForceUnicodeEscape(c) => encode_unicode_escape(c),
                Self::EncodeNormally(c) => c.to_string(),
            };

            fn encode_unicode_escape(c: char) -> String {
                let mut buf = [0; 2];
                let enc = c.encode_utf16(&mut buf);
                let mut res = String::new();
                for x in enc {
                    res += &format!("\\u{x:0>4x}");
                }
                res
            }
        }
    }

    pub(super) fn any_json_string() -> impl Strategy<Value = (String, JsonString)> {
        prop_oneof![
            Just(JsonStringTokenEncodingMode::SingleQuoted),
            Just(JsonStringTokenEncodingMode::DoubleQuoted)
        ]
        .prop_flat_map(|mode| {
            prop::collection::vec(
                (prop::char::any(), prop::bool::ANY).prop_map(|(c, b)| {
                    if b {
                        JsonStringToken::EncodeNormally(c)
                    } else {
                        JsonStringToken::ForceUnicodeEscape(c)
                    }
                }),
                SizeRange::default(),
            )
            .prop_map(move |v| {
                let q = match mode {
                    JsonStringTokenEncodingMode::SingleQuoted => '\'',
                    JsonStringTokenEncodingMode::DoubleQuoted => '"',
                };
                let mut s = String::new();
                let mut l = String::new();
                for x in v {
                    s += &x.encode(mode);
                    l.push(x.raw());
                }
                (format!("{q}{s}{q}"), JsonString::new(&l))
            })
        })
    }
}

mod filters {
    use proptest::{num, prelude::*, strategy};
    use rsonpath_syntax::{
        num::{JsonFloat, JsonNumber},
        str::JsonString,
        Comparable, ComparisonExpr, ComparisonOp, JsonPathQuery, Literal, LogicalExpr, SingularJsonPathQuery,
        SingularSegment, TestExpr,
    };

    pub(super) fn any_logical_expr(
        test_query_strategy: Option<BoxedStrategy<(String, JsonPathQuery)>>,
    ) -> impl Strategy<Value = (String, LogicalExpr)> {
        any_atomic_logical_expr(test_query_strategy).prop_recursive(3, 10, 2, |inner| {
            prop_oneof![
                (inner.clone(), proptest::bool::ANY).prop_map(|((s, f), force_paren)| (
                    match f {
                        LogicalExpr::Test(_) if !force_paren => format!("!{s}"),
                        _ => format!("!({s})"),
                    },
                    LogicalExpr::Not(Box::new(f))
                )),
                (inner.clone(), inner.clone(), proptest::bool::ANY, proptest::bool::ANY).prop_map(
                    |((lhs_s, lhs_e), (rhs_s, rhs_e), force_left_paren, force_right_paren)| {
                        let put_left_paren = force_left_paren || matches!(lhs_e, LogicalExpr::Or(_, _));
                        let put_right_paren =
                            force_right_paren || matches!(rhs_e, LogicalExpr::Or(_, _) | LogicalExpr::And(_, _));
                        let s = match (put_left_paren, put_right_paren) {
                            (true, true) => format!("({lhs_s})&&({rhs_s})"),
                            (true, false) => format!("({lhs_s})&&{rhs_s}"),
                            (false, true) => format!("{lhs_s}&&({rhs_s})"),
                            (false, false) => format!("{lhs_s}&&{rhs_s}"),
                        };
                        (s, LogicalExpr::And(Box::new(lhs_e), Box::new(rhs_e)))
                    }
                ),
                (inner.clone(), inner.clone(), proptest::bool::ANY, proptest::bool::ANY).prop_map(
                    |((lhs_s, lhs_e), (rhs_s, rhs_e), force_left_paren, force_right_paren)| {
                        let put_left_paren = force_left_paren || matches!(lhs_e, LogicalExpr::Or(_, _));
                        let put_right_paren = force_right_paren;
                        let s = match (put_left_paren, put_right_paren) {
                            (true, true) => format!("({lhs_s})||({rhs_s})"),
                            (true, false) => format!("({lhs_s})||{rhs_s}"),
                            (false, true) => format!("{lhs_s}||({rhs_s})"),
                            (false, false) => format!("{lhs_s}||{rhs_s}"),
                        };
                        (s, LogicalExpr::Or(Box::new(lhs_e), Box::new(rhs_e)))
                    }
                )
            ]
        })
    }

    fn any_atomic_logical_expr(
        test_query_strategy: Option<BoxedStrategy<(String, JsonPathQuery)>>,
    ) -> impl Strategy<Value = (String, LogicalExpr)> {
        if let Some(test_query_strategy) = test_query_strategy {
            prop_oneof![
                any_test(test_query_strategy).prop_map(|(s, t)| (s, LogicalExpr::Test(t))),
                any_comparison().prop_map(|(s, c)| (s, LogicalExpr::Comparison(c))),
            ]
            .boxed()
        } else {
            any_comparison()
                .prop_map(|(s, c)| (s, LogicalExpr::Comparison(c)))
                .boxed()
        }
    }

    fn any_test(
        test_query_strategy: BoxedStrategy<(String, JsonPathQuery)>,
    ) -> impl Strategy<Value = (String, TestExpr)> {
        (proptest::bool::ANY, test_query_strategy).prop_map(|(relative, (mut s, q))| {
            if relative {
                assert_eq!(s.as_bytes()[0], b'$');
                s.replace_range(0..1, "@");
                (s, TestExpr::Relative(q))
            } else {
                (s, TestExpr::Absolute(q))
            }
        })
    }

    fn any_comparison() -> impl Strategy<Value = (String, ComparisonExpr)> {
        (any_comparable(), any_comparison_op(), any_comparable()).prop_map(
            |((lhs_s, lhs_e), (op_s, op_e), (rhs_s, rhs_e))| {
                (
                    format!("{lhs_s}{op_s}{rhs_s}"),
                    ComparisonExpr::from_parts(lhs_e, op_e, rhs_e),
                )
            },
        )
    }

    fn any_comparable() -> impl Strategy<Value = (String, Comparable)> {
        prop_oneof![
            any_literal().prop_map(|(s, l)| (s, Comparable::Literal(l))),
            (proptest::bool::ANY, any_singular_query()).prop_map(|(relative, (mut s, q))| {
                if relative {
                    assert_eq!(s.as_bytes()[0], b'$');
                    s.replace_range(0..1, "@");
                    (s, Comparable::RelativeSingularQuery(q))
                } else {
                    (s, Comparable::AbsoluteSingularQuery(q))
                }
            })
        ]
    }

    prop_compose! {
        fn any_singular_query()(segments in prop::collection::vec(any_singular_segment(), 0..10)) -> (String, SingularJsonPathQuery) {
            let mut s = "$".to_string();
            let mut v = vec![];

            for (segment_s, segment) in segments {
                s.push_str(&segment_s);
                v.push(segment);
            }

            (s, SingularJsonPathQuery::from_iter(v))
        }
    }

    fn any_singular_segment() -> impl Strategy<Value = (String, SingularSegment)> {
        prop_oneof![
            super::any_json_int().prop_map(|(s, i)| (format!("[{s}]"), SingularSegment::Index(i.into()))),
            super::any_short_name().prop_map(|n| (format!(".{n}"), SingularSegment::Name(JsonString::new(&n)))),
            super::strings::any_json_string().prop_map(|(s, n)| (format!("[{s}]"), SingularSegment::Name(n))),
        ]
    }

    fn any_literal() -> impl Strategy<Value = (String, Literal)> {
        prop_oneof![
            strategy::Just(("null".to_string(), Literal::Null)),
            proptest::bool::ANY.prop_map(|b| (b.to_string(), Literal::Bool(b))),
            any_json_number().prop_map(|(s, n)| (s, Literal::Number(n))),
            super::strings::any_json_string().prop_map(|(raw, s)| (raw, Literal::String(s)))
        ]
    }

    fn any_json_number() -> impl Strategy<Value = (String, JsonNumber)> {
        prop_oneof![
            super::any_json_int().prop_map(|(s, i)| (s, JsonNumber::Int(i))),
            any_json_float().prop_map(|(s, f)| (s, JsonNumber::Float(f))),
        ]
        .prop_map(|(x, n)| (x, n.normalize()))
    }

    fn any_json_float() -> impl Strategy<Value = (String, JsonFloat)> {
        // We first generate the target f64 value we want and then pick one of its possible string reprs.
        // Because an "int float" is also interesting we generate those half the time.
        // If there is no exponent, there is only one possible representation.
        // If we include an exponent we can move the floating point however far we want one way or the other.
        return prop_oneof![
            any_float().prop_map(|f| (f.to_string(), JsonFloat::try_from(f).unwrap())),
            any_float()
                .prop_flat_map(|f| arbitrary_exp_repr(f).prop_map(move |s| (s, JsonFloat::try_from(f).unwrap()))),
        ];

        fn any_float() -> impl Strategy<Value = f64> {
            prop_oneof![num::f64::NORMAL, num::f64::NORMAL.prop_map(f64::trunc)]
        }

        fn arbitrary_exp_repr(f: f64) -> impl Strategy<Value = String> {
            let s = f.to_string();
            let fp_pos: isize = s.find('.').unwrap_or(s.len()).try_into().unwrap();
            let num_digits = if fp_pos == s.len() as isize {
                s.len()
            } else {
                s.len() - 1
            } - if f.is_sign_negative() {
                // Subtract the minus char.
                1
            } else {
                0
            };
            (-1024..=1024_isize, proptest::bool::ANY, proptest::bool::ANY).prop_map(
                move |(exp, force_sign, uppercase_e)| {
                    let new_pos = fp_pos - exp;
                    let mut res = String::new();
                    if f.is_sign_negative() {
                        res.push('-');
                    }
                    let mut orig_digits = s.chars().filter(|c| *c != '.');

                    // There are three cases:
                    //   1. the new point is before all existing digits;
                    //     in this case we need to append 0.000... at the front
                    //   2. the new point position falls within the existing string;
                    //     this is straightforward, we just emplace it there
                    //   3. the new point is after all existing digits;
                    //     in this case we need to append 0000... at the end
                    // After this operation we need to manually trim the zeroes.
                    if new_pos <= 0 {
                        // Case 1.
                        res.push_str("0.");
                        for _ in 0..(-new_pos) {
                            res.push('0');
                        }
                        for orig_digit in orig_digits {
                            res.push(orig_digit);
                        }
                    } else if new_pos < num_digits as isize {
                        // Case 2.
                        let mut pos = 0;
                        let mut pushed_non_zero = false;
                        loop {
                            if pos == new_pos {
                                if !pushed_non_zero {
                                    res.push('0');
                                }
                                pushed_non_zero = true;
                                res.push('.');
                            } else {
                                let Some(orig_digit) = orig_digits.next() else { break };
                                if orig_digit == '0' {
                                    if pushed_non_zero {
                                        res.push(orig_digit);
                                    }
                                } else {
                                    pushed_non_zero = true;
                                    res.push(orig_digit);
                                }
                            }
                            pos += 1;
                        }
                    } else if f == 0.0 {
                        // Case 3. special case.
                        // Note that -0.0 is handled here as well, as it is equal to 0.0 and the sign is appended above.
                        res.push('0');
                    } else {
                        // Case 3.
                        // First skip zeroes. There has to be at least one non-zero since we checked
                        // f == 0.0 above.
                        let skip_zeroes = orig_digits.skip_while(|x| *x == '0');
                        for orig_digit in skip_zeroes {
                            res.push(orig_digit);
                        }
                        for _ in 0..(new_pos - num_digits as isize) {
                            res.push('0');
                        }
                    }

                    res.push(if uppercase_e { 'E' } else { 'e' });

                    if exp > 0 {
                        if force_sign {
                            res.push('+');
                        }
                        res.push_str(&exp.to_string());
                    } else {
                        res.push_str(&exp.to_string());
                    }

                    res
                },
            )
        }
    }

    fn any_comparison_op() -> impl Strategy<Value = (String, ComparisonOp)> {
        prop_oneof![
            strategy::Just(("==".to_string(), ComparisonOp::EqualTo)),
            strategy::Just(("!=".to_string(), ComparisonOp::NotEqualTo)),
            strategy::Just(("<".to_string(), ComparisonOp::LessThan)),
            strategy::Just((">".to_string(), ComparisonOp::GreaterThan)),
            strategy::Just(("<=".to_string(), ComparisonOp::LesserOrEqualTo)),
            strategy::Just((">=".to_string(), ComparisonOp::GreaterOrEqualTo)),
        ]
    }
}
