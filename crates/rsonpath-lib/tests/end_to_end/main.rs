use std::{error::Error, process::ExitCode};

const CASE_DIRECTORY_PATH: &str = "./tests/end_to_end/cases";

fn main() -> Result<ExitCode, Box<dyn Error>> {
    rsonpath_test::test(CASE_DIRECTORY_PATH)
}
