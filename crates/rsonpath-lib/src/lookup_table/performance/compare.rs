use std::{
    io::{self, Write},
    process::Command,
};

use crate::lookup_table::{
    lut_distance::{self, LutDistance},
    lut_naive::LutNaive,
    lut_perfect_naive::LutPerfectNaive,
    util_path, LookUpTable,
};

pub fn compare_luts_in_speed_and_size(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("- compare");

    let file = std::fs::File::open(json_path)?;
    let filename = util_path::get_filename_from_path(json_path);

    // lut_naive
    let start_build = std::time::Instant::now();
    let lut_naive = LutNaive::build(&json_path)?;
    let naive_build_time = start_build.elapsed();

    // lut_distance
    let start_build = std::time::Instant::now();
    let lut_distance = LutDistance::build(&json_path)?;
    let distance_build_time = start_build.elapsed();

    // lut_perfect_naive
    let start_build = std::time::Instant::now();
    let lut_perfect_naive = LutPerfectNaive::build(&json_path)?;
    let perfect_naive_build_time = start_build.elapsed();

    // Open or create the CSV file for appending
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(csv_file, "name,input_size,build_naive,build_distance,build_perfect_naive,naive_size,distance_size,perfect_naive_size")?;
    }
    writeln!(
        csv_file,
        "{},{},{:.5},{:.5},{:.5},{},{},{}",
        filename,
        file.metadata().expect("Can't open file").len(),
        naive_build_time.as_secs_f64(),
        distance_build_time.as_secs_f64(),
        perfect_naive_build_time.as_secs_f64(),
        lut_naive.estimate_cbor_size(),
        lut_distance.estimate_cbor_size(),
        lut_perfect_naive.estimate_cbor_size(),
    )?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn run_python_statistics_builder(csv_path: &str) {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/compare.py")
        .arg(csv_path)
        .output()
        .expect(&format!("Failed to open csv_path: {}", csv_path));

    if output.status.success() {
        if let Err(e) = io::stdout().write_all(&output.stdout) {
            eprintln!("Failed to write stdout: {}", e);
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
