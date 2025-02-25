use lazy_static::lazy_static;
use log::debug;
use std::collections::HashMap;
use std::fs::{metadata, File, OpenOptions};
use std::io::{BufWriter, Result, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use crate::lookup_table::performance::lut_skip_evaluation::{self, SkipMode};

const ORDER: core::sync::atomic::Ordering = Ordering::Relaxed;

lazy_static! {
    static ref SKIP_TRACKER_LUT: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
    static ref SKIP_TRACKER_ITE: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

static LUT_COUNT: AtomicU64 = AtomicU64::new(0);
static ITE_COUNT: AtomicU64 = AtomicU64::new(0);
static LUT_DISTANCE: AtomicU64 = AtomicU64::new(0);
static ITE_DISTANCE: AtomicU64 = AtomicU64::new(0);

pub fn track_distance_lut(distance: usize) {
    if lut_skip_evaluation::MODE == SkipMode::COUNT {
        LUT_COUNT.fetch_add(1, Ordering::Relaxed);
    } else if lut_skip_evaluation::MODE == SkipMode::TRACK {
        let mut map = SKIP_TRACKER_LUT.lock().unwrap();
        *map.entry(distance).or_insert(0) += 1;
    }

    LUT_DISTANCE.fetch_add(distance as u64, ORDER);
}

pub fn track_distance_ite(distance: usize) {
    if lut_skip_evaluation::MODE == SkipMode::COUNT {
        ITE_COUNT.fetch_add(1, Ordering::Relaxed);
    } else if lut_skip_evaluation::MODE == SkipMode::TRACK {
        let mut map = SKIP_TRACKER_ITE.lock().unwrap();
        *map.entry(distance).or_insert(0) += 1;
    }

    ITE_DISTANCE.fetch_add(distance as u64, ORDER);
}

pub fn save_track_to_csv(file_path: &str) -> std::io::Result<()> {
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

pub fn save_count_to_csv(json_path: &str, csv_path: &str, filename: &str, query_name: &str, query_text: &str) {
    let lut_count = LUT_COUNT.load(ORDER);
    let ite_count = ITE_COUNT.load(ORDER);
    let total_count = lut_count + ite_count;
    let lut_distance = LUT_DISTANCE.load(ORDER);
    let ite_distance = ITE_DISTANCE.load(ORDER);
    let total_distance = lut_distance + ite_distance;
    let json_size: u64 = metadata(json_path).expect("Fail @ reading file metadata").len();

    let percentage_total_skip: f64 = if json_size > 0 {
        (total_distance as f64) / (json_size as f64)
    } else {
        0.0
    };
    let percentage_lut_skip: f64 = if json_size > 0 {
        (lut_distance as f64) / (json_size as f64)
    } else {
        0.0
    };
    let percentage_ite_skip: f64 = if json_size > 0 {
        (ite_distance as f64) / (json_size as f64)
    } else {
        0.0
    };

    let path = Path::new(csv_path);
    let file_existed = path.exists();
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Fail @ opening file");
    let mut writer = BufWriter::new(file);

    // Add the header if the file does not exist
    if !file_existed {
        writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            "FILENAME",
            "QUERY_NAME",
            "LUT_PERCENT_SKIP",
            "ITE_PERCENT_SKIP",
            "TOTAL_PERCENT_SKIP",
            "LUT_COUNT",
            "ITE_COUNT",
            "TOTAL_COUNT",
            "LUT_DISTANCE",
            "ITE_DISTANCE",
            "TOTAL_DISTANCE",
            "FILE_SIZE",
        )
        .expect("Fail @ writing head");
    }

    // Write data to CSV
    writeln!(
        writer,
        "{},{},{:.6},{:.6},{:.6},{},{},{},{},{},{},{}",
        filename,
        query_name,
        percentage_lut_skip,
        percentage_ite_skip,
        percentage_total_skip,
        lut_count,
        ite_count,
        total_count,
        lut_distance,
        ite_distance,
        total_distance,
        json_size,
    )
    .expect("Fail @ writing line");

    println!("TOTAL_SKIP_PERCENT = {}", percentage_total_skip);

    writer.flush().expect("Fail @ writing csv");
    reset();
}

fn reset() {
    LUT_COUNT.store(0, ORDER);
    ITE_COUNT.store(0, ORDER);

    LUT_DISTANCE.store(0, ORDER);
    ITE_DISTANCE.store(0, ORDER);
}
