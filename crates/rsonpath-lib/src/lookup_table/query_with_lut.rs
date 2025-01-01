use std::{any::type_name, fs, io::BufReader};

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{LookUpTable, LookUpTableImpl},
};

use std::{error::Error, io::Read};

pub fn query_with_lut(json_path: &str, json_query: &str) -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(LevelFilter::Trace).init()?;

    // Build query
    let query = rsonpath_syntax::parse(json_query)?;
    let mut engine = RsonpathEngine::compile_query(&query)?;

    // Build and add lut
    let start_build = std::time::Instant::now();
    let lut = LookUpTableImpl::build(json_path)?;
    let build_time = start_build.elapsed().as_secs_f64();
    engine.add_lut(lut);

    // Get results
    let start_query = std::time::Instant::now();
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path)?);
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        // Here you can define whether to use OwnedBytes (padding), Mmap (padding = 0) or Borrowed (padding)
        OwnedBytes::new(buf)
    };
    let mut sink = vec![];
    engine.matches(&input, &mut sink)?;
    let results = sink
        .into_iter()
        .map(|m| String::from_utf8_lossy(m.bytes()).to_string())
        .collect::<Vec<_>>();
    let query_time = start_query.elapsed().as_secs_f64();

    // Print results
    println!("Results found: ");
    let num_results = results.len();
    for (i, result) in results.into_iter().enumerate() {
        println!("Result {}:", i);
        println!("{result}");
    }

    println!("#### Stats ####");
    println!(" - Num results:    {}", num_results);
    println!(" - LUT type:       {}", type_name::<LookUpTableImpl>());
    println!(" - LUT build time: {} seconds", build_time);
    println!(" - LUT query time: {} seconds", query_time);
    println!(" - LUT size:       {} bytes", engine.allocated_bytes_by_lut());

    Ok(())
}
