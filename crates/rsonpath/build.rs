use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    println!("SRAM DO RYJA");
    EmitBuilder::builder().all_git().emit()?;
    Ok(())
}
