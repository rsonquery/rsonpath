use log::debug;
use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{
        performance::lut_query_data::{QUERY_BESTBUY, QUERY_GOOGLE, QUERY_POKEMON_SHORT, QUERY_TWITTER},
        LookUpTable, LookUpTableImpl,
    },
};
use std::{
    error::Error,
    fs,
    io::{BufReader, Read},
};

// JSON files
const JOHN_BIG_JSON: &str = "../../.a_lut_tests/test_data/kB_1/john_big.json";
const POKEMON_JSON: &str = "../../.a_lut_tests/test_data/MB_15/pokemon_(6MB).json";

/// cargo test --test lut_query_tests -- query_john_big_log --nocapture | rg "(tail_skipping|lut_query_tests|distance_counter)"
#[test]
fn query_john_big_log() -> Result<(), Box<dyn Error>> {
    debug!("Starting test for query_john_big");
    query_with_lut(JOHN_BIG_JSON, "$.person.spouse.person.phoneNumber[*]", vec![858, 996])
}

#[test]
fn query_pokemon() -> Result<(), Box<dyn Error>> {
    query_with_lut(POKEMON_JSON, "$.cfgs[0].Name", vec![49])
}

fn query_with_lut(json_path: &str, query_text: &str, expected_result: Vec<usize>) -> Result<(), Box<dyn Error>> {
    // Enables to see log messages when running tests
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    // Build lut
    let lut = LookUpTableImpl::build(json_path, 0)?;

    // Build query
    let query = rsonpath_syntax::parse(query_text)?;
    let mut engine = RsonpathEngine::compile_query(&query)?;
    engine.add_lut(lut);

    // Get results
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path)?);
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        // Here you can define whether to use OwnedBytes (padding), Mmap (padding = 0)  or Borrowed (padding)
        OwnedBytes::new(buf)
    };
    let mut sink = vec![];
    engine.matches(&input, &mut sink)?;
    let results = sink.into_iter().map(|m| m.span().start_idx()).collect::<Vec<_>>();

    // Compare expected result with result
    debug!("Found:    {:?}", results);
    debug!("Expected: {:?}", expected_result);
    assert_eq!(expected_result, results);

    Ok(())
}

/// cargo test --test lut_query_tests -- full_pokemon_short --nocapture | rg "(lut_query_tests)"
#[test]
fn full_pokemon_short() -> Result<(), Box<dyn Error>> {
    test_all_queries(QUERY_POKEMON_SHORT)
}

#[test]
fn full_google() -> Result<(), Box<dyn Error>> {
    test_all_queries(QUERY_GOOGLE)
}

#[test]
fn full_bestbuy() -> Result<(), Box<dyn Error>> {
    test_all_queries(QUERY_BESTBUY)
}

#[test]
fn full_twitter() -> Result<(), Box<dyn Error>> {
    test_all_queries(QUERY_TWITTER)
}

fn test_all_queries(test_data: (&str, &[(&str, &str)])) -> Result<(), Box<dyn Error>> {
    // Enables to see log messages when running tests
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    // Build LUT once at the beginning
    let (json_file_path, queries) = test_data;
    let json_path = format!("../../{}", json_file_path);
    debug!("Building LUT: {}", json_path);
    let mut lut = LookUpTableImpl::build(&json_path, 0).expect("Fail @ building LUT");

    // Run all queries
    for &(query_name, query_text) in queries {
        debug!("Query: {}", query_name);
        lut = compare_results_lut_vs_ite(lut, &json_path, query_text).expect("Fail @ compare_results");
    }

    Ok(())
}

fn compare_results_lut_vs_ite(
    lut: LookUpTableImpl,
    json_path: &str,
    query_text: &str,
) -> Result<LookUpTableImpl, Box<dyn Error>> {
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
        let mut buf = vec![];
        file.read_to_end(&mut buf).expect("Fail @ file read");
        OwnedBytes::new(buf)
    };
    let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");

    // Query normally and skip iteratively (ITE)
    let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
    let result = engine.count(&input).expect("Failed to run query normally");

    // Query normally and skip using the lookup table (LUT)
    engine.add_lut(lut);
    let lut_result = engine.count(&input).expect("LUT: Failed to run query normally");

    assert_eq!(lut_result, result);

    Ok(engine.take_lut().expect("Failed to retrieve LUT from engine"))
}
