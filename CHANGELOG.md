# Changelog

All notable changes to this project will be documented in this file.

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
