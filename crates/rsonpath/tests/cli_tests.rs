#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .default_bin_name("rq")
        .env("RUST_BACKTRACE", "0")
        .case("tests/cmd/*.toml")
        .case("../../README.md")
        .case("../../book/src/**/*.md")
        .run();
}
