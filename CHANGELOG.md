# Changelog

All notable changes to this project will be documented in this file.

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
    See: https://slsa.dev/spec/v1.0/about

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
  - This fixes build issues with the `aarch64` target. It also turns out our CI did not actually compile to all the targets we claimed it did, which is a bit embarrassing. We now _do_ actually support all Rust Tier 1 targets and run tests for all **except** `aarch64-unknown-linux-gnu`, because there's no image for aarch64 on GitHub.

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
