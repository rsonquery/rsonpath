use crate::lookup_table::lut_hash_map_double::LutHashMapDouble;
use crate::lookup_table::{
    count_distances, lut_hash_map::LutHashMap, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, util_path, LookUpTable, LookUpTableLambda,
};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::io::Write;
use std::os::unix::thread;
use std::{
    alloc::System,
    io::{self},
    process::Command,
};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

#[inline]
pub fn run(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);
    let num_keys = count_distances::count_num_pairs(json_path);

    let mut head_line = String::from("name,input_size_bytes,num_keys,");
    let mut data_line = format!("{},{},{},", filename, file.metadata()?.len(), num_keys);

    // Measure LUTs without lambda parameter
    // eval_ram::<LutHashMap>(json_path, "hash_map", &mut head_line, &mut data_line);
    // eval_ram::<LutHashMapDouble>(json_path, "hash_map_double", &mut head_line, &mut data_line);
    // eval_ram::<LutPerfectNaive>(json_path, "perfect_naive", &mut head_line, &mut data_line);

    // Process each LUT that has a lambda parameter with lambda [1, ..., 5]
    for lambda in vec![1, 5] {
        let threaded = false;
        eval_ram_lambda::<LutPHF>(lambda, json_path, "phf", &mut head_line, &mut data_line, threaded);
        eval_ram_lambda::<LutPHFDouble>(
            lambda,
            json_path,
            "phf_double",
            &mut head_line,
            &mut data_line,
            threaded,
        );
        eval_ram_lambda::<LutPHFGroup>(lambda, json_path, "phf_group", &mut head_line, &mut data_line, threaded);

        // let threaded = true;
        // eval_ram_lambda::<LutPHF>(lambda, json_path, "phf(T)", &mut head_line, &mut data_line, threaded);
        // eval_ram_lambda::<LutPHFDouble>(lambda, json_path, "phf_double(T)", &mut head_line, &mut data_line, threaded);
        // eval_ram_lambda::<LutPHFGroup>(lambda, json_path, "phf_group(T)", &mut head_line, &mut data_line, threaded);
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

fn eval_ram<T: LookUpTable>(json_path: &str, name: &str, head_line: &mut String, data_line: &mut String) {
    println!("  - {}", name);
    let reg = Region::new(GLOBAL);
    let lut = T::build(json_path).expect("Fail @ build lut");
    let stats = heap_value(reg.change());
    head_line.push_str(&format!("{}_heap,{}_capacity,", name, name));
    data_line.push_str(&format!("{},{},", stats, lut.allocated_bytes()));
}

fn eval_ram_lambda<T: LookUpTableLambda>(
    lambda: usize,
    json_path: &str,
    name: &str,
    head_line: &mut String,
    data_line: &mut String,
    threaded: bool,
) {
    println!("  - {}:λ={},T={}", name, lambda, threaded);
    let reg = Region::new(GLOBAL);
    let lut = T::build_with_lambda(lambda, json_path, threaded).expect("Fail @ build with lambda");
    let stats = heap_value(reg.change());
    head_line.push_str(&format!("λ={}:{}_heap,λ={}:{}_capacity,", lambda, name, lambda, name));
    data_line.push_str(&format!("{},{},", stats, lut.allocated_bytes()));
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
