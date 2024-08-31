use std::{error::Error, fs, path::Path};

use rsonpath::lookup_table::{lut_performance, util};

// For example run with:
// cargo run --bin lut --release -- .a_ricardo/test_data/memory_test/small .a_ricardo/performance
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

    lut_performance::performance_test(json_path, &csv_path)?;

    Ok(())
}
