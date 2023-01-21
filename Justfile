[private]
default: build-bin

# === INIT ===

# Initialize the repository for development.
init: check-cargo hooks-init checkout-benchmarks

# Check if cargo is installed and install it from rustup if not.
[private]
check-cargo:
    @cargo --version || \
      (echo "Installing rustup from https://sh.rustup.rs" && \
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
    @. ${HOME}/.cargo/env
    rustup install stable
    rustup install nightly

# Initialize git hooks.
[private]
@hooks-init:
    echo "#!/bin/sh\n\njust hook-pre-commit" > ./.git/hooks/pre-commit

# Checkout and populate the benchmarks repository, excluding datasets.
[private]
checkout-benchmarks:
    git submodule init
    git submodule update

# === BUILD ===

# Build the rsonpath binary.
build-bin: build-lib
    cargo build --bin rsonpath --release

# Build the rsonpath-lib library.
build-lib: check-cargo 
    cargo build --package rsonpath-lib --release

# Build the rsonpath-benchmarks harness.
build-bench: build-lib
    cargo build --package rsonpath-benchmarks --release

# Build all rsonpath parts, the binary, library, and benches.
build-all: build-lib build-bin build-bench

# Build and open the library documentation.
doc: build-lib
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --package rsonpath-lib

# === TEST ===

# Run all tests.
test: build-bin
    cargo install cargo-hack
    cargo rsontest

# === INSTALL ===

# Install the rsonpath binary from current source.
install: build-bin
    cargo install --path ./rsonpath

# Uninstall the rsonpath binary.
uninstall: check-cargo 
	cargo uninstall rsonpath

# === VERIFICATION ===

# Run all lints and checks required.
verify: build-all verify-clippy verify-doc verify-fmt test

# Run clippy lints on all packages.
verify-clippy: build-all
	cargo +nightly clippy --workspace --no-default-features --release -- --deny warnings
	cargo +nightly clippy --workspace --all-features --release -- --deny warnings

# Verify that documentation successfully builds for rsonpath-lib.
verify-doc: build-bin
	RUSTDOCFLAGS='-Dwarnings --cfg docsrs' cargo +nightly doc --package rsonpath-lib --no-default-features --no-deps
	RUSTDOCFLAGS='-Dwarnings --cfg docsrs' cargo +nightly doc --package rsonpath-lib --all-features --no-deps

# Verify formatting rules are not violated.
verify-fmt: build-all
	cargo fmt -- --check

# === CLEAN ===

tmpdir := `mktemp -d -t criterion-reports-tmp-XXXXXXXX`

# Clean all build artifacts without deleting benchmark results.
clean: check-cargo
    -cp -r ./target/criterion/* {{tmpdir}}/
    cargo clean
    mkdir -p ./target/criterion
    -cp -r {{tmpdir}}/* ./target/criterion
    rm -rf {{tmpdir}}

# Delete benchmark results.
clean-benches:
	-rm -rf ./target/criterion/*

# Clean all artifacts, including benchmark results.
clean-all: clean clean-benches

# === HOOKS ===

[private]
@hook-pre-commit: verify-clippy verify-fmt

[private]
@hook-pre-push: assert-benchmarks-committed

dirty-submodules: `git diff HEAD ./crates/rsonpath-benchmarks | grep "^+Subproject commit [a-f0-9]*-dirty$" --count`

[private]
@assert-benchmarks-committed:
    if {{dirty-submodules}} != "0" {
        error("Cannot push")
    }
