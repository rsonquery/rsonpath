# rsonpath &ndash; SIMD-powered JSONPath 🚀 <img src="img/rsonquery-logo.svg" width="50em" align="left" />

[![Rust](https://github.com/rsonquery/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/rsonquery/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath-lib?logo=docs.rs)](https://docs.rs/crate/rsonpath-lib/latest)
[![Book](https://img.shields.io/badge/book-available-4DC720?logo=mdbook)](https://rsonquery.github.io/rsonpath/)

[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/7790/badge)](https://www.bestpractices.dev/projects/7790)
[![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/rsonquery/rsonpath/badge)](https://securityscorecards.dev/viewer/?uri=github.com/rsonquery/rsonpath)
[![fuzzing](https://github.com/rsonquery/rsonpath/actions/workflows/clusterfuzzlite-batch.yml/badge.svg)](https://github.com/rsonquery/rsonpath/actions/workflows/clusterfuzzlite-batch.yml)

[![Crates.io](https://img.shields.io/crates/v/rsonpath?logo=docs.rs)](https://crates.io/crates/rsonpath)
[![GitHub Release Date](https://img.shields.io/github/release-date/rsonquery/rsonpath?logo=github)](https://github.com/rsonquery/rsonpath/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/rsonquery/rsonpath?logo=github)](https://github.com/rsonquery/rsonpath/commits/main)

![MSRV](https://img.shields.io/badge/msrv-v1.89.0-orange?logo=rust "Minimum Supported Rust Version for `rsonpath-lib`")
[![License](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/)

Experimental JSONPath engine for querying massive streamed datasets.

The `rsonpath` crate provides a JSONPath parser and a query execution engine `rq`,
which utilizes SIMD instructions to provide massive throughput improvements over conventional engines.

Benchmarks of `rsonpath` against a reference no-SIMD engine on the
[Pison dataset](https://github.com/AutomataLab/Pison). **NOTE: Scale is logarithmic!**
![Main throughput plot](/img/main-plot.svg)

## Usage

To run a JSONPath query on a file execute:

```console,ignore
rq '$..a.b' ./file.json
```

If the file is omitted, the engine reads standard input. JSON can also be passed inline:

```console
$ rq '$..a.b' --json '{"c":{"a":{"b":42}}}'
42

```

For details, consult `rq --help` or [the rsonbook](https://rsonquery.github.io/rsonpath/).

### Results

The result of running a query is a sequence of matched values, delimited by newlines.
Alternatively, passing `--result count` returns only the number of matches, which might be much faster.
For other result modes consult the `--help` usage page.

## Installation

See [Releases](https://github.com/rsonquery/rsonpath/releases/latest) for precompiled binaries for
all first-class support targets.

### `cargo`

Easiest way to install is via [`cargo`](https://doc.rust-lang.org/cargo/getting-started/installation.html).

```console,ignore
$ cargo install rsonpath
...
```

### Native CPU optimizations

If maximum speed is paramount, you should install `rsonpath` with native CPU instructions support.
This will result in a binary that is _not_ portable and might work incorrectly on any other machine,
but will squeeze out every last bit of throughput.

To do this, run the following `cargo install` variant:

```console,ignore
$ RUSTFLAGS="-C target-cpu=native" cargo install rsonpath
...
```

Check out [the relevant chapter in the rsonbook](https://rsonquery.github.io/rsonpath/user/installation/manual.html).

## Query language

The project is actively developed and currently supports only a subset of the JSONPath query language.
A query is a sequence of segments, each containing one or more selectors.

### Supported segments

| Segment                        | Syntax                           | Supported | Since  | Tracking Issue |
|--------------------------------|----------------------------------|-----------|--------|---------------:|
| Child segment (single)         | `[<selector>]`                   | ✔️        | v0.1.0 |                |
| Child segment (multiple)       | `[<selector1>,...,<selectorN>]`  | ❌        |        |                |
| Descendant segment (single)    | `..[<selector>]`                 | ✔️        | v0.1.0 |                |
| Descendant segment (multiple)  | `..[<selector1>,...,<selectorN>]`| ❌        |        |                |

### Supported selectors

| Selector                                 | Syntax                           | Supported | Since  | Tracking Issue |
|------------------------------------------|----------------------------------|-----------|--------|---------------:|
| Root                                     | `$`                              | ✔️        | v0.1.0 |                |
| Name                                     | `.<member>`, `[<member>]`        | ✔️        | v0.1.0 |                |
| Wildcard                                 | `.*`, `..*`, `[*]`               | ✔️        | v0.4.0 |                |
| Index (array index)                      | `[<index>]`                      | ✔️        | v0.5.0 |                |
| Index (array index from end)             | `[-<index>]`                     | ❌        |        |                |
| Array slice (forward, positive bounds)   | `[<start>:<end>:<step>]`         | ✔️        | v0.9.0       | [#152](https://github.com/rsonquery/rsonpath/issues/152) |
| Array slice (forward, arbitrary bounds)  | `[<start>:<end>:<step>]`         | ❌        |        |                |
| Array slice (backward, arbitrary bounds) | `[<start>:<end>:-<step>]`        | ❌        |        |                |
| Filters &ndash; existential tests        | `[?<path>]`                      | ❌        |        | [#154](https://github.com/rsonquery/rsonpath/issues/154) |
| Filters &ndash; const atom comparisons   | `[?<path> <binop> <atom>]`       | ❌        |        | [#156](https://github.com/rsonquery/rsonpath/issues/156) |
| Filters &ndash; logical expressions      | `&&`, `\|\|`, `!`                | ❌        |        |                |
| Filters &ndash; nesting                  | `[?<expr>[?<expr>]...]`          | ❌        |        |                |
| Filters &ndash; arbitrary comparisons    | `[?<path> <binop> <path>]`       | ❌        |        |                |
| Filters &ndash; function extensions      | `[?func(<path>)]`                | ❌        |        |                |

## Supported platforms

The crate is continuously built and tested for all Tier 1 Rust targets.
Pre-built binaries are also available for some Tier 2 targets, but without testing.
Currently, these are MUSL targets -- if you require other binaries create an issue.
SIMD is available on x86 and ARM (64-bit) platforms.

| Target triple             | nosimd build | SIMD support        | Continuous testing | Tracking issues |
|:--------------------------|:-------------|:--------------------|:-------------------|----------------:|
| aarch64-apple-darwin      | ✔️          | ✔️                  | ✔️                | |
| aarch64-pc-windows-msvc   | ✔️          | ✔️                  | ✔️                | |
| aarch64-unknown-linux-gnu | ✔️          | ✔️                  | ✔️                | |
| i686-pc-windows-msvc      | ✔️          | ✔️                  | ✔️                | |
| i686-unknown-linux-gnu    | ✔️          | ✔️                  | ✔️                | |
| x86_64-pc-windows-gnu     | ✔️          | ✔️                  | ✔️                | |
| x86_64-pc-windows-msvc    | ✔️          | ✔️                  | ✔️                | |
| x86_64-unknown-linux-gnu  | ✔️          | ✔️                  | ✔️                | |
| aarch64-unknown-linux-musl| ✔️          | ✔️                  | ❌                | |
| i686-unknown-linux-musl   | ✔️          | ✔️                  | ❌                | |
| x86_64-unknown-linux-musl | ✔️          | ✔️                  | ❌                | |

### SIMD support

SIMD support is enabled on a module-by-module basis. Generally, any CPU released in the past
decade supports AVX2, which enables all available optimizations. On ARM, we support NEON.

Older CPUs with SSE2 or higher get partial support. You can check what exactly is enabled
with `rq --version` &ndash; check the `SIMD support` field:

```console,ignore
$ rq --version
rq 0.9.1

Commit SHA:      c024e1bab89610455537b77aed249d2a05a81ed6
Features:        default,simd
Opt level:       3
Target triple:   x86_64-unknown-linux-gnu
Codegen flags:   link-arg=-fuse-ld=lld
SIMD support:    avx2;fast_quotes;fast_popcnt
```

The `fast_quotes` capability depends on the `pclmulqdq` instruction (on x86) or the `aes` feature (ARM),
and `fast_popcnt` on the `popcnt` instruction (always available on ARM).

## Caveats and limitations

### JSONPath

Not all selectors are supported, see the support table above.

### Duplicate keys

The engine assumes that every object in the input JSON has no duplicate keys.
Behavior on duplicate keys is not guaranteed to be stable, but currently
the engine will simply match the _first_ such key.

```console
$ rq '$.key' --json '{"key":"value","key":"other value"}'
"value"

```

### Unicode

The engine does _not_ parse unicode escape sequences in member names.
This means that a key `"a"` is different from a key `"\u0041"`, even though semantically they represent the same string.
This is actually as-designed with respect to the current JSONPath spec.
Parsing unicode sequences is costly, so the support for this was postponed
in favour of high performance. This is tracked as [#117](https://github.com/rsonquery/rsonpath/issues/117).

## Contributing

The gist is: fork, implement, make a PR back here. More details are in the [CONTRIBUTING](/CONTRIBUTING.md) doc.

### Build & test

The dev workflow utilizes [`just`](https://github.com/casey/just).
Use the included `Justfile`. It will automatically install Rust for you using the `rustup` tool if it detects there is no Cargo in your environment.

```console,ignore
$ just build
...
$ just test
...
```

## Benchmarks

Benchmarks for `rsonpath` are located in the benchmark crate of this repository. 
Easiest way to run all the benchmarks is `just bench` within the directory `crates/rsonpath-benchmarks` . For details, look at the README in this directory.

## Background

We have a paper on `rsonpath` to be published at [ASPLOS '24](https://www.asplos-conference.org/asplos2024/)! You can read it
[here](/pdf/supporting-descendants-in-simd-accelerated-jsonpath.pdf).

This project was conceived as [my thesis](/pdf/fast-execution-of-jsonpath-queries.pdf). You can read it for details on the theoretical
background on the engine and details of its implementation.

We also have a short talk from ASPLOS 2024 about rsonpath!

https://gienieczko.com/asplos-2024-talk.mp4

(excuse the audio quality, the sound in the source video was corrupted and we had to salvage)

## Dependencies

Showing direct dependencies, for full graph see below.

```bash
cargo tree --package rsonpath --edges normal --depth 1
```

<!-- rsonpath dependencies start -->
```ini
rsonpath v0.10.1 (/home/mat/src/rsonpath/crates/rsonpath)
├── clap v4.6.1
├── color-eyre v0.6.5
├── eyre v0.6.12
├── log v0.4.30
├── rsonpath-lib v0.10.1 (/home/mat/src/rsonpath/crates/rsonpath-lib)
├── rsonpath-syntax v0.4.2 (/home/mat/src/rsonpath/crates/rsonpath-syntax)
└── simple_logger v5.2.0
[build-dependencies]
├── rustflags v0.1.7
├── vergen v9.1.0
│   [build-dependencies]
└── vergen-git2 v9.1.0
    [build-dependencies]
```
<!-- rsonpath dependencies end -->

```bash
cargo tree --package rsonpath-lib --edges normal --depth 1
```

<!-- rsonpath-lib dependencies start -->
```ini
rsonpath-lib v0.10.1 (/home/mat/src/rsonpath/crates/rsonpath-lib)
├── cfg-if v1.0.4
├── log v0.4.30
├── memmap2 v0.9.10
├── rsonpath-syntax v0.4.2 (/home/mat/src/rsonpath/crates/rsonpath-syntax)
├── serde v1.0.228
├── smallvec v1.15.1
├── static_assertions v1.1.0
├── thiserror v2.0.18
└── vector-map v1.1.0
```
<!-- rsonpath-lib dependencies end -->

### Justification

- `clap` &ndash; standard crate to provide the CLI.
- `color-eyre`, `eyre` &ndash; more accessible error messages for the parser.
- `log`, `simple-logger` &ndash; diagnostic logs during compilation and execution.
- `cfg-if` &ndash; used to support SIMD and no-SIMD versions.
- `memmap2` &ndash; for fast reading of source files via a memory map instead of buffered copies.
- `nom` &ndash; for parser implementation.
- `serde` &ndash; optional dependency under a feature, used to allow serialization/deserialization of compiled queries.
- `smallvec` &ndash; crucial for small-stack performance.
- `static_assertions` &ndash; additional reliability by some constant assumptions validated at compile time.
- `thiserror` &ndash; idiomatic `Error` implementations.
- `vector_map` &ndash; used in the query compiler for measurably better performance.

## Full dependency tree

```bash
cargo tree --package rsonpath --edges normal
```

<!-- rsonpath-full dependencies start -->
```ini
rsonpath v0.10.1 (/home/mat/src/rsonpath/crates/rsonpath)
├── clap v4.6.1
│   ├── clap_builder v4.6.0
│   │   ├── anstream v1.0.0
│   │   │   ├── anstyle v1.0.14
│   │   │   ├── anstyle-parse v1.0.0
│   │   │   │   └── utf8parse v0.2.2
│   │   │   ├── anstyle-query v1.1.5
│   │   │   │   └── windows-sys v0.61.2
│   │   │   │       └── windows-link v0.2.1
│   │   │   ├── anstyle-wincon v3.0.11
│   │   │   │   ├── anstyle v1.0.14
│   │   │   │   ├── once_cell_polyfill v1.70.2
│   │   │   │   └── windows-sys v0.61.2 (*)
│   │   │   ├── colorchoice v1.0.5
│   │   │   ├── is_terminal_polyfill v1.70.2
│   │   │   └── utf8parse v0.2.2
│   │   ├── anstyle v1.0.14
│   │   ├── clap_lex v1.1.0
│   │   ├── strsim v0.11.1
│   │   └── terminal_size v0.4.4
│   │       ├── rustix v1.1.4
│   │       │   ├── bitflags v2.11.1
│   │       │   ├── errno v0.3.14
│   │       │   │   ├── libc v0.2.186
│   │       │   │   └── windows-sys v0.61.2 (*)
│   │       │   ├── libc v0.2.186
│   │       │   ├── linux-raw-sys v0.12.1
│   │       │   └── windows-sys v0.61.2 (*)
│   │       └── windows-sys v0.61.2 (*)
│   └── clap_derive v4.6.1 (proc-macro)
│       ├── heck v0.5.0
│       ├── proc-macro2 v1.0.106
│       │   └── unicode-ident v1.0.24
│       ├── quote v1.0.45
│       │   └── proc-macro2 v1.0.106 (*)
│       └── syn v2.0.117
│           ├── proc-macro2 v1.0.106 (*)
│           ├── quote v1.0.45 (*)
│           └── unicode-ident v1.0.24
├── color-eyre v0.6.5
│   ├── backtrace v0.3.76
│   │   ├── addr2line v0.25.1
│   │   │   └── gimli v0.32.3
│   │   ├── cfg-if v1.0.4
│   │   ├── libc v0.2.186
│   │   ├── miniz_oxide v0.8.9
│   │   │   └── adler2 v2.0.1
│   │   ├── object v0.37.3
│   │   │   └── memchr v2.8.1
│   │   ├── rustc-demangle v0.1.27
│   │   └── windows-link v0.2.1
│   ├── eyre v0.6.12
│   │   ├── indenter v0.3.4
│   │   └── once_cell v1.21.4
│   ├── indenter v0.3.4
│   ├── once_cell v1.21.4
│   └── owo-colors v4.3.0
├── eyre v0.6.12 (*)
├── log v0.4.30
├── rsonpath-lib v0.10.1 (/home/mat/src/rsonpath/crates/rsonpath-lib)
│   ├── cfg-if v1.0.4
│   ├── log v0.4.30
│   ├── memmap2 v0.9.10
│   │   └── libc v0.2.186
│   ├── rsonpath-syntax v0.4.2 (/home/mat/src/rsonpath/crates/rsonpath-syntax)
│   │   ├── nom v8.0.0
│   │   │   └── memchr v2.8.1
│   │   ├── owo-colors v4.3.0
│   │   ├── thiserror v2.0.18
│   │   │   └── thiserror-impl v2.0.18 (proc-macro)
│   │   │       ├── proc-macro2 v1.0.106 (*)
│   │   │       ├── quote v1.0.45 (*)
│   │   │       └── syn v2.0.117 (*)
│   │   └── unicode-width v0.2.2
│   ├── smallvec v1.15.1
│   ├── static_assertions v1.1.0
│   ├── thiserror v2.0.18 (*)
│   └── vector-map v1.1.0
├── rsonpath-syntax v0.4.2 (/home/mat/src/rsonpath/crates/rsonpath-syntax) (*)
└── simple_logger v5.2.0
    ├── colored v3.1.1
    │   └── windows-sys v0.61.2 (*)
    ├── log v0.4.30
    ├── time v0.3.47
    │   ├── deranged v0.5.8
    │   │   └── powerfmt v0.2.0
    │   ├── itoa v1.0.18
    │   ├── libc v0.2.186
    │   ├── num-conv v0.2.2
    │   ├── num_threads v0.1.7
    │   │   └── libc v0.2.186
    │   ├── powerfmt v0.2.0
    │   ├── time-core v0.1.8
    │   └── time-macros v0.2.27 (proc-macro)
    │       ├── num-conv v0.2.2
    │       └── time-core v0.1.8
    └── windows-sys v0.61.2 (*)
[build-dependencies]
├── rustflags v0.1.7
├── vergen v9.1.0
│   ├── anyhow v1.0.102
│   ├── cargo_metadata v0.23.1
│   │   ├── camino v1.2.2
│   │   │   └── serde_core v1.0.228
│   │   │       └── serde_derive v1.0.228 (proc-macro)
│   │   │           ├── proc-macro2 v1.0.106 (*)
│   │   │           ├── quote v1.0.45 (*)
│   │   │           └── syn v2.0.117 (*)
│   │   ├── cargo-platform v0.3.3
│   │   │   ├── serde v1.0.228
│   │   │   │   ├── serde_core v1.0.228 (*)
│   │   │   │   └── serde_derive v1.0.228 (proc-macro) (*)
│   │   │   └── serde_core v1.0.228 (*)
│   │   ├── semver v1.0.28
│   │   │   ├── serde v1.0.228 (*)
│   │   │   └── serde_core v1.0.228 (*)
│   │   ├── serde v1.0.228 (*)
│   │   ├── serde_json v1.0.150
│   │   │   ├── itoa v1.0.18
│   │   │   ├── memchr v2.8.1
│   │   │   ├── serde v1.0.228 (*)
│   │   │   ├── serde_core v1.0.228 (*)
│   │   │   └── zmij v1.0.21
│   │   └── thiserror v2.0.18 (*)
│   ├── derive_builder v0.20.2
│   │   └── derive_builder_macro v0.20.2 (proc-macro)
│   │       ├── derive_builder_core v0.20.2
│   │       │   ├── darling v0.20.11
│   │       │   │   ├── darling_core v0.20.11
│   │       │   │   │   ├── fnv v1.0.7
│   │       │   │   │   ├── ident_case v1.0.1
│   │       │   │   │   ├── proc-macro2 v1.0.106 (*)
│   │       │   │   │   ├── quote v1.0.45 (*)
│   │       │   │   │   ├── strsim v0.11.1
│   │       │   │   │   └── syn v2.0.117 (*)
│   │       │   │   └── darling_macro v0.20.11 (proc-macro)
│   │       │   │       ├── darling_core v0.20.11 (*)
│   │       │   │       ├── quote v1.0.45 (*)
│   │       │   │       └── syn v2.0.117 (*)
│   │       │   ├── proc-macro2 v1.0.106 (*)
│   │       │   ├── quote v1.0.45 (*)
│   │       │   └── syn v2.0.117 (*)
│   │       └── syn v2.0.117 (*)
│   ├── regex v1.12.3
│   │   ├── aho-corasick v1.1.4
│   │   │   └── memchr v2.8.1
│   │   ├── memchr v2.8.1
│   │   ├── regex-automata v0.4.14
│   │   │   ├── aho-corasick v1.1.4 (*)
│   │   │   ├── memchr v2.8.1
│   │   │   └── regex-syntax v0.8.10
│   │   └── regex-syntax v0.8.10
│   ├── rustc_version v0.4.1
│   │   └── semver v1.0.28 (*)
│   └── vergen-lib v9.1.0
│       ├── anyhow v1.0.102
│       └── derive_builder v0.20.2 (*)
│       [build-dependencies]
│       └── rustversion v1.0.22 (proc-macro)
│   [build-dependencies]
│   └── rustversion v1.0.22 (proc-macro)
└── vergen-git2 v9.1.0
    ├── anyhow v1.0.102
    ├── derive_builder v0.20.2 (*)
    ├── git2 v0.20.4
    │   ├── bitflags v2.11.1
    │   ├── libc v0.2.186
    │   ├── libgit2-sys v0.18.4+1.9.3
    │   │   ├── libc v0.2.186
    │   │   └── libz-sys v1.1.28
    │   │       └── libc v0.2.186
    │   │       [build-dependencies]
    │   │       ├── cc v1.2.62
    │   │       │   ├── find-msvc-tools v0.1.9
    │   │       │   ├── jobserver v0.1.34
    │   │       │   │   ├── getrandom v0.3.4
    │   │       │   │   │   ├── cfg-if v1.0.4
    │   │       │   │   │   ├── libc v0.2.186
    │   │       │   │   │   ├── r-efi v5.3.0
    │   │       │   │   │   └── wasip2 v1.0.3+wasi-0.2.9
    │   │       │   │   │       └── wit-bindgen v0.57.1
    │   │       │   │   └── libc v0.2.186
    │   │       │   ├── libc v0.2.186
    │   │       │   └── shlex v1.3.0
    │   │       ├── pkg-config v0.3.33
    │   │       └── vcpkg v0.2.15
    │   │   [build-dependencies]
    │   │   ├── cc v1.2.62 (*)
    │   │   └── pkg-config v0.3.33
    │   ├── log v0.4.30
    │   └── url v2.5.8
    │       ├── form_urlencoded v1.2.2
    │       │   └── percent-encoding v2.3.2
    │       ├── idna v1.1.0
    │       │   ├── idna_adapter v1.2.2
    │       │   │   ├── icu_normalizer v2.2.0
    │       │   │   │   ├── icu_collections v2.2.0
    │       │   │   │   │   ├── displaydoc v0.2.6 (proc-macro)
    │       │   │   │   │   │   ├── proc-macro2 v1.0.106 (*)
    │       │   │   │   │   │   ├── quote v1.0.45 (*)
    │       │   │   │   │   │   └── syn v2.0.117 (*)
    │       │   │   │   │   ├── potential_utf v0.1.5
    │       │   │   │   │   │   └── zerovec v0.11.6
    │       │   │   │   │   │       ├── yoke v0.8.2
    │       │   │   │   │   │       │   ├── stable_deref_trait v1.2.1
    │       │   │   │   │   │       │   ├── yoke-derive v0.8.2 (proc-macro)
    │       │   │   │   │   │       │   │   ├── proc-macro2 v1.0.106 (*)
    │       │   │   │   │   │       │   │   ├── quote v1.0.45 (*)
    │       │   │   │   │   │       │   │   ├── syn v2.0.117 (*)
    │       │   │   │   │   │       │   │   └── synstructure v0.13.2
    │       │   │   │   │   │       │   │       ├── proc-macro2 v1.0.106 (*)
    │       │   │   │   │   │       │   │       ├── quote v1.0.45 (*)
    │       │   │   │   │   │       │   │       └── syn v2.0.117 (*)
    │       │   │   │   │   │       │   └── zerofrom v0.1.8
    │       │   │   │   │   │       │       └── zerofrom-derive v0.1.7 (proc-macro)
    │       │   │   │   │   │       │           ├── proc-macro2 v1.0.106 (*)
    │       │   │   │   │   │       │           ├── quote v1.0.45 (*)
    │       │   │   │   │   │       │           ├── syn v2.0.117 (*)
    │       │   │   │   │   │       │           └── synstructure v0.13.2 (*)
    │       │   │   │   │   │       ├── zerofrom v0.1.8 (*)
    │       │   │   │   │   │       └── zerovec-derive v0.11.3 (proc-macro)
    │       │   │   │   │   │           ├── proc-macro2 v1.0.106 (*)
    │       │   │   │   │   │           ├── quote v1.0.45 (*)
    │       │   │   │   │   │           └── syn v2.0.117 (*)
    │       │   │   │   │   ├── utf8_iter v1.0.4
    │       │   │   │   │   ├── yoke v0.8.2 (*)
    │       │   │   │   │   ├── zerofrom v0.1.8 (*)
    │       │   │   │   │   └── zerovec v0.11.6 (*)
    │       │   │   │   ├── icu_normalizer_data v2.2.0
    │       │   │   │   ├── icu_provider v2.2.0
    │       │   │   │   │   ├── displaydoc v0.2.6 (proc-macro) (*)
    │       │   │   │   │   ├── icu_locale_core v2.2.0
    │       │   │   │   │   │   ├── displaydoc v0.2.6 (proc-macro) (*)
    │       │   │   │   │   │   ├── litemap v0.8.2
    │       │   │   │   │   │   ├── tinystr v0.8.3
    │       │   │   │   │   │   │   ├── displaydoc v0.2.6 (proc-macro) (*)
    │       │   │   │   │   │   │   └── zerovec v0.11.6 (*)
    │       │   │   │   │   │   ├── writeable v0.6.3
    │       │   │   │   │   │   └── zerovec v0.11.6 (*)
    │       │   │   │   │   ├── writeable v0.6.3
    │       │   │   │   │   ├── yoke v0.8.2 (*)
    │       │   │   │   │   ├── zerofrom v0.1.8 (*)
    │       │   │   │   │   ├── zerotrie v0.2.4
    │       │   │   │   │   │   ├── displaydoc v0.2.6 (proc-macro) (*)
    │       │   │   │   │   │   ├── yoke v0.8.2 (*)
    │       │   │   │   │   │   └── zerofrom v0.1.8 (*)
    │       │   │   │   │   └── zerovec v0.11.6 (*)
    │       │   │   │   ├── smallvec v1.15.1
    │       │   │   │   └── zerovec v0.11.6 (*)
    │       │   │   └── icu_properties v2.2.0
    │       │   │       ├── icu_collections v2.2.0 (*)
    │       │   │       ├── icu_locale_core v2.2.0 (*)
    │       │   │       ├── icu_properties_data v2.2.0
    │       │   │       ├── icu_provider v2.2.0 (*)
    │       │   │       ├── zerotrie v0.2.4 (*)
    │       │   │       └── zerovec v0.11.6 (*)
    │       │   ├── smallvec v1.15.1
    │       │   └── utf8_iter v1.0.4
    │       └── percent-encoding v2.3.2
    ├── time v0.3.47
    │   ├── deranged v0.5.8 (*)
    │   ├── itoa v1.0.18
    │   ├── libc v0.2.186
    │   ├── num-conv v0.2.2
    │   ├── num_threads v0.1.7 (*)
    │   ├── powerfmt v0.2.0
    │   └── time-core v0.1.8
    ├── vergen v9.1.0 (*)
    └── vergen-lib v9.1.0 (*)
    [build-dependencies]
    └── rustversion v1.0.22 (proc-macro)
```
<!-- rsonpath-full dependencies end -->
