use std::io;
use std::process::Command;
use std::{error::Error, fs, io::Write};

use crate::lookup_table::{lut_builder, util};

/// Runs performance tests on directory of JSON files and saves the results to a CSV file.
///
/// # Arguments
/// * `json_path` - Path to the JSON file or directory containing JSON files.
/// * `output_path` - Directory path to save the output files.
/// * `csv_path` - Path to the CSV file where the results will be saved.
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Returns `Ok` if successful, or an error if something goes wrong.
#[inline]
pub fn performance_test(json_folder: &str, output_path: &str, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(json_folder)?;

    if metadata.is_dir() {
        for entry in fs::read_dir(json_folder)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let sub_json_path = path.to_str().expect("Failed to convert path to string");
                measure_time_and_size(sub_json_path, output_path, csv_path)?;
            }
        }

        println!("Saving stats to {}", csv_path);
        run_python_statistics_builder(csv_path)?;
    }

    Ok(())
}

/// Measures and records time and size metrics for building and serializing a lookup table from a JSON file.
///
/// # Arguments
/// * `json_path` - Path to the input JSON file.
/// * `output_path` - Directory path to save the output files.
/// * `csv_path` - Path to the CSV file where the results will be recorded.
///
/// # Returns
/// * `Result<(), Box<dyn Error>>` - Returns `Ok` if successful, or an error if something goes wrong.
fn measure_time_and_size(json_path: &str, output_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input JSON file
    let file = std::fs::File::open(json_path)?;
    let filename = util::get_filename_from_path(json_path);
    println!("Processing: {}", filename);

    // Get the input JSON file size in bytes
    let input_file_metadata = file.metadata()?;
    let input_file_size = input_file_metadata.len();

    // Measure build duration
    let start_build = std::time::Instant::now();
    if let Ok(lut_naive) = lut_builder::run(&file) {
        let build_duration = start_build.elapsed();

        // Measure JSON serialization & deserialization duration
        let lut_json_path = &format!("{}/{}.json", output_path, filename);

        let start_json = std::time::Instant::now();
        lut_naive.serialize(lut_json_path)?;
        let json_serialize_duration = start_json.elapsed() + build_duration;

        let start_json_deserialize = std::time::Instant::now();
        lut_naive.deserialize(lut_json_path)?;
        let json_deserialize_duration = start_json_deserialize.elapsed() + json_serialize_duration;

        // Measure CBOR serialization & deserialization duration
        let lut_cbor_path = &format!("{}/{}.cbor", output_path, filename);

        let start_cbor = std::time::Instant::now();
        lut_naive.serialize(lut_cbor_path)?;
        let cbor_serialize_duration = start_cbor.elapsed() + build_duration;

        let start_cbor_deserialize = std::time::Instant::now();
        lut_naive.deserialize(lut_cbor_path)?;
        let cbor_deserialize_duration = start_cbor_deserialize.elapsed() + cbor_serialize_duration;

        // Open or create the CSV file for appending
        let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;

        // If the file is freshly created, add the header row
        if csv_file.metadata()?.len() == 0 {
            writeln!(
                csv_file,
                "name,input_size,build,cbor_serialize,json_serialize,json_deserialize,cbor_deserialize,cbor_size,json_size"
            )?;
        }

        // Write the results to the CSV file with durations rounded to 5 decimal places
        writeln!(
            csv_file,
            "{},{},{:.5},{:.5},{:.5},{:.5},{:.5},{},{}",
            filename,
            input_file_size,
            build_duration.as_secs_f64(),
            cbor_serialize_duration.as_secs_f64(),
            json_serialize_duration.as_secs_f64(),
            json_deserialize_duration.as_secs_f64(),
            cbor_deserialize_duration.as_secs_f64(),
            lut_naive.estimate_cbor_size(),
            lut_naive.estimate_json_size(),
        )?;
    }

    Ok(())
}

/// Executes a Python script to process the CSV file and generate statistics.
///
/// # Arguments
/// * `csv_path` - Path to the CSV file that will be processed by the Python script.
///
/// # Returns
/// * `io::Result<()>` - Returns `Ok` if successful, or an I/O error if something goes wrong.
fn run_python_statistics_builder(csv_path: &str) -> io::Result<()> {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/main.py")
        .arg(csv_path)
        .output()?;

    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
