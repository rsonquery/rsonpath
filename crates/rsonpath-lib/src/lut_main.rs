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
    cargo run --bin lut --release -- setup\n
    cargo run --bin lut --release -- <json_folder> <output_folder> <csv_folder>\n
    cargo run --bin lut --release -- distances .<json_folder>
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
