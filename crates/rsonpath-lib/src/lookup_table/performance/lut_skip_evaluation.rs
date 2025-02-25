use crate::lookup_table::{LookUpTable, LookUpTableImpl};
use crate::{
    engine::{
        skip_tracker::{self, save_track_to_csv},
        Compiler, Engine, RsonpathEngine,
    },
    input::OwnedBytes,
};
use lazy_static::lazy_static;
use std::sync::atomic::AtomicU64;
use std::{
    error::Error,
    fs,
    io::{self, BufReader, Read, Write},
    path::Path,
    process::Command,
    sync::Mutex,
    time::Instant,
};

use super::lut_skip_counter::COUNTER_FILE_PATH;
use super::lut_test_data::{TEST_BESTBUY, TEST_GOOGLE, TEST_TWITTER};

// ############
// # Settings #
// ############
pub const TRACK_SKIPPING_DURING_PERFORMANCE_TEST: bool = true;
pub const MODE: SkipMode = SkipMode::OFF;
pub const DISTANCE_CUT_OFF: usize = 1024;
const REPETITIONS: u64 = 1;

static SKIP_TIME_ATOMIC: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, PartialEq)]
pub enum SkipMode {
    COUNT, // Track how many jumps are happening
    TRACK, // Track each jump value individually in a data structure (slow)
    OFF,   // Turned off, tracking nothing
}

const RESULT_CSV_PATH: &str = ".a_lut_tests/performance/skip_evaluation/total.csv";

pub fn skip_evaluation() {
    // eval_test_data(TEST_BESTBUY);
    // eval_test_data(TEST_GOOGLE);
    eval_test_data(TEST_TWITTER);
}

pub fn add_skip_time(added_time: u64) {
    SKIP_TIME_ATOMIC.fetch_add(added_time, std::sync::atomic::Ordering::Relaxed);
}

pub fn reset_skip_time() {
    SKIP_TIME_ATOMIC.store(0, std::sync::atomic::Ordering::Relaxed);
}

fn eval_test_data(test_data: (&str, &[(&str, &str)])) {
    let (json_path, queries) = test_data;

    let path = get_next_valid_name(RESULT_CSV_PATH);
    let csv_path = Path::new(&path);
    check_if_exists(json_path);

    let head_line = format!(
        "{},{},{},{},{},{},{},{},{}",
        "FILENAME",
        "QUERY_NAME",
        "T_ORIGINAL",
        "T_ORIGINAL_SKIP",
        "T_OPTIMUM",
        "T_LUT",
        "T_LUT_SKIP",
        "T_LUT_BUILD",
        "LUT_CAPACITY"
    );

    // Build LUT once
    let start_build_time = Instant::now();
    let mut lut = LookUpTableImpl::build(json_path, DISTANCE_CUT_OFF).expect("Failed to build LUT");
    let t_lut_build = start_build_time.elapsed().as_nanos() as u64;
    let capacity = lut.allocated_bytes();

    for &(query_name, query_text) in queries {
        let (data_line, new_lut) = evaluate(lut, json_path, query_name, query_text, t_lut_build, capacity);
        lut = new_lut;

        // Write CSV header and data
        let mut csv_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(csv_path)
            .expect("Failed to open CSV file");

        if csv_file.metadata().expect("Failed to read CSV metadata").len() == 0 {
            writeln!(csv_file, "{}", head_line).expect("Failed to write header to CSV");
        }
        writeln!(csv_file, "{}", data_line).expect("Failed to write data to CSV");
    }

    plot_with_python(csv_path.to_str().unwrap(), get_filename(json_path));
}

fn evaluate(
    mut lut: LookUpTableImpl,
    json_path: &str,
    query_name: &str,
    query_text: &str,
    t_lut_build: u64,
    capacity: usize,
) -> (String, LookUpTableImpl) {
    // Build query
    let query = rsonpath_syntax::parse(query_text).expect("Failed to parse query");
    let mut engine = RsonpathEngine::compile_query(&query).expect("Failed to build engine");

    // Read input data
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path).expect("Failed to load file"));
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).expect("Failed to read input data");
        OwnedBytes::new(buf)
    };

    let mut t_original: u64 = 0;
    let mut t_original_skip: u64 = 0;
    let mut t_lut: u64 = 0;
    let mut t_lut_skip: u64 = 0;

    println!("Time: {}, {}", get_filename(json_path), query_name);
    for _ in 0..REPETITIONS {
        reset_skip_time();
        let start_query_time = Instant::now();
        engine.count(&input).expect("Failed to run query normally");
        t_original += start_query_time.elapsed().as_nanos() as u64;
        t_original_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);
    }

    // Add LUT
    engine.add_lut(lut);

    for _ in 0..REPETITIONS {
        reset_skip_time();
        let start_query_time = Instant::now();
        engine.count(&input).expect("Failed to run query normally");
        t_lut += start_query_time.elapsed().as_nanos() as u64;
        t_lut_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);
    }

    // Take LUT back
    lut = engine.take_lut().expect("Failed to retrieve LUT from engine");

    t_original /= REPETITIONS;
    t_original_skip /= REPETITIONS;
    let t_optimum = t_original - t_original_skip;
    t_lut /= REPETITIONS;
    t_lut_skip /= REPETITIONS;

    // Format result string
    let result = format!(
        "{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{}",
        get_filename(json_path),
        query_name,
        t_original,
        t_original_skip,
        t_optimum,
        t_lut,
        t_lut_skip,
        t_lut_build,
        capacity,
    );

    (result, lut)
}

fn plot_with_python(csv_path: &str, filename: &str) {
    let counter_file_path = format!("{}{}.csv", COUNTER_FILE_PATH, filename);

    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/lut_skip_evaluation.py")
        .args(&[csv_path, &counter_file_path])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            if let Err(e) = io::stdout().write_all(&output.stdout) {
                eprintln!("Failed to write stdout: {}", e);
            }
        }
        Ok(output) => {
            eprintln!("Python script error: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            eprintln!("Failed to execute Python script: {}", e);
        }
    }
}

pub fn get_filename(path: &str) -> &str {
    Path::new(path).file_stem().and_then(|name| name.to_str()).unwrap_or("")
}

fn check_if_exists(path: &str) {
    if fs::metadata(path).is_err() {
        panic!("Error: The provided file '{}' does not exist.", path);
    } else if Path::new(path).is_dir() {
        panic!("Error: The provided file '{}' is not a file but a directory.", path);
    }
}

fn get_next_valid_name(path: &str) -> String {
    let mut new_path = path.to_string();
    let mut counter = 1;

    while Path::new(&new_path).exists() {
        new_path = format!("{}_({}).csv", path.trim_end_matches(".csv"), counter);
        counter += 1;
    }

    new_path
}
