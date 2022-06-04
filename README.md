# rsonpath

Library for fast execution of JSONPath queries.

## Build & test

Use the included `Makefile`. It will automatically install Rust for you using the `rustup` tool if it detects there is no Cargo in your environment.

```bash
make
make test
```

## Install

To install the `rsonpath` executable run:

```bash
make install
```

## Run benchmarks

Note: it is recommended to install `gnuplot` before generating reports.

This highly depends on the exact scenario you want to benchmark, so there is no `Makefile`. To run the stack-based vs stackless bench run:

```bash
cargo bench --bench rsonpath_stack_based_vs_stackless
```

If you want to bench the no-SIMD scenario, disable the default `simd_x86` feature flag:

```bash
cargo bench --bench rsonpath_stack_based_vs_stackless --no-default-features
```

For details about benchmarking refer to [Criterion.rs docs](https://github.com/bheisler/criterion.rs).
