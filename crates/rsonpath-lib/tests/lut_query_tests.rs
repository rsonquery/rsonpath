use log::debug;
use rsonpath::{
    engine::{
        skip_tracker::{self, save_track_results_to_csv, SkipMode},
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

// ##############
// # SKIP TRACK #
// ##############

#[test]
fn count_skips() -> Result<(), Box<dyn Error>> {
    let count_mode = false; // true = COUNT, false = TRACK

    let _ = track_skips(TWITTER_SHORT_JSON, "q0", "$..entities.user_mentions[1]", count_mode);
    let _ = track_skips(TWITTER_SHORT_JSON, "q1", "$..entities.user_mentions[2]", count_mode);
    let _ = track_skips(TWITTER_SHORT_JSON, "q2", "$..[0]", count_mode);

    // let _ = track_skips(BESTBUY_JSON, "q0","$.products[5].videoChapters", count_mode);
    // let _ = track_skips(BESTBUY_JSON, "q1","$.products[*].videoChapters", count_mode);
    // let _ = track_skips(BESTBUY_JSON, "q2","$.products[*].categoryPath[2]", count_mode);

    // let _ = track_skips(POKEMON_JSON, "q0","$.cfgs[17].Moves[*]", count_mode);
    // let _ = track_skips(POKEMON_JSON, "q1","$.cfgs[0].Name", count_mode);
    // let _ = track_skips(POKEMON_JSON, "q2","$.cfgs[*].Name", count_mode);
    // let _ = track_skips(POKEMON_JSON, "q3","$.cfgs[*].Moves[*].levelLearnedAt", count_mode);
    // let _ = track_skips(POKEMON_JSON, "q4","$.cfgs[*].Moves[*]", count_mode);

    // let _ = track_skips(WALMART_SHORT_JSON, "q0","$.items[*].name", count_mode);
    // let _ = track_skips(WALMART_SHORT_JSON, "q1","$.items[50].stock", count_mode);

    // let _ = track_skips(WALMART_JSON, "q0","$.items[50].stock", count_mode);

    // let _ = track_skips(TWITTER_JSON, "q0","$.search_metadata.count", count_mode);

    // let _ = track_skips(GOOGLE_MAP_JSON, "q0","$[*].available_travel_modes", count_mode);
    // let _ = track_skips(GOOGLE_MAP_JSON, "q1","$[*].routes[*].legs[*].steps[*].distance.text", count_mode);

    Ok(())
}

fn track_skips(json_path: &str, query_name: &str, query_text: &str, count_mode: bool) -> Result<(), Box<dyn Error>> {
    // Set the correct mode for tracking skips
    if count_mode {
        skip_tracker::set_mode(SkipMode::COUNT);
    } else {
        skip_tracker::set_mode(SkipMode::TRACK);
    }

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
    let _ = sink.into_iter().map(|m| m.span().start_idx()).collect::<Vec<_>>();

    let filename = get_filename(json_path);
    if count_mode {
        println!("File = {filename}, Query = {query_text} ");
        let _ = skip_tracker::print_count_results_and_save_in_csv(
            "../../.a_lut_tests/performance/skip_tracker/COUNTER.csv",
            filename,
            query_text,
        );
    } else {
        // Save the tracked skips to a csv
        let file_path = format!(
            "../../.a_lut_tests/performance/skip_tracker/{}_{}.csv",
            filename, query_name
        );
        let save_result = save_track_results_to_csv(&file_path);
        if let Err(e) = save_result {
            eprintln!("Failed to save to CSV: {}", e);
        }
        plot_tracked_skips(&file_path);
    }

    Ok(())
}

fn get_filename(path: &str) -> &str {
    Path::new(path).file_stem().and_then(|name| name.to_str()).unwrap_or("")
}

fn plot_tracked_skips(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("../../crates/rsonpath-lib/src/lookup_table/python_statistic/skip_tracker_distribution.py")
        .arg(csv_path)
        .output()
        .expect(&msg);

    if output.status.success() {
        if let Err(e) = io::stdout().write_all(&output.stdout) {
            eprintln!("Failed to write stdout: {}", e);
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
