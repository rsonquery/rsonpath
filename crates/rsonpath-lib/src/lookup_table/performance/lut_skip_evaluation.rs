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

const REPETITIONS: u64 = 100000;
pub const TRACK_SKIPPING: bool = true;
static SKIP_TIME_ATOMIC: AtomicU64 = AtomicU64::new(0);

pub const TWITTER_MINI_JSON: &str = ".a_lut_tests/test_data/MB_1/twitter_(767kB).json";

pub const TWITTER_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/twitter_short_(80MB).json";
pub const BESTBUY_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/bestbuy_short_(103MB).json";
pub const GOOGLE_MAP_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/google_map_short_(107MB).json";
pub const WALMART_SHORT_JSON: &str = ".a_lut_tests/test_data/MB_100/walmart_short_(95MB).json";

pub const BESTBUY_JSON: &str = ".a_lut_tests/test_data/GB_1/bestbuy_large_record_(1GB).json";
pub const WALMART_JSON: &str = ".a_lut_tests/test_data/GB_1/walmart_large_record_(995MB).json";
pub const TWITTER_JSON: &str = ".a_lut_tests/test_data/GB_1/twitter_large_record_(843MB).json";
pub const GOOGLE_MAP_JSON: &str = ".a_lut_tests/test_data/GB_1/google_map_large_record_(1.1GB).json";

const TEST_DATA: &[(&str, &str, &str, &str)] = &[
    ("Twitter Mini b0", TWITTER_MINI_JSON, "$.search_metadata.count", "blue"),
    (
        "Twitter Mini y0",
        TWITTER_MINI_JSON,
        "$.search_metadata.count",
        "yellow",
    ),
];

// const TEST_DATA: &[(&str, &str, &str, &str)] = &[
//     ("Twitter b0", TWITTER_JSON, "$.search_metadata.count", "blue"),

//     ("Twitter y0", TWITTER_JSON, "$.search_metadata.count", "yellow"),
// ];

// google_map_large
// const TEST_DATA: &[(&str, &str, &str, &str)] = &[
// ("Google b0", GOOGLE_MAP_JSON, "$[*].available_travel_modes", "blue"),
// ("Google b1", GOOGLE_MAP_JSON, "$[*].routes[*].legs[*].steps[*]", "blue"),
// ("Google b2", GOOGLE_MAP_JSON, "$[*].routes[*].legs[*]", "blue"),
// ("Google b3", GOOGLE_MAP_JSON, "$[1]", "blue"),
// ("Google b4", GOOGLE_MAP_JSON, "$[200].routes[1].legs[5].steps[*].distance.text", "blue"),
// ("Google b5", GOOGLE_MAP_JSON, "$[*].routes[*].legs[*].steps[1]", "blue"),
// ("Google b6", GOOGLE_MAP_JSON, "$[500].routes[*].legs[5].steps[*].distance.text", "blue"),
// ("Google b7", GOOGLE_MAP_JSON, "$[1000].routes[1].legs[5].steps[*].distance.text", "blue"),
// ("Google b8", GOOGLE_MAP_JSON, "$[10000].routes[1].legs[5].steps[*].distance.text", "blue"),
// ("Google b9", GOOGLE_MAP_JSON, "$[10000].routes[*]", "blue"),
// ("Google b10", GOOGLE_MAP_JSON, "$[10000].routes[*].legs[*].steps[1]", "blue"),
// ("Google b11", GOOGLE_MAP_JSON, "$[10000].routes[*].legs[1].steps[*].distance.text", "blue"),

// ("Google y0", GOOGLE_MAP_JSON, "$[*].routes[*].legs[*].steps[*].distance.text", "yellow"),
// ("Google y1", GOOGLE_MAP_JSON, "$[*].routes[*]", "yellow"),
// ("Google y2", GOOGLE_MAP_JSON, "$[*].routes[*].warnings", "yellow"),
// ("Google y3", GOOGLE_MAP_JSON, "$[*].routes[*].bounds[*]", "yellow"),
// ("Google y4", GOOGLE_MAP_JSON, "$[*].routes[*].legs[*].steps[1].distance.text", "yellow"),
// ("Google y5", GOOGLE_MAP_JSON, "$[*].routes[*].legs[1].steps[*].distance.text", "yellow"),
// ("Google y6", GOOGLE_MAP_JSON, "$[*].routes[1].legs[*].steps[*].distance.text", "yellow"),
// ("Google y7", GOOGLE_MAP_JSON, "$[1].routes[*].legs[*].steps[*].distance.text", "yellow"),
// ];

