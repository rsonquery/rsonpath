# `rsonpath-syntax` &ndash; JSONPath parser

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath-syntax?logo=docs.rs)](https://docs.rs/crate/rsonpath-syntax/latest)

[![Crates.io](https://img.shields.io/crates/v/rsonpath-syntax?logo=docs.rs)](https://crates.io/crates/rsonpath-syntax)

![MSRV](https://img.shields.io/badge/msrv-v1.67.1-orange?logo=rust "Minimum Supported Rust Version for `rsonpath-syntax`")
[![License](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/)

Complete, fast, and fully spec-compliant JSONPath query parser.

## Usage

Parse a query to its AST with the `parse` function.

```rust
let query = rsonpath_syntax::parse("$.jsonpath[*]")?;
```

For advanced usage consult the crate documentation.

## Feature flags

There are three optional features:

- `arbitrary`, which enables a dependency on the [`arbitrary` crate](https://docs.rs/arbitrary/latest/arbitrary/) to provide `Arbitrary` implementations on query types; this is used e.g. for fuzzing.
- `color`, which enables a dependency on the [`owo_colors` crate](https://docs.rs/owo-colors/latest/owo_colors/) to provide colorful `Display` representations of `ParseError` with the `colored` function.
- `serde`, which enables a dependency on the [`serde` crate](https://docs.rs/serde/latest/serde/) to provide serialization and deserialization of `JsonPathQuery` and all the underlying types.

## Examples

There are two examples programs, [`builder`](./examples/builder.rs) showcases usage of the `JsonPathQueryBuilder`
struct; [`cli`](./examples/cli.rs) is a small CLI tool that takes one argument, a query to parse, and prints a debug representation of the result query, or an error message &ndash; this is useful for debugging when developing the crate itself.

## State of the crate

This is an in-development version that supports only name, index, and wildcard selectors.
However, these are fully supported, tested, and fuzzed. The planned roadmap is:

- [x] support slices
- [x] support filters (without functions)
- [ ] support functions (including type check)
- [ ] polish the API
- [ ] 1.0.0 stable release

## Dependencies

Showing direct dependencies.

```bash
cargo tree --package rsonpath-lib --edges normal --depth 1 --target=all --all-features
```

<!-- rsonpath-syntax dependencies start -->
```ini
rsonpath-syntax v0.4.0 (/home/mat/src/rsonpath/crates/rsonpath-syntax)
├── nom v8.0.0
├── owo-colors v4.2.3
├── serde v1.0.228
├── thiserror v2.0.18
└── unicode-width v0.2.2
```
<!-- rsonpath-syntax dependencies end -->

### Justification

- `arbitrary` &ndash; optional `Arbitrary` support for fuzzing.
- `nom` &ndash; combinator-based parsing used throughout the crate.
- `owo-colors` &ndash; optional feature for pretty error messages.
- `serde` &ndash; optional dependency for serialization and deserialization of compiled engines.
- `thiserror` &ndash; idiomatic `Error` implementations.
- `unicode-width` &ndash; used to display error messages correctly in presence of wider Unicode characters in the query string.
