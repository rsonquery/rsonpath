use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let json_path = &std::env::args().collect::<Vec<_>>()[1];
    let file = fs::File::open(json_path)?;

    rsonpath::ricardo::run(&file)?;

    Ok(())
}