// bestbuy large
// const TEST_DATA: &[(&str, &str, &str, &str)] = &[
//     ("Bestbuy b0", BESTBUY_JSON, "$.products[5].videoChapters", "blue"),
//     ("Bestbuy b1", BESTBUY_JSON, "$.products[*].videoChapters", "blue"),
//     ("Bestbuy b2", BESTBUY_JSON, "$.products[2].categoryPath[*].id", "blue"),
//     ("Bestbuy b3", BESTBUY_JSON, "$.products[5].categoryPath[1].id", "blue"),
//     ("Bestbuy b4", BESTBUY_JSON, "$.products[5].shippingLevelsOfService[1].serviceLevelName", "blue"),
//     ("Bestbuy b5", BESTBUY_JSON, "$.products[10].shippingLevelsOfService[1].serviceLevelName", "blue"),
//     ("Bestbuy b6", BESTBUY_JSON, "$.products[*].videoChapters[1].chapter", "blue"),
//     ("Bestbuy b7", BESTBUY_JSON, "$.products[20].monthlyRecurringChargeGrandTotal", "blue"),
//     ("Bestbuy b8", BESTBUY_JSON, "$.products[*].videoChapters[5].chapter", "blue"),
//     ("Bestbuy b9", BESTBUY_JSON, "$.products[*].monthlyRecurringChargeGrandTotal", "blue"),

//     ("Bestbuy y0", BESTBUY_JSON, "$.total", "yellow"),
//     ("Bestbuy y1", BESTBUY_JSON, "$.products[*].shipping[*]", "yellow"),
//     ("Bestbuy y2", BESTBUY_JSON, "$.products[*].shippingLevelsOfService[1].serviceLevelName", "yellow"),
//     ("Bestbuy y3", BESTBUY_JSON, "$.products[*].categoryPath[2]", "yellow"),
//     ("Bestbuy y4", BESTBUY_JSON, "$.products[*].shippingLevelsOfService[*].serviceLevelName", "yellow"),
// ];

pub fn skip_evaluation() {
    let path = get_next_valid_name(".a_lut_tests/performance/skip_evaluation/total.csv");

    for &(test_name, json_path, query_text, color) in TEST_DATA {
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

        for i in 0..REPETITIONS {
            engine.count(&input).expect("Failed to run query normally");
        }
    }
}

pub fn skip_evaluation_old() {
    let path = get_next_valid_name(".a_lut_tests/performance/skip_evaluation/total.csv");
    let csv_path = Path::new(&path);

    for &(test_name, json_path, query_text, color) in TEST_DATA {
        check_if_exists(json_path);

        // Setup data trackers
        let head_line =
            "TEST,JSON,QUERY,COLOR,T_ORIGINAL,T_ORIGINAL_SKIP,T_OPTIMUM,T_LUT,T_LUT_SKIP,T_LUT_BUILD,LUT_CAPACITY";
        let mut data_line = format!("{},{},{},{},", test_name, get_filename(json_path), query_text, color);

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
        for i in 0..REPETITIONS {
            println!("\t{i}");
            // Time normal query, track total time
            reset_skip_time();
            // let mut sink = Vec::new();
            let start_query_time = Instant::now();
            // engine.matches(&input, &mut sink).expect("Failed to run query normally");
            engine.count(&input).expect("Failed to run query normally");
            t_original += start_query_time.elapsed().as_nanos() as u64;
            t_original_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);

            // Build and add LUT
            let distance_cutoff = 0;
            let start_build = Instant::now();
            let lut = LookUpTableImpl::build(json_path, distance_cutoff).expect("Failed to build LUT");
            t_lut_build += start_build.elapsed().as_nanos() as u64;
            lut_capacity = lut.allocated_bytes();
            engine.add_lut(lut);

            // Time LUT query, track total time
            // sink.clear();
            reset_skip_time();
            let start_query_time = Instant::now();
            // engine.matches(&input, &mut sink).expect("Failed to run query normally");
            engine.count(&input).expect("Failed to run query normally");
            t_lut += start_query_time.elapsed().as_nanos() as u64;
            t_lut_skip += SKIP_TIME_ATOMIC.load(std::sync::atomic::Ordering::Relaxed);
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

    plot_with_python(csv_path.to_str().unwrap());
}

pub fn add_skip_time(added_time: u64) {
    SKIP_TIME_ATOMIC.fetch_add(added_time, std::sync::atomic::Ordering::Relaxed);
}

pub fn reset_skip_time() {
    SKIP_TIME_ATOMIC.store(0, std::sync::atomic::Ordering::Relaxed);
}

fn plot_with_python(csv_path: &str) {
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
