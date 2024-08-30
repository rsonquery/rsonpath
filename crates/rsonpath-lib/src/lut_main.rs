use std::{error::Error, fs};

use rsonpath::lookup_table::lut_builder;

fn main() -> Result<(), Box<dyn Error>> {
    let json_path = &std::env::args().collect::<Vec<_>>()[1];
    let file = fs::File::open(json_path)?;

    lut_builder::run(&file);

    // if let Some(lut_naive) = lut_builder::run(&file) {
    //     let filename = "test_a";
    //     lut_naive.overview();
    //     lut_naive.serialize(&format!(".a_ricardo/output/{}.json", filename))?;
    //     lut_naive.serialize(&format!(".a_ricardo/output/{}.cbor", filename))?;
    // }

    Ok(())
}
