use crate::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{self, lut_hash_map, pair_data, LookUpTable, LUT},
};

use serde_json::json;
use std::{
    fs,
    io::{BufReader, Read},
};

// kB_1
pub const JOHN: &str = ".a_lut_tests/test_data/kB_1/john_119.json";
pub const JOHN_BIG: &str = ".a_lut_tests/test_data/kB_1/john_big.json";
pub const BUGS: &str = ".a_lut_tests/test_data/kB_1/bugs.json";
pub const BUGS_2: &str = ".a_lut_tests/test_data/kB_1/bugs_2.json";
pub const ALPHABET: &str = ".a_lut_tests/test_data/kB_1/alphabet_(2kB).json";

// MB_1
pub const TWITTER_MINI: &str = ".a_lut_tests/test_data/MB_1/twitter_(767kB).json";

// MB_15
pub const POKEMON_MINI: &str = ".a_lut_tests/test_data/MB_15/pokemon_(6MB).json";

// MB_100
pub const TWITTER_SHORT: &str = ".a_lut_tests/test_data/MB_100/twitter_short_(80MB).json";
pub const BESTBUY_SHORT: &str = ".a_lut_tests/test_data/MB_100/bestbuy_short_(103MB).json";
pub const GOOGLE_SHORT: &str = ".a_lut_tests/test_data/MB_100/google_map_short_(107MB).json";
pub const WALMART_SHORT: &str = ".a_lut_tests/test_data/MB_100/walmart_short_(95MB).json";
pub const POKEMON_SHORT: &str = ".a_lut_tests/test_data/MB_100/pokemon_(173MB).json";

// GB_1
pub const BESTBUY: &str = ".a_lut_tests/test_data/GB_1/bestbuy_large_record_(1GB).json";
pub const WALMART: &str = ".a_lut_tests/test_data/GB_1/walmart_large_record_(995MB).json";
pub const TWITTER: &str = ".a_lut_tests/test_data/GB_1/twitter_large_record_(843MB).json";
pub const GOOGLE: &str = ".a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json";

pub const QUERY_GOOGLE: (&str, &[(&str, &str)]) = (
    GOOGLE,
    &[
        // Random queries
        // ("0", "$[*]..bounds"),
        // ("1", "$[*]..bounds.northeast"),
        // ("2", "$[*]..bounds.northeast.lat"),
        // ("3", "$[*]..bounds.northeast.lng"),
        // ("4", "$[*]..copyrights"),
        // ("5", "$[*]..summary"),
        // ("6", "$[*]..warnings"),
        // ("7", "$[*]..waypoint_order"),
        // ("8", "$[*].routes[*]"),
        // ("9", "$[*].routes[*]..legs"),
        // ("10", "$[*].routes[*]..points"),
        // ("11", "$[*].routes[*]..steps[*]"),
        // ("12", "$[*].routes[*].bounds"),
        // ("13", "$[*].routes[*].bounds.northeast"),
        // ("14", "$[*].routes[*].bounds.northeast.lat"),
        // ("15", "$[*].routes[*].bounds.northeast.lng"),
        // ("16", "$[*].routes[*].legs[*].start_location.lat"),
        // ("17", "$[*].routes[*].legs[*].steps[1]"),
        // ("18", "$[*].routes[*].legs[*].steps[1].distance.text"),
        // ("19", "$[*].routes[*].legs[*].traffic_speed_entry"),
        // ("20", "$[*].routes[*].overview_polyline"),
        // ("21", "$[*].routes[*].overview_polyline.points"),
        // ("22", "$[*].routes[*].summary"),
        // ("23", "$[*].routes[*].warnings"),
        // ("24", "$[*].routes[*].waypoint_order"),
        // ("25", "$[1]"),
        // ("26", "$[10].routes[*].bounds"),
        // ("27", "$[100].routes[*].bounds"),
        // ("28", "$[1000].routes[*].bounds"),
        // ("29", "$[2000].routes[*].bounds"),
        // ("30", "$[3000].routes[*].bounds"),
        // // More queries
        ("100", "$[*].routes[*].legs[*]"),
        // ("101", "$[*].routes[*].legs[*].steps[*]"),
        // ("102", "$[*].routes[*].legs[*].steps[*].distance"),
        // ("103", "$[*].routes[*].legs[*].steps[*].distance.text"),
        // ("104", "$[*].routes[*].legs[*].steps[*].distance.value"),
        // ("108", "$[*].routes[*].legs[*].steps[*].duration"),
        // ("109", "$[*].routes[*].legs[*].steps[*].polyline"),
        // ("110", "$[*].routes[*].legs[*].steps[*].polyline.points"),
        // ("111", "$[*].routes[*].legs[*].steps[*].end_location"),
        // ("112", "$[*].routes[*].legs[*].steps[*].html_instructions"),
        // ("113", "$[*].routes[*].legs[*].steps[*].travel_mode"),
        // ("114", "$[*].routes[*].legs[*].steps[*].start_location"),
        // ("115", "$[*].routes[*].legs[*].steps[*].start_location.lat"),
        // ("116", "$[*].routes[*].legs[*].steps[*].start_location.lng"),
        // ("117", "$[*].routes[*].legs[*].steps[*].maneuver"),
        // ("118", "$[*].routes[*].legs[*]..lat"),
        // ("119", "$[*].routes[*].legs[*]..lng"),
        // ("200", "$[*].available_travel_modes"),
        // ("202", "$[*].routes[*]"),
        // ("203", "$[*].routes[*].legs[*]"),
        // ("204", "$[4000].routes[*].bounds"),
    ],
);

