use std::{
    fs,
    io::{self, BufReader, Read, Write},
    process::Command,
};

use csv::Writer;
use stats_alloc::Region;

use crate::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{
        lut_ptr_hash_double::LutPtrHashDouble, performance::lut_evaluation::GLOBAL, util_path, LookUpTable, LUT,
    },
};

use super::lut_query_data;

const QUERY_REPETITIONS: usize = 1;
const RESULTS_PATH: &str = ".a_lut_tests/performance/distance_cutoff_evaluation";

pub fn evaluate() {
    let distance_cutoffs = vec![0, 1000];

    eval_query_set(lut_query_data::QUERY_GOOGLE, distance_cutoffs);
}

fn eval_query_set(test_data: (&str, &[(&str, &str)]), distance_cutoffs: Vec<usize>) {
    let (json_path, queries) = test_data;
    let filename = util_path::extract_filename(json_path);
    println!("JSON: {}", json_path);

    // CSV structure: (filename, cutoff, build_time, heap_bytes)
    let mut build_results = Vec::new();
    // CSV structure: (filename, cutoff, query_id, query_time_avg)
    let mut query_results = Vec::new();

    let mut lut;
    for cutoff in distance_cutoffs {
        print!("  cutoff {cutoff}");

        // Build time & heap size
        let start_heap = Region::new(GLOBAL);

        let start_build = std::time::Instant::now();
        lut = LUT::build(json_path, cutoff).expect("Fail @ build lut");
        let build_time = start_build.elapsed().as_secs_f64();

        let heap_bytes = heap_value(start_heap.change());
        println!(" build = {:.5}s, size = {} B", build_time, heap_bytes);

        // Store build result
        build_results.push((filename.clone(), cutoff, build_time, heap_bytes));

        // Queries
        for query in queries {
            let (query_id, query_text) = query;

            // Build query and add the LUT to it
            let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");
            let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
            engine.add_lut(lut);

            let start_query = std::time::Instant::now();
            for _i in 0..QUERY_REPETITIONS {
                // Get results
                let input = {
                    let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
                    let mut buf = vec![];
                    file.read_to_end(&mut buf).expect("Fail @ file read");
                    OwnedBytes::new(buf)
                };
                let _ = engine.count(&input).expect("Failed to run query normally");
            }
            let query_time_average = start_query.elapsed().as_secs_f64() / (QUERY_REPETITIONS as f64);
            lut = engine.take_lut().expect("Fail at taking LUT back");
            println!("  - query = {query_id}, time = {:.5}s ", query_time_average);

            // Store query result
            query_results.push((filename.clone(), cutoff, query_id.to_string(), query_time_average));
        }
        drop(lut);
    }

    // Save results to CSV
    let build_csv_path = format!("{}/{}_build_results.csv", RESULTS_PATH, filename);
    let query_csv_path = format!("{}/{}_query_results.csv", RESULTS_PATH, filename);
    let counter_csv_path = format!("{}/../skip_tracker/COUNTER_{}.csv", RESULTS_PATH, filename);
    let distance_image_path = format!("{}/../distance_distribution/{}_plot.png", RESULTS_PATH, filename);

    fs::create_dir_all(&RESULTS_PATH).expect("Could not create results directory");

    // Write build csv
    let mut wtr = Writer::from_path(&build_csv_path).expect("Could not open build CSV");
    wtr.write_record(&["JSON", "CUTOFF", "BUILD_TIME_SECONDS", "SIZE_IN_BYTES"])
        .unwrap();
    for (json, cutoff, build_time, size) in build_results {
        wtr.write_record(&[json, cutoff.to_string(), format!("{:.5}", build_time), size.to_string()])
            .unwrap();
    }
    wtr.flush().unwrap();

    // Write query csv
    let mut wtr = Writer::from_path(&query_csv_path).expect("Could not open query CSV");
    wtr.write_record(&["JSON", "CUTOFF", "QUERY_ID", "QUERY_TIME_SECONDS"])
        .unwrap();
    for (json, cutoff, query_id, query_time) in query_results {
        wtr.write_record(&[json, cutoff.to_string(), query_id, format!("{:.5}", query_time)])
            .unwrap();
    }
    wtr.flush().unwrap();

    // Plot it with python
    run_python_statistics_builder(
        &build_csv_path,
        &query_csv_path,
        &counter_csv_path,
        &distance_image_path,
    );
}

// We take the allocated bytes minus the deallocated and ignore the reallocated bytes because we are interested
// in the total heap space taken
fn heap_value(stats: stats_alloc::Stats) -> isize {
    stats.bytes_allocated as isize - stats.bytes_deallocated as isize
}

fn run_python_statistics_builder(
    build_csv_path: &str,
    query_csv_path: &str,
    counter_csv_path: &str,
    distance_image_path: &str,
) {
    let msg = format!("Failed to open csv_path: {}", build_csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/distance_cutoff_evaluation.py")
        .arg(build_csv_path)
        .arg(query_csv_path)
        .arg(counter_csv_path)
        .arg(distance_image_path)
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
