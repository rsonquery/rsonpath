use pretty_assertions::assert_eq;
use rsonpath_syntax::{builder::JsonPathQueryBuilder, num::JsonUInt, str::JsonString};
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