pub const QUERY_BESTBUY: (&str, &[(&str, &str)]) = (
    BESTBUY,
    &[
        ("100", "$.products[5].videoChapters"),
        ("101", "$.products[*].videoChapters"),
        ("102", "$.products[*].videoChapters[1].chapter"),
        ("103", "$.products[*].shipping[*]"),
        ("104", "$.products[*].shipping[*].ground"),
        ("105", "$.products[*].shipping[*].nextDay"),
        ("106", "$.products[*].shipping[*].secondDay"),
        ("107", "$.products[*].shipping[*].vendorDelivery"),
        ("108", "$.products[*].shippingLevelsOfService[*]"),
        ("109", "$.products[*].shippingLevelsOfService[*].serviceLevelId"),
        ("110", "$.products[*].shippingLevelsOfService[*].serviceLevelName"),
        ("111", "$.products[*].shippingLevelsOfService[*].unitShippingPrice"),
        ("112", "$.products[*].categoryPath[2]"),
        ("113", "$.products[*].categoryPath[*].id"),
        ("114", "$.products[*].categoryPath[*].name"),
        ("115", "$.products[*].quantityLimit"),
        ("116", "$.products[*].earlyTerminationFees[*]"),
        ("117", "$.products[*].frequentlyPurchasedWith[*]"),
        ("118", "$.products[*].includedItemList[*]"),
        ("119", "$.products[*].accessories[*]"),
        ("120", "$.products[*].planFeatures[*]"),
        ("121", "$.products[*].homeDelivery"),
        ("122", "$.products[*].carrierPlans[*]"),
        ("123", "$.products[*].freeShipping"),
        ("124", "$.products[*].additionalFeatures[*]"),
        ("125", "$.products[*].additionalFeatures[*].feature"),
        ("126", "$.products[*].dollarSavings"),
        ("127", "$.products[*].lengthInMinutes"),
        ("128", "$.products[*].screenFormat"),
    ],
);

pub const QUERY_TWITTER: (&str, &[(&str, &str)]) = (
    TWITTER,
    &[
        // Random Queries
        ("100", "$[*].geo"),
        ("101", "$[*].id"),
        ("102", "$[*].source"),
        ("103", "$[*].timestamp_ms"),
        ("104", "$[*].user.created_at"),
        ("105", "$[*].user.followers_count"),
        ("106", "$[*].user.lang"),
        ("107", "$[*].user.listed_count"),
        ("108", "$[*].user.profile_image_url_https"),
        ("109", "$[*].user.profile_sidebar_border_color"),
        ("110", "$[*].user.screen_name"),
        ("111", "$[*].user.verified"),
        ("112", "$[*].retweeted_status.id"),
        ("113", "$[*].retweeted_status.filter_level"),
        ("114", "$[*].retweeted_status.user.following"),
        ("115", "$[*].retweeted_status.user.name"),
        ("116", "$[*].retweeted_status[*]"),
        ("117", "$[*].retweeted_status[*]..entities..url"),
        ("118", "$[*].retweeted_status[*]..entities.symbols[*]"),
        ("119", "$[*].retweeted_status[*]..entities.user_mentions[*]"),
        ("120", "$[*].retweeted_status[*]..id"),
        ("121", "$[*].retweeted_status[*]..in_reply_to_screen_name"),
        ("122", "$[*].retweeted_status[*].user.lang"),
        ("123", "$[*].entities.hashtags[*]"),
        ("124", "$[*].entities.symbols[*]"),
        ("125", "$[*].entities.symbols[1]"),
        ("126", "$[*].entities.urls[*].display_url"),
        ("127", "$[*].entities..symbols[*]"),
        ("128", "$[*].entities..url"),
        ("129", "$[*]..id"),
        // Queries where LUT is faster than ITE
        ("200", "$[*].entities..symbols[*]"),
        ("201", "$[*].entities..url"),
        ("202", "$[*].entities.symbols[*]"),
        ("203", "$[*].entities.symbols[1]"),
        ("204", "$[*].entities.urls[*].display_url"),
        ("205", "$[*].timestamp_ms"),
    ],
);

