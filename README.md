# `rsonpath` &ndash; SIMD-powered JSONPath 🚀

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath?logo=docs.rs)](https://docs.rs/rsonpath)

[![Crates.io](https://img.shields.io/crates/v/rsonpath?logo=docs.rs)](https://crates.io/crates/rsonpath)
[![GitHub Release Date](https://img.shields.io/github/release-date/v0ldek/rsonpath)](https://github.com/V0ldek/rsonpath/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/v0ldek/rsonpath?logo=github)](https://github.com/V0ldek/rsonpath/commits/main)

[![Crates.io](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/)

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

## Usage

To run a JSONPath query on a file execute:

```bash
rsonpath '$..a.b' ./file.json
```

If the file is omitted, the engine reads standard input.

For details, consult `rsonpath --help`.

### Results

The results are presented as an array of indices at which a colon of a matching record was found.
Alternatively, passing `--result count` returns only the number of matches.

### Engine

By default, the main SIMD engine is used. On machines not supporting SIMD, the recursive implementation
might be faster in some cases. To change the engine use `--engine recursive`.

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

## Dependencies

```bash
cargo tree --package rsonpath --edges normal
```

```ini
rsonpath v0.1.0 (/home/v0ldek/rsonpath/rsonpath)
├── aligners v0.0.9
│   ├── cfg-if v1.0.0
│   ├── lazy_static v1.4.0
│   └── page_size v0.4.2
│       └── libc v0.2.126
├── cfg-if v1.0.0
├── clap v3.1.18
│   ├── atty v0.2.14
│   │   └── libc v0.2.126
│   ├── bitflags v1.3.2
│   ├── clap_derive v3.1.18 (proc-macro)
│   │   ├── heck v0.4.0
│   │   ├── proc-macro-error v1.0.4
│   │   │   ├── proc-macro-error-attr v1.0.4 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.39
│   │   │   │   │   └── unicode-ident v1.0.0
│   │   │   │   └── quote v1.0.18
│   │   │   │       └── proc-macro2 v1.0.39 (*)
│   │   │   │   [build-dependencies]
│   │   │   │   └── version_check v0.9.3
│   │   │   ├── proc-macro2 v1.0.39 (*)
│   │   │   ├── quote v1.0.18 (*)
│   │   │   └── syn v1.0.75
│   │   │       ├── proc-macro2 v1.0.39 (*)
│   │   │       ├── quote v1.0.18 (*)
│   │   │       └── unicode-xid v0.2.2
│   │   │   [build-dependencies]
│   │   │   └── version_check v0.9.3
│   │   ├── proc-macro2 v1.0.39 (*)
│   │   ├── quote v1.0.18 (*)
│   │   └── syn v1.0.75 (*)
│   ├── clap_lex v0.2.0
│   │   └── os_str_bytes v6.1.0
│   ├── indexmap v1.8.2
│   │   └── hashbrown v0.11.2
│   │   [build-dependencies]
│   │   └── autocfg v1.0.1
│   ├── lazy_static v1.4.0
│   ├── strsim v0.10.0
│   ├── termcolor v1.1.3
│   └── textwrap v0.15.0
├── color-eyre v0.6.1
│   ├── backtrace v0.3.65
│   │   ├── addr2line v0.17.0
│   │   │   └── gimli v0.26.1
│   │   ├── cfg-if v1.0.0
│   │   ├── libc v0.2.126
│   │   ├── miniz_oxide v0.5.1
│   │   │   └── adler v1.0.2
│   │   ├── object v0.28.3
│   │   │   └── memchr v2.5.0
│   │   └── rustc-demangle v0.1.21
│   │   [build-dependencies]
│   │   └── cc v1.0.73
│   ├── eyre v0.6.8
│   │   ├── indenter v0.3.3
│   │   └── once_cell v1.10.0
│   ├── indenter v0.3.3
│   ├── once_cell v1.10.0
│   └── owo-colors v3.3.0
├── eyre v0.6.8 (*)
├── len-trait v0.6.1
│   └── cfg-if v0.1.10
├── log v0.4.17
│   └── cfg-if v1.0.0
├── memchr v2.5.0
├── nom v7.1.1
│   ├── memchr v2.5.0
│   └── minimal-lexical v0.2.1
├── simple_logger v2.1.0
│   ├── colored v2.0.0
│   │   ├── atty v0.2.14 (*)
│   │   └── lazy_static v1.4.0
│   ├── log v0.4.17 (*)
│   └── time v0.3.9
│       ├── itoa v1.0.2
│       ├── libc v0.2.126
│       ├── num_threads v0.1.6
│       └── time-macros v0.2.4 (proc-macro)
├── smallvec v1.8.0
├── static_assertions v1.1.0
└── vector-map v1.0.1
    ├── contracts v0.4.0 (proc-macro)
    │   ├── proc-macro2 v1.0.39 (*)
    │   ├── quote v1.0.18 (*)
    │   └── syn v1.0.75 (*)
    └── rand v0.7.3
        ├── getrandom v0.1.16
        │   ├── cfg-if v1.0.0
        │   └── libc v0.2.126
        ├── libc v0.2.126
        ├── rand_chacha v0.2.2
        │   ├── ppv-lite86 v0.2.16
        │   └── rand_core v0.5.1
        │       └── getrandom v0.1.16 (*)
        └── rand_core v0.5.1 (*)
```

### Justification

- `aligners` &ndash; SIMD operations require correct input data alignment, putting those requirements at type level makes our code more robust.
- `cfg-if` &ndash; used to support SIMD and no-SIMD versions.
- `clap` &ndash; standard crate to provide the CLI.
- `color-eyre`, `eyre` &ndash; more accessible error messages for the parser.
- `log`, `simple-logger` &ndash; diagnostic logs during compilation and execution.
- `nom` &ndash; for parser implementation.
- `smallvec` &ndash; crucial for small-stack performance.
- `vector_map` &ndash; used in the query compiler for better performance.
