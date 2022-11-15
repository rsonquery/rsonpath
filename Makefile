CARGO=cargo

make: rsonpath

rsonpath: rsonpath_lib check_cargo
	$(CARGO) build --bin rsonpath --release

rsonpath_lib: check_cargo
	$(CARGO) build --package rsonpath-lib --release

.PHONY: bench check_cargo clean clean_benches doc install uninstall test

bench: rsonpath_lib
	$(CARGO) bench --config 'patch.crates-io.rsonpath-lib.path = "./crates/rsonpath-lib"'

# Check if cargo is present, if not, use rustup to setup.
check_cargo:
	$(CARGO) --version || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
	. ${HOME}/.cargo/env

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
