use std::{
    io::{self, BufReader, Read, Write},
    process::Command,
};

use crate::{
    engine::{skip_tracker, Engine},
    lookup_table::LUT,
};

use super::{
    lut_query_data::{QUERY_BESTBUY, QUERY_GOOGLE, QUERY_TWITTER},
    lut_skip_evaluation::{get_filename, SkipMode},
};

use crate::lookup_table::performance::lut_query_data::{
    QUERY_CROSSREF1, QUERY_CROSSREF2, QUERY_CROSSREF4, QUERY_NSPL, QUERY_WALMART, QUERY_WIKI,
};
use crate::lookup_table::SKIP_MODE;
use crate::{
    engine::{Compiler, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::LookUpTable,
};
use std::fs;

pub const COUNTER_FILE_PATH: &str = ".a_lut_tests/performance/skip_tracker/COUNTER_";

// Make sure SkipMode==COUNT
// run with: cargo run --bin lut --release -- skip-count
pub fn track_skips() {
    let cutoff = 0;

    // GB_1
    track_skip_count(QUERY_BESTBUY, cutoff);
    track_skip_count(QUERY_CROSSREF1, cutoff);
    track_skip_count(QUERY_CROSSREF2, cutoff);
    track_skip_count(QUERY_CROSSREF4, cutoff);
    track_skip_count(QUERY_GOOGLE, cutoff);
    track_skip_count(QUERY_NSPL, cutoff);
    track_skip_count(QUERY_TWITTER, cutoff);
    track_skip_count(QUERY_WALMART, cutoff);
    track_skip_count(QUERY_WIKI, cutoff);
}

fn track_skip_count(test_data: (&str, &[(&str, &str)]), cutoff: usize) {
    let (json_path, queries) = test_data;

    let mut lut = LUT::build(json_path, 0).expect("Fail @ building LUT");

    for &(query_name, query_text) in queries {
        let new_lut = track(lut, cutoff, json_path, query_name, query_text);
        lut = new_lut;
    }
}

fn track(lut: LUT, cutoff: usize, json_path: &str, query_name: &str, query_text: &str) -> LUT {
    if !(SKIP_MODE == SkipMode::OFF) {
        println!("Mode={:?}: Process query: {} = {}", SKIP_MODE, query_name, query_text);
    } else {
        println!("No tracking set. Abort.");
        return lut;
    }

    // Build query and LUT
    let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");
    let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
    engine.add_lut(lut);

    // Get results
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
        let mut buf = vec![];
        file.read_to_end(&mut buf).expect("Fail @ file read");
        OwnedBytes::new(buf)
    };
    let result = engine.count(&input).expect("Failed to run query normally");
    print!("  COUNT = {} ", result);

    let filename = get_filename(json_path);
    if SKIP_MODE == SkipMode::COUNT {
        let csv_path = format!("{}{}.csv", COUNTER_FILE_PATH, filename);
        let _ = skip_tracker::save_count_to_csv(json_path, &csv_path, filename, query_name, query_text);
    } else if SKIP_MODE == SkipMode::TRACK {
        // Save the tracked skips to a csv
        let file_path = format!(".a_lut_tests/performance/skip_tracker/{}_{}.csv", filename, query_name);
        let save_result = skip_tracker::save_track_to_csv(&file_path);
        if let Err(e) = save_result {
            eprintln!("Failed to save to CSV: {}", e);
        }
        plot_tracked_skips(&file_path);
    }

    engine.take_lut().expect("Failed to retrieve LUT from engine")
}

fn plot_tracked_skips(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/skip_tracker_distribution.py")
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
