CARGO=cargo

make: rsonpath

rsonpath: check_cargo rsonpath_lib
	$(CARGO) build --bin rsonpath --release

rsonpath_lib: check_cargo
	$(CARGO) build --package rsonpath-lib --release

rsonpath_bench: check_cargo rsonpath_lib
	$(CARGO) build --package rsonpath-benchmarks --release

.PHONY: all bench check_cargo clean clean_benches doc install test uninstall verify verify-clippy verify-doc verify-fmt

all: rsonpath_lib rsonpath rsonpath_bench

bench: rsonpath_lib
	$(CARGO) bench --config 'patch.crates-io.rsonpath-lib.path = "./crates/rsonpath-lib"'

# Check if cargo is present, if not, use rustup to setup.
check_cargo:
	$(CARGO) --version || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
	. ${HOME}/.cargo/env
	rustup install stable
	rustup install nightly

# Handle the criterion reports directory separately to avoid losing previous benches.
clean:
	$(eval TMPDIR := $(shell mktemp -d -t criterion-reports-tmp-XXXXXXXX))
	-cp -r ./target/criterion/* $(TMPDIR)/
	$(CARGO) clean
	mkdir -p ./target/criterion
	-cp -r $(TMPDIR)/* ./target/criterion
	rm -rf $(TMPDIR)

clean_benches:
	rm -rf ./target/criterion/*

doc: rsonpath_lib
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --package rsonpath-lib

install: rsonpath
	$(CARGO) install --path ./rsonpath

test: rsonpath_lib
	cargo install cargo-hack
	$(CARGO) rsontest

uninstall:
	$(CARGO) uninstall rsonpath

verify: rsonpath verify-clippy verify-doc verify-fmt test

verify-clippy: rsonpath
	cargo +nightly clippy --workspace --exclude rsonpath-benchmarks --no-default-features --release -- --deny warnings
	cargo +nightly clippy --workspace --exclude rsonpath-benchmarks --all-features --release -- --deny warnings

verify-doc: rsonpath
	RUSTDOCFLAGS='-Dwarnings --cfg docsrs' cargo +nightly doc --package rsonpath-lib --no-default-features --no-deps
	RUSTDOCFLAGS='-Dwarnings --cfg docsrs' cargo +nightly doc --package rsonpath-lib --all-features --no-deps

verify-fmt:
	cargo fmt --package rsonpath rsonpath-lib -- --check