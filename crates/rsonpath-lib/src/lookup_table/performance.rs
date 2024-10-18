use crate::lookup_table::util_path;
use std::{error::Error, fs, path::Path};

pub mod build_time_eval;
pub mod get_time_eval;
pub mod heap_eval;
pub mod stats;

pub const HEAP_EVAL_DIR: &str = "heap_evaluation";
pub const BUILD_TIME_EVAL_DIR: &str = "build_time_evaluation";
pub const GET_TIME_EVAL_DIR: &str = "get_time_evaluation";

/// INPUT: json_dir, OUTPUT: csv_dir
#[inline]
pub fn performance_test(json_dir: &str, csv_dir: &str, tasks: u16) {
    match tasks {
        0 => build_time_evaluation(json_dir, csv_dir),
        1 => heap_evaluation(json_dir, csv_dir),
        2 => get_time_evaluation(json_dir, csv_dir),
        3 => {
            build_time_evaluation(json_dir, csv_dir);
            heap_evaluation(json_dir, csv_dir);
            get_time_evaluation(json_dir, csv_dir);
        }
        _ => eprintln!("Invalid task selection"),
    }
}

fn evaluate(
    json_dir: &str,
    csv_dir: &str,
    eval_type: &str,
    eval_fn: impl Fn(&str, &str) -> Result<(), Box<dyn Error>>,
) {
    let evaluation_dir = format!("{}/{}", csv_dir, eval_type);
    fs::create_dir_all(&evaluation_dir).expect("Failed to create output directory");

    let metadata = fs::metadata(json_dir).expect("Can't open json_dir");
    if metadata.is_dir() {
        let dir_name = util_path::extract_filename(json_dir);
        let suffix = get_next_valid_filename(json_dir, csv_dir);
        let csv_path = format!("{}/{}/{}_compare{}.csv", csv_dir, eval_type, dir_name, suffix);

        for entry in fs::read_dir(json_dir).expect("Can't build iterator") {
            let path = entry.expect("Can't open file").path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let sub_json_path = path.to_str().expect("Failed to convert path to string");
                println!(
                    "Measuring {}: {}",
                    eval_type,
                    util_path::extract_filename(sub_json_path)
                );
                eval_fn(sub_json_path, &csv_path).expect("Failed at measuring");
            }
        }
    }
}

fn build_time_evaluation(json_dir: &str, csv_dir: &str) {
    evaluate(
        json_dir,
        csv_dir,
        BUILD_TIME_EVAL_DIR,
        build_time_eval::compare_build_time,
    );
}

fn heap_evaluation(json_dir: &str, csv_dir: &str) {
    evaluate(json_dir, csv_dir, HEAP_EVAL_DIR, heap_eval::compare_heap_size);
}

fn get_time_evaluation(json_dir: &str, csv_dir: &str) {
    evaluate(json_dir, csv_dir, GET_TIME_EVAL_DIR, get_time_eval::compare_get_time);
}

/// Check if csv_path already exists and if it does rename it with a unique number
fn get_next_valid_filename(json_folder: &str, csv_folder: &str) -> String {
    let base_path = format!("{}/{}_stats", csv_folder, util_path::extract_filename(json_folder));
    let mut counter = 0;

    while Path::new(&format!("{}.csv", base_path)).exists() {
        counter += 1;
    }

    if counter > 0 {
        format!("({})", counter)
    } else {
        String::new()
    }
}
