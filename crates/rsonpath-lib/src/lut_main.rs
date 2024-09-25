use std::{error::Error, fs, path::Path};

use rsonpath::lookup_table::{count_distances, lut_phf, performance};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if args[1] == "setup" {
            create_folder_setup()?;
            return Ok(());
        } else if args[1] == "phf" {
            lut_phf::build_and_test();
            lut_phf::build_and_test_large();
            return Ok(());
        } else {
            print_bad_input_error_msg();
        }
    }

    if args.len() == 3 {
        if args[1] == "distances" {
            count_distances::count_distance_for_each_json_in_folder(&args[2]);
            return Ok(());
        } else {
            print_bad_input_error_msg();
        }
    }

    if args.len() != 4 {
        print_bad_input_error_msg();
    }

    let json_folder = &args[1];
    let output_folder = &args[2];
    let csv_folder = &args[3];

    check_if_folder_exists(json_folder);
    check_if_folder_exists(output_folder);
    check_if_folder_exists(csv_folder);

    performance::performance_test(json_folder, output_folder, csv_folder)?;

    Ok(())
}

/// Creates the required folder structure if it does not exist.
fn create_folder_setup() -> std::io::Result<()> {
    let dirs = [
        ".a_lut_tests",
        ".a_lut_tests/performance",
        ".a_lut_tests/distances",
        ".a_lut_tests/output",
        ".a_lut_tests/test_data",
    ];

    // Iterate over each path and create the directory if it doesn't exist
    for dir in &dirs {
        let path = Path::new(dir);
        if !path.exists() {
            fs::create_dir_all(path)?;
            println!("Created directory: {}", dir);
        } else {
            println!("Directory already exists: {}", dir);
        }
    }

    Ok(())
}

fn print_bad_input_error_msg() {
    eprintln!(
        "Usage:\n
    cargo run --bin lut --release -- <json_path> <output_folder> <csv_folder>\n
    cargo run --bin lut --release -- setup\n
    "
    );
    std::process::exit(1);
}

fn check_if_folder_exists(path: &str) {
    if fs::metadata(path).is_err() {
        eprintln!("Error: The provided folder '{}' does not exist.", path);
        std::process::exit(1);
    } else if !Path::new(path).is_dir() {
        eprintln!("Error: The provided folder '{}' is not a directory.", path);
        std::process::exit(1);
    }
}

// Assume every single character is a '{' character. This is of course an upper bound calculation then.
#[allow(dead_code)]
fn calc_max_digits() {
    // We iterate from 1 to 64 bits
    for bit in 1..=64 {
        // Calculate the maximum JSON size based on the number of bits
        let max_json_size = 1_u64 << bit;

        // Determine the size in appropriate units
        let size_in_kb = max_json_size as f64 / 1024.0;
        let size_in_mb = size_in_kb / 1024.0;
        let size_in_gb = size_in_mb / 1024.0;
        let size_in_tb = size_in_gb / 1024.0;

        // Print the size in different units based on the magnitude
        println!(
            "Bit: {}, Max JSON size: {} B, {:.2} kB, {:.2} MB, {:.2} GB, {:.2} TB",
            bit, max_json_size, size_in_kb, size_in_mb, size_in_gb, size_in_tb
        );
    }

    // Idea: At 43 bits the input json is 64TB
    // 1) what if instead of usize we use u_43_bits since the other bits will always be 0
    // 2) 64 - 43 = 21 . We have 21 bits where we could save more structural data per key.
    //      - maybe the usage counter? This way we know which values to cache, because high counter means often skipped?
    //      - the deepness, but I am not sure what value this brings
    // 3) When the input JSON is < 32GB we can even use 32bit per key
}
