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
    lookup_table::{performance::lut_evaluation::HEAP_TRACKER, util_path, LookUpTable, LUT},
};

use super::lut_query_data;

const QUERY_REPETITIONS: usize = 3;
const RESULTS_PATH: &str = ".a_lut_tests/performance/distance_cutoff_evaluation";

pub fn evaluate() {
    let cutoffs = vec![16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192];
    // let cutoffs = vec![64, 128, 1024, 2048];

    only_plot(lut_query_data::QUERY_GOOGLE);
    only_plot(lut_query_data::QUERY_TWITTER);
    only_plot(lut_query_data::QUERY_BESTBUY);

    // eval_all(lut_query_data::QUERY_GOOGLE, &cutoffs);
    // eval_all(lut_query_data::QUERY_TWITTER, &cutoffs);
    // eval_all(lut_query_data::QUERY_BESTBUY, &cutoffs);
    // eval_all(lut_query_data::QUERY_POKEMON_SHORT, &cutoffs);
}

fn only_plot(test_data: (&str, &[(&str, &str)])) {
    // Extract input
    let (json_path, queries) = test_data;
    let filename = util_path::extract_filename(json_path);
    println!("JSON: {}", json_path);

    // All necessary paths to CSV and PNG
    let build_csv = format!("{}/{}_build_results.csv", RESULTS_PATH, filename);
    let query_csv = format!("{}/{}_query_results.csv", RESULTS_PATH, filename);
    let counter_csv_path = format!("{}/../skip_tracker/COUNTER_{}.csv", RESULTS_PATH, filename);
    let distance_image_path = format!("{}/../distance_distribution/{}_plot.png", RESULTS_PATH, filename);
    fs::create_dir_all(&RESULTS_PATH).expect("Could not create results directory");

    // Plot it with python
    run_python_statistics_builder(&build_csv, &query_csv, &counter_csv_path, &distance_image_path);
}

fn eval_all(test_data: (&str, &[(&str, &str)]), cutoffs: &Vec<usize>) {
    // Extract input
    let (json_path, queries) = test_data;
    let filename = util_path::extract_filename(json_path);
    println!("JSON: {}", json_path);

    // All necessary paths to CSV and PNG
    let build_csv = format!("{}/{}_build_results.csv", RESULTS_PATH, filename);
    let query_csv = format!("{}/{}_query_results.csv", RESULTS_PATH, filename);
    let counter_csv_path = format!("{}/../skip_tracker/COUNTER_{}.csv", RESULTS_PATH, filename);
    let distance_image_path = format!("{}/../distance_distribution/{}_plot.png", RESULTS_PATH, filename);
    fs::create_dir_all(&RESULTS_PATH).expect("Could not create results directory");

    // Write headers for the CSV files
    let mut wtr = Writer::from_path(&build_csv).expect("Could not open build CSV");
    wtr.write_record(&["JSON", "CUTOFF", "BUILD_TIME_SECONDS", "SIZE_IN_BYTES"])
        .unwrap();
    wtr.flush().unwrap();

    let mut wtr = Writer::from_path(&query_csv).expect("Could not open query CSV");
    wtr.write_record(&["JSON", "CUTOFF", "QUERY_ID", "QUERY_TIME_SECONDS"])
        .unwrap();
    wtr.flush().unwrap();

    // Measurements
    eval_lut(&json_path, &filename, &queries, &build_csv, &query_csv, &cutoffs);
    eval_ite(&json_path, &filename, &queries, &build_csv, &query_csv);

    // Plot it with python
    run_python_statistics_builder(&build_csv, &query_csv, &counter_csv_path, &distance_image_path);
}

fn eval_lut(
    json_path: &str,
    filename: &str,
    queries: &[(&str, &str)],
    build_csv: &str,
    query_csv: &str,
    cutoffs: &Vec<usize>,
) {
    let mut lut;
    for cutoff in cutoffs {
        print!("  cutoff {cutoff}");

        let start_heap = Region::new(HEAP_TRACKER);
        let start_build = std::time::Instant::now();
        lut = LUT::build(json_path, *cutoff).expect("Fail @ build lut");
        let build_time = start_build.elapsed().as_secs_f64();
        let heap_bytes = heap_value(start_heap.change());

        println!(" build = {:.5}s, size = {} B", build_time, heap_bytes);

        // Append build result
        let mut wtr = Writer::from_writer(fs::OpenOptions::new().append(true).open(build_csv).unwrap());
        wtr.write_record(&[
            filename,
            &cutoff.to_string(),
            &format!("{:.5}", build_time),
            &heap_bytes.to_string(),
        ])
        .unwrap();
        wtr.flush().unwrap();

        // Append each query result
        let mut wtr_query = Writer::from_writer(fs::OpenOptions::new().append(true).open(query_csv).unwrap());

        for (query_id, query_text) in queries {
            let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");
            let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");
            engine.add_lut(lut);

            let start_query = std::time::Instant::now();
            for _ in 0..QUERY_REPETITIONS {
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

            wtr_query
                .write_record(&[
                    filename,
                    &cutoff.to_string(),
                    query_id,
                    &format!("{:.5}", query_time_average),
                ])
                .unwrap();
        }

        wtr_query.flush().unwrap();
        drop(lut);
    }
}

fn eval_ite(json_path: &str, filename: &str, queries: &[(&str, &str)], build_csv: &str, query_csv: &str) {
    let cutoff: usize = 0;
    let build_time: f64 = 0.0;
    let heap_bytes: usize = 0;

    print!("  ITE");

    // Append build info
    let mut wtr = Writer::from_writer(fs::OpenOptions::new().append(true).open(build_csv).unwrap());
    wtr.write_record(&[
        filename,
        &cutoff.to_string(),
        &format!("{:.5}", build_time),
        &heap_bytes.to_string(),
    ])
    .unwrap();
    wtr.flush().unwrap();

    // Append query results
    let mut wtr_query = Writer::from_writer(fs::OpenOptions::new().append(true).open(query_csv).unwrap());

    for (query_id, query_text) in queries {
        let query = rsonpath_syntax::parse(query_text).expect("Fail @ parse query");
        let engine = RsonpathEngine::compile_query(&query).expect("Fail @ compile query");

        let start_query = std::time::Instant::now();
        for _ in 0..QUERY_REPETITIONS {
            let input = {
                let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ open File"));
                let mut buf = vec![];
                file.read_to_end(&mut buf).expect("Fail @ file read");
                OwnedBytes::new(buf)
            };
            let _ = engine.count(&input).expect("Failed to run query normally");
        }
        let query_time_average = start_query.elapsed().as_secs_f64() / (QUERY_REPETITIONS as f64);
        println!("  - query = {query_id}, time = {:.5}s ", query_time_average);

        wtr_query
            .write_record(&[
                filename,
                &cutoff.to_string(),
                query_id,
                &format!("{:.5}", query_time_average),
            ])
            .unwrap();
    }

    wtr_query.flush().unwrap();
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
