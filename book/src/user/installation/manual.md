# Manual build for maximum performance

The packaged installation methods are portable and the same executable can
be safely shared between different machines with the same basic architecture
(x86, ARM).

Building `rq` for a specific CPU makes it not portable, but creates code
explicitly optimized for the machine its built on, enabling better
performance.

## Building from source

Building from source requires your machine to have the rust tooling available.
We default to linking with `lld`, so you need that as well.

First, clone the
[`rsonpath` repository](https://github.com/V0ldek/rsonpath):

```bash
git clone https://github.com/V0ldek/rsonpath.git
```

Building and installing is done most easily with `just`:

```bash
just install-native
```

Without `just` one can use:

```bash
RUSTFLAGS="-C target-cpu=native" cargo install --path ./crates/rsonpath
```

## Building from `crates.io`

You can enable native CPU codegen when installing from `crates.io` as well,
by overriding `rustc` flags.

```bash
RUSTFLAGS="-C target-cpu=native" cargo install rsonpath
```

## Verifying native optimizations are enabled

To verify that your `rq` installation has native CPU support,
consult `rq --version` and look for `target-cpu=native` in the "Codegen flags"
field.

```console,ignore
$ rq --version
rq 0.9.1

Commit SHA:      05ced6146b2dcc4e474f2dbc17c2e6d0986a7181
Features:        default,simd
Opt level:       3
Target triple:   x86_64-unknown-linux-gnu
Codegen flags:   target-cpu=native,link-arg=-fuse-ld=lld
SIMD support:    avx2;fast_quotes;fast_popcnt
```
