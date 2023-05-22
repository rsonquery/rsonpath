# `rsonpath` &ndash; SIMD-powered JSONPath 🚀

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath-lib?logo=docs.rs)](https://docs.rs/crate/rsonpath-lib/latest)

[![Crates.io](https://img.shields.io/crates/v/rsonpath?logo=docs.rs)](https://crates.io/crates/rsonpath)
[![GitHub Release Date](https://img.shields.io/github/release-date/v0ldek/rsonpath)](https://github.com/V0ldek/rsonpath/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/v0ldek/rsonpath?logo=github)](https://github.com/V0ldek/rsonpath/commits/main)

[![Crates.io](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/)

Experimental JSONPath engine for querying massive streamed datasets.

## Features

The `rsonpath` crate provides a JSONPath parser and a query execution engine,
which utilizes SIMD instructions to provide massive throughput improvements over conventional engines.

Benchmarks of `rsonpath` against a reference no-SIMD engine on the
[Pison dataset](https://github.com/AutomataLab/Pison). **NOTE: Scale is logarithmic!**
![Main throughput plot](/img/main-plot.svg)

### Supported selectors

The project is actively developed and currently supports only a subset of the JSONPath query language.

| Selector                       | Syntax                          | Supported | Since  | Tracking Issue |
|--------------------------------|---------------------------------|-----------|--------|---------------:|
| Root                           | `$`                             | ✔️        | v0.1.0 |   |
| Dot                            | `.<member>`                     | ✔️        | v0.1.0 |   |
| Index (object member)          | `[<member>]`                    | ✔️        | v0.1.0 |   |
| Index (array index)            | `[<index>]`                     | ❌        | -      | [#64](https://github.com/V0ldek/rsonpath/issues/64) |
| Index (array index from end)   | `[-<index>]`                    | ❌        | -      |   |
| Descendant                     | `..`                            | ✔️        | v0.1.0 |   |
| Child wildcard                 | `.*`, `.[*]`                    | ✔️        | v0.3.0 |   |
| Descendant wildcard            | `..*`, `..[*]`                  | ✔️        | v0.4.0 |   |
| Slice                          | `[<start>:<end>:<step>]`        | ❌        | -      |   |
| List                           | `[<sel1>, <sel2>, ..., <selN>]` | ❌        | -      |   |
| Filter                         | `[?(<expr>)]`                   | ❌        | -      |   |

## Installation

See [Releases](https://github.com/V0ldek/rsonpath/releases/latest) for precompiled binaries for
all first-class support targets.

Easiest way to install is via [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```bash
cargo install rsonpath
```

This might fail with the following error:

```ini
Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.
```

This means the SIMD features of the engine are not implemented for the machine's CPU.
You can still use `rsonpath`, but the speed will be limited (see the reference engine in the chart above). To install without simd, run `cargo install --no-default-features -F default-optimizations`.

Alternatively, you can download the source code and manually run `just install` (requires [`just`](https://github.com/casey/just))
or `cargo install --path ./crates/rsonpath`.

### Native CPU optimizations

If maximum speed is paramount, you should install `rsonpath` with native CPU instructions support.
This will result in a binary that is _not_ portable and might work incorrectly on any other machine,
but will squeeze out every last bit of throughput.

To do this, run the following `cargo install` variant:

```bash
RUSTFLAGS="-C target-cpu=native" cargo install rsonpath
```

## Usage

To run a JSONPath query on a file execute:

```bash
rsonpath '$..a.b' ./file.json
```

If the file is omitted, the engine reads standard input.

For details, consult `rsonpath --help`.

### Results

The results are presented as an array of indices at which a colon of a matching record was found,
the comma directly preceding the matched record in a list,
or the opening bracket of the list in case of the first element in it.
Alternatively, passing `--result count` returns only the number of matches.
Work to support more useful result reports is ongoing.

### Engine

By default, the main SIMD engine is used. On machines not supporting SIMD, the recursive implementation
might be faster in some cases. To change the engine use `--engine recursive`.

## Supported platforms

The crate is continuously built for all Tier 1 Rust targets, and tests are continuously ran for targets that can be ran with GitHub action images. SIMD is supported only on x86-64 platforms for AVX2, while nosimd builds are always available for all targets.

| Target triple             | nosimd build | SIMD support        | Continuous testing | Tracking issues |
|:--------------------------|:-------------|:--------------------|:-------------------|----------------:|
| aarch64-unknown-linux-gnu | ✔️          | ❌                  | ❌                | [#21](https://github.com/V0ldek/rsonpath/issues/21), [#115](https://github.com/V0ldek/rsonpath/issues/115) |
| i686-unknown-linux-gnu    | ✔️          | ❌                  | ✔️                | [#14](https://github.com/V0ldek/rsonpath/issues/14) |
| x86_64-unknown-linux-gnu  | ✔️          | ✔️ avx2+pclmulqdq   | ✔️                | |
| x86_64-apple-darwin       | ✔️          | ❌                  | ✔️                | |
| i686-pc-windows-gnu       | ✔️          | ❌                  | ✔️                | [#14](https://github.com/V0ldek/rsonpath/issues/14) |
| i686-pc-windows-msvc      | ✔️          | ❌                  | ✔️                | [#14](https://github.com/V0ldek/rsonpath/issues/14) |
| x86_64-pc-windows-gnu     | ✔️          | ✔️ avx2+pclmulqdq   | ✔️                | |
| x86_64-pc-windows-msvc    | ✔️          | ✔️ avx2+pclmulqdq   | ✔️                | |

## Caveats and limitations

### JSONPath

Not all selectors are supported, see the support table above.

### Duplicate keys

The engine assumes that every object in the input JSON has no duplicate keys.
Behavior on duplicate keys is not guaranteed to be stable, but currently
the engine will simply match the _first_ such key.

```bash
> rsonpath '$.key'
{"key":"value","key":"other value"}
[6]
```

This behavior can be overriden with a custom installation of `rsonpath`, disabling the default `unique-members` feature. This will hurt performance.

```bash
> cargo install rsonpath --no-default-features -F simd -F head-skip -F tail-skip
> rsonpath '$.key'
{"key":"value","key":"other value"}
[6, 20]
```

### Unicode

The engine does _not_ parse unicode escape sequences in member names.
This means that a key `"a"` is different from a key `"\u0041"`, even though semantically they represent the same string.
Parsing unicode sequences is costly, so the support for this was postponed
in favour of high performance. It would be possible for a flag to exist
to trigger this behaviour, but it is not currently worked on.

## Build & test

The dev workflow utilizes [`just`](https://github.com/casey/just).
Use the included `Justfile`. It will automatically install Rust for you using the `rustup` tool if it detects there is no Cargo in your environment.

```bash
just build
just test
```

## Benchmarks

Benchmarks for `rsonpath` are located in a [separate repository](https://github.com/v0ldek/rsonpath-benchmarks),
included as a [git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules) in this main repository.

Easiest way to run all the benchmarks is `just bench`. For details, look at the README in the submodule.

## Background

This project is the result of [my thesis](/pdf/Fast_execution_of_JSONPath_queries.pdf). You can read it for details on the theoretical
background on the engine and details of its implementation.

## Dependencies

Showing direct dependencies, for full graph see below.

```bash
cargo tree --package rsonpath --edges normal --depth 1
```

<!-- rsonpath dependencies start -->
```ini
rsonpath v0.4.0 (/home/mat/rsonpath/crates/rsonpath)
├── clap v4.1.11
├── color-eyre v0.6.2
├── eyre v0.6.8
├── log v0.4.17
├── rsonpath-lib v0.4.0 (/home/mat/rsonpath/crates/rsonpath-lib)
└── simple_logger v4.1.0
```
<!-- rsonpath dependencies end -->

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

<!-- rsonpath-full dependencies start -->
```ini
rsonpath v0.4.0 (/home/mat/rsonpath/crates/rsonpath)
├── clap v4.1.11
│   ├── bitflags v2.0.2
│   ├── clap_derive v4.1.9 (proc-macro)
│   │   ├── heck v0.4.1
│   │   ├── proc-macro-error v1.0.4
│   │   │   ├── proc-macro-error-attr v1.0.4 (proc-macro)
│   │   │   │   ├── proc-macro2 v1.0.52
│   │   │   │   │   └── unicode-ident v1.0.6
│   │   │   │   └── quote v1.0.26
│   │   │   │       └── proc-macro2 v1.0.52 (*)
│   │   │   ├── proc-macro2 v1.0.52 (*)
│   │   │   ├── quote v1.0.26 (*)
│   │   │   └── syn v1.0.107
│   │   │       ├── proc-macro2 v1.0.52 (*)
│   │   │       ├── quote v1.0.26 (*)
│   │   │       └── unicode-ident v1.0.6
│   │   ├── proc-macro2 v1.0.52 (*)
│   │   ├── quote v1.0.26 (*)
│   │   └── syn v1.0.107 (*)
│   ├── clap_lex v0.3.1
│   │   └── os_str_bytes v6.4.1
│   ├── is-terminal v0.4.3
│   │   ├── io-lifetimes v1.0.5
│   │   │   └── libc v0.2.139
│   │   └── rustix v0.36.8
│   │       ├── bitflags v1.3.2
│   │       ├── io-lifetimes v1.0.5 (*)
│   │       ├── libc v0.2.139
│   │       └── linux-raw-sys v0.1.4
│   ├── once_cell v1.17.0
│   ├── strsim v0.10.0
│   ├── termcolor v1.2.0
│   └── terminal_size v0.2.3
│       └── rustix v0.36.8 (*)
├── color-eyre v0.6.2
│   ├── backtrace v0.3.67
│   │   ├── addr2line v0.19.0
│   │   │   └── gimli v0.27.1
│   │   ├── cfg-if v1.0.0
│   │   ├── libc v0.2.139
│   │   ├── miniz_oxide v0.6.2
│   │   │   └── adler v1.0.2
│   │   ├── object v0.30.3
│   │   │   └── memchr v2.5.0
│   │   └── rustc-demangle v0.1.21
│   ├── eyre v0.6.8
│   │   ├── indenter v0.3.3
│   │   └── once_cell v1.17.0
│   ├── indenter v0.3.3
│   ├── once_cell v1.17.0
│   └── owo-colors v3.5.0
├── eyre v0.6.8 (*)
├── log v0.4.17
│   └── cfg-if v1.0.0
├── rsonpath-lib v0.4.0 (/home/mat/rsonpath/crates/rsonpath-lib)
│   ├── aligners v0.0.10
│   │   ├── cfg-if v1.0.0
│   │   ├── lazy_static v1.4.0
│   │   └── page_size v0.4.2
│   │       └── libc v0.2.139
│   ├── cfg-if v1.0.0
│   ├── log v0.4.17 (*)
│   ├── memchr v2.5.0
│   ├── nom v7.1.3
│   │   ├── memchr v2.5.0
│   │   └── minimal-lexical v0.2.1
│   ├── replace_with v0.1.7
│   ├── smallvec v1.10.0
│   ├── thiserror v1.0.40
│   │   └── thiserror-impl v1.0.40 (proc-macro)
│   │       ├── proc-macro2 v1.0.52 (*)
│   │       ├── quote v1.0.26 (*)
│   │       └── syn v2.0.4
│   │           ├── proc-macro2 v1.0.52 (*)
│   │           ├── quote v1.0.26 (*)
│   │           └── unicode-ident v1.0.6
│   └── vector-map v1.0.1
│       ├── contracts v0.4.0 (proc-macro)
│       │   ├── proc-macro2 v1.0.52 (*)
│       │   ├── quote v1.0.26 (*)
│       │   └── syn v1.0.107 (*)
│       └── rand v0.7.3
│           ├── getrandom v0.1.16
│           │   ├── cfg-if v1.0.0
│           │   └── libc v0.2.139
│           ├── libc v0.2.139
│           ├── rand_chacha v0.2.2
│           │   ├── ppv-lite86 v0.2.17
│           │   └── rand_core v0.5.1
│           │       └── getrandom v0.1.16 (*)
│           └── rand_core v0.5.1 (*)
└── simple_logger v4.1.0
    ├── colored v2.0.0
    │   ├── atty v0.2.14
    │   │   └── libc v0.2.139
    │   └── lazy_static v1.4.0
    ├── log v0.4.17 (*)
    └── time v0.3.17
        ├── itoa v1.0.5
        ├── libc v0.2.139
        ├── num_threads v0.1.6
        ├── time-core v0.1.0
        └── time-macros v0.2.6 (proc-macro)
            └── time-core v0.1.0
```
<!-- rsonpath-full dependencies end -->
