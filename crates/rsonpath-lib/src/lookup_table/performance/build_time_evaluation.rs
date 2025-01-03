use std::{
    io::{self, Write},
    process::Command,
};

use crate::lookup_table::{
    count_distances, lut_hash_map::LutHashMap, lut_hash_map_double::LutHashMapDouble,
    lut_perfect_naive::LutPerfectNaive, lut_phf::LutPHF, lut_phf_double::LutPHFDouble, lut_phf_group::LutPHFGroup,
    pair_finder, util_path, LookUpTable, LookUpTableLambda,
};
/// Helper struct to reduce the number of parameters when calling functions
pub struct EvalConfig<'a> {
    json_path: &'a str,
    keys: Vec<usize>,
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

    let (keys, _) = pair_finder::get_keys_and_values(json_path).expect("Fail @ finding pairs.");

    let mut config = EvalConfig {
        json_path,
        keys,
        head_line: &mut head_line,
        data_line: &mut data_line,
    };

    // Measure LUTs without lambda parameter
    eval::<LutHashMap>(&mut config, "hash_map");
    eval::<LutHashMapDouble>(&mut config, "hash_map_double");
    eval::<LutPerfectNaive>(&mut config, "perfect_naive");

    // Measure LUTs with lambda parameter
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

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn eval<T: LookUpTable>(config: &mut EvalConfig, name: &str) {
    println!("  - {}", name);

    // Build time
    let start_build = std::time::Instant::now();
    let lut = T::build(config.json_path).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    // Query time
    let start_query = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &config.keys));
    let query_time = start_query.elapsed().as_secs_f64();

    // Use the fields directly without destructuring
    config
        .head_line
        .push_str(&format!("{}_build_time,{}_query_time,", name, name));
    config.data_line.push_str(&format!("{},{},", build_time, query_time));
}

fn eval_lambda<T: LookUpTableLambda>(config: &mut EvalConfig, name: &str, lambda: usize, threaded: bool) {
    println!("  - {}:λ={}", name, lambda);

    // Build time
    let start_build = std::time::Instant::now();
    let lut = T::build_lambda(lambda, config.json_path, threaded).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    // Query time
    let start_build = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &config.keys));
    let query_time = start_build.elapsed().as_secs_f64();

    config.head_line.push_str(&format!(
        "λ={}:{}_build_time,λ={}:{}_query_time,",
        lambda, name, lambda, name
    ));
    config.data_line.push_str(&format!("{},{},", build_time, query_time));
}

fn eval_bucket(config: &mut EvalConfig, name: &str, bit_mask: usize, lambda: usize, threaded: bool) {
    println!("  - {}:#{}_λ={}", name, bit_mask + 1, lambda);

    // Build time
    let start_build = std::time::Instant::now();
    let lut = LutPHFGroup::build_buckets(lambda, config.json_path, bit_mask, threaded).expect("Fail @ build lut");
    let build_time = start_build.elapsed().as_secs_f64();

    // Query time
    let start_build = std::time::Instant::now();
    // Call a black box function that does nothing so that the compiler does not optimize away get_every_key_once
    my_black_box(get_every_key_once(&lut, &config.keys));
    let query_time = start_build.elapsed().as_secs_f64();

    let power_of_two = bit_mask + 1;
    config.head_line.push_str(&format!(
        "#{power_of_two}_λ={lambda}:{name}_build_time,#{power_of_two}_λ={lambda}:{name}_query_time,",
    ));
    config.data_line.push_str(&format!("{},{},", build_time, query_time));
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
fn my_black_box<T>(_whatever: T) {}
