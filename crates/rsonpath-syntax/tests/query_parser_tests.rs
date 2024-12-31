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
    use proptest::prelude::*;
    use rsonpath_syntax_proptest::ArbitraryJsonPathQuery;

    mod correct_strings {
        use super::*;
        use pretty_assertions::assert_eq;

        proptest! {
            #![proptest_config(ProptestConfig::with_cases(512))]
            #[test]
            fn parses_expected_query(query in proptest::arbitrary::any::<ArbitraryJsonPathQuery>()) {
                let result = rsonpath_syntax::parse(&query.string).expect("expected Ok");

                assert_eq!(query.parsed, result);
            }

            #[test]
            fn round_trip(query in proptest::arbitrary::any::<ArbitraryJsonPathQuery>()) {
                let input = query.parsed.to_string();
                let result = rsonpath_syntax::parse(&input).expect("expected Ok");

                assert_eq!(query.parsed, result);
            }
        }
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;
        use pretty_assertions::assert_eq;
        use rsonpath_syntax::JsonPathQuery;

        /// This is a proptest regression test.
        /// It relies on serde_json using the `float_roundtrip` feature.
        /// See: https://github.com/serde-rs/json/issues/1170
        #[test]
        fn float_roundtrip() {
            let string = "$[?$&&!($&&!$[?$&&($||$&&$..[?(405831638439668000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000==null||null==null)||null==null]||$)])]";
            let query = rsonpath_syntax::parse(string).unwrap();

            let json_str = serde_json::to_string(&query).unwrap();
            let query_deser = serde_json::from_str::<JsonPathQuery>(&json_str).unwrap();

            assert_eq!(query, query_deser);
        }

        proptest! {
            #[test]
            fn query_cbor_roundtrips(ArbitraryJsonPathQuery { parsed, .. } in prop::arbitrary::any::<ArbitraryJsonPathQuery>()) {
                use std::io;
                struct ReadBuf<'a> {
                    buf: &'a [u8],
                    idx: usize,
                }
                impl<'a> io::Read for &mut ReadBuf<'a> {
                    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
                        let len = std::cmp::min(self.buf.len() - self.idx, buf.len());
                        buf.copy_from_slice(&self.buf[self.idx..self.idx + len]);
                        self.idx += len;
                        Ok(len)
                    }
                }

                let mut buf = vec![];
                ciborium::into_writer(&parsed, &mut buf)?;

                let mut read = ReadBuf { buf: &buf, idx: 0 };
                let query_deser = ciborium::from_reader(&mut read)?;

                assert_eq!(parsed, query_deser);
            }

            #[test]
            fn query_json_roundtrips(ArbitraryJsonPathQuery { parsed, .. } in prop::arbitrary::any::<ArbitraryJsonPathQuery>()) {
                let json_str = serde_json::to_string(&parsed)?;
                let query_deser = serde_json::from_str::<JsonPathQuery>(&json_str)?;

                assert_eq!(parsed, query_deser);
            }

            #[test]
            fn query_message_pack_roundtrips(ArbitraryJsonPathQuery { parsed, .. } in prop::arbitrary::any::<ArbitraryJsonPathQuery>()) {
                let buf = rmp_serde::to_vec(&parsed)?;
                let query_deser = rmp_serde::from_slice(&buf)?;

                assert_eq!(parsed, query_deser);
            }
        }
    }
}
