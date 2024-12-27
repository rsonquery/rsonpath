use std::{fs, io::BufReader};

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
    // println!("Using LUT: {}", LookUpTableImpl.name());
    // TODO print the correct lut name, add build time and query time
    println!("Using LUT:");
    let lut = LookUpTableImpl::build(json_path)?;
    engine.add_lut(lut);

    // Get results
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

    // Print results
    println!("Found: ");
    for res in results {
        println!("{res}");
    }

    Ok(())
}
