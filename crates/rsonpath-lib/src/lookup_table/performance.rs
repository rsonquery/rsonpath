use crate::lookup_table::util_path;
use std::{fs, path::Path};

pub mod distance_cutoff_evaluation;
pub mod lut_evaluation;
pub mod lut_query_correctness;
pub mod lut_query_data;
pub mod lut_skip_counter;
pub mod lut_skip_evaluation;

pub const EVAL_DIR: &str = "evaluation";

#[inline]
pub fn performance_test(json_dir: &str, out_dir: &str) {
    let evaluation_dir = format!("{}/{}", out_dir, EVAL_DIR);
    fs::create_dir_all(&evaluation_dir).expect("Failed to create output directory");

    let metadata = fs::metadata(json_dir).expect("Can't open json_dir");
    if metadata.is_dir() {
        let json_dir_name = util_path::extract_filename(json_dir);
        let csv_path = get_next_valid_csv_path(out_dir, &json_dir_name);

        for entry in fs::read_dir(json_dir).expect("Can't build iterator") {
            let path = entry.expect("Can't open file").path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let sub_json_path = path.to_str().expect("Failed to convert path to string");
                lut_evaluation::evaluate(sub_json_path, &csv_path).expect("Failed at measuring");
            }
        }
    }
}

/// Check if csv_path already exists and if it does rename it with a unique number and retry
fn get_next_valid_csv_path(csv_dir: &str, json_dir_name: &str) -> String {
    let mut csv_path = format!("{}/{}/{}.csv", csv_dir, EVAL_DIR, json_dir_name);

    let mut counter = 1;
    while Path::new(&csv_path).exists() {
        csv_path = format!("{}/{}/{}({}).csv", csv_dir, EVAL_DIR, json_dir_name, counter);
        counter += 1;
    }

    csv_path
}
