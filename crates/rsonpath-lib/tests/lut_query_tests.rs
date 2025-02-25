use log::debug;
use rsonpath::{
    engine::{
        skip_tracker::{self, save_track_to_csv, SkipMode},
        Compiler, Engine, RsonpathEngine,
    },
    input::OwnedBytes,
    lookup_table::{LookUpTable, LookUpTableImpl},
};
use std::{
    error::Error,
    fs,
    io::{self, BufReader, Read, Write},
    path::Path,
    process::Command,
};

// JSON files
const JOHN_BIG_JSON: &str = "../../.a_lut_tests/test_data/kB_1/john_big.json";

const POKEMON_JSON: &str = "../../.a_lut_tests/test_data/MB_15/pokemon_(6MB).json";

const TWITTER_SHORT_JSON: &str = "../../.a_lut_tests/test_data/MB_100/twitter_short_(80MB).json";
const BESTBUY_SHORT_JSON: &str = "../../.a_lut_tests/test_data/MB_100/bestbuy_short_(103MB).json";
const GOOGLE_MAP_SHORT_JSON: &str = "../../.a_lut_tests/test_data/MB_100/google_map_short_(107MB).json";
const WALMART_SHORT_JSON: &str = "../../.a_lut_tests/test_data/MB_100/walmart_short_(95MB).json";

const BESTBUY_JSON: &str = "../../.a_lut_tests/test_data/GB_1/bestbuy_large_record_(1GB).json";
const WALMART_JSON: &str = "../../.a_lut_tests/test_data/GB_1/walmart_large_record_(995MB).json";
const TWITTER_JSON: &str = "../../.a_lut_tests/test_data/GB_1/twitter_large_record_(843MB).json";
const GOOGLE_MAP_JSON: &str = "../../.a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json";

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
