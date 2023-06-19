use pretty_assertions::assert_eq;
use rsonpath::engine::{main::MainEngine, Compiler, Engine};
use rsonpath::input::*;
use rsonpath::query::JsonPathQuery;
use rsonpath::result::*;
use std::error::Error;
#[test]
fn document_short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_query_select_exact_path_with_name_and_index_selectors_input_owned_input_result_count_result(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 1usize);
    Ok(())
}
#[test]
fn document_short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_query_select_exact_path_with_name_and_index_selectors_input_owned_input_result_index_result(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $.phoneNumbers[0].type (select exact path with name and index selectors) with Input impl OwnedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$.phoneNumbers[0].type")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![239usize,]);
    Ok(())
}
#[test]
fn document_short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_query_descendant_search_for_number_input_owned_input_result_count_result(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedInput and result mode CountResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, CountResult>(&input)?;
    assert_eq!(result.get(), 2usize);
    Ok(())
}
#[test]
fn document_short_json_with_objects_and_lists_given_as_an_example_on_jsonpath_com_query_descendant_search_for_number_input_owned_input_result_index_result(
) -> Result<(), Box<dyn Error>> {
    println ! ("on document jsonpath_com_example.toml running the query $..number (descendant search for 'number') with Input impl OwnedInput and result mode IndexResult");
    let jsonpath_query = JsonPathQuery::parse("$..number")?;
    let raw_json = "{\n    \"firstName\": \"John\",\n    \"lastName\": \"doe\",\n    \"age\": 26,\n    \"address\": {\n        \"streetAddress\": \"naist street\",\n        \"city\": \"Nara\",\n        \"postalCode\": \"630-0192\"\n    },\n    \"phoneNumbers\": [\n        {\n            \"type\": \"iPhone\",\n            \"number\": \"0123-4567-8888\"\n        },\n        {\n            \"type\": \"home\",\n            \"number\": \"0123-4567-8910\"\n        }\n    ]\n}\n" ;
    let input = OwnedBytes::new(&raw_json.as_bytes())?;
    let engine = MainEngine::compile_query(&jsonpath_query)?;
    let result = engine.run::<_, IndexResult>(&input)?;
    assert_eq!(result.get(), vec![271usize, 359usize,]);
    Ok(())
}
