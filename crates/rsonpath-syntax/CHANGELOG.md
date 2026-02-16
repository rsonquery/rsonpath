# Changelog

All notable changes to this project will be documented in this file.

## [0.4.1] - 2026-02-16

### Features

- Removed the `arbitrary` feature and the `Arbitrary` implementations.

### Bug Fixes

- Error messages blowing up on long inputs. ([#842](https://github.com/V0ldek/rsonpath/issues/842), [#749](https://github.com/V0ldek/rsonpath/issues/749))
  - Previously, when displaying a ParseError every underlying
    SyntaxError would be printed with the full query input as context.
    If the density of errors in the input was high this would effectively
    cause a quadratic blowup during printing.
  - It's probably unlikely inputs like this would be given by a user,
    but they do happen during fuzzing (when we're throwing long strings
    of essentially random characters at the parser) and could potentially
    be used as a DoS attack vector (intentionally supplying nonsensical
    large queries and forcing error messages to be sent back).
  - Additionally fixed an invalid error message given when a side of
    a comparison operator was a non-singular query.

### Reliability

- The structural fuzzer now correctly runs on a nightly basis. ([#749](https://github.com/V0ldek/rsonpath/issues/749))

### Dependencies

- Update `nom` from 7.1.3 to 8.0.0
- Update `owo-colors` from 4.1.0 to 4.2.3
- Update `serde` from 1.0.217 to 1.0.228
- Update `thiserror` from 2.0.9 to 2.0.18
- Update `unicode-width` from 0.2.0 to 0.2.2
- Remove `arbitrary` from dependencies

## [0.4.0] - 2024-12-31

### Features

- Serde support for `JsonPathQuery` and all the constutent types.
  - Implemented `serde::Serialize` and `serde::Deserialize` for
    `JsonPathQuery` and all its substructures, including `JsonString`
    and the numeric types. The `serde` dependency is guarded behind the optional `serde` feature.
  - Also added snapshot tests for serialization based on `insta`.

### Dependencies

- `serde` (1.0.217) is now an optional dependency

## [0.3.2] - 2024-12-22

### Dependencies

- Bump arbitrary from 1.3.1 to 1.3.2
- Bump owo-colors from 4.0.0 to 4.1.0
- Bump thiserror from 1.0.57 to 2.0.9 (#617). ([#617](https://github.com/V0ldek/rsonpath/issues/617))
- Bump unicode-width from 0.1.11 to 0.2.0

## [0.3.1] - 2024-03-28

### Features

- Added the [`Step::is_forward`](https://docs.rs/rsonpath-syntax/0.3.1/rsonpath_syntax/enum.Step.html#method.is_forward) and [`Step::is_backward`](https://docs.rs/rsonpath-syntax/0.3.1/rsonpath_syntax/enum.Step.html#method.is_backward) methods.
- Added `From<JsonUInt> for i64`.

## [0.3.0] - 2024-02-29

### Features

- [**breaking**] Parsing `Filter` selectors.
  - Added the `Selector::Filter` variant and related parsing.

### Dependencies

- Bump thiserror from 1.0.56 to 1.0.57

## [0.2.0] - 2024-01-15

### Features

- [**breaking**] Parsing `Slice` selectors.
  - Added the `Selector::Slice` variant and related parsing.

### Bug Fixes

- U+001A-U+001F in name selectors.
  - Characters U+001A through U+001F were erroneously accepted unescaped.
    This is now a hard error.

### Reliability

- Added jsonpath cts.
  - Parser is now tested with the official
[JSONPath Compliance Test Suite](https://github.com/jsonpath-standard/jsonpath-compliance-test-suite)

## [0.1.0] - 2024-01-10

### Features

- Parsing of name, index, and wildcard selectors.
- Robust error handling, messages, and suggestions.
