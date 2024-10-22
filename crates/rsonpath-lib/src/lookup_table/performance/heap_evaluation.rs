use crate::lookup_table::{
    count_distances, lut_naive::LutNaive, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, util_path, LookUpTable,
};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::{
    alloc::System,
    io::{self, Write},
    process::Command,
};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

#[inline]
pub fn compare_heap_size(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);

    let num_keys = count_distances::count_num_pairs(json_path);

    // lut_naive
    let reg = Region::new(GLOBAL);
    let lut = LutNaive::build(json_path)?;
    let stats_naive = reg.change();
    let naive_capacity = lut.allocated_bytes();
    drop(lut);

    // lut_perfect_naive
    let reg = Region::new(GLOBAL);
    let lut = LutPerfectNaive::build(json_path)?;
    let stats_perfect_naive = reg.change();
    let perfect_naive_capacity = lut.allocated_bytes();
    drop(lut);

    // lut_phf
    let reg = Region::new(GLOBAL);
    let lut = LutPHF::build(json_path)?;
    let stats_phf = reg.change();
    let phf_capacity = lut.allocated_bytes();
    drop(lut);

    // lut_phf_double
    let reg = Region::new(GLOBAL);
    let lut = LutPHFDouble::build(json_path)?;
    let stats_phf_double = reg.change();
    let phf_double_capacity = lut.allocated_bytes();
    drop(lut);

    // lut_phf_group
    let reg = Region::new(GLOBAL);
    let lut = LutPHFGroup::build(json_path)?;
    let stats_phf_group = reg.change();
    let phf_group_capacity = lut.allocated_bytes();
    drop(lut);

    // Open or create the CSV file for appending
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(
            csv_file,
            "name,input_size,num_keys,\
            naive,perfect_naive,phf,phf_double,phf_group,\
            naive_capacity,perfect_naive_capacity,phf_capacity,phf_double_capacity,phf_group_capacity,\
            "
        )?;
    }

    writeln!(
        csv_file,
        "{},{},{},{},{},{},{},{},{},{},{},{},{}",
        filename,
        file.metadata().expect("Can't open file").len(),
        num_keys,
        heap_value(stats_naive),
        heap_value(stats_perfect_naive),
        heap_value(stats_phf),
        heap_value(stats_phf_double),
        heap_value(stats_phf_group),
        naive_capacity,
        perfect_naive_capacity,
        phf_capacity,
        phf_double_capacity,
        phf_group_capacity
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
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/heap_evaluation.py")
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
