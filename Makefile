RUSTC=rustc
CARGO=cargo

make: simdpath

simdpath: check_cargo
	RUSTC=$(RUSTC) $(CARGO) build --bin simdpath --release

.PHONY: check_cargo clean install uninstall

check_cargo:
	$(CARGO) --version || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)

clean:
	$(CARGO) clean

install: simdpath
	$(CARGO) install --path ./simdpath

uninstall:
	$(CARGO) uninstall simdpath