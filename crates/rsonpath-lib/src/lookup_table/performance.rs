use std::path::Path;
use std::{error::Error, fs};

use crate::lookup_table::util_path;

pub mod compare;
pub mod stats;

#[inline]
pub fn performance_test(json_folder: &str, output_path: &str, csv_folder: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(json_folder)?;

    if metadata.is_dir() {
        let folder_name = util_path::get_filename_from_path(json_folder);
        let suffix = get_next_valid_filename(json_folder, csv_folder);
        let csv_path_stats = format!("{}/{}_stats{}.csv", csv_folder, folder_name, suffix);
        let csv_path_compare = format!("{}/{}_compare{}.csv", csv_folder, folder_name, suffix);

        for entry in fs::read_dir(json_folder)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let sub_json_path = path.to_str().expect("Failed to convert path to string");

                println!("Processing: {}", util_path::get_filename_from_path(sub_json_path));
                stats::measure_stats(sub_json_path, output_path, &csv_path_stats)?;
                compare::compare_luts_in_speed_and_size(sub_json_path, &csv_path_compare)?;
            }
        }
    }

    Ok(())
}

fn get_next_valid_filename(json_folder: &str, csv_folder: &str) -> String {
    // Check if csv_path already exists and if it does rename it with a unique number
    let mut csv_path = format!(
        "{}/{}_stats.csv",
        csv_folder,
        util_path::get_filename_from_path(json_folder)
    );
    let mut counter = 0;
    while Path::new(&csv_path).exists() {
        counter += 1;
        csv_path = format!(
            "{}/{}_stats({}).csv",
            csv_folder,
            util_path::get_filename_from_path(json_folder),
            counter
        );
    }

    if counter > 0 {
        format!("({})", counter)
    } else {
        String::new()
    }
}
