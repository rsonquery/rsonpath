use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let java_home = std::env::var("JAVA_HOME")?;

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={java_home}/lib/server");

    Ok(())
}
