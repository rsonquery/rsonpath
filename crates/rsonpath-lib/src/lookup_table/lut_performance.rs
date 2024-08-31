use std::{error::Error, fs, io::Write, time::Instant};

use crate::lookup_table::{lut_builder, util};

pub fn performance_test(json_path: &str, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(json_path)?;
    if metadata.is_file() {
        measure_time_and_size(json_path, &csv_path)?;
    } else if metadata.is_dir() {
        for entry in fs::read_dir(&json_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                measure_time_and_size(path.to_str().unwrap(), &csv_path)?;
            }
        }
    }

    Ok(())
}

fn measure_time_and_size(json_path: &str, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(json_path)?;
    let filename = util::get_filename_from_path(&json_path);
    println!("Processing: {}", filename);

    let start_build = Instant::now();
    if let Ok(lut_naive) = lut_builder::run(&file) {
        let build_duration = start_build.elapsed();

        let start_json = Instant::now();
        lut_naive.serialize(&format!(".a_ricardo/output/{}.json", filename))?;
        let json_duration = start_json.elapsed() + build_duration;

        let start_cbor = Instant::now();
        lut_naive.serialize(&format!(".a_ricardo/output/{}.cbor", filename))?;
        let cbor_duration = start_cbor.elapsed() + build_duration;

        let mut csv_file = fs::OpenOptions::new().append(true).create(true).open(csv_path)?;

        // If the file is freshly created add the head row
        if csv_file.metadata()?.len() == 0 {
            writeln!(csv_file, "name,build,cbor,json_total,cbor_size,json_size")?;
        }

        // Round durations to 2 decimal places
        let build_duration = build_duration.as_secs_f64();
        let json_duration = json_duration.as_secs_f64();
        let cbor_duration = cbor_duration.as_secs_f64();
        let cbor_size = lut_naive.estimate_cbor_size();
        let json_size = lut_naive.estimate_json_size();

        writeln!(
            csv_file,
            "{},{:.5},{:.5},{:.5},{},{}",
            filename, build_duration, cbor_duration, json_duration, cbor_size, json_size
        )?;

        lut_naive.overview()
    }

    Ok(())
}
