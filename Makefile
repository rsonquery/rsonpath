CARGO=cargo

make: simdpath

simdpath: check_cargo
	$(CARGO) build --bin simdpath --release

.PHONY: check_cargo clean install uninstall test

check_cargo:
	$(CARGO) --version || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)

clean:
	$(eval TMPDIR := $(shell mktemp -d -t criterion-reports-tmp-XXXXXXXX))
	cp -r ./target/criterion/* $(TMPDIR)/
	$(CARGO) clean
	mkdir -p ./target/criterion
	cp -r $(TMPDIR)/* ./target/criterion
	rm -rf $(TMPDIR)

install: simdpath
	$(CARGO) install --path ./simdpath

test:
	cd simdpath
	$(CARGO) test --manifest-path ./simdpath/Cargo.toml && \
	$(CARGO) test --manifest-path ./simdpath/Cargo.toml --features nosimd
	

uninstall:
	$(CARGO) uninstall simdpath