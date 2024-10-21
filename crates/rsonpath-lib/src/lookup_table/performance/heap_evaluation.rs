use std::{
    alloc::System,
    io::{self, Write},
    process::Command,
};

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use crate::lookup_table::{
    lut_distance::LutDistance, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, util_path, LookUpTable,
};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

#[inline]
pub fn compare_heap_size(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);

    // lut_naive
    let reg = Region::new(GLOBAL);
    let _lut = LutNaive::build(json_path)?;
    let stats_naive = reg.change();
    drop(_lut);

    // lut_distance
    let reg = Region::new(GLOBAL);
    let _lut = LutDistance::build(json_path)?;
    let stats_distance = reg.change();
    drop(_lut);

    // lut_perfect_naive
    // let reg = Region::new(GLOBAL);
    // let _lut = LutPerfectNaive::build(json_path)?;
    // let stats_perfect_naive = reg.change();
    // drop(_lut);

    // lut_phf
    let reg = Region::new(GLOBAL);
    let _lut = LutPHF::build(json_path)?;
    let stats_phf = reg.change();
    drop(_lut);

    // lut_phf_double
    let reg = Region::new(GLOBAL);
    let _lut = LutPHFDouble::build(json_path)?;
    let stats_phf_double = reg.change();
    drop(_lut);

    // lut_phf_group
    let reg = Region::new(GLOBAL);
    let _lut = LutPHFGroup::build(json_path)?;
    let stats_phf_group = reg.change();
    drop(_lut);

    // Open or create the CSV file for appending
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(
            csv_file,
            "name,input_size,naive,distance,perfect_naive,phf,phf_double,phf_group, \
            naive1,distance1,perfect_naive1,phf1,phf_double1,phf_group1, \
            naive2,distance2,perfect_naive2,phf2,phf_double2,phf_group2, \
            naive3,distance3,perfect_naive3,phf3,phf_double3,phf_group3"
        )?;
    }

    writeln!(
        csv_file,
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        filename,
        file.metadata().expect("Can't open file").len(),
        heap_value(stats_naive),
        heap_value(stats_distance),
        // heap_value(stats_perfect_naive),
        heap_value(stats_phf),
        heap_value(stats_phf_double),
        heap_value(stats_phf_group),
        stats_naive.bytes_allocated,
        stats_distance.bytes_allocated,
        // stats_perfect_naive.bytes_allocated,
        stats_phf.bytes_allocated,
        stats_phf_double.bytes_allocated,
        stats_phf_group.bytes_allocated,
        stats_naive.bytes_deallocated,
        stats_distance.bytes_deallocated,
        // stats_perfect_naive.bytes_deallocated,
        stats_phf.bytes_deallocated,
        stats_phf_double.bytes_deallocated,
        stats_phf_group.bytes_deallocated,
        stats_naive.bytes_reallocated,
        stats_distance.bytes_reallocated,
        // stats_perfect_naive.bytes_reallocated,
        stats_phf.bytes_reallocated,
        stats_phf_double.bytes_reallocated,
        stats_phf_group.bytes_reallocated,
    )?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn heap_value(stats: stats_alloc::Stats) -> isize {
    // stats.bytes_allocated as isize - stats.bytes_deallocated as isize + stats.bytes_reallocated
    stats.bytes_allocated as isize - stats.bytes_deallocated as isize
}

fn run_python_statistics_builder(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/heap_eval.py")
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
