use pretty_assertions::assert_eq;
use rsonpath_syntax::{builder::JsonPathQueryBuilder, num::JsonUInt, str::JsonString, JsonPathQuery};
use test_case::test_case;

#[test]
fn should_infer_root_from_empty_string() {
    let input = "";
    let expected_query = JsonPathQueryBuilder::new().into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn root() {
    let input = "$";
    let expected_query = JsonPathQueryBuilder::new().into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test_case("$.*"; "asterisk")]
#[test_case("$[*]"; "bracketed asterisk")]
fn child_wildcard_selector_test(input: &str) {
    let expected_query = JsonPathQueryBuilder::new().any_child().into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test_case("$..*"; "asterisk")]
#[test_case("$..[*]"; "bracketed asterisk")]
fn descendant_wildcard_selector(input: &str) {
    let expected_query = JsonPathQueryBuilder::new().any_descendant().into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn wildcard_child_selector() {
    let input = "$.*.a.*";
    let expected_query = JsonPathQueryBuilder::new()
        .any_child()
        .child(JsonString::new("a"))
        .any_child()
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn descendant_nonnegative_array_indexed_selector() {
    let input = "$..[5]";
    let expected_query = JsonPathQueryBuilder::new()
        .array_index_descendant(5_u64.try_into().unwrap())
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn nonnegative_array_indexed_selector() {
    let input = "$[5]";
    let expected_query = JsonPathQueryBuilder::new()
        .array_index_child(5_u64.try_into().unwrap())
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn multiple_nonnegative_array_indexed_selector() {
    let input = "$[5][2]";
    let expected_query = JsonPathQueryBuilder::new()
        .array_index_child(5_u64.try_into().unwrap())
        .array_index_child(2_u64.try_into().unwrap())
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn zeroth_array_indexed_selector() {
    let input = "$[0]";
    let expected_query = JsonPathQueryBuilder::new().array_index_child(JsonUInt::ZERO).into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn indexed_wildcard_child_selector() {
    let input = r#"$[*]['*']["*"]"#;
    let expected_query = JsonPathQueryBuilder::new()
        .any_child()
        .child(JsonString::new("*"))
        .child(JsonString::new("*"))
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn wildcard_descendant_selector() {
    let input = "$..*.a..*";
    let expected_query = JsonPathQueryBuilder::new()
        .any_descendant()
        .child(JsonString::new("a"))
        .any_descendant()
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn indexed_wildcard_descendant_selector_nested() {
    let input = r#"$..[*]..['*']..["*"]"#;
    let expected_query = JsonPathQueryBuilder::new()
        .any_descendant()
        .descendant(JsonString::new("*"))
        .descendant(JsonString::new("*"))
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn escaped_single_quote_in_single_quote_member() {
    let input = r"['\'']";
    let expected_query = JsonPathQueryBuilder::new().child(JsonString::new("'")).into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn unescaped_double_quote_in_single_quote_member() {
    let input = r#"['"']"#;
    let expected_query = JsonPathQueryBuilder::new().child(JsonString::new(r#"""#)).into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn name_and_wildcard_selectors_bracketed_and_raw() {
    let input = "$.a['b']..c..['d'].*[*]..*..[*]";
    let expected_query = JsonPathQueryBuilder::new()
        .child(JsonString::new("a"))
        .child(JsonString::new("b"))
        .descendant(JsonString::new("c"))
        .descendant(JsonString::new("d"))
        .any_child()
        .any_child()
        .any_descendant()
        .any_descendant()
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

mod proptests {
    use super::*;
    use proptest::{prelude::*, sample::SizeRange};
    use rsonpath_syntax::num::JsonUInt;

    /* Approach: we generate a sequence of Selectors, each having its generated string
     * and a tag describing what selector it represents, and, optionally, what string is attached.
     * This can then easily be turned into the input (the string is attached) and the expected
     * parser result (transform the sequence of tags).
     */

    #[derive(Debug, Clone)]
    enum SelectorTag {
        WildcardChild,
        Child(String),
        WildcardDescendant,
        Descendant(String),
        ArrayIndexChild(JsonUInt),
        ArrayIndexDescendant(JsonUInt),
    }

    #[derive(Debug, Clone)]
    struct Selector {
        string: String,
        tag: SelectorTag,
    }

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
                JsonStringToken::EncodeNormally('\u{0008}') => r"\b".to_owned(),
                JsonStringToken::EncodeNormally('\t') => r"\t".to_owned(),
                JsonStringToken::EncodeNormally('\n') => r"\n".to_owned(),
                JsonStringToken::EncodeNormally('\u{000C}') => r"\f".to_owned(),
                JsonStringToken::EncodeNormally('\r') => r"\r".to_owned(),
                JsonStringToken::EncodeNormally('"') => match mode {
                    JsonStringTokenEncodingMode::DoubleQuoted => r#"\""#.to_owned(),
                    JsonStringTokenEncodingMode::SingleQuoted => r#"""#.to_owned(),
                },
                JsonStringToken::EncodeNormally('\'') => match mode {
                    JsonStringTokenEncodingMode::DoubleQuoted => r#"'"#.to_owned(),
                    JsonStringTokenEncodingMode::SingleQuoted => r#"\'"#.to_owned(),
                },
                JsonStringToken::EncodeNormally('/') => r"\/".to_owned(),
                JsonStringToken::EncodeNormally('\\') => r"\\".to_owned(),
                JsonStringToken::EncodeNormally(c @ ..='\u{001F}') => encode_unicode_escape(c),
                JsonStringToken::EncodeNormally(c) => c.to_string(),
                JsonStringToken::ForceUnicodeEscape(c) => encode_unicode_escape(c),
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

    // Cspell: disable
    fn any_selector() -> impl Strategy<Value = Selector> {
        prop_oneof![
            any_wildcard_child(),
            any_child(),
            any_wildcard_descendant(),
            any_descendant(),
            any_array_index_child(),
            any_array_index_descendant(),
        ]
    }

    // .* or [*]
    fn any_wildcard_child() -> impl Strategy<Value = Selector> {
        r"(\.\*|\[\*\])".prop_map(|x| Selector {
            string: x,
            tag: SelectorTag::WildcardChild,
        })
    }

    // ..* or ..[*]
    fn any_wildcard_descendant() -> impl Strategy<Value = Selector> {
        r"(\*|\[\*\])".prop_map(|x| Selector {
            string: format!("..{x}"),
            tag: SelectorTag::WildcardDescendant,
        })
    }

    // .label or ['label']
    fn any_child() -> impl Strategy<Value = Selector> {
        prop_oneof![any_short_name().prop_map(|x| (format!(".{x}"), x)), any_name(),].prop_map(|(s, l)| Selector {
            string: s,
            tag: SelectorTag::Child(l),
        })
    }

    // ..label or ..['label']
    fn any_descendant() -> impl Strategy<Value = Selector> {
        prop_oneof![any_short_name().prop_map(|x| (x.clone(), x)), any_name(),].prop_map(|(x, l)| Selector {
            string: format!("..{x}"),
            tag: SelectorTag::Descendant(l),
        })
    }

    fn any_array_index_child() -> impl Strategy<Value = Selector> {
        any_non_negative_array_index().prop_map(|i| Selector {
            string: format!("[{}]", i.as_u64()),
            tag: SelectorTag::ArrayIndexChild(i),
        })
    }

    fn any_array_index_descendant() -> impl Strategy<Value = Selector> {
        any_non_negative_array_index().prop_map(|i| Selector {
            string: format!("..[{}]", i.as_u64()),
            tag: SelectorTag::ArrayIndexDescendant(i),
        })
    }

    fn any_short_name() -> impl Strategy<Value = String> {
        r"([A-Za-z]|_|[^\u0000-\u007F])([A-Za-z0-9]|_|[^\u0000-\u007F])*"
    }

    fn any_name() -> impl Strategy<Value = (String, String)> {
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
                (format!("[{q}{s}{q}]"), l)
            })
        })
    }

    fn any_non_negative_array_index() -> impl Strategy<Value = JsonUInt> {
        const MAX: u64 = (1 << 53) - 1;
        (0..MAX).prop_map(|x| JsonUInt::try_from(x).expect("in-range JsonUInt"))
    }
    // Cspell: enable

    prop_compose! {
        fn any_valid_query()(has_root in any::<bool>(), selectors in prop::collection::vec(any_selector(), 0..20)) -> (String, JsonPathQuery) {
            let mut result: String = String::new();
            let mut query = JsonPathQueryBuilder::new();

            if has_root {
                result += "$";
            }

            for selector in selectors {
                result += &selector.string;

                query = match selector.tag {
                    SelectorTag::WildcardChild => query.any_child(),
                    SelectorTag::Child(name) => query.child(JsonString::new(&name)),
                    SelectorTag::WildcardDescendant => query.any_descendant(),
                    SelectorTag::Descendant(name) => query.descendant(JsonString::new(&name)),
                    SelectorTag::ArrayIndexChild(idx) => query.array_index_child(idx),
                    SelectorTag::ArrayIndexDescendant(idx) => query.array_index_descendant(idx)
                };
            }

            (result, query.into())
        }
    }

    mod correct_strings {
        use super::*;
        use pretty_assertions::assert_eq;

        proptest! {
            #[test]
            fn parses_expected_query((input, expected) in any_valid_query()) {
                let result = JsonPathQuery::parse(&input).expect("expected Ok");

                assert_eq!(expected, result);
            }
        }
    }
}
