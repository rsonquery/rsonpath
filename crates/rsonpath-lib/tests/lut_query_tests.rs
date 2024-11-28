use std::{
    error::Error,
    fs,
    io::{BufReader, Read},
};

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

#[test]
fn query_john_big() -> Result<(), Box<dyn Error>> {
    let lut = LookUpTableImpl::build(JOHN_BIG_JSON)?;

    let query = rsonpath_syntax::parse("$.person.spouse.person.phoneNumber[*]")?;
    let engine = RsonpathEngine::compile_query(&query)?;

    let input = {
        let mut file = BufReader::new(fs::File::open(JOHN_BIG_JSON)?);
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        OwnedBytes::new(buf)
    };
    let mut sink = vec![];

    engine.matches_with_lut(&input, lut, &mut sink)?;

    let results = sink.into_iter().map(|m| m.span().start_idx()).collect::<Vec<_>>();

    assert_eq!(vec![858, 996], results);

    Ok(())
}
