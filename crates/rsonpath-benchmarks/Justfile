[private]
default: build-bench

# === BUILD ===

alias b := build-bench

# Build the rsonpath-benchmarks harness.
build-bench:
    cargo build --package rsonpath-benchmarks --profile release

# === VERIFICATION/LINTING ===

alias v := verify-quick
alias verify := verify-full

# Run all lints and checks required.
verify-full: build-bench verify-clippy verify-fmt

# Run a quick formatting and compilation check.
verify-quick: verify-fmt verify-check

# Run cargo check on non-benchmark packages.
verify-check:
	cargo check --all-features

# Run clippy lints on all packages.
verify-clippy: (build-bench)
	cargo +nightly clippy --no-default-features --release -- --deny warnings
	cargo +nightly clippy --all-features --release -- --deny warnings

# Verify formatting rules are not violated.
verify-fmt:
    cargo fmt -- --check

# === BENCHES ===

# Run *all* benches (very long!).
bench-all: (build-bench)
    cargo bench --package rsonpath-benchmarks

# Run a given bench target.
bench target="main": (build-bench)
    cargo bench --package rsonpath-benchmarks --bench {{target}}

# === CLEAN ===

tmpdir := `mktemp -d -t criterion-reports-tmp-XXXXXXXX`

# Clean all build artifacts without deleting benchmark results.
clean:
    -cp -r ./target/criterion/* {{tmpdir}}/
    cargo clean
    mkdir -p ./target/criterion
    -cp -r {{tmpdir}}/* ./target/criterion
    rm -rf {{tmpdir}}

# Delete benchmark results.
clean-benches:
	-rm -rf ./target/criterion/*

# Clean all artifacts, including benchmark results.
clean-all:
    cargo clean