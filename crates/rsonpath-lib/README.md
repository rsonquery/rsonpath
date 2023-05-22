# `rsonpath-lib` &ndash; SIMD-powered JSONPath, as a library 🚀

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath-lib?logo=docs.rs)](https://docs.rs/crate/rsonpath-lib/latest)

[![Crates.io](https://img.shields.io/crates/v/rsonpath?logo=docs.rs)](https://crates.io/crates/rsonpath)
[![GitHub Release Date](https://img.shields.io/github/release-date/v0ldek/rsonpath)](https://github.com/V0ldek/rsonpath/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/v0ldek/rsonpath?logo=github)](https://github.com/V0ldek/rsonpath/commits/main)

[![Crates.io](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/)

Library for [`rsonpath`](https://crates.io/crates/rsonpath), the JSONPath engine for querying massive streamed datasets.

The main target of this crate is the `rsonpath` CLI tool. Note that this API is unstable until we reach
v1.0.0.

## Unsafe

The library uses `unsafe` for SIMD operations, because it has to, at least until `portable-simd` gets stabilized.
Because of this, a compiled library is *not* portable &ndash; if you build on a platform supporting
AVX2 and then use the same compiled code on an ARM platform, it will crash.
We put special care to not use `unsafe` code anywhere else &ndash; in fact, the crate uses `#[forbid(unsafe_code)]`
when compiled without the default `simd` feature.

## Build & test

The dev workflow utilizes [`just`](https://github.com/casey/just).
Use the included `Justfile`. It will automatically install Rust for you using the `rustup` tool if it detects there is no Cargo in your environment.

```bash
just build
just test
```

## Architecture diagram

Below is a simplified overview of the module interactions and interfaces,
and how data flows from the user's input (query, document) through the pipeline to produce results.

![Architecture diagram](/img/rsonpath-architecture.svg)

## Dependencies

Showing direct dependencies.

```bash
cargo tree --package rsonpath-lib --edges normal --depth 1
```

<!-- rsonpath-lib dependencies start -->
```ini
rsonpath-lib v0.4.0 (/home/mat/rsonpath/crates/rsonpath-lib)
├── aligners v0.0.10
├── cfg-if v1.0.0
├── log v0.4.17
├── memchr v2.5.0
├── nom v7.1.3
├── replace_with v0.1.7
├── smallvec v1.10.0
├── thiserror v1.0.40
└── vector-map v1.0.1
```
<!-- rsonpath-lib dependencies end -->

### Justification

- `aligners` &ndash; SIMD operations require correct input data alignment, putting those requirements at type level makes our code more robust.
- `cfg-if` &ndash; used to support SIMD and no-SIMD versions.
- `log` &ndash; diagnostic logs during compilation and execution.
- `memchr` &ndash; rapid, SIMDified substring search for fast-forwarding to specific members.
- `nom` &ndash; for parser implementation.
- `replace_with` &ndash; for safe handling of internal classifier state when switching classifiers.
- `smallvec` &ndash; crucial for small-stack performance.
- `thiserror` &ndash; idiomatic `Error` implementations.
- `vector_map` &ndash; used in the query compiler for measurably better performance.
