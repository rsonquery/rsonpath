use std::{
    io::{self, Write},
    process::Command,
};

use crate::lookup_table::{
    lut_distance::LutDistance, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, util_path, LookUpTable,
};

#[inline]
pub fn compare_build_time(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);

    // lut_naive
    let start_build = std::time::Instant::now();
    let _ = LutNaive::build(json_path)?;
    let naive_build_time = start_build.elapsed();

    // lut_distance
    let start_build = std::time::Instant::now();
    let _ = LutDistance::build(json_path)?;
    let distance_build_time = start_build.elapsed();

    // lut_perfect_naive
    let start_build = std::time::Instant::now();
    let _ = LutPerfectNaive::build(json_path)?;
    let perfect_naive_build_time = start_build.elapsed();

    // lut_phf
    let start_build = std::time::Instant::now();
    let _ = LutPHF::build(json_path)?;
    let phf_build_time = start_build.elapsed();

    // lut_phf_double
    let start_build = std::time::Instant::now();
    let _ = LutPHFDouble::build(json_path)?;
    let phf_double_build_time = start_build.elapsed();

    // lut_phf_group
    let start_build = std::time::Instant::now();
    let _ = LutPHFGroup::build(json_path)?;
    let phf_group_build_time = start_build.elapsed();

    // Open or create the CSV file for appending
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(
            csv_file,
            "name,input_size,naive,distance,perfect_naive,phf,phf_double,phf_group"
        )?;
    }
    writeln!(
        csv_file,
        "{},{},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5}",
        filename,
        file.metadata().expect("Can't open file").len(),
        naive_build_time.as_secs_f64(),
        distance_build_time.as_secs_f64(),
        perfect_naive_build_time.as_secs_f64(),
        phf_build_time.as_secs_f64(),
        phf_double_build_time.as_secs_f64(),
        phf_group_build_time.as_secs_f64(),
    )?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn run_python_statistics_builder(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/build_time_eval.py")
        .arg(csv_path)
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
