use crate::{
    engine::{Compiler, Engine, RsonpathEngine},
    input,
    input::OwnedBytes,
    lookup_table::{pair_data, LookUpTable, LUT},
};

use crate::input::MmapInput;
use crate::lookup_table::implementations::lut_hash_map;
use crate::lookup_table::performance::lut_query_data::*;
use std::{
    fs,
    io::{BufReader, Read},
};

// Run with: cargo run --bin lut --release -- test-query
pub fn test_build_and_queries() {
    let cutoff = 64 * 7;

    // ###########
    // ## BUILD ##
    // ###########
    // kB_1
    // test_build_correctness(QUERY_ALPHABET, cutoff);
    // test_build_correctness(QUERY_BUGS, cutoff);
    // test_build_correctness(QUERY_BUGS_2, cutoff);
    // test_build_correctness(QUERY_JOHN, cutoff);
    // test_build_correctness(QUERY_JOHN_BIG, cutoff);
    // test_build_correctness(QUERY_NUMBERS, cutoff);
    // test_build_correctness(QUERY_SMALL, cutoff);

    // MB_1
    // test_build_correctness(QUERY_CANADA, cutoff);
    // test_build_correctness(QUERY_OPENFOOD, cutoff);
    // test_build_correctness(QUERY_PEOPLE, cutoff);
    // test_build_correctness(QUERY_PRETTY_PEOPLE, cutoff);
    // test_build_correctness(QUERY_TWITTER_MINI, cutoff);

    // MB_15
    // test_build_correctness(QUERY_AST, cutoff);
    // test_build_correctness(QUERY_DUMMY_10, cutoff);
    // test_build_correctness(QUERY_DUMMY_20, cutoff);
    // test_build_correctness(QUERY_POKEMON_MINI, cutoff);

    // MB_100
    // test_build_correctness(QUERY_APP, cutoff);
    // test_build_correctness(QUERY_BESTBUY_SHORT, cutoff);
    // test_build_correctness(QUERY_CROSSREF0, cutoff);
    // test_build_correctness(QUERY_GOOGLE_SHORT, cutoff);
    // test_build_correctness(QUERY_POKEMON, cutoff);
    // test_build_correctness(QUERY_TWITTER_SHORT, cutoff);
    // test_build_correctness(QUERY_WALMART_SHORT, cutoff);

    // GB_1
    // test_build_correctness(QUERY_BESTBUY, cutoff);
    // test_build_correctness(QUERY_CROSSREF1, cutoff);
    // test_build_correctness(QUERY_CROSSREF2, cutoff);
    // test_build_correctness(QUERY_CROSSREF4, cutoff);
    // test_build_correctness(QUERY_GOOGLE, cutoff);
    // test_build_correctness(QUERY_NSPL, cutoff);
    // test_build_correctness(QUERY_TWITTER, cutoff);
    // test_build_correctness(QUERY_WALMART, cutoff);
    // test_build_correctness(QUERY_WIKI, cutoff);

    // GB_25
    // test_build_correctness(QUERY_NESTED_COL, cutoff);

    // ###########
    // ## QUERY ##
    // ###########
    // kB_1
    // test_query_correctness_count(QUERY_ALPHABET, cutoff);
    // test_query_correctness_count(QUERY_BUGS, cutoff);
    // test_query_correctness_count(QUERY_BUGS_2, cutoff);
    // test_query_correctness_count(QUERY_JOHN, cutoff);
    // test_query_correctness_count(QUERY_JOHN_BIG, cutoff);
    // test_query_correctness_count(QUERY_NUMBERS, cutoff);
    // test_query_correctness_count(QUERY_SMALL, cutoff);

    // MB_1
    // test_query_correctness_count(QUERY_CANADA, cutoff);
    // test_query_correctness_count(QUERY_OPENFOOD, cutoff);
    // test_query_correctness_count(QUERY_PEOPLE, cutoff);
    // test_query_correctness_count(QUERY_PRETTY_PEOPLE, cutoff);
    // test_query_correctness_count(QUERY_TWITTER_MINI, cutoff);

    // MB_15
    // test_query_correctness_count(QUERY_AST, cutoff);
    // test_query_correctness_count(QUERY_DUMMY_10, cutoff);
    // test_query_correctness_count(QUERY_DUMMY_20, cutoff);
    // test_query_correctness_count(QUERY_POKEMON_MINI, cutoff);

    // MB_100
    // test_query_correctness_count(QUERY_APP, cutoff);
    // test_query_correctness_count(QUERY_BESTBUY_SHORT, cutoff);
    // test_query_correctness_count(QUERY_CROSSREF0, cutoff);
    // test_query_correctness_count(QUERY_GOOGLE_SHORT, cutoff);
    // test_query_correctness_count(QUERY_POKEMON, cutoff);
    // test_query_correctness_count(QUERY_TWITTER_SHORT, cutoff);
    // test_query_correctness_count(QUERY_WALMART_SHORT, cutoff);

    // GB_1
    test_query_correctness_count(QUERY_BESTBUY, cutoff);
    test_query_correctness_count(QUERY_CROSSREF1, cutoff);
    test_query_correctness_count(QUERY_CROSSREF2, cutoff);
    test_query_correctness_count(QUERY_CROSSREF4, cutoff);
    test_query_correctness_count(QUERY_GOOGLE, cutoff);
    test_query_correctness_count(QUERY_NSPL, cutoff);
    test_query_correctness_count(QUERY_TWITTER, cutoff);
    test_query_correctness_count(QUERY_WALMART, cutoff);
    test_query_correctness_count(QUERY_WIKI, cutoff);

    // GB_25
    // test_query_correctness_count_big_json(QUERY_NESTED_COL, cutoff);
}

