use crate::lookup_table::pair_data;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufWriter, Write};

const RESULTS_FOLDER: &str = ".a_lut_tests/analysis";

pub fn create_json_size_csv(json_folder_path: &str) {
    eval(json_folder_path, RESULTS_FOLDER);
}

fn eval(json_folder_path: &str, result_folder_path: &str) {
    // Create CSV file
    let output_path = format!("{}/json_size_analysis.csv", result_folder_path);
    let file = File::create(&output_path).expect("Could not create output CSV file");
    let mut writer = BufWriter::new(file);

    // Write header
    writeln!(writer, "NAME,SIZE_BYTES,NUM_BRACKETS,CURLY_PERCENT,SQUARY_PERCENT").expect("Could not write CSV header");

    // Iterate over every .json file in the folder
    let paths = fs::read_dir(json_folder_path).expect("Failed to read input directory");

    for entry in paths.flatten() {
        let path = entry.path();
        if path.extension() == Some(OsStr::new("json")) {
            let json_path = path.to_str().unwrap();
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let metadata = fs::metadata(&path).expect("Could not read file metadata");
            let size_bytes = metadata.len();

            let (num_curly, num_squary) = pair_data::count_brackets(json_path, 0).expect("Error while counting");
            let total = num_curly + num_squary;

            if total == 0 {
                println!("Total is 0, WRONG INPUT");
                continue; // avoid division by zero
            }

            let curly_percent = (num_curly as f64) / (total as f64) * 100.0;
            let squary_percent = (num_squary as f64) / (total as f64) * 100.0;

            writeln!(
                writer,
                "{},{},{},{:.2},{:.2}",
                file_name, size_bytes, total, curly_percent, squary_percent
            )
            .expect("Could not write CSV line");
        }
    }
}
