use lazy_static::lazy_static;
use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;

// Track how many times every distance appears in the skipping process, only track the ones with frequency >= 1
lazy_static! {
    static ref SKIP_TRACKER_LUT: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
    static ref SKIP_TRACKER_ITE: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

// Increment the frequency of given distance by 1, or initialize to 1 if not present
pub fn increment_lut(distance: usize) {
    let mut map = SKIP_TRACKER_LUT.lock().unwrap();
    *map.entry(distance).or_insert(0) += 1;
}

// Increment the value by 1, or initialize to 1 if not present
pub fn increment_ite(distance: usize) {
    let mut map = SKIP_TRACKER_ITE.lock().unwrap();
    *map.entry(distance).or_insert(0) += 1;
}

// Save HashMap to a CSV file
pub fn save_to_csv(file_path: &str) -> std::io::Result<()> {
    debug!("Saving to {}", file_path);
    let path = Path::new(file_path);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Headline
    writeln!(writer, "distance,frequency,skip_type");

    // LUT
    let lut_map = SKIP_TRACKER_LUT.lock().unwrap();
    for (key, value) in lut_map.iter() {
        // debug!("Writing {}, {}", key, value);
        writeln!(writer, "{},{},lut", key, value)?;
    }

    // ITE
    let ite_map = SKIP_TRACKER_ITE.lock().unwrap();
    for (key, value) in ite_map.iter() {
        // debug!("Writing {}, {}", key, value);
        writeln!(writer, "{},{},ite", key, value)?;
    }

    Ok(())
}
