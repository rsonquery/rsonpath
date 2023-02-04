[private]
default: (build-all "release")

# === INIT ===

# Initialize the repository for development.
init: check-cargo hooks-init checkout-benchmarks

# Check if cargo is installed and install it from rustup if not.
[private]
@check-cargo:
    cargo --version || \
      (echo "Installing rustup from https://sh.rustup.rs" && \
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
    . ${HOME}/.cargo/env
    rustup install stable
    rustup install nightly

# Initialize git hooks.
[private]
@hooks-init:
    echo "#!/bin/sh\n\njust hook-pre-commit" > ./.git/hooks/pre-commit
    echo "#!/bin/sh\n\njust hook-post-checkout" > ./.git/hooks/post-checkout
    chmod u+x ./.git/hooks/pre-commit
    chmod u+x ./.git/hooks/post-checkout

# Checkout and populate the benchmarks repository, excluding datasets.
[private]
checkout-benchmarks:
    git submodule init
    git submodule update

# === BUILD ===

alias b := build-bin

# alias for build-all release
build: (build-all "release")

# Build the rsonpath binary.
build-bin profile="dev": (build-lib profile)
    cargo build --bin rsonpath --profile {{profile}}

# Build the rsonpath-lib library.
build-lib profile="dev":
    cargo build --package rsonpath-lib --profile {{profile}}

# Build the rsonpath-benchmarks harness.
build-bench profile="dev": (build-lib profile)
    cargo build --package rsonpath-benchmarks --profile {{profile}}

# Build all rsonpath parts, the binary, library, and benches.
build-all profile="dev": (build-lib profile) (build-bin profile) (build-bench profile)

# Build and open the library documentation.
doc:
	RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --open --package rsonpath-lib

# === RUN ===

alias r := run-debug

# Run the CLI in debug profile. ARGS are passed to the rsonpath executable.
[no-exit-message]
run-debug *ARGS: (build-bin "dev")
    ./target/debug/rsonpath {{ARGS}}

# Run the CLI in release profile. ARGS are passed to the rsonpath executable.
[no-exit-message]
run *ARGS: (build-bin "release")
    ./target/release/rsonpath {{ARGS}}

# === TEST ===

# Run all tests.

alias t := test-unit
alias test := test-full
alias doctest := test-doc

# Run the quick unit tests of the library with all features.
test-unit:
    cargo rsontest --lib

# Run the main engine end-to-end tests on default features.
test-engine:
    cargo test --test engine_correctness_test

# Run all tests, including real dataset tests, on the feature powerset of the project.
test-full:
    -cargo install cargo-hack
    cargo rsontest

# Run doctests on the library.
test-doc:
    -cargo install cargo-hack
    cargo rsontest --doc

# === INSTALL ===

# Install the rsonpath binary from current source.
install: (build-bin "release")
    cargo install --path ./crates/rsonpath

# Uninstall the rsonpath binary.
uninstall:
	cargo uninstall rsonpath

# === VERIFICATION/LINTING ===

alias v := verify-quick
alias verify := verify-full

# Run all lints and checks required.
verify-full: build-all verify-clippy verify-doc verify-fmt test-full

# Run a quick formatting and compilation check.
verify-quick: verify-fmt verify-check

# Run cargo check on non-benchmark packages.
verify-check:
	cargo check --workspace --exclude rsonpath-benchmarks --all-features

# Run clippy lints on all packages.
verify-clippy: (build-all "release")
	cargo +nightly clippy --workspace --no-default-features --release -- --deny warnings
	cargo +nightly clippy --workspace --all-features --release -- --deny warnings

# Verify that documentation successfully builds for rsonpath-lib.
verify-doc: (build-bin "release")
	RUSTDOCFLAGS='-Dwarnings --cfg docsrs' cargo +nightly doc --package rsonpath-lib --no-default-features --no-deps
	RUSTDOCFLAGS='-Dwarnings --cfg docsrs' cargo +nightly doc --package rsonpath-lib --all-features --no-deps

# Verify formatting rules are not violated.
verify-fmt:
    cargo fmt -- --check

# === BENCHES ===

# Run *all* benches (very long!).
bench: (build-bench "release")
    cargo bench --package rsonpath-benchmarks

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

# === GIT ===

# Commit (add all first) both rsonpath and the benchmarks with a given message.
[no-exit-message]
commit msg:
    -cd ./crates/rsonpath-benchmarks && git add --all && git commit -m '{{msg}}'
    -git add --all && git commit -am '{{msg}}'

# === HOOKS ===

tmpdiff := `mktemp -t pre-commit-hook-diff-XXXXXXXX.$$`

[private]
hook-pre-commit: 
    just assert-benchmarks-committed
    git diff --full-index --binary > {{tmpdiff}}
    git stash -q --keep-index
    (just verify-fmt && just verify-check); \
    git apply --whitespace=nowarn < {{tmpdiff}} && git stash drop -q; rm {{tmpdiff}}

[private]
@hook-post-checkout: checkout-benchmarks

[private]
assert-benchmarks-committed:
    #!/bin/sh
    count=$(git diff HEAD ./crates/rsonpath-benchmarks | grep "^+Subproject commit [a-f0-9]*-dirty$" --count)
    if [ $count -ne 0 ]
    then
        echo "\033[31;1mCannot commit when rsonpath-benchmarks submodule is dirty, as this can lead to unexpected behaviour.
    First commit the changes in rsonpath-benchmarks by cd-ing into ./crates/rsonpath-benchmarks, or use just commit.\033[0"
        exit 1
    fi

# === RELEASE ===

# Perform release dry run for the given version.
release-dry ver:
    just release-patch {{ver}}
    just release-readme
    just commit 'release v{{ver}}'
    cargo release --sign-tag --sign-commit --exclude rsonpath-benchmarks

# Actually execute a release for the given version.
release-execute ver:
    just release-patch {{ver}}
    just release-readme
    just commit 'release v{{ver}}'
    cargo release --sign-tag --sign-commit --exclude rsonpath-benchmarks --execute

[private]
release-patch ver:
    #!/usr/bin/env nu
    let ver = "{{ver}}";
    let crates = ["rsonpath", "rsonpath-lib", "rsonpath-benchmarks"];
    $crates | each { |cr|
        let path = $"./crates/($cr)/Cargo.toml";
        sed -i $'s/^version = "[^"]*"/version = "($ver)"/;s/^rsonpath-lib = { version = "[^"]*"/rsonpath-lib = { version = "($ver)"/' $path;
    };

rsonpath-deps := `cargo tree --package rsonpath --edges normal --depth 1`
rsonpath-lib-deps := `cargo tree --package rsonpath-lib --edges normal --depth 1`
rsonpath-full-deps := `cargo tree --package rsonpath --edges normal`

[private]
release-readme:
    #!/usr/bin/env nu
    let params = [
        ["{{rsonpath-deps}}", "rsonpath"],
        ["{{rsonpath-lib-deps}}", "rsonpath-lib"],
        ["{{rsonpath-full-deps}}", "rsonpath-full"]
    ];
    $params | each {|x|
        let deps = ($x.0 | str replace '\n' '\n' --all | str replace '/' '\/' --all);
        sed -z -i $'s/<!-- ($x.1) dependencies start -->\n```ini\n.*```\n<!-- ($x.1) dependencies end -->/<!-- ($x.1) dependencies start -->\n```ini\n($deps)\n```\n<!-- ($x.1) dependencies end -->/' ./README.md
    };