use pretty_assertions::assert_eq;
use rsonpath_syntax::{
    builder::JsonPathQueryBuilder,
    num::{JsonFloat, JsonUInt},
    str::JsonString,
};
use test_case::test_case;

#[test]
fn root() {
    let input = "$";
    let expected_query = JsonPathQueryBuilder::new().into();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test_case("$.*"; "asterisk")]
#[test_case("$[*]"; "bracketed asterisk")]
fn child_wildcard_selector_test(input: &str) {
    let expected_query = JsonPathQueryBuilder::new().child_wildcard().to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test_case("$..*"; "asterisk")]
#[test_case("$..[*]"; "bracketed asterisk")]
fn descendant_wildcard_selector(input: &str) {
    let expected_query = JsonPathQueryBuilder::new().descendant_wildcard().to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn wildcard_child_selector() {
    let input = "$.*.a.*";
    let expected_query = JsonPathQueryBuilder::new()
        .child_wildcard()
        .child_name(JsonString::new("a"))
        .child_wildcard()
        .to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn descendant_nonnegative_array_indexed_selector() {
    let input = "$..[5]";
    let expected_query = JsonPathQueryBuilder::new().descendant_index(5).to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn nonnegative_array_indexed_selector() {
    let input = "$[5]";
    let expected_query = JsonPathQueryBuilder::new().child_index(5).to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn multiple_nonnegative_array_indexed_selector() {
    let input = "$[5][2]";
    let expected_query = JsonPathQueryBuilder::new().child_index(5).child_index(2).to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn zeroth_array_indexed_selector() {
    let input = "$[0]";
    let expected_query = JsonPathQueryBuilder::new().child_index(JsonUInt::ZERO).to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn indexed_wildcard_child_selector() {
    let input = r#"$[*]['*']["*"]"#;
    let expected_query = JsonPathQueryBuilder::new()
        .child_wildcard()
        .child_name("*")
        .child_name("*")
        .to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn wildcard_descendant_selector() {
    let input = "$..*.a..*";
    let expected_query = JsonPathQueryBuilder::new()
        .descendant_wildcard()
        .child_name("a")
        .descendant_wildcard()
        .to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn indexed_wildcard_descendant_selector_nested() {
    let input = r#"$..[*]..['*']..["*"]"#;
    let expected_query = JsonPathQueryBuilder::new()
        .descendant_wildcard()
        .descendant_name("*")
        .descendant_name("*")
        .to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn escaped_single_quote_in_single_quote_member() {
    let input = r"$['\'']";
    let expected_query = JsonPathQueryBuilder::new().child_name("'").to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn unescaped_double_quote_in_single_quote_member() {
    let input = r#"$['"']"#;
    let expected_query = JsonPathQueryBuilder::new().child_name(r#"""#).to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn name_and_wildcard_selectors_bracketed_and_raw() {
    let input = "$.a['b']..c..['d'].*[*]..*..[*]";
    let expected_query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_name("b")
        .descendant_name("c")
        .descendant_name("d")
        .child_wildcard()
        .child_wildcard()
        .descendant_wildcard()
        .descendant_wildcard()
        .to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn basic_filter() {
    let input = r#"$.a[?@.b == "abc"]"#;
    let expected_query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|f| f.comparison(|x| x.query_relative(|x| x.name("b")).equal_to().literal("abc")))
        .to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn nested_filter() {
    let input = r#"$.a[?@.b == "abc" && $[*][?@.c > 3.13]]"#;
    let expected_query = JsonPathQueryBuilder::new()
        .child_name("a")
        .child_filter(|f| {
            f.comparison(|x| x.query_relative(|x| x.name("b")).equal_to().literal("abc"))
                .and(|f| {
                    f.test_absolute(|t| {
                        t.child_wildcard().child_filter(|f| {
                            f.comparison(|c| {
                                c.query_relative(|q| q.name("c"))
                                    .greater_than()
                                    .literal(JsonFloat::try_from(3.13).unwrap())
                            })
                        })
                    })
                })
        })
        .to_query();

    let result = rsonpath_syntax::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

mod proptests {
    use proptest::{option, prelude::*, strategy};
    mod correct_strings {
        use super::*;
        use pretty_assertions::assert_eq;

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(512))]
            #[test]
            fn parses_expected_query((input, expected) in any_valid_query()) {
                let result = rsonpath_syntax::parse(&input).expect("expected Ok");

                assert_eq!(expected, result);
            }

            #[test]
            fn round_trip((_, query) in any_valid_query()) {
                let input = query.to_string();
                let result = rsonpath_syntax::parse(&input).expect("expected Ok");

                assert_eq!(query, result);
            }
        }
    }
    use rsonpath_syntax::{
        builder::SliceBuilder, num::JsonInt, str::JsonString, JsonPathQuery, LogicalExpr, Segment, Selector, Selectors,
    };

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

    fn any_valid_query() -> impl Strategy<Value = (String, JsonPathQuery)> {
        return prop::collection::vec(any_segment(None), 0..10)
            .prop_map(map_prop_segments)
            .prop_recursive(3, 10, 5, |query_strategy| {
                prop::collection::vec(any_segment(Some(query_strategy)), 0..10).prop_map(map_prop_segments)
            });

        fn map_prop_segments(segments: Vec<(String, PropSegment)>) -> (String, JsonPathQuery) {
            let mut s = "$".to_string();
            let mut v = vec![];

            for (segment_s, segment) in segments {
                s.push_str(&segment_s);
                match segment {
                    PropSegment::ShortChildWildcard => v.push(Segment::Child(Selectors::one(Selector::Wildcard))),
                    PropSegment::ShortChildName(n) => v.push(Segment::Child(Selectors::one(Selector::Name(n)))),
                    PropSegment::ShortDescendantWildcard => {
                        v.push(Segment::Descendant(Selectors::one(Selector::Wildcard)))
                    }
                    PropSegment::ShortDescendantName(n) => {
                        v.push(Segment::Descendant(Selectors::one(Selector::Name(n))))
                    }
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
    }

    fn any_segment(
        recursive_query_strategy: Option<BoxedStrategy<(String, JsonPathQuery)>>,
    ) -> impl Strategy<Value = (String, PropSegment)> {
        return prop_oneof![
            strategy::Just((".*".to_string(), PropSegment::ShortChildWildcard)),
            strategy::Just(("..*".to_string(), PropSegment::ShortDescendantWildcard)),
            any_short_name().prop_map(|name| (format!(".{name}"), PropSegment::ShortChildName(JsonString::new(&name)))),
            any_short_name().prop_map(|name| (
                format!("..{name}"),
                PropSegment::ShortDescendantName(JsonString::new(&name))
            )),
            prop::collection::vec(any_selector(recursive_query_strategy.clone()), 1..5).prop_map(|reprs| {
                let mut s = "[".to_string();
                let v = collect_reprs(reprs, &mut s);
                s.push(']');
                (s, PropSegment::BracketedChild(v))
            }),
            prop::collection::vec(any_selector(recursive_query_strategy), 1..5).prop_map(|reprs| {
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

    fn any_json_int() -> impl Strategy<Value = (String, JsonInt)> {
        (-((1_i64 << 53) + 1)..((1_i64 << 53) - 1)).prop_map(|i| (i.to_string(), JsonInt::try_from(i).unwrap()))
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

        pub fn any_json_string() -> impl Strategy<Value = (String, JsonString)> {
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

        pub fn any_logical_expr(
            test_query_strategy: Option<BoxedStrategy<(String, JsonPathQuery)>>,
        ) -> impl Strategy<Value = (String, LogicalExpr)> {
            any_atomic_logical_expr(test_query_strategy).prop_recursive(8, 32, 2, |inner| {
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
                prop_oneof![num::f64::NORMAL, num::f64::NORMAL.prop_map(|f| f.trunc())]
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
}
