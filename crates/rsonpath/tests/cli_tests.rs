#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .default_bin_name("rq")
        .case("tests/cmd/*.toml")
        .case("../../README.md")
        .case("../../book/src/**/*.md")
        .run();
}
