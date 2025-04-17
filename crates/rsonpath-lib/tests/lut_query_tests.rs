use log::debug;
use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{
        performance::lut_query_data::{QUERY_BESTBUY, QUERY_GOOGLE, QUERY_POKEMON_SHORT, QUERY_TWITTER},
        LookUpTable, LUT,
    },
};
use std::{
    error::Error,
    fs,
    io::{BufReader, Read},
};

#[test]
fn query_pokemon_short() -> Result<(), Box<dyn Error>> {
    test_all_queries(QUERY_POKEMON_SHORT)
}

#[test]
fn query_google() -> Result<(), Box<dyn Error>> {
    test_all_queries(QUERY_GOOGLE)
}

#[test]
fn query_bestbuy() -> Result<(), Box<dyn Error>> {
    test_all_queries(QUERY_BESTBUY)
}

#[test]
fn query_twitter() -> Result<(), Box<dyn Error>> {
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
    let mut lut = LUT::build(&json_path, 0).expect("Fail @ building LUT");

    // Run all queries
    for &(query_name, query_text) in queries {
        debug!("Query: {}", query_name);

        let input = {
            let mut file = BufReader::new(fs::File::open(&json_path).expect("Fail @ open File"));
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

        lut = engine.take_lut().expect("Failed to retrieve LUT from engine")
    }

    Ok(())
}
