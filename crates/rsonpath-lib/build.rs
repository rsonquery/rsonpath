use std::process::Command;
use std::fs;

use eyre::WrapErr;

fn main() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./tests/toml/*");
    println!("cargo:rerun-if-changed=./tests/end_to_end.rs");
    println!("cargo:rerun-if-changed=../rsonpath-test/*");

    const CASE_DIRECTORY_PATH: &str = "./tests/toml";
    const OUTPUT_FILE_PATH: &str = "./tests/end_to_end.rs";

    let tokens = rsonpath_test::test_source(CASE_DIRECTORY_PATH).wrap_err("error generating end-to-end tests")?;
    let source = format!("{}", tokens);

    fs::write(OUTPUT_FILE_PATH, source).wrap_err("error writing to test file")?;

    let rustfmt_status = Command::new("rustfmt").arg(OUTPUT_FILE_PATH).status()?;

    assert!(
        rustfmt_status.success(),
        "'rustfmt {}' excited with code {}",
        OUTPUT_FILE_PATH,
        rustfmt_status
    );

    #[cfg(feature = "simd")]
    {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if is_x86_feature_detected!("avx2") {
                eprintln!("AVX2 support detected, using simd=avx2");
                println!(r#"cargo:rustc-cfg=simd="avx2""#);
                return Ok(());
            }
        }

        Err(eyre::eyre!(
            "Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature."
        ))
    }
    #[cfg(not(feature = "simd"))]
    {
        println!("cargo:warning=Building rsonpath without SIMD support, expect lower performance.");
        Ok(())
    }
}
