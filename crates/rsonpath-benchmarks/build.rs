use eyre::{eyre, Result};
use std::process::Command;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    setup_jsonski()?;
    setup_jsurfer()?;

    Ok(())
}

fn setup_jsonski() -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir("implementations/jsonski")?;
    let cpps = dir
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "cpp"))
        .map(|e| e.path());

    cc::Build::new()
        .cpp(true)
        .files(cpps)
        .opt_level(3)
        .flag("-std=c++11")
        .flag("-mavx")
        .flag("-mavx2")
        .flag("-msse")
        .flag("-msse2")
        .flag("-msse4")
        .flag("-msse4.2")
        .flag("-mpclmul")
        .flag("-lpthread")
        .flag("-mcmodel=medium")
        .flag("-static-libstdc++")
        .flag("-w")
        .pic(false)
        .compile("jsonski");

    println!("cargo:rerun-if-changed=implementations/jsonski");

    Ok(())
}

fn setup_jsurfer() -> Result<()> {
    let gradlew_status = Command::new("./gradlew")
        .arg("shadowJar")
        .current_dir("./implementations/jsurferShim")
        .status()?;

    if !gradlew_status.success() {
        return Err(eyre!(
            "gradlew execution failed with status code: {}",
            gradlew_status
        ));
    }

    let java_home = std::env::var("JAVA_HOME")?;
    let jar_absolute_path =
        std::path::Path::new("./implementations/jsurferShim/lib/jsurferShim.jar").canonicalize()?;

    println!("cargo:rerun-if-changed=implementations/jsurferShim");
    println!("cargo:rustc-env=LD_LIBRARY_PATH={java_home}/lib/server");
    println!(
        "cargo:rustc-env=RSONPATH_BENCH_JSURFER_SHIM_JAR_PATH={}",
        jar_absolute_path.display()
    );

    Ok(())
}
