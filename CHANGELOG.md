# Changelog

All notable changes to this project will be documented in this file.

## [0.10.0] - 2025-02-16

### Features

- Support for AVX512 SIMD.
  - available on x86-64
- Support for Neon SIMD.
  - available on aarch64
- MSRV bumped to 1.89.
  - stable AVX512 since 1.89

- Revised the list of Tier 1 targets that are continuously built and released.
  - The following targets are now in CI and release:
    - `aarch64-apple-darwin`
    - `aarch64-pc-windows-msvc`
  - The following targets are no longer built or released:
    - `i686-pc-windows-gnu` 
    - `x86_64-apple-darwin`
- Added prebuilt binaries for a few MUSL-based Tier 2 targets.
  - Binaries now released for:
    - `aarch64-unknown-linux-musl`
    - `i686-unknown-linux-musl`
    - `x86_64-unknown-linux-musl`

### Bug Fixes

- Skipping inside arrays on comma-atomic ([#757](https://github.com/V0ldek/rsonpath/issues/757), [#751](https://github.com/V0ldek/rsonpath/issues/751))
  - Tail skipping was not triggered when the item matching the unitary
    transition was an atomic value inside a list. For example,
    selecting `$[0]` from a long list of integers would never skip,
    massively degrading performance.
  - Skipping was added to `handle_comma` in the same vein as it was in
    `handle_colon` to enable this.

- Fix panic in specific cases of reclassification at end of file ([#788](https://github.com/V0ldek/rsonpath/issues/788))
  - A particular combination of reclassification after tail-skipping
    at the very end of the file could cause a panic if the file-ending
    closing occurred directly after the skipped-to character.

### Reliability

- Use github hosted ARM runners. ([#718](https://github.com/V0ldek/rsonpath/issues/718))

- ARM SIMD is now tested in CI.

- Fix serde proptests. ([#742](https://github.com/V0ldek/rsonpath/issues/742))
  - Proptests in automaton serde were not properly guarding for
    arbitrary generated queries being too complex and exceeding the
    automaton size limit.

- Add CodeQl for Rust scanning.
  - We now require the analysis to pass before PR merges.

- All fuzzers now correctly run on a nightly basis (#868). ([#868](https://github.com/V0ldek/rsonpath/issues/868))


### Dependencies

- Update `cfg-if` from 1.0.0 to 1.0.4
- Update `clap` from 4.5.23 to 4.5.58
- Update `color-eyre` from 0.6.3 to 0.6.5
- Update `log` from 0.4.22 to 0.4.29
- Update `memmap2` from 0.9.5 to 0.9.9
- Update `rustflags` from 0.1.6 to 0.1.7
- Update `serde` from 1.0.217 to 1.0.228
- Update `simple_logger` from 5.0.0 to 5.1.0
- Update `smallvec` from 1.13.2 to 1.15.1
- Update `thiserror` from 2.0.9 to 2.0.18
- Update `vector-map` from 1.0.1 to 1.0.2
- Update `vergen` from 9.0.2 to 9.1.0
- Update `vergen-git2` from 1.0.2 to 9.1.0
- Remove `vergen-gitcl` from build dependencies

### Documentation

- Fix old link to rsonbook in readme.
- Add a strict no-LLM policy to CONTRIBUTING.

## [0.9.4] - 2024-12-31

### Library

- Serde support for `MainEngine`.
  - Implemented `serde::Serialize` and `serde::Deserialize` for `MainEngine` in rsonpath-lib,
    The `serde` dependency is guarded behind the optional `serde` feature.
  - The serialization format is not stable, as the `Automaton`
    is expected to evolve. Thus, serialization includes a version
    and deserialization will fail if the version disagrees.
  - Also added snapshot tests for serialization based on `insta`.
- Added the `MainEngine::automaton` function to retrieve a reference to the compiled query.
- Removed the `arbitrary` feature from `rsonpath-lib` as it didn't actually do anything anymore.

### Dependencies

- `serde` (1.0.217) is now an optional dependency for `rsonpath` and `rsonpath-lib`

## [0.9.3] - 2024-12-24

### Library

- Made `MainEngine` `Send` and `Sync`
  - Changed internal `Rc`s to `Arc`s in the automaton labels.
  - Added a `static_assert` to make sure `MainEngine` is `Send+Sync` forever.
- Added a `Debug` impl for `MainEngine`.

## [0.9.2] - 2024-12-22

### Library

- [**breaking**] Added `StringPattern` and made `Automaton` no longer borrow the query. ([#117](https://github.com/V0ldek/rsonpath/issues/117)[#613](https://github.com/V0ldek/rsonpath/issues/613))

  - The `Automaton` struct borrowed the source query, which also caused the Engine to carry the query's lifetime with it.
    The actual data being borrowed were the `JsonString` values for member transitions.
    In preparation for [#117](https://github.com/V0ldek/rsonpath/issues/117)we remove the borrowed `JsonString` and replace it
    with `StringPattern`. For UTF-8 the `StringPattern` will be a more complex struct that precomputes some stuff for efficient matching later.
    For now, it's a thin wrapper over a `JsonString`.
  - During construction we may create many transitions over the same pattern.
    To reduce the size of the automaton we cache the patterns and put them into an `Rc`.
    This may get optimised later to instead use some kind of inline storage, but it's unlike to actually matter.
    I ran the benchmarks and saw no measurable difference between the previous version and this one.
  - This is a breaking API change -- the `MainEngine` is now lifetimeless and the `Compiler` trait requires the
    returned engine to be lifetimeless.

### Dependencies

- Bump arbitrary from 1.3.1 to 1.4.1
- Bump clap from 4.5.2 to 4.5.23
- Bump color-eyre from 0.6.2 to 0.6.3
- Bump log from 0.4.21 to 0.4.22
- Bump memmap2 from 0.9.4 to 0.9.5
- Bump simple_logger from 4.3.3 to 5.0.0
- Bump smallvec from 1.13.1 to 1.13.2
- Bump thiserror from 1.0.58 to 2.0.9 (#617). ([#617](https://github.com/V0ldek/rsonpath/issues/617))
- Remove `nom` as a direct dependency of `rsonpath-lib`

## [0.9.1] - 2024-04-03

### Bug Fixes

- Child slice selectors only selecting first matching index (#499). ([#499](https://github.com/V0ldek/rsonpath/issues/499))
  - Fixed a bug where the compiler would erroneously mark states
    with a single slice transition as unitary, even though such
    transitions could match more than one index.

## [0.9.0] - 2024-03-28

### Features

- Array slice selector. ([#152](https://github.com/V0ldek/rsonpath/issues/152))
  - Simple slicing: forward step and positive bounds.
  Includes an overhaul to how array transitions are compiled.

### Performance

- Improve performance of the index selector. ([#138](https://github.com/V0ldek/rsonpath/issues/138))
  - Added more structure and metadata to the automaton,
    improving perf of all queries in general (~6% thpt) and
    array-index queries in particular (~12% thpt).

### Reliability

- Run JSONPath Compliance Test Suite on basic queries.
  - CTS is now run in CI on queries that the engine supports.

### Dependencies

- Bump clap from 4.5.1 to 4.5.2
- Bump thiserror from 1.0.57 to 1.0.58

## [0.8.7] - 2024-02-29

### Features

- [**breaking**] Parsing filter expressions. ([#154](https://github.com/V0ldek/rsonpath/issues/154))
  - This is mainly an `rsonpath-syntax` change &ndash; the selectors are parsed,
    but `rq` will give you an unsupported error and a link to [#154](https://github.com/V0ldek/rsonpath/issues/154)
    if you put them in a query.
  
### Reliability

- Add msrv verify as ci check. ([#480](https://github.com/V0ldek/rsonpath/issues/480))
  - The MSRV got unknowingly bumped before, with this CI check we will avoid it in the future.

### Dependencies

- Bump clap from 4.4.16 to 4.5.1
- Bump eyre from 0.6.11 to 0.6.12
- Bump log from 0.4.20 to 0.4.21
- Bump memmap2 from 0.9.3 to 0.9.4
- Bump smallvec from 1.12.0 to 1.13.1
- Bump thiserror from 1.0.56 to 1.0.57
- Bump vergen from 8.2.7 to 8.3.1

## [0.8.6] - 2024-01-15

### Features

- [**breaking**] Parsing `Slice` selectors.
  - This is mainly an `rsonpath-syntax` change &ndash; the selectors are parsed,
    but `rq` will give you an unsupported error and a link to [#152](https://github.com/V0ldek/rsonpath/issues/152)
    if you put them in a query.

### Bug Fixes

- Bug in `-c` graph display.
  - dot format was temporarily broken by doubling double quotes in labels

- U+001A-U+001F in name selectors.
  - Characters U+001A through U+001F were erroneously accepted unescaped.
    This is now a hard error.

### Dependencies

- Bump clap from 4.4.14 to 4.4.16
- Bump vergen from 8.2.6 to 8.2.7.

## [0.8.5] - 2024-01-10

### Features

- [**breaking**] Separate `rsonpath-syntax`.
  - The parsing logic and query AST are now moved to a separately published subcrate.
  - The crate is versioned separately. Changes to it that do not affect `rq` will be documented
    in its separate changelog. See the `crates/rsonpath-syntax` subdirectory.
- [**breaking**] Rework numeric types in the query parser.
  - renamed `NonNegativeArrayIndex` to `JsonUInt`
  - added the `JsonInt` and `JsonNonZeroUInt` types
- Fancy error handling in the parser.

### Reliability

- Use self-hosted runner for ARM.
  - We now have a self-hosted runner to continuously test rsonpath on ARM64!
- Set restrictive egress rules on runners.
  - Following up on StepSecurity upgrades, runners now block egress
  traffic by default and allow only specific trusted endpoints.

### Dependencies

- Bump arbitrary from 1.3.0 to 1.3.2.
- Bump clap from 4.4.7 to 4.4.14.
- Bump eyre from 0.6.8 to 0.6.11.
- Bump memmap2 from 0.9.0 to 0..3.
- Bump simple_logger from 4.2.0 to 4.3.3.
- Bump smallvec from 1.11.1 to 1.11.2.
- Bump thiserror from 1.0.49 to 1.0.56.
- Bump vergen from 8.2.5 to 8.2.6.

## [0.8.4] - 2023-10-30

### Features

- [**breaking**] Refactor the `Input` implementors with automatic padding (#[276](https://github.com/V0ldek/rsonpath/issues/276)).
  - Padding and alignment is now handled automatically by the input types,
    allowing them to work safely without copying the entire input. The overhead is now
    limited to the padding, which is at most 256 bytes in total.
  - [`BorrowedBytes`](https://docs.rs/rsonpath-lib/0.8.4/rsonpath/input/borrowed/struct.BorrowedBytes.html) is now safe to construct.
  - [`OwnedBytes`](https://docs.rs/rsonpath-lib/0.8.4/rsonpath/input/owned/struct.OwnedBytes.html) no longer copies
    the entire source on construction.
  
### Bug Fixes

- Atomic values getting invalid spans (#327). ([#327](https://github.com/V0ldek/rsonpath/issues/327))
  - Fixed an issue where atomic values would be matched with all
    trailing characters up until the next closing.

### Performance

- Improve SIMD codegen.
  - Improved the way we dispatch to SIMD-intensive functions.
    This results in slightly larger binaries, but *massive* speedups &ndash;
    throughput increase of 5, 10, 20, or in case of `google_map::travel_modes/rsonpath_direct_count`
    59 (fifty-nine) percent.

### Reliability

- Harden GitHub Actions.
  - We now use the StepSecurity [harden-runner](https://github.com/step-security/harden-runner) in audit mode
    to test a more secure approach to GitHub CI.
- End to end test refactor.
  - tests are now generated into many separate files instead of one gigantic file.
    This improves compilation times, responsiveness of rust-analyzer,
    and in general makes the tooling happier.

### Dependencies

- Bump arbitrary from 1.3.0 to 1.3.2.
- Bump clap from 4.4.6 to 4.4.7.
- Bump thiserror from 1.0.49 to 1.0.50.

## [0.8.3] - 2023-10-04

### Bug Fixes

- Missing openings from node results. ([#297](https://github.com/V0ldek/rsonpath/issues/297))
  - Fixed an issue where the opening
  characters of matched nodes would not be
  included in the result when head-skipping
  and the opening happened on a block boundary.

- Lib MSRV.
  - In v0.8.0 we inadvertently broke the MSRV,
  and the project only built with 1.71.1
  It was restored to 1.70.0 for the binary
  and 1.67.1 for the lib.

### Dependencies

- Bump clap from 4.4.4 to 4.4.6.
- Bump memmap2 from 0.7.1 to 0.9.0.
- Bump thiserror from 1.0.48 to 1.0.49.

## [0.8.2] - 2023-09-23

### Performance

- Improved handling of the root-only query `$`. ([#160](https://github.com/V0ldek/rsonpath/issues/160))
  - Full nodes result when asking for root: 2 times throughput increase.
  - Indices/count result when asking for root: basically unboundedly faster,
    no longer looks at the entire document.

### Documentation

- Clarified that the `approximate_spans` guarantees.
  - Now documentation mentions that the returned `MatchSpan`s can potentially
    have their end indices farther than one would expect the input to logically end,
    due to internal padding.

### Bug fixes

- Fixed handling of the root-only query `$` on atomic documents. ([#160](https://github.com/V0ldek/rsonpath/issues/160))
  - Previously only object and array roots were supported.
- Fixed a bug when head-skipping to a single-byte key would panic. ([#281](https://github.com/V0ldek/rsonpath/issues/281))
  - This was detected by fuzzing!
  - The queries `$..["{"]` and `$..["["]` would panic
    on inputs starting with the bytes `{"` or `["`, respectively.
- Fixed a bug where disabling the `simd` feature would not actually
  disable SIMD acceleration.

### Reliability

- Made the ClusterFuzzLite batch workflow automatically create an issue
  on failure to make sure the maintainers are notified.

## [0.8.1] - 2023-09-20

### Features

- [**breaking**] Refactored the [`Match`]/[`MatchSpan`] types.
  - [`Match`] now takes 32 bytes, down from 40.
  - All fields are now private, accessible via associated functions.
  - Added the `len` function to [`MatchSpan`].
- Added `approximate_spans` result mode. ([#242](https://github.com/V0ldek/rsonpath/issues/242))
  - Engine can return an approximate span of the match,
    where "approximate" means the start index is correct,
    but the end index might include trailing whitespace after the match.
  - This mode is much faster that full `matches`, close to the performance
    of `count`, especially for large result sets.
  - This is a library-only feature.
- Library exposes a new optional feature, `arbitrary`.
  - When enabled, includes [`arbitrary`](https://lib.rs/crates/arbitrary)
    as a dependency and provides an `Arbitrary` impl for `JsonPathQuery`,
    `JsonString`, and `NonNegativeArrayIndex`.

### Bug fixes

- Fixed a bug when memmem acceleration would fail for empty keys.
  - This was detected by fuzzing! The query `$..[""]` would panic
    on certain inputs due to invalid indexing.
- Fixed a panic when parsing invalid queries with wide UTF8 characters.
  - This was detected by fuzzing! Parsing a query with invalid syntax
    caused by a longer-than-byte UTF-8 character would panic when
    the error handler tried to resume parsing from the next *byte*
    instead of respecting char boundaries.
- Fixed a panic caused by node results in invalid JSON documents.
  - This was detected by fuzzing! Invalid JSON documents could
    cause the NodeRecorder to panic if the apparent match span
    was of length 1.
- Fixed erroneous match span end reporting. ([#247](https://github.com/V0ldek/rsonpath/issues/247))
  - Fixed a bug where `MatchSpan` values given by the engine were
    almost always invalid.

### Reliability

- Fuzzing integration with libfuzzer and ClusterFuzzLite.
  - [`cargo-fuzz`](https://lib.rs/crates/cargo-fuzz) can be used
    to fuzz the project with libfuzzer. Currently we have three fuzzing targets,
    one for stressing the query parser, one for stressing the engine with arbitrary
    bytes, and one stressing the engine with structure-aware queries and JSONs.
  - Fuzzing is now enabled on every PR. Using ClusterFuzzLite
    we will fuzz the project every day on a cron schedule
    to establish a corpus.
- Added correctness tests for match spans reporting ([#247](https://github.com/V0ldek/rsonpath/issues/247))

### Dependencies

- Bump clap from 4.4.2 to 4.4.4.
- Bump vergen from 8.2.4 to 8.2.5.

## [0.8.0] - 2023-09-10

### Features

- Portable binaries. ([#231](https://github.com/V0ldek/rsonpath/issues/231))
  - SIMD capabilities are now discovered at runtime,
  allowing us to distribute one binary per target.
  - Requirements for SIMD are now more granular,
  allowing weaker CPUs to still get some of the acceleration:
    - Base SIMD is either SSE2, SSSE3, or AVX2.
    - Structural classification works on SSSE3 and above.
    - Quote classification works if `pclmulqdq` is available.
    - Depth classification works if `popcnt` is available.
  - To counteract the increased binary size debug info is no longer
  included in distributed binaries.
  - Codegen for distributed binaries is improved with fat LTO and setting
  codegen units to 1.
  - SIMD capabilities are listed with `rq --version`.

### Reliability

- Change clippy to auguwu/clippy-action
  - The "official" action was not maintained for 3 years now.
    This one is actively maintained (thanks Noel!).

## [0.7.1] - 2023-09-09

### Bug Fixes

- Panic when head-skipping block boundary. ([#249](https://github.com/V0ldek/rsonpath/issues/249))
  - Fixed an issue when head-skipping acceleration in nodes result mode would
  panic in very specific input circumstances, or if the input had really long JSON keys.

### Dependencies

- Bump thiserror from 1.0.47 to 1.0.48.

## [0.7.0] - 2023-09-02

### Features

- Added 32-bit and SSSE3 SIMD support.
  - Refactored all SIMD code to enable modularity and more target feature types.
  - Building for x86 now chooses one of four SIMD implementations:
    - AVX2 64-bit
    - AVX2 32-bit
    - SSSE3 64-bit
    - SSSE3 32-bit
  - These are also now distributed as separate binaries.

### Reliability

- Fine-grained action permissions.
  - Actions now use explicit, lowest possible permissions for all jobs.
- Add SLSA3 provenance to the release pipeline.
  - Future releases will include cryptographically signed provenance for all binaries.
    See: <https://slsa.dev/spec/v1.0/about>

- [StepSecurity](https://www.stepsecurity.io/) Apply security best practices.
  - All CI uses hash-pinned dependencies now.
  - Run the [OSSF Scorecard check](https://github.com/ossf/scorecard) on each PR.
  - Add Dependency review.
- Removed test-codegen deps from `Cargo.lock`
  - By removing the codegen crate from the workspace their deps
    are now separated and don't pollute the lock of the actual end product.
- `cargo-deny` now runs with the CI to keep tabs on our deps.
  - Configured to reject Medium+ CVEs and non-compatible licenses.

### Dependencies

- Bump clap from 4.3.19 to 4.4.2.
- Bump log from 0.4.19 to 0.4.20.
- Bump thiserror from 1.0.44 to 1.0.47.
- Bump trycmd from 0.14.16 to 0.14.17.
- Removed `memchr` as a dependency.
  - It was no longer needed after the custom `memmem` classifier
    introduced in v0.6.0.
- Removed `replace_with` as a dependency.
  - That code path was refactored earlier, dep was now unused.

### Documentation

- Added the OpenSSF badge.
  - We will be trying to achieve the Passing level before v1.0.0.
- Added the scorecard badge.

## [0.6.1] - 2023-08-07

### Features

- [**breaking**] Remove the `unique-members`` feature.
  - This clutters the API more than anything.
If supporting duplicate keys is required in the future,
it can be easily added as a `const` config option,
not a compilation feature.

- Add the `--json` CLI option for passing JSONs inline.

### Reliability

- Added snapshot tests for `rq` using [`trycmd`](https://crates.io/crates/trycmd).
  - This is another layer of E2E tests, makes sure documentation examples
  in the book are correct, and that our `--help` and `--version` outputs
  remain consistent.

### Documentation

- [We have a book!](https://v0ldek.github.io/rsonpath/)
  - The first part is a usage guide for `rq`, and contains a short
    JSONPath reference.
  - Other parts will follow, with a plan to finalize at least the library
    usage guide before 1.0.0.

## [0.6.0] - 2023-08-02

### Features

- [**breaking**] Full match result mode. ([#56](https://github.com/V0ldek/rsonpath/issues/56))
This includes a revamp of all the internals that would be too long to describe in the log.
In short:
  - `memmem` was rewritten to a custom implementation (courtesy of @charles-paperman)
  - Each of the result modes has a separate `Recorder` that takes care of producing the results
  - The results are written to a `Sink`, provided by the user; this might be a `Vec`, the stdout,
    or some other `io::Write` implementation.
  - Matches contain the full byte span of the value matched.
  - A lot of `Input` and classifier APIs have massive breaking changes to accomodate this.

- [**breaking**] Removed the Recursive engine.
  - The Recursive implementation has outlived its usefulness.
Over time it became a near-duplicate of Main,
which was manifested by a need to implement
the same features twice with the exact same code
and to refactor/fix bugs with exact same code changes
but in two different files. We will focus efforts on the Main engine.
The `--engine` CLI option was disabled, as there is only one engine now.

### Reliability

- Qol improvement by separate test gen crate.
  - This removes the confusing `gen-tests` feature from lib,
reduces its build dependencies, should improve
build times.

### Dependencies

- Bump clap from 4.3.10 to 4.3.19.
- Bump colored (dependency of simple_logger) from 2.0.0 to 2.0.4.
  - This removes a transitive dependency on atty with a CVE.
- Bump rustflags from 0.1.3 to 0.1.4.
- Bump smallvec from 1.10.0 to 1.11.0.
- Bump thiserror from 1.0.40 to 1.0.44.

## [0.5.1] - 2023-07-03

### Features

- Consistent index result output. ([#161](https://github.com/V0ldek/rsonpath/issues/161))
  - The `--result bytes` mode now consistently reports the first byte of the value it matched. This can be used to extract the actual value from the JSON by parsing from the reported byte.

### Bug Fixes

- Remove SHA from --version on crates.io. ([#157](https://github.com/V0ldek/rsonpath/issues/157))
  - The Commit SHA part was incorrect, and there seems to be no way to get it when the crate is in registry

### Library

- [**breaking**] Remove `tail-skip` and `head-skip` features.
  - These are now non-optional and integrated into the engines.

### Reliability

- Generate strings in classifier tests. ([#173](https://github.com/V0ldek/rsonpath/issues/173), [#20](https://github.com/V0ldek/rsonpath/issues/20))
  - Improve classifier correctness tests by including quoted strings with escapes
in the generated proptest cases.

- More tests for wildcard compilation.
  - Added more cases for compiling the NFA and minimizing
for queries with wildcards.

- Automated declarative end-to-end engine tests. ([#134](https://github.com/V0ldek/rsonpath/issues/134))
  - Engine tests were rewritten to use declarative TOML configurations
for ease of creating new tests, maintenance and debugging ease.
Test coverage was increased, since compressed variants of inputs are
automatically generated and tested, and we now test all combinations
of input-engine-result types.

### Dependencies

- Bump clap from 4.3.4 to 4.3.10.
- Bump memmap2 from 0.7.0 to 0.7.1.
- Bump vergen from v8.2.1 to v8.2.3

### Documentation

- Rearrange readme to put usage first.
- Update bug report issue form.
  - Changed the issue form to be more streamlined and use more polite language.
- Add MSRV to README.

## [0.5.0] - 2023-06-14

### Features

- Parser support for array index selector. ([#60](https://github.com/V0ldek/rsonpath/issues/60))
  - Parser now recognizes the array index selector with positive index values conforming to the I-JSON specification.
- Index selector engine support (#132). ([#132](https://github.com/V0ldek/rsonpath/issues/132)[#132](https://github.com/V0ldek/rsonpath/issues/132))
  - The automaton transition model has been changed to incorporate index-labelled transitions.
- Both engines now support queries with the index selector.
- New `Input` API. ([#23](https://github.com/V0ldek/rsonpath/issues/23)[#23](https://github.com/V0ldek/rsonpath/issues/23))
  - A more abstract API to access the underlying byte stream replacing the reliance of the engines on a direct `&[u8]` slice access, to allow adding buffered input streams (#23) in the future. Two types were added, `OwnedBytes` and `BorrowedBytes`, to support the current easy scenario of having the bytes already in memory.
- Rename bin to `rq` and lib to `rsonpath`.
- Add long version to CLI.
- Mmap support. ([#23](https://github.com/V0ldek/rsonpath/issues/23))
  - Added `MmapInput` which maps a file into memory on unix and windows.
- The CLI app now automatically decides which input to use, favoring mmap in most cases. This can be overridden with `--force-input`.

### Library

- Rename `Label` to `JsonString` (#139). ([#139](https://github.com/V0ldek/rsonpath/issues/139)[#131](https://github.com/V0ldek/rsonpath/issues/131))
  - `query::Label` is now `query::JsonString`
- The `unique-labels` feature is now `unique-members`
- `EngineError:MalformedLabelQuotes` renamed to `EngineError:MalformedStringQuotes`

### Reliability

- Proptests for parsing array indices queries. ([#51](https://github.com/V0ldek/rsonpath/issues/51))

### Dependencies

- Bump clap from 4.1.11 to 4.3.4.
- Bump log from 0.4.17 to 0.4.19.
- Bump proptest from 1.1.0 to 1.2.0.
- Bump simple_logger from 4.1.0 to 4.2.0.

## [0.4.0] - 2023-04-20

### Features

- Wildcard descendant support.
  - You can now use the `..*`/`..[*]` selector that selects all nodes in the document it acts upon.

- Switch `Structural` to `BracketType`. ([#10](https://github.com/V0ldek/rsonpath/issues/10))
  - The `Opening` and `Closing` variants now differentiate between curly
  and square brackets with a value of the `BracketType` enum.

### Bug fixes

- Fix parser incorrectly escaping labels.
  - Queries like `$['\'']` would cause a parsing error, even though they were valid (match a child with key equal to "`'`").
  - The `\u` escape sequence is no longer recognized, since without UTF-8 handling they were meaningless.
    See ([#117](https://github.com/V0ldek/rsonpath/issues/117)).

- Empty query array behavior.
  - Running the query `$` on a document `[]` was giving zero results. Now correctly matches the root array.

### Documentation

- The grammar in top-level documentation now matches the implementation.

### Reliability

- Added proptests for query parsing.
  - Currently checks that correct queries are parsed correctly.
    We still need tests for error conditions (see [#51](https://github.com/V0ldek/rsonpath/issues/51)).

## [0.3.3] - 2023-03-29

### Bug Fixes

- Properly flow simd feature to dependencies. ([#111](https://github.com/V0ldek/rsonpath/issues/111))
  - This fixes build issues with the `aarch64` target. It also turns out our CI did not actually compile to all the targets we claimed it did, which is a bit embarrassing. We now *do* actually support all Rust Tier 1 targets and run tests for all **except** `aarch64-unknown-linux-gnu`, because there's no image for aarch64 on GitHub.

### Dependencies

- Bump clap from 4.1.6 to 4.1.11.

- Bump thiserror from 1.0.38 to 1.0.40.

- Bump simple_logger from 4.0.0 to 4.1.0.

- Bump dev-dependency tempfile from v3.3.0 to v3.4.0.
  - Resolves a dev-dependency security vulnerability of `remove-dir-all` by removing the dependency entirely.

## [0.3.2] - 2023-02-24

### Performance

- Faster toggling of commas/colons.
  - Shortened toggle by 1 SIMD instruction, improving perf by ~5% on heavily switching queries

## [0.3.1] - 2023-02-15

### Bug Fixes

- Duplicate results with `.*` on singleton list. ([#100](https://github.com/V0ldek/rsonpath/issues/100)[#96](https://github.com/V0ldek/rsonpath/issues/96))
  - If the query ended with a wildcard selector and was applied to a list with a singleton complex value, that value was being matched twice.

### Dependencies

- Bump clap from 4.1.4 to 4.1.6 (#99). ([#99](https://github.com/V0ldek/rsonpath/issues/99))

### Documentation

- Update main plot in README. ([#98](https://github.com/V0ldek/rsonpath/issues/98))

## [0.3.0] - 2023-02-14

### Features

- Better error reporting. (#88)
  - Added separate `engine::error::DepthError` type.
  - Additional context for depth-related `EngineError`s including the character at which depth overflow occurred.
  - New error, `EngineError::MissingClosingCharacter` reported if the engine reaches end of JSON and cannot match opening characters.
  - Improvements to the CLI error reporting/display.
- Increase max automaton size to 256 from 128.
- Compiling wildcard child selectors. (#90, #7)
  - Expressions parsed in #6 are now compiled into correct automata.
- Wildcard child support in engines. (#9, #8, #73)
  - Large overhaul to the query engines to enable processing the wildcard child selector. This closes the #9 epic of wildcard child support.
  - Both `main` and `recursive` engines now support wildcard child selectors.
  - The `commas` feature flag was removed.
  - Feature flags of `head-skip`, `tail-skip`, and `unique-members` were introduced to guard optimization paths.
    - The `head-skip` and `tail-skip` features make the code faster without significant tradeoffs.
    - The `unique-members` feature utilizes the assumption of key uniqueness within a single JSON object to speed up query execution, but it will not work correctly when an object with duplicate keys is given. Currently only the first occurence of such a key will be processed.
  - Many changes to the library structure and module visibility.

### Bug Fixes

- Too complex query now produces an error. (#88)
  - Previously the compiled automaton was silently truncated, which would cause incorrect results.

### Reliability

- Rename engine modules. (#88)
  - The `Runner` trait was renamed to `Engine`.
  - The `stackless` module is now `engine::main`.
  - The `stack_based` module is now `engine::recursive`.
  - The `StacklessRunner` is now the `MainEngine`, and is also reexported as `engine::RsonpathEngine`
  - The `StackBasedRunner` is now the `RecursiveEngine`.
- Added the `Compiler` trait. (#88)
  - The `compile_query` function creating engines is now part of that trait.
- Rename `NotSupportedError` to `NotSupported`.
- Moved `result` to a standalone module.
- Move all classifiers to `classification`.
  - Module `classify` renamed to `classification`.
  - Moved all resumption related things to `classification` proper.
- Removed only use of `unsafe` outside of SIMD.
- Forbid unsafe code outside of simd.
- Added test for heterogenous list.
- Hide `debug` and `bin` macros.
- Added `Compiler::from_compiled_query`.

### Dependencies

- Bump clap from 4.0.25 to 4.1.4.

- Bumped a number of dependencies.
  - backtrace (required by color-eyre) from 0.3.65 to 0.3.67.
  - once_cell (required by color-eyre) from 1.16.0 to 1.17.0.
  - owo-colors (required by color-eyre) from 3.3.0 to 3.5.0.
  - ppv-lite86 (required by thiserror) from 0.2.16 to 0.2.17.
  - itoa (required by simple_logger) from 1.0.2 to 1.0.5.
- Remove benchmarks crate from workspace.
  - This drastically reduces the number of dependencies tracked for the binary.
- Make some deps optional.
  - `memchr` is now included only with the `head-skip` feature
  - `replace_with` is now included only with the `tail-skip` feature

### Documentation

- Update toplevel lib docs.
- Add separate README for lib.
- Updated most of module docs.
- Added architecture diagram to lib README.

## [0.2.1] - 2023-01-25

### Features

- Wildcard child selector parser support. ([#6](https://github.com/V0ldek/rsonpath/issues/6))
  - Both shorthand `.*` and full `[*]` forms are recognised.

- Compile-only CLI flag. ([#76](https://github.com/V0ldek/rsonpath/issues/76))
  - Specifying `--compile` or `-c` will cause rsonpath to compile the query and output its automaton, without running the engine.
This option is mutually exclusive with `--engine` or providing an input path.

### Bug Fixes

- Compile error on `cargo install rsonpath`. ([#86](https://github.com/V0ldek/rsonpath/issues/86))

### Reliability

- Added install check to release CI/CD. ([#86](https://github.com/V0ldek/rsonpath/issues/86))
  - This will catch issues with the simplest `cargo install rsonpath` invocation before release to avoid these issues in the future.

### Dependencies

- Bump cc from 1.0.76 to 1.0.78. ([#82](https://github.com/V0ldek/rsonpath/issues/82))
- Bump nom from 7.1.1 to 7.1.3. ([#85](https://github.com/V0ldek/rsonpath/issues/85))

## [0.2.0] - 2023-01-15

### Features

- Librification ([#41](https://github.com/V0ldek/rsonpath/issues/41))
  - Project split into two crates: binary `rsonpath` and library `rsonpath-lib`

- Separate quote from structural classifiers.
([#17](https://github.com/V0ldek/rsonpath/issues/17))

- Implemented flexible classifiers.

- Implemented depth tail-skipping.

### Bug Fixes

- Escape classifier boundary error.

- Correctly set features for rsonpath-lib.

- Flaky jsonski benches due to their bugs

### Reliability

- Reenable Windows tests.

- Update for benchmarks integration.

- Update workflows and create Release workflow.
([#44](https://github.com/V0ldek/rsonpath/issues/44))
  - Created a `release` workflow that automatically build the crate on supported targets and creates a GitHub Release with appropriate artifacts.

Updated the `rust` workflow to run tests for all configurations supported in `release`, and properly run clippy on both SIMD and no-SIMD versions of the code.

List of supported targets at this point:

| Target triple             | nosimd build | SIMD support        |
|:--------------------------|:-------------|:--------------------|
| aarch64-unknown-linux-gnu | Yes          | No                  |
| i686-unknown-linux-gnu    | Yes          | Yes, avx2+pclmulqdq |
| x86_64-unknown-linux-gnu  | Yes          | Yes, avx2+pclmulqdq |
| x86_64-apple-darwin       | Yes          | No                  |
| i686-pc-windows-gnu       | Yes          | Yes, avx2+pclmulqdq |
| i686-pc-windows-msvc      | Yes          | Yes, avx2+pclmulqdq |
| x86_64-pc-windows-gnu     | Yes          | Yes, avx2+pclmulqdq |
| x86_64-pc-windows-msvc    | Yes          | Yes, avx2+pclmulqdq |

- `query` module is now panic-free.
([#38](https://github.com/V0ldek/rsonpath/issues/38) )
  - All errors are now reported via `QueryError`.

- Panic-free classifiers and engines.
([#39](https://github.com/V0ldek/rsonpath/issues/39) [#40](https://github.com/V0ldek/rsonpath/issues/40) [#31](https://github.com/V0ldek/rsonpath/issues/31))
  - Detectable errors now use proper error types instead of panics.
Added lints to prevent adding more panics or undocumented errors.

### Dependencies

- Bumped Criterion to 0.4.0.

- Removed usage of eyre from library code.

- Bump simple_logger to 4.0.0.

- Update clap to v4.

- Bump a bunch of minor versions.

- Removed `len_trait` dependency (#46).
([#46](https://github.com/V0ldek/rsonpath/issues/46))

## [0.1.2] - 2022-09-17

### Features

- Classify commas to prepare for the new wildcard selectors

### Bug Fixes

- Non-ASCII characters like (¡) breaking SIMD classification.

### Documentation

- Include usage in README.md.

## [0.1.1] - 2022-07-26

### Bug Fixes

- Supported simd is now autodetected

  - Instead of relying on the target_feature compiler flag
the build script now autodetects whether AVX2
is supported and compiles the correct version.

### Dependencies

- Update to use `criterion_decimal_throughput`.

- Equalise `aligners` versions (`0.0.9` across the project).

- Remove unnecessary dependencies.

  - Removed `memchr` and `static_assertions`.

### Documentation

- Changelog, code of conduct, contributing (#2).
([#2](https://github.com/V0ldek/rsonpath/issues/2))

- Badges for crates.io.

## [0.1.0] - 2022-07-15

### Features

- Engine implementation for child and recursive selectors.

<!-- generated by git-cliff -->
