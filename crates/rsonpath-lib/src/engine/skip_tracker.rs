use lazy_static::lazy_static;
use log::debug;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Result, Write};
use std::path::Path;
use std::sync::Mutex;

#[derive(PartialEq)]
pub enum SkipMode {
    COUNT,
    TRACK,
}

// Define a global mode variable here!
lazy_static! {
    static ref MODE: Mutex<SkipMode> = Mutex::new(SkipMode::COUNT); // Define mode here
    static ref LUT_COUNTER: Mutex<u64> = Mutex::new(0);
    static ref ITE_COUNTER: Mutex<u64> = Mutex::new(0);
    static ref SKIP_TRACKER_LUT: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
    static ref SKIP_TRACKER_ITE: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

pub fn set_mode(new_mode: SkipMode) {
    let mut mode = MODE.lock().unwrap();
    *mode = new_mode;
}

// Increment the frequency of given distance by 1, or initialize to 1 if not present
pub fn increment_lut(distance: usize) {
    let mode = MODE.lock().unwrap();
    if *mode == SkipMode::COUNT {
        let mut counter = LUT_COUNTER.lock().unwrap();
        *counter += 1;
    } else if *mode == SkipMode::TRACK {
        let mut map = SKIP_TRACKER_LUT.lock().unwrap();
        *map.entry(distance).or_insert(0) += 1;
    }
}

// Increment the value by 1, or initialize to 1 if not present
pub fn increment_ite(distance: usize) {
    let mode = MODE.lock().unwrap();
    if *mode == SkipMode::COUNT {
        let mut counter = ITE_COUNTER.lock().unwrap();
        *counter += 1;
    } else if *mode == SkipMode::TRACK {
        let mut map = SKIP_TRACKER_ITE.lock().unwrap();
        *map.entry(distance).or_insert(0) += 1;
    }
}

// Save HashMap to a CSV file
pub fn save_track_results_to_csv(file_path: &str) -> std::io::Result<()> {
    debug!("Saving to {}", file_path);
    let path = Path::new(file_path);
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // Headline
    writeln!(writer, "distance,frequency,skip_type")?;

    // LUT
    let lut_map = SKIP_TRACKER_LUT.lock().unwrap();
    for (key, value) in lut_map.iter() {
        writeln!(writer, "{},{},lut", key, value)?;
    }

    // ITE
    let ite_map = SKIP_TRACKER_ITE.lock().unwrap();
    for (key, value) in ite_map.iter() {
        writeln!(writer, "{},{},ite", key, value)?;
    }

    Ok(())
}

pub fn print_count_results_and_save_in_csv(file_path: &str, filename: &str, query_text: &str) -> Result<()> {
    let lut_counter = *LUT_COUNTER.lock().unwrap();
    let ite_counter = *ITE_COUNTER.lock().unwrap();
    let total = lut_counter + ite_counter;

    println!("\tCounts: LUT = {lut_counter}, ITE = {ite_counter}, TOTAL = {total}");

    // Add the headline if the file does not exist yet
    let path = Path::new(file_path);
    if !path.exists() {
        let mut writer = BufWriter::new(File::create(path)?);
        writeln!(writer, "Filename,Query,LUT,ITE,TOTAL")?;
    }

    let file = OpenOptions::new()
        .create(true)  // Create file if it does not exist
        .append(true)  // Append to the file if it exists
        .open(path)?;

    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "{},{},{},{},{}",
        filename, query_text, lut_counter, ite_counter, total
    )?;
    writer.flush()?;

    reset_counts();

    Ok(())
}

fn reset_counts() {
    let mut lut_counter = LUT_COUNTER.lock().unwrap();
    *lut_counter = 0;
    let mut ite_counter = ITE_COUNTER.lock().unwrap();
    *ite_counter = 0;
}
