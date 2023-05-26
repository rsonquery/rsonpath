use pretty_assertions::assert_eq;
use rsonpath_lib::query::{builder::JsonPathQueryBuilder, JsonPathQuery, JsonString};

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
        .array_index_descendant(5.try_into().unwrap())
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn nonnegative_array_indexed_selector() {
    let input = "$[5]";
    let expected_query = JsonPathQueryBuilder::new()
        .array_index_child(5.try_into().unwrap())
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn multiple_nonnegative_array_indexed_selector() {
    let input = "$[5][2]";
    let expected_query = JsonPathQueryBuilder::new()
        .array_index_child(5.try_into().unwrap())
        .array_index_child(2.try_into().unwrap())
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn zeroth_array_indexed_selector() {
    let input = "$[0]";
    let expected_query = JsonPathQueryBuilder::new()
        .array_index_child(0.try_into().unwrap())
        .into();

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
fn wildcard_descendant_selector_test() {
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
fn indexed_wildcard_descendant_selector_nested_test() {
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
    let input = r#"['\'']"#;
    let expected_query = JsonPathQueryBuilder::new().child(JsonString::new("'")).into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn unescaped_double_quote_in_single_quote_member() {
    let input = r#"['"']"#;
    let expected_query = JsonPathQueryBuilder::new().child(JsonString::new(r#"\""#)).into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

/// Turn escapes of `'` and `\` into unescaped forms, and unescaped
/// `"` into escaped. So `\'` becomes `'`, and `"` into `\"`, but `\n` stays as `\n`.
///
/// This is how we expect strings to be parsed.
fn transform_json_escape_sequences(str: String) -> String {
    let mut result = String::new();
    let mut escaped = false;

    for char in str.chars() {
        escaped = match char {
            '\'' | '/' if escaped => {
                result.push(char);
                false
            }
            '"' if !escaped => {
                result.push('\\');
                result.push('"');
                false
            }
            _ if escaped => {
                result.push('\\');
                result.push(char);
                false
            }
            '\\' => true,
            _ => {
                result.push(char);
                false
            }
        };
    }

    result
}

// Just a few sanity tests for the unescape_unnecessary_escapes helper.
mod transform_json_escape_sequences_tests {
    use test_case::test_case;

    #[test_case("" => ""; "empty is unchanged")]
    #[test_case("abc" => "abc")]
    #[test_case(r#"['\n']"# => r#"['\n']"#; "endline is unchanged")]
    #[test_case(r#"['\t']"# => r#"['\t']"#; "tab is unchanged")]
    #[test_case(r#"['\\']"# => r#"['\\']"#; "backslash is unchanged")]
    #[test_case(r#"['\'']"# => r#"[''']"#; "single quote is unescaped")]
    #[test_case(r#"['"']"# => r#"['\"']"#; "unescaped double quote is escaped")]
    #[test_case(r#"['\"']"# => r#"['\"']"#; "escaped double quote is unchanged")]
    #[test_case(r#"['\/']"# => r#"['/']"#; "slash is unescaped")]
    #[test_case(r#"['\\"']"# => r#"['\\\"']"#; "escapes don't flow")]
    #[test_case(r#"['\\'']"# => r#"['\\'']"#; "escapes don't flow2")]
    fn cases(input: &str) -> String {
        super::transform_json_escape_sequences(input.to_owned())
    }
}

mod proptests {
    use super::*;
    use proptest::prelude::*;
    use rsonpath_lib::query::NonNegativeArrayIndex;

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
        ArrayIndexChild(NonNegativeArrayIndex),
        ArrayIndexDescendant(NonNegativeArrayIndex),
    }

    #[derive(Debug, Clone)]
    struct Selector {
        string: String,
        tag: SelectorTag,
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
        r#"(\.\*|\[\*\])"#.prop_map(|x| Selector {
            string: x,
            tag: SelectorTag::WildcardChild,
        })
    }

    // ..* or ..[*]
    fn any_wildcard_descendant() -> impl Strategy<Value = Selector> {
        r#"(\*|\[\*\])"#.prop_map(|x| Selector {
            string: format!("..{x}"),
            tag: SelectorTag::WildcardDescendant,
        })
    }

    // .label or ['label']
    fn any_child() -> impl Strategy<Value = Selector> {
        prop_oneof![any_member().prop_map(|x| (format!(".{x}"), x)), any_name(),].prop_map(|(s, l)| Selector {
            string: s,
            tag: SelectorTag::Child(l),
        })
    }

    // ..label or ..['label']
    fn any_descendant() -> impl Strategy<Value = Selector> {
        prop_oneof![any_member().prop_map(|x| (x.clone(), x)), any_name(),].prop_map(|(x, l)| Selector {
            string: format!("..{x}"),
            tag: SelectorTag::Descendant(l),
        })
    }

    fn any_array_index_child() -> impl Strategy<Value = Selector> {
        any_non_negative_array_index().prop_map(|i| Selector {
            string: format!("[{}]", i.get_index()),
            tag: SelectorTag::ArrayIndexChild(i),
        })
    }

    fn any_array_index_descendant() -> impl Strategy<Value = Selector> {
        any_non_negative_array_index().prop_map(|i| Selector {
            string: format!("..[{}]", i.get_index()),
            tag: SelectorTag::ArrayIndexDescendant(i),
        })
    }

    fn any_member() -> impl Strategy<Value = String> {
        r#"([A-Za-z]|_|[^\u0000-\u007F])([A-Za-z0-9]|_|[^\u0000-\u007F])*"#
    }

    fn any_name() -> impl Strategy<Value = (String, String)> {
        any_quoted_member().prop_map(|(s, l)| (format!("[{s}]"), l))
    }

    fn any_quoted_member() -> impl Strategy<Value = (String, String)> {
        prop_oneof![
            any_single_quoted_member().prop_map(|x| (format!("'{x}'"), x)),
            any_double_quoted_member().prop_map(|x| (format!("\"{x}\""), x))
        ]
    }

    fn any_single_quoted_member() -> impl Strategy<Value = String> {
        r#"([^'"\\\u0000-\u001F]|(\\[btnfr/\\])|["]|(\\'))*"#
    }

    fn any_double_quoted_member() -> impl Strategy<Value = String> {
        r#"([^'"\\\u0000-\u001F]|(\\[btnfr/\\])|[']|(\\"))*"#
    }

    fn any_non_negative_array_index() -> impl Strategy<Value = NonNegativeArrayIndex> {
        const MAX: u64 = (1 << 53) - 1;
        (0..MAX).prop_map(NonNegativeArrayIndex::new)
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
                    SelectorTag::Child(name) => query.child(JsonString::new(&transform_json_escape_sequences(name))),
                    SelectorTag::WildcardDescendant => query.any_descendant(),
                    SelectorTag::Descendant(name) => query.descendant(JsonString::new(&transform_json_escape_sequences(name))),
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
