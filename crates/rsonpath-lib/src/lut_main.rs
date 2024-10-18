use std::{error::Error, fs, path::Path};

use rsonpath::lookup_table::{count_distances, performance};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        if args[1] == "setup" {
            create_folder_setup()?;
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

    let input_json_dir = &args[1];
    let output_performance_dir = &args[2];
    let tasks = &args[3].parse::<u16>().unwrap();
    check_if_dir_exists(input_json_dir);
    check_if_dir_exists(output_performance_dir);

    performance::performance_test(input_json_dir, output_performance_dir, *tasks);

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
    cargo run --bin lut --release -- setup\n
    cargo run --bin lut --release -- <json_folder> <csv_folder> <tasks>\n
    cargo run --bin lut --release -- distances .<json_folder>
    "
    );
    std::process::exit(1);
}

fn check_if_dir_exists(path: &str) {
    if fs::metadata(path).is_err() {
        eprintln!("Error: The provided folder '{}' does not exist.", path);
        std::process::exit(1);
    } else if !Path::new(path).is_dir() {
        eprintln!("Error: The provided folder '{}' is not a directory.", path);
        std::process::exit(1);
    }
}
