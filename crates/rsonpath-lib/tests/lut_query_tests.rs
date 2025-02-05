use log::debug;
use rsonpath::{
    engine::{distance_counter::save_to_csv, Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{LookUpTable, LookUpTableImpl},
};
use std::{
    error::Error,
    fs,
    io::{BufReader, Read},
    path::Path,
};

// JSON files
const JOHN_BIG_JSON: &str = "tests/json/john_big.json";
const POKEMON_JSON: &str = "../../.a_lut_tests/test_data/MB_15/pokemon_(6MB).json";
const TWITTER_SHORT_JSON: &str = "tests/json/twitter_short_(80MB).json";
const BESTBUY_JSON: &str = "tests/json/bestbuy_short_(103MB).json";
const ERROR_1_JSON: &str = "tests/json/error_1.json";

/// Run this with:
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
    let lut = LookUpTableImpl::build(json_path)?;

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

// #########
// # COUNT #
// #########

// cargo test --test lut_query_tests -- count_skips_pokemon --nocapture | rg "(tail_skipping|lut_query_tests|distance_counter)"
#[test]
fn count_skips_pokemon_q0() -> Result<(), Box<dyn Error>> {
    query_count_skips(POKEMON_JSON, "q0", "$.cfgs[0].Name", vec![49])
}

fn query_count_skips(
    json_path: &str,
    query_name: &str,
    query_text: &str,
    expected_result: Vec<usize>,
) -> Result<(), Box<dyn Error>> {
    // Just do the test as normal but now its tracking the skips
    let result = query_with_lut(json_path, query_text, expected_result);

    // Save the tracked skips to a csv
    let file_name = get_filename(json_path);
    let file_path = format!(
        "../../.a_lut_tests/performance/query_distribution/{}_{}.csv",
        file_name, query_name
    );
    let save_result = save_to_csv(&file_path);
    if let Err(e) = save_result {
        eprintln!("Failed to save to CSV: {}", e);
    }

    // Plot that
    // TODO

    result
}

fn get_filename(path: &str) -> &str {
    Path::new(path).file_stem().and_then(|name| name.to_str()).unwrap_or("")
}
