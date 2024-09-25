use std::{
    alloc::System,
    io::{self, Write},
    process::Command,
};

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use crate::lookup_table::{
    lut_distance, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF, util_path,
};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub fn compare_heap_size(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("- heap_size");

    let file = std::fs::File::open(json_path)?;
    let filename = util_path::get_filename_from_path(json_path);

    // lut_naive
    let reg = Region::new(&GLOBAL);
    let lut_naive = LutNaive::build_with_json(&file)?;
    let stats_naive = reg.change();

    // lut_distance
    let reg = Region::new(&GLOBAL);
    let lut_distance = lut_distance::build(&file)?;
    let stats_distance = reg.change();

    // lut_perfect_naive
    let reg = Region::new(&GLOBAL);
    let lut_perfect_naive = LutPerfectNaive::build_with_json(&file)?;
    let stats_perfect_naive = reg.change();

    // lut_phf
    let reg = Region::new(&GLOBAL);
    let lut_phf = LutPHF::build_with_json(&file)?;
    let stats_phf = reg.change();

    // Open or create the CSV file for appending
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(
            csv_file,
            "{},{},{},{},{},{}",
            "name",
            "input_size",
            "naive_allocations",
            "distance_allocations",
            "perfect_naive_allocations",
            "phf_allocations"
        )?;
    }

    writeln!(
        csv_file,
        "{},{},{},{},{},{}",
        filename,
        file.metadata().expect("Can't open file").len(),
        stats_naive.bytes_allocated.to_string(),
        stats_distance.bytes_allocated.to_string(),
        stats_perfect_naive.bytes_allocated.to_string(),
        stats_phf.bytes_allocated.to_string(),
    )?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn run_python_statistics_builder(csv_path: &str) {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/heap_size.py")
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
