use crate::engine::{Compiler, Engine, RsonpathEngine};
use crate::input::OwnedBytes;
use crate::lookup_table::{LookUpTable, LUT};
use std::{
    fs,
    io::{BufReader, Read},
};

// Run with: cargo run --bin lut --release -- hot
// or for samply:
// cargo build --bin lut --release
// samply record ./target/release/lut hot
pub fn test_hotness() {
    let json_path = ".a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json";
    let query_text = "$[4000].routes[*].bounds"; // 99% skip
    let cutoff = 128;

    hot_lut(&json_path, &query_text, cutoff);
    hot_ite(&json_path, &query_text);
}

fn hot_lut(json_path: &str, query_text: &str, cutoff: usize) {
    println!("Building LUT: {}", json_path);
    let lut = LUT::build(&json_path, cutoff).expect("Fail @ building LUT");

    println!(" Query: \"{}\" ... ", query_text);
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
        let mut buf = vec![];
        file.read_to_end(&mut buf).expect("Fail @ file read");
        OwnedBytes::new(buf)
    };
    let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");
    let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
    engine.add_lut(lut);

    let start_queries = std::time::Instant::now();
    let mut sum = 0;
    for _ in 0..100 {
        let lut_count = engine.count(&input).expect("LUT: Failed to run query normally");
        sum += lut_count;
    }
    let query_time = start_queries.elapsed().as_secs_f64();
    println!(" Total: {}, Time {}", sum, query_time);
}

fn hot_ite(json_path: &str, query_text: &str) {
    println!(" Query: \"{}\" ... ", query_text);
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
        let mut buf = vec![];
        file.read_to_end(&mut buf).expect("Fail @ file read");
        OwnedBytes::new(buf)
    };
    let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");
    let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");

    let start_queries = std::time::Instant::now();
    let mut sum = 0;
    for _ in 0..100 {
        let lut_count = engine.count(&input).expect("LUT: Failed to run query normally");
        sum += lut_count;
    }
    let query_time = start_queries.elapsed().as_secs_f64();
    println!(" Total: {}, Time {}", sum, query_time);
}
