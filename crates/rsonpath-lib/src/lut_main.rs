use std::{error::Error, fs};

mod lookup_table {
    pub mod lut_naive;
}

use crate::lookup_table::lut_naive;

fn main() -> Result<(), Box<dyn Error>> {
    let json_path = &std::env::args().collect::<Vec<_>>()[1];
    let file = fs::File::open(json_path)?;

    rsonpath::lut_counter::run(&file)?;

    lut_naive::example_usage(".a_ricardo/output/lut_naive.json", "json");
    lut_naive::example_usage(".a_ricardo/output/lut_naive.cbor", "cbor");

    Ok(())
}
