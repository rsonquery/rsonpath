use std::io;
use std::path::Path;
use std::process::Command;
use std::{error::Error, fs, io::Write};

use crate::lookup_table::util;
use crate::lookup_table::{lut_distance, lut_naive};

#[inline]
pub fn performance_test(json_folder: &str, output_path: &str, csv_folder: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(json_folder)?;

    if metadata.is_dir() {
        let folder_name = util::get_filename_from_path(json_folder);
        let suffix = check_counter(json_folder, csv_folder);
        let csv_path_stats = format!("{}/{}_stats{}.csv", csv_folder, folder_name, suffix);
        let csv_stats_vs = format!("{}/{}_naive_vs_distance{}.csv", csv_folder, folder_name, suffix);

        for entry in fs::read_dir(json_folder)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let sub_json_path = path.to_str().expect("Failed to convert path to string");

                println!("Processing: {}", util::get_filename_from_path(sub_json_path));
                println!("- stats");
                measure_time_and_size(sub_json_path, output_path, &csv_path_stats)?;
                println!("- vs");
                compare_naive_and_distance(sub_json_path, &csv_stats_vs)?;
            }
        }

        run_python_statistics_builder(&csv_path_stats, &csv_stats_vs)?;
    }

    Ok(())
}

fn check_counter(json_folder: &str, csv_folder: &str) -> String {
    // Check if csv_path already exists and if it does rename it with a unique number
    let mut csv_path = format!("{}/{}_stats.csv", csv_folder, util::get_filename_from_path(json_folder));
    let mut counter = 0;
    while Path::new(&csv_path).exists() {
        counter += 1;
        csv_path = format!(
            "{}/{}_stats({}).csv",
            csv_folder,
            util::get_filename_from_path(json_folder),
            counter
        );
    }

    if counter > 0 {
        format!("({})", counter)
    } else {
        String::new()
    }
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

    // Get the input JSON file size in bytes
    let input_file_metadata = file.metadata()?;
    let input_file_size = input_file_metadata.len();

    // Measure build duration
    let start_build = std::time::Instant::now();
    if let Ok(lut_naive) = lut_naive::build(&file) {
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

fn compare_naive_and_distance(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the input JSON file
    let file = std::fs::File::open(json_path)?;
    let filename = util::get_filename_from_path(json_path);

    // Get the input JSON file size in bytes
    let input_file_metadata = file.metadata()?;
    let input_size = input_file_metadata.len();

    // lut_naive
    let start_build = std::time::Instant::now();
    let lut_naive = lut_naive::build(&file)?;
    let lut_naive_build_duration = start_build.elapsed();

    // lut_distance
    let start_build = std::time::Instant::now();
    let lut_distance = lut_distance::build(&file)?;
    let lut_distance_build_duration = start_build.elapsed();

    // Open or create the CSV file for appending
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;

    // If the file is freshly created, add the header row
    if csv_file.metadata()?.len() == 0 {
        writeln!(
            csv_file,
            "name,input_size,build_naive,build_distance,naive_size,distance_size"
        )?;
    }

    // Write the results to the CSV file with durations rounded to 5 decimal places
    writeln!(
        csv_file,
        "{},{},{:.5},{:.5},{},{}",
        filename,
        input_size,
        lut_naive_build_duration.as_secs_f64(),
        lut_distance_build_duration.as_secs_f64(),
        lut_naive.estimate_cbor_size(),
        lut_distance.estimate_cbor_size(),
    )?;

    Ok(())
}

fn run_python_statistics_builder(csv_path_stats: &str, csv_path_vs: &str) -> io::Result<()> {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/main.py")
        .arg(csv_path_stats)
        .arg(csv_path_vs)
        .output()?;

    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
