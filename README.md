# `rsonpath` &ndash; SIMD-powered JSONPath 🚀

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
<!-- [![docs.rs](https://img.shields.io/docsrs/rsonpath?logo=docs.rs)](https://docs.rs/rsonpath) -->

<!-- [![Crates.io](https://img.shields.io/crates/v/rsonpath?logo=docs.rs)](https://crates.io/crates/rsonpath) -->
[![GitHub Release Date](https://img.shields.io/github/release-date/v0ldek/rsonpath)](https://github.com/V0ldek/rsonpath/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/v0ldek/rsonpath?logo=github)](https://github.com/V0ldek/rsonpath/commits/main)

<!-- [![Crates.io](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/) -->

Experimental JSONPath engine for querying massive streamed datasets.

## Features

The `rsonpath` crate provides a JSONPath parser and a query execution engine,
which utilizes SIMD instructions to provide massive throughput improvements over conventional engines.

![Main throughput plot](/img/main-plot.svg)

### Supported selectors

The project is actively developed and currently supports only a subset of the JSONPath query language.

| Selector              | Syntax                          | Supported | Since  |   |
|-----------------------|---------------------------------|-----------|--------|---|
| Root                  | `$`                             | ✔️        | v0.1.0 |   |
| Dot                   | `.<label>`                      | ✔️        | v0.1.0 |   |
| Index (object member) | `[<label>]`                     | ✔️        | v0.1.0 |   |
| Index (array index)   | `[<index>]`                     | ❌        | -      |   |
| Descendant            | `..`                            | ✔️        | v0.1.0 |   |
| Dot wildcard          | `.*`                            | ❌        | -      |   |
| Index wildcard        | `[*]`                           | ❌        | -      |   |
| Slice                 | `[<start>:<end>:<step>]`        | ❌        | -      |   |
| List                  | `[<sel1>, <sel2>, ..., <selN>]` | ❌        | -      |   |
| Filter                | `[?(<expr>)]`                   | ❌        | -      |   |

## Installation

Easiest way to install is via [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```bash
cargo install rsonpath
```

This might fail with the following error:

```ini
Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.
```

This means the SIMD features of the engine are not implemented for the machine's CPU.
You can still use `rsonpath`, but the speed will be much more limited.
To install SIMD-less `rsonpath`, run:

```bash
cargo install rsonpath --no-default-features
```

Alternatively, you can download the source code and manually run `make install`.

## Build & test

Use the included `Makefile`. It will automatically install Rust for you using the `rustup` tool if it detects there is no Cargo in your environment.

```bash
make
make test
```

## Benchmarks

Note: it is recommended to install `gnuplot` before generating reports.

This highly depends on the exact scenario you want to benchmark. The main benchmark is the
Wikidata dataset benchmarking recursive and stackless, which can be ran with either

```bash
make bench
```

or

```bash
cargo bench --bench rsonpath_wikidata
```

If you want to bench the no-SIMD scenario, disable the default `simd` feature flag:

```bash
cargo bench --bench rsonpath_wikidata --no-default-features
```

You can find other benches in `./rsonpath/benches`.

For details about benchmarking refer to [Criterion.rs docs](https://github.com/bheisler/criterion.rs).

## Background

This project is the result of [my thesis](/pdf/Fast_execution_of_JSONPath_queries.pdf). You can read it for details on the theoretical
background on the engine and details of its implementation.