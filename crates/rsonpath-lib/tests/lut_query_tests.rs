use std::{
    error::Error,
    fs,
    io::{BufReader, Read},
};

use log::debug;
use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::{Input, OwnedBytes},
    lookup_table::{
        lut_hash_map::LutHashMap, lut_hash_map_double::LutHashMapDouble, lut_perfect_naive::LutPerfectNaive,
        lut_phf::LutPHF, lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, pair_finder, LookUpTable,
        LookUpTableImpl,
    },
    result::Match,
};
use rsonpath_syntax::JsonPathQuery;

// JSON files
const JOHN_BIG_JSON: &str = "tests/json/john_big.json";
const POKEMON_JSON: &str = "tests/json/pokemon_(6MB).json";
const TWITTER_SHORT_JSON: &str = "tests/json/twitter_short_(80MB).json";
const BESTBUY_JSON: &str = "tests/json/bestbuy_short_(103MB).json";

#[test_log::test]
fn query_john_big_log() -> Result<(), Box<dyn Error>> {
    debug!("Starting test for query_john_big");
    // query_with_lut(JOHN_BIG_JSON, "$.person.spouse.person.phoneNumber[*]", vec![858, 996])
    query_with_lut(JOHN_BIG_JSON, "$.person.spouse.person.phoneNumber[*]", vec![858, 1000])
}

// #[test]
// fn query_john_big() -> Result<(), Box<dyn Error>> {
//     query_with_lut(JOHN_BIG_JSON, "$.person.spouse.person.phoneNumber[*]", vec![858, 996])
// }

#[test]
fn query_pokemon() -> Result<(), Box<dyn Error>> {
    query_with_lut(POKEMON_JSON, "$.cfgs[0].Name", vec![858, 996])
}

fn query_with_lut(json_path: &str, query_text: &str, expected_result: Vec<usize>) -> Result<(), Box<dyn Error>> {
    // Build lut
    let lut = LookUpTableImpl::build(json_path)?;

    // Build query
    let query = rsonpath_syntax::parse(query_text)?;
    let engine = RsonpathEngine::compile_query(&query)?;

    // Get results
    let input = {
        let mut file = BufReader::new(fs::File::open(JOHN_BIG_JSON)?);
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        OwnedBytes::new(buf)
    };
    let mut sink = vec![];
    engine.matches_with_lut(&input, lut, &mut sink)?;
    let results = sink.into_iter().map(|m| m.span().start_idx()).collect::<Vec<_>>();

    // Compare expected result with result
    assert_eq!(expected_result, results);

    Ok(())
}
