# Simdpath

Library for fast execution of JSONPath queries.

## Build & test

Use the included `Makefile`. It will autoinstall Rust for you using the `rustup` tool if it detects there is no Cargo in your environment.

```
make
make test
```

## Install

To install the `simdpath` executable run:

```
make install
```

## Run benchmarks

Note: it is recommended to install `gnuplot` before generating reports.

This highly depends on the exact scenario you want to benchmark, so there is no `Makefile`. To run the stack-based vs stackless bench run:

```
cargo bench --bench simdpath_stack_based_vs_stackless
```

If you want to bench the no-SIMD scenario add the `nosimd` feature flag:

```
cargo bench --bench simdpath_stack_based_vs_stackless --features nosimd
```

For details about benchmarking refer to [Criterion.rs docs](https://github.com/bheisler/criterion.rs).
