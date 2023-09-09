# Installation

When released, `rq` will be available as a package in many distributions.
Currently the only way to install it is via `cargo` or manual build.

## Install with `cargo`

The `rq` binary is contained in the `rsonpath` crate.

```bash
cargo install rsonpath
```

If installation fails with the following error:

```ini
Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.
```

then your CPU is not supported by our SIMD acceleration features.
You can install `rq` without acceleration:

```bash
cargo install rsonpath --no-default-features -F default-optimizations
```

This will greatly inhibit its performance, but the functional feature set
will be the same.

## Verify

To verify installation, check if `rq` is available from your command line:

```console
$ rq -V
rq 0.7.1

```
