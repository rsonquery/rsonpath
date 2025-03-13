use std::{any::type_name, fs, io::BufReader};

use log::LevelFilter;
use simple_logger::SimpleLogger;

use crate::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
    lookup_table::{LookUpTable, LUT},
};

use std::io::Read;

#[inline]
pub fn query_with_lut(json_path: &str, json_query: &str) {
    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .init()
        .expect("Fail init logger.");

    // Build query
    let query = rsonpath_syntax::parse(json_query).expect("Fail @ parsing query.");
    let mut engine = RsonpathEngine::compile_query(&query).expect("Fail @ compiling.");

    // Build and add lut
    let start_build = std::time::Instant::now();
    let lut = LUT::build(json_path, 0).expect("Fail @ building LookUp-table.");
    let build_time = start_build.elapsed().as_secs_f64();
    engine.add_lut(lut);

    // Get results
    let start_query = std::time::Instant::now();
    let input = {
        let mut file = BufReader::new(fs::File::open(json_path).expect("Fail @ reading file"));
        let mut buf = vec![];
        file.read_to_end(&mut buf).expect("Fail @ read");
        // Here you can define whether to use OwnedBytes (padding), Mmap (padding = 0) or Borrowed (padding)
        OwnedBytes::new(buf)
    };
    let mut sink = vec![];
    engine.matches(&input, &mut sink).expect("Fail @ engine matching.");
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
    println!(" - LUT type:       {}", type_name::<LUT>());
    println!(" - LUT build time: {} seconds", build_time);
    println!(" - LUT query time: {} seconds", query_time);
    println!(" - LUT size:       {} bytes", engine.allocated_bytes_by_lut());
}
