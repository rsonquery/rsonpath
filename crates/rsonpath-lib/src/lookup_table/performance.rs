use crate::lookup_table::util_path;
use std::{error::Error, fs, path::Path};

pub mod build_time_evaluation;
pub mod heap_evaluation;

pub const HEAP_EVAL_DIR: &str = "heap_evaluation";
pub const BUILD_TIME_EVAL_DIR: &str = "build_time_evaluation";

#[inline]
pub fn performance_test(json_dir: &str, out_dir: &str, tasks: u16) {
    match tasks {
        0 => {
            build_time_evaluation(json_dir, out_dir);
            heap_evaluation(json_dir, out_dir);
        }
        1 => build_time_evaluation(json_dir, out_dir),
        2 => heap_evaluation(json_dir, out_dir),
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
        let json_dir_name = util_path::extract_filename(json_dir);
        let csv_path = get_next_valid_csv_path(csv_dir, eval_type, &json_dir_name);

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
    evaluate(json_dir, csv_dir, BUILD_TIME_EVAL_DIR, build_time_evaluation::run);
}

fn heap_evaluation(json_dir: &str, csv_dir: &str) {
    evaluate(json_dir, csv_dir, HEAP_EVAL_DIR, heap_evaluation::run);
}

/// Check if csv_path already exists and if it does rename it with a unique number and retry
fn get_next_valid_csv_path(csv_dir: &str, eval_type: &str, json_dir_name: &str) -> String {
    let mut csv_path = format!("{}/{}/{}.csv", csv_dir, eval_type, json_dir_name);

    let mut counter = 1;
    while Path::new(&csv_path).exists() {
        csv_path = format!("{}/{}/{}({}).csv", csv_dir, eval_type, json_dir_name, counter);
        counter += 1;
    }

    csv_path
}
