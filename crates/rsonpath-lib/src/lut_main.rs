use std::{error::Error, fs, path::Path};

use rsonpath::lookup_table::{lut_perfect_hashing, lut_performance, util};

/// Main function that processes command-line arguments, validates paths,
/// and runs performance tests.
fn main() -> Result<(), Box<dyn Error>> {
    calc_max_digits();

    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if args[1] == "setup" {
            create_folder_setup()?;
            return Ok(());
        } else {
            print_bad_input_error_msg();
        }
    }

    if args.len() != 4 {
        print_bad_input_error_msg();
    }

    let json_path = &args[1];
    let output_folder = &args[2];
    let csv_folder = &args[3];

    // Check if json_path is an existing folder or an existing .json file
    if !fs::metadata(json_path)?.is_dir() && (!json_path.ends_with(".json") || !fs::metadata(json_path)?.is_file()) {
        eprintln!("Error: The provided json_path is not a valid .json file or folder.");
        std::process::exit(1);
    }

    // Check if output_folder is an existing folder path
    if !fs::metadata(output_folder)?.is_dir() {
        eprintln!("Error: The provided output_folder is not a valid folder path.");
        std::process::exit(1);
    }

    // Check if csv_folder is an existing folder path
    if !fs::metadata(csv_folder)?.is_dir() {
        eprintln!("Error: The provided csv_folder is not a valid folder path.");
        std::process::exit(1);
    }

    // Check if csv_path already exists and if it does rename it with a unique number
    let mut csv_path = format!("{}/{}_stats.csv", csv_folder, util::get_filename_from_path(json_path));
    let mut counter = 1;
    while Path::new(&csv_path).exists() {
        csv_path = format!(
            "{}/{}_stats({}).csv",
            csv_folder,
            util::get_filename_from_path(json_path),
            counter
        );
        counter += 1;
    }

    lut_performance::performance_test(json_path, output_folder, &csv_path)?;

    // lut_perfect_hashing::demo_perfect_hashing();
    // lut_perfect_hashing::test_perfect_hashing();

    Ok(())
}

/// Creates the required folder structure if it does not exist.
fn create_folder_setup() -> std::io::Result<()> {
    let dirs = [
        ".a_lut_tests",
        ".a_lut_tests/performance",
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

// Assume every single character is a '{' character. This is of course an upper bound calculation then.
fn calc_max_digits() {
    // We iterate from 1 to 64 bits
    for bit in 1..=64 {
        // Calculate the maximum JSON size based on the number of bits
        let max_json_size = (1_u64 << bit) * 8;

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
