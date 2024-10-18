use crate::lookup_table::{
    lut_distance::LutDistance, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, pair_finder, util_path, LookUpTable,
};
use std::{
    io::{self, Write},
    process::Command,
};

#[inline]
pub fn compare_get_time(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);

    let (keys, _) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");

    // lut_naive
    let lut = LutNaive::build(json_path)?;
    let start_build = std::time::Instant::now();
    get_every_key_once(&lut, &keys);
    let naive_query_time = start_build.elapsed();

    // lut_distance
    let lut = LutDistance::build(json_path)?;
    let start_build = std::time::Instant::now();
    get_every_key_once(&lut, &keys);
    let distance_query_time = start_build.elapsed();

    // lut_perfect_naive
    let lut = LutPerfectNaive::build(json_path)?;
    let start_build = std::time::Instant::now();
    get_every_key_once(&lut, &keys);
    let perfect_naive_query_time = start_build.elapsed();

    // lut_phf
    let lut = LutPHF::build(json_path)?;
    let start_build = std::time::Instant::now();
    get_every_key_once(&lut, &keys);
    let phf_query_time = start_build.elapsed();

    // lut_phf_double
    let lut = LutPHFDouble::build(json_path)?;
    let start_build = std::time::Instant::now();
    get_every_key_once(&lut, &keys);
    let phf_double_query_time = start_build.elapsed();

    // lut_phf_group
    let lut = LutPHFGroup::build(json_path)?;
    let start_build = std::time::Instant::now();
    get_every_key_once(&lut, &keys);
    let phf_group_query_time = start_build.elapsed();

    // Open or create the CSV file for appending
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(
            csv_file,
            "name, input_size, naive, distance, perfect_naive, phf, phf_double, phf_group"
        )?;
    }

    writeln!(
        csv_file,
        "{},{},{:.5},{:.5},{:.5},{:.5},{:.5},{:.5}",
        filename,
        file.metadata().expect("Can't open file").len(),
        naive_query_time.as_secs_f64(),
        distance_query_time.as_secs_f64(),
        perfect_naive_query_time.as_secs_f64(),
        phf_query_time.as_secs_f64(),
        phf_double_query_time.as_secs_f64(),
        phf_group_query_time.as_secs_f64(),
    )?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn get_every_key_once(lut: &dyn LookUpTable, keys: &[usize]) {
    for key in keys {
        let _ = lut.get(key);
    }
}

fn run_python_statistics_builder(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/get_time_eval.py")
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
