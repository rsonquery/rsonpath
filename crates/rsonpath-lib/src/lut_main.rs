use std::{error::Error, fs};

use rsonpath::lookup_table::{lut_performance, util};

fn main() -> Result<(), Box<dyn Error>> {
    let json_path = &std::env::args().collect::<Vec<_>>()[1];
    let csv_folder = &std::env::args().collect::<Vec<_>>()[2];

    // Check if json_path is an existing folder or an existing .json file
    if !fs::metadata(json_path)?.is_dir() && (!json_path.ends_with(".json") || !fs::metadata(json_path)?.is_file()) {
        eprintln!("Error: The provided json_path is not a valid .json file or folder.");
        std::process::exit(1);
    }

    // Check if csv_path is an existing folder path
    if !fs::metadata(csv_folder)?.is_dir() {
        eprintln!("Error: The provided csv_path is not a valid folder path.");
        std::process::exit(1);
    }

    let csv_path = format!("{}/{}_stats.csv", csv_folder, util::get_filename_from_path(json_path));
    lut_performance::performance_test(json_path, &csv_path)?;

    Ok(())
}
