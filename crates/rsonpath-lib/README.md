# `rsonpath-lib` &ndash; SIMD-powered JSONPath, as a library 🚀

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath-lib?logo=docs.rs)](https://docs.rs/crate/rsonpath-lib/latest)
[![Book](https://img.shields.io/badge/book-available-4DC720?logo=mdbook)](https://v0ldek.github.io/rsonpath/)

[![Crates.io](https://img.shields.io/crates/v/rsonpath?logo=docs.rs)](https://crates.io/crates/rsonpath)
[![GitHub Release Date](https://img.shields.io/github/release-date/v0ldek/rsonpath?logo=github)](https://github.com/V0ldek/rsonpath/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/v0ldek/rsonpath?logo=github)](https://github.com/V0ldek/rsonpath/commits/main)

![MSRV](https://img.shields.io/badge/msrv-v1.67.1-orange?logo=rust "Minimum Supported Rust Version for `rsonpath-lib`")
[![License](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/)

Library for [`rsonpath`](https://crates.io/crates/rsonpath), the JSONPath engine for querying massive streamed datasets.

The main target of this crate is the `rsonpath` CLI tool. Note that this API is unstable until we reach
v1.0.0. This *is* going to happen (we have a roadmap), but our dev resources are quite limited.
Contributions are welcome and appreciated.

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

## Optional features

The `simd` feature is enabled by default and is recommended to make use of the performance benefits of the project.

The `arbitrary` feature is optional and enables the [`arbitrary` dependency](https://lib.rs/crates/arbitrary),
which provides an implementation of [`Arbitrary`](https://docs.rs/arbitrary/latest/arbitrary/trait.Arbitrary.html)
for the query struct.

## Dependencies

Showing direct dependencies.

```bash
cargo tree --package rsonpath-lib --edges normal --depth 1
```

<!-- rsonpath-lib dependencies start -->
```ini
rsonpath-lib v0.9.1 (/home/mat/src/rsonpath/crates/rsonpath-lib)
├── arbitrary v1.3.2
├── cfg-if v1.0.0
├── log v0.4.21
├── memmap2 v0.9.4
├── nom v7.1.3
├── rsonpath-syntax v0.3.1 (/home/mat/src/rsonpath/crates/rsonpath-syntax)
├── smallvec v1.13.2
├── static_assertions v1.1.0
├── thiserror v1.0.58
└── vector-map v1.0.1
```
<!-- rsonpath-lib dependencies end -->

### Justification

- `cfg-if` &ndash; used to support SIMD and no-SIMD versions.
- `memchr` &ndash; rapid, SIMDified substring search for fast-forwarding to labels.
- `memmap2` &ndash; for fast reading of source files via a memory map instead of buffered copies.
- `nom` &ndash; for parser implementation.
- `replace_with` &ndash; for safe handling of internal classifier state when switching classifiers.
- `smallvec` &ndash; crucial for small-stack performance.
- `static_assertions` &ndash; additional reliability by some constant assumptions validated at compile time.
- `thiserror` &ndash; idiomatic `Error` implementations.
- `vector_map` &ndash; used in the query compiler for measurably better performance.
