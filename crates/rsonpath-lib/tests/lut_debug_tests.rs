use std::{
    fs,
    io::{BufReader, Read},
};

use log::debug;
use rsonpath::engine::main::MainEngine;
use rsonpath::input::{BorrowedBytes, Input};
use rsonpath::lookup_table::implementations::lut_hash_map;
use rsonpath::lookup_table::implementations::lut_hash_map::LutHashMap;
use rsonpath::lookup_table::implementations::lut_phf_double::LutPHFDouble;
use rsonpath::lookup_table::implementations::lut_phf_group::LutPHFGroup;
use rsonpath::lookup_table::performance::lut_query_data::{
    BUGS, QUERY_BESTBUY, QUERY_POKEMON_MINI, QUERY_POKEMON_SHORT, QUERY_TWITTER,
};
use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{
        pair_data,
        performance::lut_query_data::{
            ALPHABET, GOOGLE, POKEMON_MINI, QUERY_BUGS, QUERY_GOOGLE, QUERY_JOHN_BIG, TWITTER_SHORT,
        },
        LookUpTable, LUT,
    },
};
use serde_json::json;

/// cargo test --test lut_debug_tests -- test_build_and_queries --nocapture | rg "(lut_debug_tests)"
/// cargo test --test lut_debug_tests -- test_build_and_queries --nocapture | rg "(tail_skipping|lut_debug_tests|main)"
/// cargo test --test lut_debug_tests -- test_build_and_queries --nocapture | rg "(tail_skipping|lut_debug_tests|main|lut_hash_map)"
#[test]
fn test_build_and_queries() {
    // Enables to see log messages when running tests
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .without_timestamps()
        .init()
        .unwrap();

    let cutoff = 128;
    debug!("Start with cutoff={}", cutoff);

    // test_build_correctness(GOOGLE, cutoff);
    // test_build_correctness(WALMART, cutoff);
    // test_build_correctness(BESTBUY, cutoff);
    // test_build_correctness(TWITTER, cutoff);
    // test_build_correctness(POKEMON_MINI, cutoff);
    test_build_correctness(TWITTER_SHORT, cutoff);
    // test_build_correctness(ALPHABET, cutoff);

    test_query_correctness(QUERY_BUGS, cutoff);
    test_query_correctness(QUERY_JOHN_BIG, cutoff);
    test_query_correctness(QUERY_POKEMON_MINI, cutoff);
    test_query_correctness(QUERY_GOOGLE, cutoff);
    test_query_correctness(QUERY_TWITTER, cutoff);
    test_query_correctness(QUERY_BESTBUY, cutoff);
    test_query_correctness(QUERY_POKEMON_SHORT, cutoff);

    // test_bug();
}

fn test_build_correctness(json_name: &str, cutoff: usize) {
    let s = format!("../../{}", json_name);
    let json_path = s.as_str();

    debug!("Building LUT (Hashmap): {}", json_path);
    let lut_hash_map = LutHashMap::build(&json_path, 0).expect("Fail @ building LUT");
    debug!("Building LUT: {}", json_path);
    let lut = LUT::build(&json_path, 0).expect("Fail @ building LUT");

    debug!("Testing keys ...");
    let (keys, values) = pair_data::get_keys_and_values_absolute(json_path, cutoff).expect("Fail @ finding pairs.");
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

fn test_query_correctness(test_data: (&str, &[(&str, &str)]), cutoff: usize) {
    let (json_name, queries) = test_data;
    let s = format!("../../{}", json_name);
    let json_path = s.as_str();
    debug!("Building LUT: {}", json_path);
    let mut lut = LUT::build(&json_path, cutoff).expect("Fail @ building LUT");

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
        let ite_count = engine.count(&input).expect("Failed to run query normally");
        debug!("ITE: result count =  {}", ite_count);

        // Query normally and skip using the lookup table (LUT)
        debug!("---- LUT STYLE ----");
        engine.add_lut(lut);
        let lut_count = engine.count(&input).expect("LUT: Failed to run query normally");
        debug!("LUT: result count =  {}", lut_count);

        if lut_count != ite_count {
            debug!("Found {}, Expected {}", lut_count, ite_count);
        } else {
            debug!("Correct");
        }

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine");
    }

    std::mem::drop(lut);
}

// cargo test --test lut_debug_tests -- debug_lut_group_buckets --nocapture | rg "(lut_debug_tests|pair_data)"
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

    let (keys, values) = pair_data::get_keys_and_values_absolute(json_path, cutoff).expect("Fail @ finding pairs.");
    let lut = LutPHFGroup::build_buckets(lambda, json_path, cutoff, bit_mask, threaded).expect("Fail @ build");

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

    let (keys, values) =
        pair_data::get_keys_and_values_absolute(json_path, DISTANCE_CUT_OFF).expect("Fail @ finding pairs.");
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

use rayon::current_num_threads;
#[allow(unused_imports)]
use std::str;

fn test_bug() {
    let json_path = format!(
        "../../{}",
        "./crates/rsonpath-test/documents/json/compressed/twitter_urls.json"
    );
    let query = "$[0].url";
    let cutoff = 128;
    let requested_padding = 112;

    //println ! ("on document atomic_after_list running the query $.a..b (select the 'a' object and then the atomic integer by descendant) with Input impl BorrowedBytes and result mode NodesResult using engine MainEngine(with LUT)");
    let jsonpath_query = rsonpath_syntax::parse(query).expect("Fail at parse");
    let raw_json = fs::read_to_string(&json_path).expect("Fail at reading json");

    let json_with_leading_whitespace = {
        let mut json = String::new();
        for _ in 0..256 {
            json.push(' ');
        }
        json += &raw_json;
        json
    };
    let misalignment = json_with_leading_whitespace.as_ptr().align_offset(128);
    let aligned_json = &json_with_leading_whitespace.as_bytes()[misalignment..];
    let forced_padding_json = &aligned_json[requested_padding..];

    let input = BorrowedBytes::new(forced_padding_json);
    assert_eq!(input.leading_padding_len(), requested_padding);
    let lut = LUT::build(&json_path, cutoff).expect("Fail lut");
    let mut engine = MainEngine::compile_query(&jsonpath_query).expect("Fail compile query");

    // ITE
    debug!("---- ITE STYLE ----");
    let mut result = vec![];
    engine.matches(&input, &mut result).expect("Fail matching");
    let utf8: Result<Vec<&str>, _> = result.iter().map(|x| str::from_utf8(x.bytes())).collect();
    let utf8 = utf8.expect("valid utf8");

    // LUT
    debug!("---- LUT STYLE ----");
    engine.add_lut(lut);
    let mut result_lut = vec![];
    engine.matches(&input, &mut result_lut).expect("Fail matching");
    let utf8_lut: Result<Vec<&str>, _> = result_lut.iter().map(|x| str::from_utf8(x.bytes())).collect();
    let utf8_lut = utf8_lut.expect("valid utf8");

    let expected_str = r#""https:\/\/t.co\/blQy8JxViF""#;
    let expected: Vec<&str> = vec![expected_str];
    if utf8 == expected {
        // assert_eq!(utf8, expected, "result != expected");
        debug!("Correct ITE");
    } else {
        debug!("NOT Correct ITE");
    }
    if utf8_lut == expected {
        // assert_eq!(utf8_lut, expected, "result != expected");
        debug!("Correct LUT");
    } else {
        debug!("NOT Correct LUT");
    }
}
