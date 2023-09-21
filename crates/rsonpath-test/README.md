# Just codegen

This crate should be used only for the declarative TOML tests.

It has no code in it except for the build script and the generated script.
The build script generates test cases for `rsonpath-lib` based on TOML files in `tests/documents`
using `rsonpath-test-codegen`. This is needed for the following reasons:

1. `rsonpath-test-codegen` cannot also have a `build.rs` script to generate the tests, since it would need to build-depend on itself;
bootstrapping issue.
2. We don't want `rsonpath-lib` to have a complicated `build.rs` script.
3. We don't want `rsonpath-lib` to build-depend on `rsonpath-test-codegen` and its transitives.
    - we would have to publish `rsonpath-test-codegen` on crates.io;
    - these are needless dependencies that influence build times;
    - we tried hiding them behind a cfg feature, but it's unergonomic.
