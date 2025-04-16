use std::{
    alloc::System,
    io::{self, Write},
    process::Command,
};

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use crate::lookup_table::implementations::lut_hash_map::LutHashMap;
use crate::lookup_table::implementations::lut_hash_map_group::LutHashMapGroup;
use crate::lookup_table::implementations::lut_phf_group::LutPHFGroup;
use crate::lookup_table::{
    analysis::distance_distribution, pair_data, util_path, LookUpTable, LookUpTableLambda, DISTANCE_CUT_OFF,
};

/// Allocator to track how much allocations are happening during a specific time frame
#[global_allocator]
pub static HEAP_TRACKER: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

pub const REPETITIONS: usize = 10;

/// Helper struct to reduce the number of parameters when calling functions
pub struct EvalConfig<'a> {
    json_path: &'a str,
    keys: Vec<usize>,
    head_line: &'a mut String,
    data_line: &'a mut String,
}

pub fn evaluate(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let cutoff: usize = 0;
    println!("JSONPATH: {}, cutoff = {}", json_path, cutoff);

    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);
    let num_keys = distance_distribution::count_num_pairs(json_path);

    let mut head_line = String::from("name,input_size_bytes,num_keys,");
    let mut data_line = format!("{},{},{},", filename, file.metadata()?.len(), num_keys);

    let (keys, _) = pair_data::get_keys_and_values_absolute(json_path, cutoff).expect("Fail @ finding pairs.");

    let mut config = EvalConfig {
        json_path,
        keys,
        head_line: &mut head_line,
        data_line: &mut data_line,
    };

    print!("Measuring LUT size with REPETITIONS = {}", REPETITIONS);
    measure_performance(&mut config, cutoff)?;

    // Write CSV header and data
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(csv_file, "{}", head_line)?;
    }
    writeln!(csv_file, "{}", data_line)?;

    plot_with_python(csv_path);

    Ok(())
}

#[inline]
fn measure_performance(config: &mut EvalConfig, cutoff: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Measure normal LUTs without any special parameter
    // eval::<LutPerfectNaive>(config, "perfect_naive", cutoff);
    eval::<LutHashMap>(config, "hash_map", cutoff);
    // eval::<LutHashMapDouble>(config, "hash_map_double", cutoff);
    // eval::<LutSicHashDouble>(&mut config, "sic_hash_double", cutoff);
    // eval::<LutPtrHashDouble>(config, "ptr_hash_double", cutoff);
    // eval::<LutVFuncDouble>(config, "vfunc_double", cutoff);

    for bit_mask in [15] {
        // eval_hash_map_group(config, "hash_map_group", bit_mask, cutoff);
    }

    // Measure LUTs with lambda parameter
    for lambda in [1, 5] {
        for threaded in [false] {
            // eval_phf::<LutPHF>(config, "phf", lambda, threaded, cutoff);
            // eval_phf::<LutPHFDouble>(config, "phf_double", lambda, threaded, cutoff);
        }
    }

    // Measure LUTs with bucket parameter
    for lambda in [1, 5] {
        // for bit_mask in [3, 7, 15, 31, 63, 127] {
        // for bit_mask in [63, 127, 255, 511] {
        for bit_mask in [2047] {
            eval_phf_group(config, "phf_group", bit_mask, lambda, false, cutoff);
        }
    }

    Ok(())
}

fn eval<T: LookUpTable>(config: &mut EvalConfig, name: &str, cutoff: usize) {
    println!("  - {name}");

    // Build time
    // We do it like because the drop() of a big LUT could cost time that we do not want to include in the measurement
    let mut build_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_build = std::time::Instant::now();
        let _ = T::build(config.json_path, cutoff).expect("Fail @ build lut");
        build_time += start_build.elapsed().as_secs_f64();
    }
    build_time = build_time / (REPETITIONS as f64);

    // Size
    let start_heap = Region::new(HEAP_TRACKER);
    let lut = T::build(config.json_path, cutoff).expect("Fail @ build lut");
    let heap_bytes = heap_value(start_heap.change());

    // Query time
    let mut query_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_query = std::time::Instant::now();
        my_black_box(get_every_key_once(&lut, &config.keys));
        query_time += start_query.elapsed().as_secs_f64();
    }
    query_time = query_time / (REPETITIONS as f64);

    // Save measurements
    let name = name;
    save_measurements(config, &name, build_time, query_time, heap_bytes);
}

