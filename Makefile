CARGO=cargo

make: simdpath

simdpath: check_cargo
	$(CARGO) build --bin simdpath --release

.PHONY: bench check_cargo clean clean_benches doc install uninstall test

bench: simdpath
	$(CARGO) bench --bench simdpath_stack_based_vs_stackless

# Check if cargo is present, if not, use rustup to setup.
check_cargo:
	$(CARGO) --version || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)

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

doc: simdpath
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open

install: simdpath
	$(CARGO) install --path ./simdpath

test: simdpath
	cd simdpath
	$(CARGO) testall

uninstall:
	$(CARGO) uninstall simdpath