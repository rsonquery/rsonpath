use simdpath_core::engine::runner::Runner;
use simdpath_core::query::parse_json_path_query;
use simdpath_stack_based::StackBasedRunner;
use simdpath_stackless::run_simdpath3;
use std::fs;

const ROOT_TEST_DIRECTORY: &str = "./data";

fn get_contents(test_path: &str) -> String {
    let path = format!("{}/{}", ROOT_TEST_DIRECTORY, test_path);
    fs::read_to_string(path).unwrap()
}

#[test]
fn simdpath_stackless_small_no_list() {
    let contents = get_contents("small_no_list.json");
    let result = run_simdpath3(&contents, "person", "phoneNumber", "number");

    assert_eq!(2, result);
}

#[test]
fn simdpath_stackless_small() {
    let contents = get_contents("small.json");
    let result = run_simdpath3(&contents, "person", "phoneNumber", "number");

    assert_eq!(4, result);
}

#[test]
fn simdpath_stackless_twitter() {
    let contents = get_contents("twitter.json");
    let result = run_simdpath3(&contents, "user", "entities", "url");

    assert_eq!(44, result);
}

#[test]
fn simdpath_stackless_wikidata_person() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let result = run_simdpath3(&contents, "claims", "references", "hash");

    assert_eq!(37736, result);
}

#[test]
fn simdpath_stackless_wikidata_profession() {
    let contents = get_contents("wikidata/wikidata_profession.json");
    let result = run_simdpath3(&contents, "claims", "mainsnak", "value");

    assert_eq!(59112, result);
}

#[test]
fn simdpath_stackless_wikidata_properties() {
    let contents = get_contents("wikidata/wikidata_properties.json");
    let result = run_simdpath3(&contents, "qualifiers", "datavalue", "id");

    assert_eq!(18219, result);
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
fn simdpath_stack_based_wikidata_person() {
    let contents = get_contents("wikidata/wikidata_person.json");
    let query = parse_json_path_query("$..claims..references..hash").unwrap();
    let result = StackBasedRunner::compile_query(&query).count(&contents);

    assert_eq!(37736, result.count);
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
