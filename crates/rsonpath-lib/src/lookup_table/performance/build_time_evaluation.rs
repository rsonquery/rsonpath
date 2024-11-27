use std::{
    io::{self, Write},
    process::Command,
};

use crate::lookup_table::{
    count_distances, lut_hash_map::LutHashMap, lut_hash_map_double::LutHashMapDouble,
    lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF, lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup,
    pair_finder, util_path, LookUpTable, LookUpTableLambda,
};

#[inline]
pub fn run(json_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::extract_filename(json_path);
    let num_keys = count_distances::count_num_pairs(json_path);

    let mut head_line = String::from("name,input_size_bytes,num_keys,");
    let mut data_line = format!("{},{},{},", filename, file.metadata()?.len(), num_keys);

    let (keys, _) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");

    // Measure LUTs without lambda parameter
    // eval::<LutHashMap>(json_path, &keys, "hash_map", &mut head_line, &mut data_line);
    // eval::<LutHashMapDouble>(json_path, &keys, "hash_map_double", &mut head_line, &mut data_line);
    // eval::<LutPerfectNaive>(json_path, &keys, "perfect_naive", &mut head_line, &mut data_line);

    // Measure LUTs with lambda parameter
    for l in vec![1, 5] {
        let t = false;
        // eval_lambda::<LutPHF>(l, json_path, &keys, "phf", &mut head_line, &mut data_line, t);
        // eval_lambda::<LutPHFDouble>(l, json_path, &keys, "phf_double", &mut head_line, &mut data_line, t);
        // eval_lambda::<LutPHFGroup>(l, json_path, &keys, "phf_group", &mut head_line, &mut data_line, t);

        // let threaded = true;
        // eval_lambda::<LutPHF>(l, json_path, &keys, "phf(T)", &mut head_line, &mut data_line, t);
        // eval_lambda::<LutPHFDouble>(l, json_path, &keys, "phf_double(T)", &mut head_line, &mut data_line, t);
        // eval_lambda::<LutPHFGroup>(l, json_path, &keys, "phf_group(T)", &mut head_line, &mut data_line, t);
    }

    let l = 5;
    let t = false;
    eval_bucket(l, json_path, &keys, 3, "phf_group", &mut head_line, &mut data_line, t);
    eval_bucket(l, json_path, &keys, 7, "phf_group", &mut head_line, &mut data_line, t);
    eval_bucket(l, json_path, &keys, 15, "phf_group", &mut head_line, &mut data_line, t);
    eval_bucket(l, json_path, &keys, 31, "phf_group", &mut head_line, &mut data_line, t);
    eval_bucket(l, json_path, &keys, 63, "phf_group", &mut head_line, &mut data_line, t);
    eval_bucket(l, json_path, &keys, 127, "phf_group", &mut head_line, &mut data_line, t);

    // Write CSV header and data
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(csv_file, "{}", head_line)?;
    }
    writeln!(csv_file, "{}", data_line)?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn eval<T: LookUpTable>(
    json_path: &str,
    keys: &Vec<usize>,
    name: &str,
    head_line: &mut String,
    data_line: &mut String,
) {
    println!("  - {}", name);

    // Build time
    let start_build = std::time::Instant::now();
    let lut = T::build(json_path).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    // Query time
    let start_build = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &keys));
    let query_time = start_build.elapsed().as_secs_f64();

    head_line.push_str(&format!("{}_build_time,{}_query_time,", name, name));
    data_line.push_str(&format!("{},{},", build_time, query_time));
}

fn eval_lambda<T: LookUpTableLambda>(
    lambda: usize,
    json_path: &str,
    keys: &Vec<usize>,
    name: &str,
    head_line: &mut String,
    data_line: &mut String,
    threaded: bool,
) {
    println!("  - {}:λ={}", name, lambda);

    // Build time
    let start_build = std::time::Instant::now();
    let lut = T::build_lambda(lambda, json_path, threaded).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    // Query time
    let start_build = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &keys));
    let query_time = start_build.elapsed().as_secs_f64();

    head_line.push_str(&format!(
        "λ={}:{}_build_time,λ={}:{}_query_time,",
        lambda, name, lambda, name
    ));
    data_line.push_str(&format!("{},{},", build_time, query_time));
}

fn eval_bucket(
    lambda: usize,
    json_path: &str,
    keys: &Vec<usize>,
    bit_mask: usize,
    name: &str,
    head_line: &mut String,
    data_line: &mut String,
    threaded: bool,
) {
    println!("  - {}:#{}_λ={}", name, bit_mask + 1, lambda);

    // Build time
    let start_build = std::time::Instant::now();
    let lut = LutPHFGroup::build_buckets(lambda, json_path, bit_mask, threaded).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    // Query time
    let start_build = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &keys));
    let query_time = start_build.elapsed().as_secs_f64();

    head_line.push_str(&format!(
        "#{}_λ={}:{}_build_time,#{}_λ={}:{}_query_time,",
        bit_mask + 1,
        lambda,
        name,
        bit_mask + 1,
        lambda,
        name
    ));
    data_line.push_str(&format!("{},{},", build_time, query_time));
}

fn get_every_key_once(lut: &dyn LookUpTable, keys: &[usize]) -> usize {
    let mut count = 0;
    for key in keys {
        count += lut.get(key).expect("Fail at getting value!");
    }
    count
}

fn run_python_statistics_builder(csv_path: &str) {
    let msg = format!("Failed to open csv_path: {}", csv_path);
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/build_time_evaluation.py")
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
fn my_black_box<T>(whatever: T) {}
