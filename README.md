# `rsonpath` &ndash; SIMD-powered JSONPath 🚀

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath-lib?logo=docs.rs)](https://docs.rs/crate/rsonpath-lib/0.2.0)

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
| Child wildcard        | `.*`, `.[*]`                    | ❌        | -      |   |
| Descendant wildcard   | `..*`, `..[*]`                  | ❌        | -      |   |
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

Benchmarks for `rsonpath` are located in a [separate repository](https://github.com/v0ldek/rsonpath-benchmarks),
included as a [git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules) in this main repository.

Easiest way to run all the benchmarks is `make bench`. For details, look at the README in the submodule.

## Background

This project is the result of [my thesis](/pdf/Fast_execution_of_JSONPath_queries.pdf). You can read it for details on the theoretical
background on the engine and details of its implementation.

## Dependencies

Showing direct dependencies, for full graph see below.

```bash
cargo tree --package rsonpath --edges normal --depth 1
```

```ini
rsonpath v0.2.0 (/home/v0ldek/rsonpath/crates/rsonpath)
├── clap v4.0.25
├── color-eyre v0.6.2
├── eyre v0.6.8
├── log v0.4.17
├── rsonpath-lib v0.2.0
└── simple_logger v4.0.0
```

```bash
cargo tree --package rsonpath-lib --edges normal --depth 1
```

```ini
rsonpath-lib v0.2.0 (/home/v0ldek/rsonpath/crates/rsonpath-lib)
├── aligners v0.0.10
├── cfg-if v1.0.0
├── log v0.4.17
├── memchr v2.5.0
├── nom v7.1.1
├── replace_with v0.1.7
├── smallvec v1.10.0
├── thiserror v1.0.37
└── vector-map v1.0.1
```

### Justification

- `clap` &ndash; standard crate to provide the CLI.
- `color-eyre`, `eyre` &ndash; more accessible error messages for the parser.
- `log`, `simple-logger` &ndash; diagnostic logs during compilation and execution.

- `aligners` &ndash; SIMD operations require correct input data alignment, putting those requirements at type level makes our code more robust.
- `cfg-if` &ndash; used to support SIMD and no-SIMD versions.
- `memchr` &ndash; rapid, SIMDified substring search for fast-forwarding to labels.
- `nom` &ndash; for parser implementation.
- `replace_with` &ndash; for safe handling of internal classifier state when switching classifiers.
- `smallvec` &ndash; crucial for small-stack performance.
- `thiserror` &ndash; idiomatic `Error` implementations.
- `vector_map` &ndash; used in the query compiler for measurably better performance.

## Full dependency tree

```bash
cargo tree --package rsonpath --edges normal
```

```ini
rsonpath v0.2.0
├── clap v4.0.25
│   ├── atty v0.2.14
│   │   └── libc v0.2.137
│   ├── bitflags v1.3.2
│   ├── clap_derive v4.0.21 (proc-macro)
│   │   ├── heck v0.4.0
│   │   ├── proc-macro-error v1.0.4
│   │   │   ├── proc-macro-error-attr v1.0.4 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.47
│   │   │   │   │   └── unicode-ident v1.0.0
│   │   │   │   └── quote v1.0.18
│   │   │   │       └── proc-macro2 v1.0.47 (*)
│   │   │   ├── proc-macro2 v1.0.47 (*)
│   │   │   ├── quote v1.0.18 (*)
│   │   │   └── syn v1.0.75
│   │   │       ├── proc-macro2 v1.0.47 (*)
│   │   │       ├── quote v1.0.18 (*)
│   │   │       └── unicode-xid v0.2.2
│   │   ├── proc-macro2 v1.0.47 (*)
│   │   ├── quote v1.0.18 (*)
│   │   └── syn v1.0.75 (*)
│   ├── clap_lex v0.3.0
│   │   └── os_str_bytes v6.1.0
│   ├── once_cell v1.16.0
│   ├── strsim v0.10.0
│   ├── termcolor v1.1.3
│   └── terminal_size v0.2.2
│       └── rustix v0.35.13
│           ├── bitflags v1.3.2
│           ├── io-lifetimes v0.7.5
│           ├── libc v0.2.137
│           └── linux-raw-sys v0.0.46
├── color-eyre v0.6.2
│   ├── backtrace v0.3.65
│   │   ├── addr2line v0.17.0
│   │   │   └── gimli v0.26.1
│   │   ├── cfg-if v1.0.0
│   │   ├── libc v0.2.137
│   │   ├── miniz_oxide v0.5.1
│   │   │   └── adler v1.0.2
│   │   ├── object v0.28.3
│   │   │   └── memchr v2.5.0
│   │   └── rustc-demangle v0.1.21
│   ├── eyre v0.6.8
│   │   ├── indenter v0.3.3
│   │   └── once_cell v1.16.0
│   ├── indenter v0.3.3
│   ├── once_cell v1.16.0
│   └── owo-colors v3.3.0
├── eyre v0.6.8 (*)
├── log v0.4.17
│   └── cfg-if v1.0.0
├── rsonpath-lib v0.2.0
│   ├── aligners v0.0.10
│   │   ├── cfg-if v1.0.0
│   │   ├── lazy_static v1.4.0
│   │   └── page_size v0.4.2
│   │       └── libc v0.2.137
│   ├── cfg-if v1.0.0
│   ├── log v0.4.17 (*)
│   ├── memchr v2.5.0
│   ├── nom v7.1.1
│   │   ├── memchr v2.5.0
│   │   └── minimal-lexical v0.2.1
│   ├── replace_with v0.1.7
│   ├── smallvec v1.10.0
│   ├── thiserror v1.0.37
│   │   └── thiserror-impl v1.0.37 (proc-macro)
│   │       ├── proc-macro2 v1.0.47 (*)
│   │       ├── quote v1.0.18 (*)
│   │       └── syn v1.0.75 (*)
│   └── vector-map v1.0.1
│       ├── contracts v0.4.0 (proc-macro)
│       │   ├── proc-macro2 v1.0.47 (*)
│       │   ├── quote v1.0.18 (*)
│       │   └── syn v1.0.75 (*)
│       └── rand v0.7.3
│           ├── getrandom v0.1.16
│           │   ├── cfg-if v1.0.0
│           │   └── libc v0.2.137
│           ├── libc v0.2.137
│           ├── rand_chacha v0.2.2
│           │   ├── ppv-lite86 v0.2.16
│           │   └── rand_core v0.5.1
│           │       └── getrandom v0.1.16 (*)
│           └── rand_core v0.5.1 (*)
└── simple_logger v4.0.0
    ├── colored v2.0.0
    │   ├── atty v0.2.14 (*)
    │   └── lazy_static v1.4.0
    ├── log v0.4.17 (*)
    └── time v0.3.17
        ├── itoa v1.0.2
        ├── libc v0.2.137
        ├── num_threads v0.1.6
        ├── time-core v0.1.0
        └── time-macros v0.2.6 (proc-macro)
            └── time-core v0.1.0
```
