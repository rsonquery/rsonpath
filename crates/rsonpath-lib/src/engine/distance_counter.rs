use lazy_static::lazy_static;
use log::debug;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;

// Track how many times every distance appears in the skipping process, only track the ones with frequency >= 1
lazy_static! {
    static ref DISTANCE_COUNTER_MAP: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

// Increment the value by 1, or initialize to 1 if not present
pub fn increment_value(distance: usize) {
    let mut map = DISTANCE_COUNTER_MAP.lock().unwrap();
    *map.entry(distance).or_insert(0) += 1;
}

// Save HashMap to a CSV file
pub fn save_to_csv(file_path: &str) -> std::io::Result<()> {
    debug!("Saving to {}", file_path);
    let map = DISTANCE_COUNTER_MAP.lock().unwrap();
    let path = Path::new(file_path);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    for (key, value) in map.iter() {
        debug!("Writing {}, {}", key, value);
        writeln!(writer, "{},{}", key, value)?;
    }

    Ok(())
}
