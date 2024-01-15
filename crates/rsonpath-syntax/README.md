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

There are two optional features:

- `arbitrary`, which enables a dependency on the [`arbitrary` crate](https://docs.rs/arbitrary/latest/arbitrary/) to provide `Arbitrary` implementations on query types; this is used e.g. for fuzzing.
- `color`, which enables a dependency on the [`owo_colors` crate](https://docs.rs/owo-colors/latest/owo_colors/) to provide colorful `Display` representations of `ParseError` with the `colored` function.

## Binary

A small CLI tool, `rsonpath-parse` is attached. It takes one argument, a query to parse, and prints a debug representation of the result query, or an error message. This is useful for debugging.

## State of the crate

This is an in-development version that supports only name, index, and wildcard selectors.
However, these are fully supported, tested, and fuzzed. The planned roadmap is:

- [x] support slices
- [ ] support filters (without functions)
- [ ] support functions (including type check)
- [ ] polish the API
- [ ] 1.0.0 stable release
