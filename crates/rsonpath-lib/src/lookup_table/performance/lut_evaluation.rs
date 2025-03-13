use std::{
    alloc::System,
    io::{self, Write},
    process::Command,
};

use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};

use crate::lookup_table::{
    distance_counter, lut_hash_map::LutHashMap, lut_hash_map_double::LutHashMapDouble,
    lut_hash_map_group::LutHashMapGroup, lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF,
    lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup, pair_finder, util_path, LookUpTable, LookUpTableLambda,
};

/// Allocator to track how much allocations are happening during a specific time frame
#[global_allocator]
pub static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;
/// Helper struct to reduce the number of parameters when calling functions
pub struct EvalConfig<'a> {
    json_path: &'a str,
    keys: Vec<usize>,
    head_line: &'a mut String,
    data_line: &'a mut String,
}

#[inline]
pub fn evaluate(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("JSONPATH: {}", json_path);
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);
    let num_keys = distance_counter::count_num_pairs(json_path);

    let mut head_line = String::from("name,input_size_bytes,num_keys,");
    let mut data_line = format!("{},{},{},", filename, file.metadata()?.len(), num_keys);

    let (keys, _) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");

    let mut config = EvalConfig {
        json_path,
        keys,
        head_line: &mut head_line,
        data_line: &mut data_line,
    };

    // #####################################
    // Measure LUTs without lambda parameter
    // #####################################
    // eval::<LutHashMap>(&mut config, "hash_map");
    eval::<LutHashMapDouble>(&mut config, "hash_map_double");
    // eval::<LutPerfectNaive>(&mut config, "perfect_naive");

    // for bit_mask in [3, 7, 15, 31, 63, 127] {
    // for bit_mask in [2047, 4095, 8191] {
    // for bit_mask in [2047] {
    //     eval_hash_map_group(&mut config, "hash_map_group", bit_mask);
    // }

    // #####################################
    // Measure LUTs with lambda parameter
    // #####################################
    // for lambda in [1, 5] {
    //     for threaded in [false] {
    //         eval_phf::<LutPHF>(&mut config, "phf", lambda, threaded);
    //         eval_phf::<LutPHFDouble>(&mut config, "phf_double", lambda, threaded);
    //     }
    // }

    // #####################################
    // Measure LUTs with bucket parameter
    // #####################################
    for lambda in [1, 5] {
        // for bit_mask in [3, 7, 15, 31, 63, 127] {
        for bit_mask in [2047, 4095, 8191] {
            eval_phf_group(&mut config, "phf_group", bit_mask, lambda, false);
        }
    }

    // Write CSV header and data
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(csv_file, "{}", head_line)?;
    }
    writeln!(csv_file, "{}", data_line)?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn eval<T: LookUpTable>(cfg: &mut EvalConfig, name: &str) {
    println!("  - {name}");

    // Build time & heap size
    let start_heap = Region::new(GLOBAL);

    let start_build = std::time::Instant::now();
    let lut = T::build(cfg.json_path, 0).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    let heap_bytes = heap_value(start_heap.change());
    let allocated_bytes = lut.allocated_bytes();

    // Query time
    let start_query = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &cfg.keys));
    let query_time = start_query.elapsed().as_secs_f64();

    // Save measurements
    let name = name;
    save_measurements(cfg, &name, build_time, query_time, heap_bytes, allocated_bytes);
}

fn eval_phf<T: LookUpTableLambda>(cfg: &mut EvalConfig, name: &str, lambda: usize, threaded: bool) {
    println!("  - {name}:λ={lambda},threaded={threaded}");

    // Build time & heap size
    let start_heap = Region::new(GLOBAL);

    let start_build = std::time::Instant::now();
    let lut = T::build_lambda(lambda, cfg.json_path, 0, threaded).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    let heap_bytes = heap_value(start_heap.change());
    let allocated_bytes = lut.allocated_bytes();

    // Query time
    let start_query = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &cfg.keys));
    let query_time = start_query.elapsed().as_secs_f64();

    // Save measurements
    let name = format!("λ={lambda}:{name}");
    save_measurements(cfg, &name, build_time, query_time, heap_bytes, allocated_bytes);
}

fn eval_phf_group(cfg: &mut EvalConfig, name: &str, bit_mask: usize, lambda: usize, threaded: bool) {
    let bits = bit_mask + 1;
    println!("  - {name}:#{bits}_λ={lambda}");

    // Build time & heap size
    let start_heap = Region::new(GLOBAL);

    let start_build = std::time::Instant::now();
    let lut = LutPHFGroup::build_buckets(lambda, cfg.json_path, 0, bit_mask, threaded).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    let heap_bytes = heap_value(start_heap.change());
    let allocated_bytes = lut.allocated_bytes();

    // Query time
    let start_query = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &cfg.keys));
    let query_time = start_query.elapsed().as_secs_f64();

    // Save measurements
    let name = format!("#{bits}_λ={lambda}:{name}");
    save_measurements(cfg, &name, build_time, query_time, heap_bytes, allocated_bytes);
}

fn eval_hash_map_group(cfg: &mut EvalConfig, name: &str, bit_mask: usize) {
    let bits = bit_mask + 1;
    println!("  - {name}:#{bits}");

    // Build time & heap size
    let start_heap = Region::new(GLOBAL);

    let start_build = std::time::Instant::now();
    let lut = LutHashMapGroup::build_buckets(cfg.json_path, bit_mask).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    let heap_bytes = heap_value(start_heap.change());
    let allocated_bytes = lut.allocated_bytes();

    // Query time
    let start_query = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &cfg.keys));
    let query_time = start_query.elapsed().as_secs_f64();

    // Save measurements
    let name = format!("#{bits}:{name}");
    save_measurements(cfg, &name, build_time, query_time, heap_bytes, allocated_bytes);
}

fn save_measurements(
    cfg: &mut EvalConfig,
    f: &str,
    build_time: f64,
    query_time: f64,
    heap_bytes: isize,
    allocated_bytes: usize,
) {
    cfg.head_line.push_str(&format!("{f}_build_time,{f}_query_time,",));
    cfg.data_line.push_str(&format!("{build_time},{query_time},"));
    cfg.head_line.push_str(&format!("{f}_heap,{f}_capacity,"));
    cfg.data_line.push_str(&format!("{heap_bytes},{allocated_bytes},"));

    println!("    - Build time:      {build_time}");
    println!("    - Query time:      {query_time}");
    println!("    - Heap bytes:      {heap_bytes}");
    println!("    - Allocated bytes: {allocated_bytes}");
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

fn run_python_statistics_builder(csv_path: &str) {
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

// A black box function so that the compiler will not optimize away the values passed into here. Mainly used when
// running tests.
#[inline(never)]
fn my_black_box<T>(_whatever: T) {}
