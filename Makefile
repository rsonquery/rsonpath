CARGO=cargo

make: simdpath

simdpath: check_cargo
	$(CARGO) build --bin simdpath --release

.PHONY: check_cargo clean doc install uninstall test

check_cargo:
	$(CARGO) --version || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)

clean:
	$(eval TMPDIR := $(shell mktemp -d -t criterion-reports-tmp-XXXXXXXX))
	cp -r ./target/criterion/* $(TMPDIR)/
	$(CARGO) clean
	mkdir -p ./target/criterion
	cp -r $(TMPDIR)/* ./target/criterion
	rm -rf $(TMPDIR)

doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open

install: simdpath
	$(CARGO) install --path ./simdpath

test:
	cd simdpath
	$(CARGO) test --features nosimd && \
	$(CARGO) test
	

uninstall:
	$(CARGO) uninstall simdpath