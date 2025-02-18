use crate::lookup_table::{LookUpTable, LookUpTableImpl};
use crate::{
    engine::{
        skip_tracker::{self, save_track_results_to_csv, SkipMode},
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

lazy_static! {
    static ref SKIP_TIME: Mutex<f64> = Mutex::new(0.0);
}

pub const TRACK_SKIPPING: bool = true;
static SKIP_TIME_ATOMIC: AtomicU64 = AtomicU64::new(0);

const TWITTER_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/twitter_short_(80MB).json";
const BESTBUY_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/bestbuy_short_(103MB).json";
const GOOGLE_MAP_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/google_map_short_(107MB).json";
const WALMART_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/walmart_short_(95MB).json";

const BESTBUY_JSON: &str = ".a_lut_tests/test_data/GB_1/bestbuy_large_record_(1GB).json";
const WALMART_JSON: &str = ".a_lut_tests/test_data/GB_1/walmart_large_record_(995MB).json";
const TWITTER_JSON: &str = ".a_lut_tests/test_data/GB_1/twitter_large_record_(843MB).json";
const GOOGLE_MAP_JSON: &str = ".a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json";

const TEST_DATA: &[(&str, &str, &str)] = &[
    // ("Twitter 0", TWITTER_SHORT_JSON, "$.search_metadata.count"),
    // ("Bestbuy 0", BESTBUY_SHORT_JSON, "$.products[*].videoChapters"),
    // ("Google 0", GOOGLE_MAP_SHORT_JSON, "$[*].available_travel_modes"),
    // ("Walmart 0", WALMART_SHORT_JSON, "$.items[50].stock"),
    ("Twitter 0", TWITTER_JSON, "$.search_metadata.count"),
    ("Bestbuy 0", BESTBUY_JSON, "$.products[*].videoChapters"),
    ("Google 0", GOOGLE_MAP_JSON, "$[*].available_travel_modes"),
    ("Walmart 0", WALMART_JSON, "$.items[50].stock"),
];

const REPETITIONS: u64 = 2;

pub fn skip_evaluation() {
    for &(test_name, json_path, query_text) in TEST_DATA {
        check_if_exists(json_path);

        // Setup data trackers
        let head_line =
            "TEST,JSON,QUERY,T_ORIGINAL,T_ORIGINAL_SKIP,T_OPTIMUM,T_LUT,T_LUT_SKIP,T_LUT_BUILD,LUT_CAPACITY";
        let mut data_line = format!("{},{},{},", test_name, get_filename(json_path), query_text);

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
        let mut t_lut_build: u64 = 0;
        let mut lut_capacity: usize = 0;

        println!("Time: {}", get_filename(json_path));
        for i in 1..REPETITIONS {
            println!("\t{i}");
            // Time normal query, track total time
            reset_skip_time();
            let mut sink = Vec::new();
            let start_query_time = Instant::now();
            engine.matches(&input, &mut sink).expect("Failed to run query normally");
            t_original += start_query_time.elapsed().as_nanos() as u64;
            t_original_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);

            // Time normal query, track skips
            // reset_skip_time();
            // set_flag(true);
            // sink.clear();
            // engine
            //     .matches(&input, &mut sink)
            //     .expect("Failed to run query while tracking skips");
            // t_original_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);

            // Build LUT
            let distance_cutoff = 0;
            let start_build = Instant::now();
            let lut = LookUpTableImpl::build(json_path, distance_cutoff).expect("Failed to build LUT");
            t_lut_build += start_build.elapsed().as_nanos() as u64;
            lut_capacity = lut.allocated_bytes();

            // Add the LUT
            engine.add_lut(lut);

            // Time LUT query, track total time
            sink.clear();
            reset_skip_time();
            let start_query_time = Instant::now();
            engine.matches(&input, &mut sink).expect("Failed to run query normally");
            t_lut += start_query_time.elapsed().as_nanos() as u64;
            t_lut_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);

            // Time LUT query, track skip time
            // reset_skip_time();
            // sink.clear();
            // set_flag(true);
            // engine.matches(&input, &mut sink).expect("Failed to track skip time");
            // t_lut_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);
        }

        t_original = t_original / REPETITIONS;
        t_original_skip = t_original_skip / REPETITIONS;
        let t_optimum = t_original - t_original_skip;
        t_lut = t_lut / REPETITIONS;
        t_lut_skip = t_lut_skip / REPETITIONS;
        t_lut_build = t_lut_build / REPETITIONS;

        // Collect all data
        data_line.push_str(&format!(
            "{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6}",
            t_original, t_original_skip, t_optimum, t_lut, t_lut_skip, t_lut_build, lut_capacity
        ));

        // Write CSV header and data
        let csv_path = Path::new(".a_lut_tests/performance/skip_evaluation/total.csv");
        let mut csv_file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(csv_path)
            .expect("Failed to open CSV file");

        if csv_file.metadata().expect("Failed to read CSV metadata").len() == 0 {
            writeln!(csv_file, "{}", head_line).expect("Failed to write header to CSV");
        }
        writeln!(csv_file, "{}", data_line).expect("Failed to write data to CSV");

        plot_with_python(csv_path.to_str().unwrap());
    }
}

pub fn add_skip_time(added_time: u64) {
    SKIP_TIME_ATOMIC.fetch_add(added_time, std::sync::atomic::Ordering::Relaxed);
}

pub fn reset_skip_time() {
    SKIP_TIME_ATOMIC.store(0, std::sync::atomic::Ordering::Relaxed);
}

pub fn plot_with_python(csv_path: &str) {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/lut_skip_evaluation.py")
        .arg(csv_path)
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

fn check_if_exists(path: &str) {
    if fs::metadata(path).is_err() {
        panic!("Error: The provided file '{}' does not exist.", path);
    } else if Path::new(path).is_dir() {
        panic!("Error: The provided file '{}' is not a file but a directory.", path);
    }
}

fn get_filename(path: &str) -> &str {
    Path::new(path).file_stem().and_then(|name| name.to_str()).unwrap_or("")
}
