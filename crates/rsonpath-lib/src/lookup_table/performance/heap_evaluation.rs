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
pub fn run(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Write;

    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);
    let num_keys = count_distances::count_num_pairs(json_path);

    let mut csv_head_line = String::from("name,input_size_bytes,num_keys,");
    let mut csv_info_line = format!("{},{},{},", filename, file.metadata()?.len(), num_keys);

    macro_rules! measure_heap_and_capacity {
        ($lut_type:ty, $heap_label:expr, $capacity_label:expr) => {{
            let reg = Region::new(GLOBAL);
            let lut = <$lut_type>::build(json_path)?;
            let stats = heap_value(reg.change());
            csv_head_line.push_str(&format!("{},{},", $heap_label, $capacity_label));
            csv_info_line.push_str(&format!("{},{},", stats, lut.allocated_bytes()));
            drop(lut);
        }};
    }

    // Process each LUT
    measure_heap_and_capacity!(LutNaive, "naive_heap", "naive_capacity");
    measure_heap_and_capacity!(LutPerfectNaive, "perfect_naive_heap", "perfect_naive_capacity");
    measure_heap_and_capacity!(LutPHF, "phf_heap", "phf_capacity");
    measure_heap_and_capacity!(LutPHFDouble, "phf_double_heap", "phf_double_capacity");
    measure_heap_and_capacity!(LutPHFGroup, "phf_group_heap", "phf_group_capacity");

    // Write CSV header and data
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(csv_file, "{}", csv_head_line)?;
    }
    writeln!(csv_file, "{}", csv_info_line)?;

    // Build statistics
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
