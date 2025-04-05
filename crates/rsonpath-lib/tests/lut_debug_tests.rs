use std::{
    fs,
    io::{BufReader, Read},
};

use log::debug;
use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{
        lut_hash_map,
        lut_phf_double::LutPHFDouble,
        lut_phf_group::LutPHFGroup,
        pair_finder,
        performance::{
            lut_query_data::{ALPHABET, GOOGLE, POKEMON_MINI, TWITTER_SHORT},
            lut_skip_evaluation::DISTANCE_CUT_OFF,
        },
        LookUpTable, LUT,
    },
};

/// cargo test --test lut_debug_tests -- test_build_and_queries --nocapture | rg "(lut_debug_tests)"
/// cargo test --test lut_debug_tests -- test_build_and_queries --nocapture | rg "(tail_skipping|lut_debug_tests|main)"
/// cargo test --test lut_debug_tests -- test_build_and_queries --nocapture | rg ^"(tail_skipping|lut_debug_tests|main|lut_hash_map)"
#[test]
fn test_build_and_queries() {
    // Enables to see log messages when running tests
    // simple_logger::SimpleLogger::new()
    //     .with_level(log::LevelFilter::Debug)
    //     .without_timestamps()
    //     .init()
    //     .unwrap();

    debug!("Start");

    test_build_correctness(GOOGLE);
    // test_build_correctness(WALMART);
    // test_build_correctness(BESTBUY);
    // test_build_correctness(TWITTER);
    // test_build_correctness(POKEMON_MINI);
    // test_build_correctness(TWITTER_SHORT);
    // test_build_correctness(ALPHABET);

    // test_query_correctness(QUERY_BUGS);
    // test_query_correctness(QUERY_JOHN_BIG);
    // test_query_correctness(QUERY_POKEMON_MINI);
    // test_query_correctness(QUERY_GOOGLE);
    // test_query_correctness(QUERY_TWITTER);
    // test_query_correctness(QUERY_BESTBUY);
    // test_query_correctness(QUERY_POKEMON_SHORT);
}

fn test_build_correctness(json_name: &str) {
    let s = format!("../../{}", json_name);
    let json_path = s.as_str();

    debug!("Building LUT (Hashmap): {}", json_path);
    let lut_hash_map = lut_hash_map::LutHashMap::build(&json_path, 0).expect("Fail @ building LUT");
    debug!("Building LUT: {}", json_path);
    let lut = LUT::build(&json_path, 0).expect("Fail @ building LUT");

    debug!("Testing keys ...");
    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    let mut count_incorrect = 0;
    for (i, key) in keys.iter().enumerate() {
        let value = lut.get(key).expect("Fail at getting value.");
        let value_hash = lut_hash_map.get(key).expect("Fail at getting value.");
        if value != values[i] || value != value_hash {
            count_incorrect += 1;
            debug!(
                "  i: {}, Key {}, Value {}, Expected: {}, Hash {}",
                i, key, value, values[i], value_hash
            );
        }
    }

    debug!(" Correct {}/{}", keys.len() - count_incorrect, keys.len());
    debug!(" Incorrect {}/{}", count_incorrect, keys.len());

    std::mem::drop(lut);
}

fn test_query_correctness(test_data: (&str, &[(&str, &str)])) {
    let (json_name, queries) = test_data;
    let s = format!("../../{}", json_name);
    let json_path = s.as_str();
    debug!("Building LUT: {}", json_path);
    let mut lut = LUT::build(&json_path, DISTANCE_CUT_OFF).expect("Fail @ building LUT");

    // Run all queries
    debug!("Checking queries:");
    for &(query_name, query_text) in queries {
        debug!(" Query: {} = \"{}\" ... ", query_name, query_text);
        let input = {
            let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
            let mut buf = vec![];
            file.read_to_end(&mut buf).expect("Fail @ file read");
            OwnedBytes::new(buf)
        };
        let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");

        // Query normally and skip iteratively (ITE)
        debug!("---- ITE STYLE ----");
        let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
        let count = engine.count(&input).expect("Failed to run query normally");

        // Query normally and skip using the lookup table (LUT)
        debug!("---- LUT STYLE ----");
        engine.add_lut(lut);
        let lut_count = engine.count(&input).expect("LUT: Failed to run query normally");

        if lut_count != count {
            debug!("Found {}, Expected {}", lut_count, count);
        } else {
            debug!("Correct");
        }

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine");
    }

    std::mem::drop(lut);
}

// cargo test --test lut_build_tests -- debug_lut_group_buckets --nocapture | rg "(lut_build_tests|lut_phf_group)"
#[test]
fn debug_lut_group_buckets() {
    // Enables to see log messages when running tests
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .without_timestamps()
        .init()
        .unwrap();

    // let json_file = format!("../../{}", ALPHABET);
    let json_file = format!("../../{}", POKEMON_MINI);
    let lambda = 1;
    let cutoff = 0;
    let json_path = json_file.as_str();
    let bit_mask = 3; // powers of 2 -1
    let threaded = false;

    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    let lut = LutPHFGroup::build_buckets(lambda, json_path, cutoff, bit_mask, threaded)
        .expect("Fail @ building lut_phf_double");

    let mut count_correct = 0;
    let mut count_incorrect = 0;
    for (i, key) in keys.iter().enumerate() {
        let left = values[i];
        let right = lut.get(key).expect("fail");
        if left != right {
            let distance = left - key;
            debug!(
                "Key: {}, Expected: {}, Found: {}, Expected Dist. {}",
                key, left, right, distance
            );
            count_incorrect += 1;
        } else {
            count_correct += 1;
        }
    }

    let total = count_correct + count_incorrect;
    debug!("Correct: {}/{}", count_correct, total);
    debug!("Incorrect: {}/{}", count_incorrect, total);
    assert_eq!(count_incorrect, 0);
}

#[test]
fn debug_lut_phf_double() {
    let json_file = format!("../../{}", POKEMON_MINI);
    let json_path = json_file.as_str();

    let (keys, values) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");
    let lut = LutPHFDouble::build(json_path, 0).expect("Fail @ building lut_phf_double");

    let mut count_correct = 0;
    let mut count_incorrect = 0;
    for (i, key) in keys.iter().enumerate() {
        let left = values[i];
        let right = lut.get(key).expect("fail");
        if left != right {
            let distance = left - key;
            debug!(
                "Key: {}, Expected Value: {}, Found Distance: {}, Expected Dist. {}",
                key, left, right, distance
            );
            count_incorrect += 1;
        } else {
            count_correct += 1;
        }
    }

    let total = count_correct + count_incorrect;
    debug!("Correct: {}/{}", count_correct, total);
    debug!("Incorrect: {}/{}", count_incorrect, total);
    assert_eq!(count_incorrect, 0);
}
