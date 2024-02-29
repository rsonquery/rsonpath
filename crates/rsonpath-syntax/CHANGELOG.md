# Changelog

All notable changes to this project will be documented in this file.

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
