use pretty_assertions::assert_eq;
use rsonpath_lib::query::{builder::JsonPathQueryBuilder, JsonPathQuery, Label};

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
        .child(Label::new("a"))
        .any_child()
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn indexed_wildcard_child_selector() {
    let input = r#"$[*]['*']["*"]"#;
    let expected_query = JsonPathQueryBuilder::new()
        .any_child()
        .child(Label::new("*"))
        .child(Label::new("*"))
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn wildcard_descendant_selector_test() {
    let input = "$..*.a..*";
    let expected_query = JsonPathQueryBuilder::new()
        .any_descendant()
        .child(Label::new("a"))
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
        .descendant(Label::new("*"))
        .descendant(Label::new("*"))
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn escaped_single_quote_in_single_quote_label() {
    let input = r#"['\'']"#;
    let expected_query = JsonPathQueryBuilder::new().child(Label::new("'")).into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

#[test]
fn unescaped_double_quote_in_single_quote_label() {
    let input = r#"['"']"#;
    let expected_query = JsonPathQueryBuilder::new()
        .child(Label::new(r#"\""#))
        .into();

    let result = JsonPathQuery::parse(input).expect("expected Ok");

    assert_eq!(result, expected_query);
}

/// Turn escapes of `'` and `\` into unescaped forms, and unescaped
/// `"` into escaped. So `\'` becomes `'`, and `"` into `\"`, but `\n` stays as `\n`.
///
/// This is how we expect labels to be parsed.
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

    /* Approach: we generate a sequence of Selectors, each having its generated string
     * and a tag describing what selector it represents, and, optionally, what label is attached.
     * This can then easily be turned into the input (the string is attached) and the expected
     * parser result (transform the sequence of tags).
     */

    #[derive(Debug, Clone)]
    enum SelectorTag {
        WildcardChild,
        Child(String),
        WildcardDescendant,
        Descendant(String),
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
        prop_oneof![any_label().prop_map(|x| (format!(".{x}"), x)), any_index(),].prop_map(
            |(s, l)| Selector {
                string: s,
                tag: SelectorTag::Child(l),
            },
        )
    }

    // ..label or ..['label']
    fn any_descendant() -> impl Strategy<Value = Selector> {
        prop_oneof![any_label().prop_map(|x| (x.clone(), x)), any_index(),].prop_map(|(x, l)| {
            Selector {
                string: format!("..{x}"),
                tag: SelectorTag::Descendant(l),
            }
        })
    }

    fn any_label() -> impl Strategy<Value = String> {
        r#"([A-Za-z]|_|[^\u0000-\u007F])([A-Za-z0-9]|_|[^\u0000-\u007F])*"#
    }

    fn any_index() -> impl Strategy<Value = (String, String)> {
        any_quoted_label().prop_map(|(s, l)| (format!("[{s}]"), l))
    }

    fn any_quoted_label() -> impl Strategy<Value = (String, String)> {
        prop_oneof![
            any_single_quoted_label().prop_map(|x| (format!("'{x}'"), x)),
            any_double_quoted_label().prop_map(|x| (format!("\"{x}\""), x))
        ]
    }

    fn any_single_quoted_label() -> impl Strategy<Value = String> {
        r#"([^'"\\\u0000-\u001F]|(\\[btnfr/\\])|["]|(\\'))*"#
    }

    fn any_double_quoted_label() -> impl Strategy<Value = String> {
        r#"([^'"\\\u0000-\u001F]|(\\[btnfr/\\])|[']|(\\"))*"#
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
                    SelectorTag::Child(label) => query.child(Label::new(&transform_json_escape_sequences(label))),
                    SelectorTag::WildcardDescendant => query.any_descendant(),
                    SelectorTag::Descendant(label) => query.descendant(Label::new(&transform_json_escape_sequences(label))),
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
