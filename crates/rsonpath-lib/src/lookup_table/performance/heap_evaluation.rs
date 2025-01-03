use crate::lookup_table::lut_hash_map_double::LutHashMapDouble;
use crate::lookup_table::{
    count_distances, lut_hash_map::LutHashMap, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, util_path, LookUpTable, LookUpTableLambda,
};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::io::Write;
use std::{
    alloc::System,
    io::{self},
    process::Command,
};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

/// Helper struct to reduce the number of parameters when calling functions
pub struct EvalConfig<'a> {
    json_path: &'a str,
    head_line: &'a mut String,
    data_line: &'a mut String,
}

#[inline]
pub fn run(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);
    let num_keys = count_distances::count_num_pairs(json_path);

    let mut head_line = String::from("name,input_size_bytes,num_keys,");
    let mut data_line = format!("{},{},{},", filename, file.metadata()?.len(), num_keys);

    let mut config = EvalConfig {
        json_path,
        head_line: &mut head_line,
        data_line: &mut data_line,
    };

    // Measure LUTs without lambda parameter
    eval::<LutHashMap>(&mut config, "hash_map");
    eval::<LutHashMapDouble>(&mut config, "hash_map_double");
    eval::<LutPerfectNaive>(&mut config, "perfect_naive");

    // Process each LUT that has a lambda parameter with lambda [1, ..., 5]
    for lambda in 1..5 {
        for threaded in [true, false] {
            eval_lambda::<LutPHF>(&mut config, "phf", lambda, threaded);
            eval_lambda::<LutPHFDouble>(&mut config, "phf_double", lambda, threaded);
            eval_lambda::<LutPHFGroup>(&mut config, "phf_group", lambda, threaded);
        }
    }

    for lambda in 1..5 {
        for bit_mask in [3, 7, 15, 31, 63, 127] {
            eval_bucket(&mut config, "phf_group", bit_mask, lambda, false);
        }
    }

    // Write CSV header and data
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(csv_file, "{}", head_line)?;
    }
    writeln!(csv_file, "{}", data_line)?;

    // Build statistics
    run_python_statistics_builder(csv_path);

    Ok(())
}

fn eval<T: LookUpTable>(config: &mut EvalConfig, name: &str) {
    println!("  - {}", name);
    let reg = Region::new(GLOBAL);
    let lut = T::build(config.json_path).expect("Fail @ build lut");
    let stats = heap_value(reg.change());
    config.head_line.push_str(&format!("{}_heap,{}_capacity,", name, name));
    config
        .data_line
        .push_str(&format!("{},{},", stats, lut.allocated_bytes()));
}

fn eval_lambda<T: LookUpTableLambda>(config: &mut EvalConfig, name: &str, lambda: usize, threaded: bool) {
    println!("  - {}:λ={},T={}", name, lambda, threaded);
    let reg = Region::new(GLOBAL);
    let lut = T::build_lambda(lambda, config.json_path, threaded).expect("Fail @ build with lambda");
    let stats = heap_value(reg.change());
    config
        .head_line
        .push_str(&format!("λ={lambda}:{name}_heap,λ={lambda}:{name}_capacity,"));
    config
        .data_line
        .push_str(&format!("{},{},", stats, lut.allocated_bytes()));
}

fn eval_bucket(config: &mut EvalConfig, name: &str, bit_mask: usize, lambda: usize, threaded: bool) {
    println!("  - {}:#{}_λ={},T={}", name, bit_mask + 1, lambda, threaded);
    let reg = Region::new(GLOBAL);
    let lut =
        LutPHFGroup::build_buckets(lambda, config.json_path, bit_mask, threaded).expect("Fail @ build with lambda");
    let stats = heap_value(reg.change());
    let bit_mask_plus_one = bit_mask + 1;
    config.head_line.push_str(&format!(
        "#{bit_mask_plus_one}_λ={lambda}:{name}_heap,#{bit_mask_plus_one}_λ={lambda}:{name}_capacity,"
    ));
    config
        .data_line
        .push_str(&format!("{},{},", stats, lut.allocated_bytes()));
}

fn heap_value(stats: stats_alloc::Stats) -> isize {
    // We take the allocated bytes minus the deallocated and ignore the reallocated bytes because we are interested
    // in the total heap space taken
    stats.bytes_allocated as isize - stats.bytes_deallocated as isize

    // Alternative line that should not be used:
    // stats.bytes_allocated as isize - stats.bytes_deallocated as isize + stats.bytes_reallocated
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
