# `rsonpath-syntax-proptest` &ndash; `proptest::Arbitrary` implementation for [`rsonpath-syntax`](https://crates.io/crates/rsonpath-syntax)

[![Rust](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml/badge.svg)](https://github.com/V0ldek/rsonpath/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/rsonpath-syntax-proptest?logo=docs.rs)](https://docs.rs/crate/rsonpath-syntax-proptest/latest)

[![Crates.io](https://img.shields.io/crates/v/rsonpath-syntax-proptest?logo=docs.rs)](https://crates.io/crates/rsonpath-syntax-proptest)

[![License](https://img.shields.io/crates/l/rsonpath)](https://choosealicense.com/licenses/mit/)

Utilities for property testing with types in `rsonpath-syntax`.

The crate exposes two types, `ArbitraryJsonPathQuery` and `ArbitraryJsonPathQueryParam`.
The `ArbitraryJsonPathQuery` implements [`proptest::Arbitrary`](https://docs.rs/proptest/latest/proptest/arbitrary/trait.Arbitrary.html)
which generates an arbitrary JSONPath query string representation and [`rsonpath_syntax::JsonPathQuery`](https://docs.rs/rsonpath-syntax/latest/rsonpath_syntax/) object.

## Usage

This is mostly used for internal testing of `rsonpath-lib` and `rsonpath-syntax`, but it is in general useful
for property-testing or fuzzing code that relies on JSONPath queries as input.

Example usage with `proptest`:

```rust
use proptest::prelude::*;
use rsonpath_syntax_proptest::ArbitraryJsonPathQuery;

proptest! {
    #[test]
    fn example(ArbitraryJsonPathQuery { parsed, string } in prop::arbitrary::any::<ArbitraryJsonPathQuery>()) {
        // Your test using parsed (JsonPathQuery) and/or string (String).
    }
}
```
