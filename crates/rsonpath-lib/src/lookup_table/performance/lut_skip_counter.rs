use std::{
    io::{self, BufReader, Read, Write},
    process::Command,
};

use crate::{
    engine::{skip_tracker, Engine},
    lookup_table::{LookUpTableLambda, LUT},
};

use super::{
    lut_query_data::{POKEMON_SHORT, QUERY_BESTBUY, QUERY_GOOGLE, QUERY_POKEMON_SHORT, QUERY_TWITTER},
    lut_skip_evaluation::{self, get_filename, SkipMode},
};

use crate::{
    engine::{Compiler, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::LookUpTable,
};
use std::{error::Error, fs};

pub const COUNTER_FILE_PATH: &str = ".a_lut_tests/performance/skip_tracker/COUNTER_";

pub fn track_skips() {
    let cutoff = 0;

    count_test_data(QUERY_GOOGLE, cutoff);
    // count_test_data(QUERY_BESTBUY);
    // count_test_data(TEST_TWITTER);
    // count_test_data(QUERY_POKEMON_SHORT);
}

fn count_test_data(test_data: (&str, &[(&str, &str)]), cutoff: usize) {
    let (json_path, queries) = test_data;

    let mut lut = LUT::build(json_path, 0).expect("Fail @ building LUT");

    for &(query_name, query_text) in queries {
        let new_lut = track(lut, cutoff, json_path, query_name, query_text);
        lut = new_lut;
    }
}

fn track(lut: LUT, cutoff: usize, json_path: &str, query_name: &str, query_text: &str) -> LUT {
    if !(lut_skip_evaluation::SKIP_MODE == SkipMode::OFF) {
        println!(
            "Mode={:?}: Process query: {} = {}",
            lut_skip_evaluation::SKIP_MODE,
            query_name,
            query_text
        );
    } else {
        println!("No tracking set. Abort.");
        return lut;
    }

    // Build query and LUT
    let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");
    let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
    engine.add_lut(lut, cutoff);

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
    if lut_skip_evaluation::SKIP_MODE == SkipMode::COUNT {
        let csv_path = format!("{}{}.csv", COUNTER_FILE_PATH, filename);
        let _ = skip_tracker::save_count_to_csv(json_path, &csv_path, filename, query_name, query_text);
    } else if lut_skip_evaluation::SKIP_MODE == SkipMode::TRACK {
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
