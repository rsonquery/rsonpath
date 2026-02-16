# Installation

Currently, the easiest way to get it is from [latest GitHub release](https://github.com/V0ldek/rsonpath/releases/latest).
We have a binary for each Tier 1 Rust target and a few Tier 2 targets.

## Verifying provenance

All of our binary distributions implement [SLSA level 3](https://slsa.dev/spec/v1.0/).
What that means is that any official `rq` binary can be verified to have been
built from a specific version of `rsonpath` source with our official GitHub Release CI.
This is called **provenance**.

To verify provenance you should investigate the `multiple.intoto.jsonl` file available
in the [GitHub release](https://github.com/V0ldek/rsonpath/releases) (in the standard
[in-toto format](https://in-toto.io/)), using the [`slsa-verifier` tool](https://github.com/slsa-framework/slsa-verifier).

For example, to verify the `rq-x86_64-unknown-linux-gnu` binary for version v0.8.0, run:

```console,ignore
$ slsa-verifier verify-artifact \
$ --provenance-path ./multiple.intoto.jsonl \ # Path to the released provenance file.
$ --source-uri github.com/V0ldek/rsonpath \   # Our repository URL. This is case sensitive!
$ --source-versioned-tag v0.8.0 \             # Version tag of our release, in the format v#.#.#
$ ./rq-x86_64-unknown-linux-gnu               # Path to the binary to verify.
Verified signature against tlog entry index 34193532 at URL: https://rekor.sigstore.dev/api/v1/log/entries/24296fb24b8ad77a576a14ffb58e0477203bcd311b396b9a4c8c3cc66484053a451b67faf87c1542
Verified build using builder "https://github.com/slsa-framework/slsa-github-generator/.github/workflows/generator_generic_slsa3.yml@refs/tags/v1.9.0" at commit 5e6d505182213df857c2b1cb026abf79cf3b54df
Verifying artifact ./rq-x86_64-unknown-linux-gnu: PASSED

PASSED: Verified SLSA provenance
```

PASSED guarantees that this is a properly signed, untampered-with binary generated
from our repository at a given version tag. It can be safely ran on your system.
To verify it works, check if `rq` is available from your command line:

```console
$ rq -V
rq 0.10.0

```

## Package managers

When released, `rq` will be available as a package in more distribution,
but currently you can install it via `cargo`.

## Install with `cargo`

The `rq` binary is contained in the `rsonpath` crate.

```bash
cargo install rsonpath
```
