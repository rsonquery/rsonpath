use std::{
    io::{self, Write},
    process::Command,
};

use crate::lookup_table::{lut_naive::LutNaive, util_path, LookUpTable};

pub fn measure_stats(json_path: &str, output_path: &str, csv_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("- stats");
    let file = std::fs::File::open(json_path)?;
    let filename = util_path::get_filename_from_path(json_path);

    // Measure build duration
    let start_build = std::time::Instant::now();
    let lut_naive = LutNaive::build(&json_path).expect("Unable to build lut_naive");
    let build_time = start_build.elapsed();

    // Measure JSON serialization & deserialization duration
    let lut_json_path = &format!("{}/{}.json", output_path, filename);

    let start_json = std::time::Instant::now();
    lut_naive.serialize(lut_json_path)?;
    let json_serialize_time = start_json.elapsed() + build_time;

    let start_json_deserialize = std::time::Instant::now();
    let _ = LutNaive::deserialize(lut_json_path)?;
    let json_deserialize_time = start_json_deserialize.elapsed() + json_serialize_time;

    // Measure CBOR serialization & deserialization duration
    let lut_cbor_path = &format!("{}/{}.cbor", output_path, filename);

    let start_cbor = std::time::Instant::now();
    lut_naive.serialize(lut_cbor_path)?;
    let cbor_serialize_time = start_cbor.elapsed() + build_time;

    let start_cbor_deserialize = std::time::Instant::now();
    LutNaive::deserialize(lut_cbor_path)?;
    let cbor_deserialize_time = start_cbor_deserialize.elapsed() + cbor_serialize_time;

    // Write the results to the CSV file with durations rounded to 5 decimal places
    let mut csv_file = std::fs::OpenOptions::new().append(true).create(true).open(csv_path)?;
    if csv_file.metadata()?.len() == 0 {
        writeln!(
            csv_file,
            "name,input_size,build,cbor_serialize,json_serialize,json_deserialize,cbor_deserialize,cbor_size,json_size"
        )?;
    }

    writeln!(
        csv_file,
        "{},{},{:.5},{:.5},{:.5},{:.5},{:.5},{},{}",
        filename,
        file.metadata().expect("Can't open file").len(),
        build_time.as_secs_f64(),
        cbor_serialize_time.as_secs_f64(),
        json_serialize_time.as_secs_f64(),
        json_deserialize_time.as_secs_f64(),
        cbor_deserialize_time.as_secs_f64(),
        lut_naive.estimate_cbor_size(),
        lut_naive.estimate_json_size(),
    )?;

    run_python_statistics_builder(csv_path);

    Ok(())
}

fn run_python_statistics_builder(csv_path: &str) {
    let output = Command::new("python")
        .arg("crates/rsonpath-lib/src/lookup_table/python_statistic/stats.py")
        .arg(csv_path)
        .output()
        .expect(&format!("Failed to open csv_path: {}", csv_path));

    if output.status.success() {
        if let Err(e) = io::stdout().write_all(&output.stdout) {
            eprintln!("Failed to write stdout: {}", e);
        }
    } else {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
}
