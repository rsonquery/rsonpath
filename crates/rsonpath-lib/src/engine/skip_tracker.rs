use lazy_static::lazy_static;
use log::debug;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Result, Write};
use std::path::Path;
use std::sync::atomic::AtomicU64;
use std::sync::Mutex;

// Define here what you want to track, if you want to run without tracking then set it to OFF
pub const MODE: SkipMode = SkipMode::TRACK;

#[derive(Debug, PartialEq)]
pub enum SkipMode {
    COUNT, // Track how many jumps are happening
    TRACK, // Track each jump value individually in a data structure (slow)
    OFF,   // Turned off, tracking nothing
}

lazy_static! {
    static ref SKIP_TRACKER_LUT: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
    static ref SKIP_TRACKER_ITE: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

static LUT_COUNTER: AtomicU64 = AtomicU64::new(0);
static ITE_COUNTER: AtomicU64 = AtomicU64::new(0);

// Increment the frequency of given distance by 1, or initialize to 1 if not present
pub fn increment_lut(distance: usize) {
    if is_counting() {
        LUT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    } else if is_tracking() {
        let mut map = SKIP_TRACKER_LUT.lock().unwrap();
        *map.entry(distance).or_insert(0) += 1;
    }
}

// Increment the value by 1, or initialize to 1 if not present
pub fn increment_ite(distance: usize) {
    if is_counting() {
        ITE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    } else if is_tracking() {
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
    let lut_counter = LUT_COUNTER.load(std::sync::atomic::Ordering::Relaxed);
    let ite_counter = ITE_COUNTER.load(std::sync::atomic::Ordering::Relaxed);
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

pub fn is_counting() -> bool {
    MODE == SkipMode::COUNT
}

pub fn is_tracking() -> bool {
    MODE == SkipMode::TRACK
}

pub fn is_off() -> bool {
    MODE == SkipMode::OFF
}

fn reset_counts() {
    LUT_COUNTER.store(0, std::sync::atomic::Ordering::Relaxed);
    ITE_COUNTER.store(0, std::sync::atomic::Ordering::Relaxed);
}
