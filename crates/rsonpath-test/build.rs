use std::fs;
use std::process::Command;

use eyre::WrapErr;

fn main() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./tests/documents/toml/*");
    println!("cargo:rerun-if-changed=./tests/end_to_end.rs");
    println!("cargo:rerun-if-changed=../rsonpath-test-codegen/**/*");

    const TOML_DIRECTORY_PATH: &str = "./tests/documents/toml";
    const JSON_DIRECTORY_PATH: &str = "./tests/documents/json";
    const OUTPUT_FILE_PATH: &str = "./tests/end_to_end.rs";

    let tokens = rsonpath_test_codegen::generate_tests(TOML_DIRECTORY_PATH, JSON_DIRECTORY_PATH)
        .wrap_err("error generating end-to-end tests")?;
    let source = format!("{}", tokens);

    fs::write(OUTPUT_FILE_PATH, source).wrap_err("error writing to test file")?;

    let rustfmt_status = Command::new("rustfmt").arg(OUTPUT_FILE_PATH).status()?;

    assert!(
        rustfmt_status.success(),
        "'rustfmt {}' excited with code {}",
        OUTPUT_FILE_PATH,
        rustfmt_status
    );

    Ok(())
}