fn test_build_correctness(test_data: (&str, &[(&str, &str)]), cutoff: usize) {
    let (json_path, _) = test_data;
    println!("Building LUT: {}", json_path);
    let lut = LUT::build(&json_path, cutoff).expect("Fail @ building LUT");
    let lut_hash_map = lut_hash_map::LutHashMap::build(json_path, cutoff).expect("Fail @ building lut_hash_map");

    println!("Testing keys ...");
    let (keys, values) = pair_data::get_pairs_absolute(json_path, cutoff).expect("Fail @ finding pairs.");
    let mut count_incorrect = 0;
    for (i, key) in keys.iter().enumerate() {
        let found = lut.get(key).expect("Fail @ get(key) - LUT.");
        let found_hash = lut_hash_map.get(key).expect("Fail @ get(key) - lut_hash_map.");
        if found != values[i] {
            count_incorrect += 1;
            println!(
                "  i: {}, Key {}, Found {}, Expected: {} and {}",
                i, key, found, values[i], found_hash
            );
        }
    }

    println!(" Correct {}/{}", keys.len() - count_incorrect, keys.len());
    println!(" Incorrect {}/{}", count_incorrect, keys.len());

    drop(lut);
}

fn test_query_correctness_count(test_data: (&str, &[(&str, &str)]), cutoff: usize) {
    let (json_path, queries) = test_data;
    println!("Building LUT: {}", json_path);
    let mut lut = LUT::build(&json_path, cutoff).expect("Fail @ building LUT");

    // Run all queries
    println!("Checking queries:");
    for &(query_name, query_text) in queries {
        print!(" Query: {} = \"{}\" ... ", query_name, query_text);
        let input = {
            let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
            let mut buf = vec![];
            file.read_to_end(&mut buf).expect("Fail @ file read");
            OwnedBytes::new(buf)
        };
        let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");

        // Query normally and skip iteratively (ITE)
        // println!("---- ITE STYLE ----");
        let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
        let count = engine.count(&input).expect("Failed to run query normally");

        // Query normally and skip using the lookup table (LUT)
        // println!("---- LUT STYLE ----");
        engine.add_lut(lut);
        let lut_count = engine.count(&input).expect("LUT: Failed to run query normally");

        if lut_count != count {
            println!("\n  Found {}, Expected {}", lut_count, count);
        } else {
            println!("  Correct");

            if count == 0 {
                println!("DO NOT USE THIS QUERY. IT HAS NO RESULTS!")
            }
        }

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine");
    }

    drop(lut);
}

fn test_query_correctness_nodes(test_data: (&str, &[(&str, &str)]), cutoff: usize) {
    let (json_path, queries) = test_data;
    println!("Building LUT: {}", json_path);
    let mut lut = LUT::build(&json_path, cutoff).expect("Fail @ building LUT");

    // Run all queries
    println!("Checking queries:");
    for &(query_name, query_text) in queries {
        println!(" Query: {} = \"{}\" ... ", query_name, query_text);
        let input = {
            let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
            let mut buf = vec![];
            file.read_to_end(&mut buf).expect("Fail @ file read");
            OwnedBytes::new(buf)
        };
        let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");

        // Query normally and skip iteratively (ITE)
        println!("---- ITE STYLE ----");
        let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
        let mut sink = vec![];
        engine.matches(&input, &mut sink).expect("Fail @ engine matching.");
        let results = sink
            .into_iter()
            .map(|m| String::from_utf8_lossy(m.bytes()).to_string())
            .collect::<Vec<_>>();

        // Print results
        println!("ITE Results found: ");
        for (i, result) in results.into_iter().enumerate() {
            println!("Result {}:", i);
            println!("{result}");
        }

        // Query normally and skip using the lookup table (LUT)
        println!("---- LUT STYLE ----");
        engine.add_lut(lut);
        let mut sink_lut = vec![];
        engine.matches(&input, &mut sink_lut).expect("Fail @ engine matching.");
        let results_lut = sink_lut
            .into_iter()
            .map(|m| String::from_utf8_lossy(m.bytes()).to_string())
            .collect::<Vec<_>>();

        println!("LUT Results found: ");
        for (i, result) in results_lut.into_iter().enumerate() {
            println!("Result {}:", i);
            println!("{result}");
        }

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine");
    }

    drop(lut);
}

fn test_query_correctness_count_big_json(test_data: (&str, &[(&str, &str)]), cutoff: usize) {
    let (json_path, queries) = test_data;
    println!("Building LUT: {}", json_path);
    let mut lut = LUT::build(&json_path, cutoff).expect("Fail @ building LUT");

    // Run all queries
    println!("Checking queries:");
    for &(query_name, query_text) in queries {
        print!(" Query: {} = \"{}\" ... ", query_name, query_text);
        let file = std::fs::File::open(json_path).expect("Fail to open file");
        let input = unsafe { input::MmapInput::map_file(&file).expect("Failed to map file") };
        let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");

        // Query normally and skip iteratively (ITE)
        // println!("---- ITE STYLE ----");
        let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
        let count = engine.count(&input).expect("Failed to run query normally");

        // Query normally and skip using the lookup table (LUT)
        // println!("---- LUT STYLE ----");
        engine.add_lut(lut);
        let lut_count = engine.count(&input).expect("LUT: Failed to run query normally");

        if lut_count != count {
            println!("\n  Found {}, Expected {}", lut_count, count);
        } else {
            println!("  Correct");

            if count == 0 {
                println!("DO NOT USE THIS QUERY. IT HAS NO RESULTS!")
            }
        }

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine");
    }

    drop(lut);
}
