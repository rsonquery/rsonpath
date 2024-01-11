use eyre::WrapErr;
use std::{fs, io::ErrorKind, process::Command};

const TOML_DIRECTORY_PATH: &str = "documents/toml";
const JSON_DIRECTORY_PATH: &str = "documents/json";
const TEST_OUTPUT_PATH: &str = "tests/generated";
const GEN_RUST_GLOB: &str = "tests/generated/**/*.rs";
const RUSTFMT_TOML_PATH: &str = "../../rustfmt.toml";
const CONTROL_ENV_VAR: &str = "RSONPATH_ENABLE_TEST_CODEGEN";

fn main() -> eyre::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=documents/toml/*");
    println!("cargo:rerun-if-changed=documents/json/large/*");
    println!("cargo:rerun-if-changed=../rsonpath-test-codegen/**/*");
    println!("cargo:rerun-if-env-changed={CONTROL_ENV_VAR}");

    if std::env::var_os(CONTROL_ENV_VAR).is_some_and(|x| x == "1") {
        generate()
    } else {
        Ok(())
    }
}

pub(crate) fn generate() -> eyre::Result<()> {
    match fs::remove_dir_all(TEST_OUTPUT_PATH) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
    .wrap_err("error removing earlier generated test")?;
    rsonpath_test_codegen::generate_tests(TOML_DIRECTORY_PATH, JSON_DIRECTORY_PATH, TEST_OUTPUT_PATH)
        .wrap_err("error generating end-to-end tests")?;

    // By default the output is a single line of tokens, which is completely unreadable.
    let mut rustfmt_cmd = Command::new("rustfmt");

    for rs in glob::glob(GEN_RUST_GLOB)? {
        let rs = rs?;
        rustfmt_cmd.arg(rs);
    }
    rustfmt_cmd.arg("--config-path").arg(RUSTFMT_TOML_PATH);

    let rustfmt_status = rustfmt_cmd.status()?;

    assert!(
        rustfmt_status.success(),
        "'rustfmt' excited with code {}",
        rustfmt_status
    );

    Ok(())
}