pub const QUERY_POKEMON_SHORT: (&str, &[(&str, &str)]) = (
    POKEMON_SHORT,
    &[
        // Group 1: Tail data
        ("200", "$.taildata"),
        ("201", "$.taildata2"),
        // Group 2: cfg1 fields
        ("202", "$.cfg1[*].ID"),
        ("203", "$.cfg1[*].Name"),
        ("204", "$.cfg1[*].Height"),
        ("205", "$.cfg1[*].Weight"),
        ("206", "$.cfg1[*].Abilities[*]"),
        ("207", "$.cfg1[*].Moves[*].moveName"),
        ("208", "$.cfg1[*].Moves[*].levelLearnedAt"),
        ("209", "$.cfg1[*].Moves[*].moveLearnCondition"),
        // Group 3: Recursive search
        ("210", "$..ID"),
        ("211", "$..Name"),
        ("212", "$..Height"),
        ("213", "$..Weight"),
        ("214", "$..Abilities[*]"),
        ("215", "$..Moves[*].moveName"),
        ("216", "$..Moves[*].levelLearnedAt"),
        ("217", "$..Moves[*].moveLearnCondition"),
        // Group 4: cfg10 fields
        ("218", "$.cfg10[*].ID"),
        ("219", "$.cfg10[*].Abilities[*]"),
        ("220", "$.cfg10[*].Moves[*].levelLearnedAt"),
        // Group 5: cfg25 fields
        ("221", "$.cfg25[*].ID"),
        ("222", "$.cfg25[*].Abilities[*]"),
        ("223", "$.cfg25[*].Moves[*].levelLearnedAt"),
        // Group 6: cfg40 fields
        ("224", "$.cfg40[*].ID"),
        ("225", "$.cfg40[*].Abilities[*]"),
        ("226", "$.cfg40[*].Moves[*].levelLearnedAt"),
        // Group 7: cfg50 fields
        ("227", "$.cfg50[*].ID"),
        ("228", "$.cfg50[*].Abilities[*]"),
        ("229", "$.cfg50[*].Moves[*].levelLearnedAt"),
    ],
);

pub const QUERY_POKEMON_MINI: (&str, &[(&str, &str)]) = (
    POKEMON_MINI,
    &[
        ("200", "$.cfgs[1].ID"),
        ("201", "$.cfgs[1].Name"),
        ("202", "$.cfgs[*].ID"),
        ("203", "$.cfgs[*].Name"),
        ("204", "$.cfgs[*].BaseStats[1]"),
        ("204", "$.cfgs[*].BaseStats[2]"),
        ("204", "$.cfgs[*].BaseStats[*]"),
        ("204", "$.cfgs[*].EggGroups[1]"),
        ("204", "$.cfgs[*].EggGroups[2]"),
        ("204", "$.cfgs[*].EggGroups[*]"),
    ],
);

pub const QUERY_JOHN_BIG: (&str, &[(&str, &str)]) = (
    JOHN_BIG,
    &[
        // OLD JOHN BIG
        ("200", "$.person.firstName"),
        ("201", "$.person.lastName"),
        ("202", "$.person.phoneNumber[2].type"),
        ("203", "$.person.spouse.person.phoneNumber.*"),
        ("204", "$.person.spouse.person.phoneNumber[1]"),
        ("205", "$.person.spouse.person.phoneNumber[2]"),
        ("300", "$[1]"),
    ],
);

pub const QUERY_BUGS: (&str, &[(&str, &str)]) = (BUGS, &[("1", "$.a..b")]);

pub const QUERY_BUGS_2: (&str, &[(&str, &str)]) = (BUGS_2, &[("1", "$.b[0]")]);

// ########################
// #### Test functions ####
// #########################
// Run with: cargo run --bin lut --release -- test-query
pub fn test_build_and_queries() {
    let cutoff = 128;

    // test_build_correctness(BUGS, cutoff);
    // test_build_correctness(JOHN_BIG, cutoff);
    // test_build_correctness(POKEMON_MINI, cutoff);
    // test_build_correctness(GOOGLE, cutoff);
    // test_build_correctness(WALMART, cutoff);
    // test_build_correctness(BESTBUY, cutoff);
    // test_build_correctness(TWITTER, cutoff);
    // test_build_correctness(POKEMON_SHORT, cutoff);

    test_query_correctness_count(QUERY_BUGS, cutoff);
    test_query_correctness_count(QUERY_BUGS_2, cutoff);
    test_query_correctness_count(QUERY_JOHN_BIG, cutoff);
    test_query_correctness_count(QUERY_POKEMON_MINI, cutoff);
    test_query_correctness_count(QUERY_GOOGLE, cutoff);
    test_query_correctness_count(QUERY_TWITTER, cutoff);
    test_query_correctness_count(QUERY_BESTBUY, cutoff);
    test_query_correctness_count(QUERY_POKEMON_SHORT, cutoff);

    // test_query_correctness_nodes(QUERY_BUGS_2, cutoff);
}

fn test_build_correctness(json_path: &str, cutoff: usize) {
    println!("Building LUT: {}", json_path);
    let lut = LUT::build(&json_path, cutoff).expect("Fail @ building LUT");
    let lut_hash_map = lut_hash_map::LutHashMap::build(json_path, cutoff).expect("Fail @ building lut_hash_map");

    println!("Testing keys ...");
    let (keys, values) = pair_data::get_keys_and_values_absolute(json_path, cutoff).expect("Fail @ finding pairs.");
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

    std::mem::drop(lut);
}
