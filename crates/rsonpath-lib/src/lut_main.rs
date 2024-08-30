use rsonpath::lookup_table::{lut_builder, util};
use std::io::Write;
use std::{error::Error, fs, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let json_path = &std::env::args().collect::<Vec<_>>()[1];

    let metadata = fs::metadata(json_path)?;
    if metadata.is_file() {
        measure_time(json_path, ".a_ricardo/output/stats.csv")?;
    } else if metadata.is_dir() {
        let csv_path = format!("{}/stats.csv", json_path);
        for entry in fs::read_dir(&json_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                measure_time(path.to_str().unwrap(), &csv_path)?;
            }
        }
    }

    Ok(())
}

fn measure_time(json_path: &str, csv_path: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(json_path)?;
    let filename = util::get_filename_from_path(&json_path);
    println!("Processing: {}", filename);

    let start_build = Instant::now();
    if let Ok(lut_naive) = lut_builder::run(&file) {
        let build_duration = start_build.elapsed();

        let start_json = Instant::now();
        lut_naive.serialize(&format!(".a_ricardo/output/{}.json", filename))?;
        let json_duration = start_json.elapsed();

        let start_cbor = Instant::now();
        lut_naive.serialize(&format!(".a_ricardo/output/{}.cbor", filename))?;
        let cbor_duration = start_cbor.elapsed();

        let mut csv_file = fs::OpenOptions::new().append(true).create(true).open(csv_path)?;

        // If the file is freshly created add the head row
        if csv_file.metadata()?.len() == 0 {
            writeln!(csv_file, "name,build,json,cbor")?;
        }

        writeln!(
            csv_file,
            "{},{:?},{:?},{:?}",
            filename, build_duration, json_duration, cbor_duration
        )?;
    }

    Ok(())
}
