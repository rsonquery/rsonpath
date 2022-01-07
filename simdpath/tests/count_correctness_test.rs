use simdpath::engine::{Input, Runner};
use simdpath::query::JsonPathQuery;
use simdpath::stack_based::StackBasedRunner;
use simdpath::stackless::StacklessRunner;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

fn get_contents(test_path: &str) -> Input {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    let raw = fs::read_to_string(path).unwrap();
    Input::new(raw)
}

#[test]
fn simdpath_stackless_small_no_list() {
    let contents = get_contents("basic_compressed/small_no_list.json");
    let query = JsonPathQuery::parse("$..person..phoneNumber..number").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(2, result.count);
}

#[test]
fn simdpath_stackless_small() {
    let contents = get_contents("basic_compressed/small.json");
    let query = JsonPathQuery::parse("$..person..phoneNumber..number").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(4, result.count);
}

#[test]
fn simdpath_stackless_twitter() {
    let contents = get_contents("basic_compressed/twitter.json");
    let query = JsonPathQuery::parse("$..user..entities..url").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(44, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_claims_references_hash() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..claims..references..hash").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(37736, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_references_snaks_datavalue() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..references..snaks..datavalue").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_references_snaks_datavalue_value() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..references..snaks..datavalue..value").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_references_snaks_datavalue_value_id() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(11113, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_snaks_datavalue_value() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..snaks..datavalue..value").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_datavalue_value_id() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..datavalue..value..id").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(25093, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_mainsnak_datavalue_value() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..mainsnak..datavalue..value").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(26115, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_mainsnak_datavalue_value_id() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(12958, result.count);
}

#[test]
fn simdpath_stackless_wikidata_profession() {
    let contents = get_contents("wikidata_compressed/wikidata_profession.json");
    let query = JsonPathQuery::parse("$..claims..mainsnak..value").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(59112, result.count);
}

#[test]
fn simdpath_stackless_wikidata_properties() {
    let contents = get_contents("wikidata_compressed/wikidata_properties.json");
    let query = JsonPathQuery::parse("$..qualifiers..datavalue..id").unwrap();
    let result = StacklessRunner::compile_query(&query).count(&contents);

    assert_eq!(18219, result.count);
}

#[test]
fn simdpath_stack_based_small_no_list() {
    let contents = get_contents("basic_compressed/small_no_list.json");
    let query = JsonPathQuery::parse("$..person..phoneNumber..number").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(2, result.count);
}

#[test]
fn simdpath_stack_based_small() {
    let contents = get_contents("basic_compressed/small.json");
    let query = JsonPathQuery::parse("$..person..phoneNumber..number").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(4, result.count);
}

#[test]
fn simdpath_stack_based_twitter() {
    let contents = get_contents("basic_compressed/twitter.json");
    let query = JsonPathQuery::parse("$..user..entities..url").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(44, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_claims_references_hash() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..claims..references..hash").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(37736, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_snaks_datavalue() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..references..snaks..datavalue").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_snaks_datavalue_value() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..references..snaks..datavalue..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_snaks_datavalue_value_id() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..references..snaks..datavalue..value..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(11113, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_snaks_datavalue_value() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..snaks..datavalue..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_datavalue_value_id() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..datavalue..value..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25093, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_mainsnak_datavalue_value() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..mainsnak..datavalue..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(26115, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_mainsnak_datavalue_value_id() {
    let contents = get_contents("wikidata_compressed/wikidata_person.json");
    let query = JsonPathQuery::parse("$..mainsnak..datavalue..value..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(12958, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_profession() {
    let contents = get_contents("wikidata_compressed/wikidata_profession.json");
    let query = JsonPathQuery::parse("$..claims..mainsnak..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(59112, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_properties() {
    let contents = get_contents("wikidata_compressed/wikidata_properties.json");
    let query = JsonPathQuery::parse("$..qualifiers..datavalue..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(18219, result.count);
}
