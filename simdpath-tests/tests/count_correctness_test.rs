use simdpath_core::engine::runner::Runner;
use simdpath_core::query::parse_json_path_query;
use simdpath_stack_based::StackBasedRunner;
use simdpath_stackless::{compile_query, run_simdpath5};
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

fn get_contents(test_path: &str) -> String {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    fs::read_to_string(path).unwrap()
}

#[test]
fn simdpath_stackless_small_no_list() {
    let contents = get_contents("small_no_list.json");
    let query = parse_json_path_query("$..person..phoneNumber..number").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(2, result.count);
}

#[test]
fn simdpath_stackless_small() {
    let contents = get_contents("small.json");
    let query = parse_json_path_query("$..person..phoneNumber..number").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(4, result.count);
}

#[test]
fn simdpath_stackless_twitter() {
    let contents = get_contents("twitter.json");
    let query = parse_json_path_query("$..user..entities..url").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(44, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_claims_references_hash() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..claims..references..hash").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(37736, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_references_snaks_datavalue() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..references..snaks..datavalue").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_references_snaks_datavalue_value() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..references..snaks..datavalue..value").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_references_snaks_datavalue_value_id() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..references..snaks..datavalue..value..id").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(11113, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_snaks_datavalue_value() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..snaks..datavalue..value").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_datavalue_value_id() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..datavalue..value..id").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(25093, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_mainsnak_datavalue_value() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..mainsnak..datavalue..value").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(26115, result.count);
}

#[test]
fn simdpath_stackless_wikidata_person_mainsnak_datavalue_value_id() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..mainsnak..datavalue..value..id").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(12958, result.count);
}

#[test]
fn simdpath_stackless_wikidata_profession() {
    let contents = get_contents("wikidata/wikidata_profession.json");
    let query = parse_json_path_query("$..claims..mainsnak..value").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(59112, result.count);
}

#[test]
fn simdpath_stackless_wikidata_properties() {
    let contents = get_contents("wikidata/wikidata_properties.json");
    let query = parse_json_path_query("$..qualifiers..datavalue..id").unwrap();
    let result = compile_query(&query).count(&contents);

    assert_eq!(18219, result.count);
}

#[test]
fn simdpath_stack_based_small_no_list() {
    let contents = get_contents("small_no_list.json");
    let query = parse_json_path_query("$..person..phoneNumber..number").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(2, result.count);
}

#[test]
fn simdpath_stack_based_small() {
    let contents = get_contents("small.json");
    let query = parse_json_path_query("$..person..phoneNumber..number").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(4, result.count);
}

#[test]
fn simdpath_stack_based_twitter() {
    let contents = get_contents("twitter.json");
    let query = parse_json_path_query("$..user..entities..url").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(44, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_claims_references_hash() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..claims..references..hash").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(37736, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_snaks_datavalue() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..references..snaks..datavalue").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_snaks_datavalue_value() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..references..snaks..datavalue..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_snaks_datavalue_value_id() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..references..snaks..datavalue..value..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(11113, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_snaks_datavalue_value() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..snaks..datavalue..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25118, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_datavalue_value_id() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..datavalue..value..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(25093, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_mainsnak_datavalue_value() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..mainsnak..datavalue..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(26115, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_person_references_mainsnak_datavalue_value_id() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..mainsnak..datavalue..value..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(12958, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_profession() {
    let contents = get_contents("wikidata/wikidata_profession.json");
    let query = parse_json_path_query("$..claims..mainsnak..value").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(59112, result.count);
}

#[test]
fn simdpath_stack_based_wikidata_properties() {
    let contents = get_contents("wikidata/wikidata_properties.json");
    let query = parse_json_path_query("$..qualifiers..datavalue..id").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(18219, result.count);
}
