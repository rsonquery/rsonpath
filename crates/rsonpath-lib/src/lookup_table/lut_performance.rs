use std::io;
use std::process::Command;
use std::{error::Error, fs, io::Write, time::Instant};

use crate::lookup_table::{lut_builder, util};

#[inline]
pub fn performance_test(json_path: &str, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(json_path)?;
    if metadata.is_file() {
        println!("Saving stats to {}", csv_path);
        measure_time_and_size(json_path, csv_path)?;
    } else if metadata.is_dir() {
        for entry in fs::read_dir(json_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                measure_time_and_size(path.to_str().expect("Failed to convert path to string"), csv_path)?;
            }
        }

        println!("Saving stats to {}", csv_path);
        run_python_statistics_builder(csv_path)?;
    }

    Ok(())
}

fn measure_time_and_size(json_path: &str, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(json_path)?;
    let filename = util::get_filename_from_path(json_path);
    println!("Processing: {}", filename);

    // Measure build duration
    let start_build = Instant::now();
    if let Ok(lut_naive) = lut_builder::run(&file) {
        let build_duration = start_build.elapsed();

        // Measure JSON serialization duration
        let start_json = Instant::now();
        lut_naive.serialize(&format!(".a_ricardo/output/{}.json", filename))?;
        let json_duration = start_json.elapsed() + build_duration;

        // Measure CBOR serialization duration
        let start_cbor = Instant::now();
        lut_naive.serialize(&format!(".a_ricardo/output/{}.cbor", filename))?;
        let cbor_duration = start_cbor.elapsed() + build_duration;

        // Measure JSON deserialization duration
        let start_json_deserialize = Instant::now();
        lut_naive.deserialize(&format!(".a_ricardo/output/{}.json", filename))?;
        let json_deserialize_duration = start_json_deserialize.elapsed() + json_duration;

        // Measure CBOR deserialization duration
        let start_cbor_deserialize = Instant::now();
        lut_naive.deserialize(&format!(".a_ricardo/output/{}.cbor", filename))?;
        let cbor_deserialize_duration = start_cbor_deserialize.elapsed() + cbor_duration;

        // Open or create the CSV file for appending
        let mut csv_file = fs::OpenOptions::new().append(true).create(true).open(csv_path)?;

        // If the file is freshly created, add the header row
        if csv_file.metadata()?.len() == 0 {
            writeln!(
                csv_file,
                "name,build,cbor_serialize,json_serialize,json_deserialize,cbor_deserialize,cbor_size,json_size"
            )?;
        }

        // Write the results to the CSV file with durations rounded to 5 decimal places
        writeln!(
            csv_file,
            "{},{:.5},{:.5},{:.5},{:.5},{:.5},{},{}",
            filename,
            build_duration.as_secs_f64(),
            cbor_duration.as_secs_f64(),
            json_duration.as_secs_f64(),
            json_deserialize_duration.as_secs_f64(),
            cbor_deserialize_duration.as_secs_f64(),
            lut_naive.estimate_cbor_size(),
            lut_naive.estimate_json_size(),
        )?;

        // lut_naive.overview()
    }

    Ok(())
}

fn run_python_statistics_builder(csv_path: &str) -> io::Result<()> {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/main.py")
        .arg(csv_path)
        .output()?;

    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
