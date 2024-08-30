use std::{error::Error, fs};

use rsonpath::lookup_table::lut_builder;

fn main() -> Result<(), Box<dyn Error>> {
    let json_path = &std::env::args().collect::<Vec<_>>()[1];
    let file = fs::File::open(json_path)?;

    lut_builder::run(&file)?;

    Ok(())
}