fn eval_phf<T: LookUpTableLambda>(config: &mut EvalConfig, name: &str, lambda: usize, threaded: bool, cutoff: usize) {
    println!("  - {name}:λ={lambda},threaded={threaded}");

    // Build time
    let mut build_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_build = std::time::Instant::now();
        let _ = T::build_lambda(lambda, config.json_path, 0, threaded).expect("Fail @ build lut");
        build_time += start_build.elapsed().as_secs_f64();
    }
    build_time = build_time / (REPETITIONS as f64);

    // Size
    let start_heap = Region::new(HEAP_TRACKER);
    let lut = T::build_lambda(lambda, config.json_path, 0, threaded).expect("Fail @ build lut");
    let heap_bytes = heap_value(start_heap.change());

    // Query time
    let mut query_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_query = std::time::Instant::now();
        my_black_box(get_every_key_once(&lut, &config.keys));
        query_time += start_query.elapsed().as_secs_f64();
    }
    query_time = query_time / (REPETITIONS as f64);

    // Save measurements
    let name = format!("λ={lambda}:{name}");
    save_measurements(config, &name, build_time, query_time, heap_bytes);
}

fn eval_phf_group(config: &mut EvalConfig, name: &str, bit_mask: usize, lambda: usize, threaded: bool, cutoff: usize) {
    let buckets = bit_mask + 1;
    println!("  - {name}:#{buckets}_λ={lambda}");

    // Build time
    let mut build_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_build = std::time::Instant::now();
        let _ =
            LutPHFGroup::build_buckets(lambda, config.json_path, cutoff, bit_mask, threaded).expect("Fail @ build lut");
        build_time += start_build.elapsed().as_secs_f64();
    }
    build_time = build_time / (REPETITIONS as f64);

    // Size
    let start_heap = Region::new(HEAP_TRACKER);
    let lut =
        LutPHFGroup::build_buckets(lambda, config.json_path, cutoff, bit_mask, threaded).expect("Fail @ build lut");
    let heap_bytes = heap_value(start_heap.change());

    // Query time
    let mut query_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_query = std::time::Instant::now();
        my_black_box(get_every_key_once(&lut, &config.keys));
        query_time += start_query.elapsed().as_secs_f64();
    }
    query_time = query_time / (REPETITIONS as f64);

    // Save measurements
    let name = format!("#{buckets}_λ={lambda}:{name}");
    save_measurements(config, &name, build_time, query_time, heap_bytes);
}

fn eval_hash_map_group(config: &mut EvalConfig, name: &str, bit_mask: usize, cutoff: usize) {
    let buckets = bit_mask + 1;
    println!("  - {name}:#{buckets}");

    // Build time
    let mut build_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_build = std::time::Instant::now();
        let _ = LutHashMapGroup::build_buckets(config.json_path, bit_mask, cutoff).expect("Fail @ build lut");
        build_time += start_build.elapsed().as_secs_f64();
    }
    build_time = build_time / (REPETITIONS as f64);

    // Size
    let start_heap = Region::new(HEAP_TRACKER);
    let lut = LutHashMapGroup::build_buckets(config.json_path, bit_mask, cutoff).expect("Fail @ build lut");
    let heap_bytes = heap_value(start_heap.change());

    // Query time
    let mut query_time: f64 = 0.0;
    for _i in 0..REPETITIONS {
        let start_query = std::time::Instant::now();
        my_black_box(get_every_key_once(&lut, &config.keys));
        query_time += start_query.elapsed().as_secs_f64();
    }
    query_time = query_time / (REPETITIONS as f64);

    // Save measurements
    let name = format!("#{buckets}:{name}");
    save_measurements(config, &name, build_time, query_time, heap_bytes);
}

fn save_measurements(config: &mut EvalConfig, f: &str, build: f64, query: f64, heap: isize) {
    config.head_line.push_str(&format!("{f}_BUILD,{f}_QUERY,{f}_HEAP,",));
    config.data_line.push_str(&format!("{build},{query},{heap}"));

    println!("    - Build time:      {build}");
    println!("    - Query time:      {query}");
    println!("    - Heap bytes:      {heap}");
}

fn get_every_key_once(lut: &dyn LookUpTable, keys: &[usize]) -> usize {
    let mut count = 0;
    for key in keys {
        count += lut.get(key).expect("Fail at getting value!");
    }
    count
}

fn heap_value(stats: stats_alloc::Stats) -> isize {
    // We take the allocated bytes minus the deallocated and ignore the reallocated bytes because we are interested
    // in the total heap space taken
    stats.bytes_allocated as isize - stats.bytes_deallocated as isize

    // Alternative line that should not be used:
    // stats.bytes_allocated as isize - stats.bytes_deallocated as isize + stats.bytes_reallocated
}

// A black box function so that the compiler will not optimize away the values passed into here. Mainly used when
// running tests.
#[inline(never)]
fn my_black_box<T>(_whatever: T) {}

fn plot_with_python(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/lut_evaluation.py")
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
