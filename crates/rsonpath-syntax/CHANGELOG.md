# Changelog

All notable changes to this project will be documented in this file.

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
