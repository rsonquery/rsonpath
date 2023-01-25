//! Blazing fast execution of JSONPath queries.
//!
//! JSONPath parser, execution engines and byte stream utilities useful when parsing
//! JSON structures.
//!
//! # Examples
//! ```rust
//! use rsonpath_lib::engine::{Input, Runner, result::CountResult};
//! use rsonpath_lib::query::JsonPathQuery;
//! use rsonpath_lib::stackless::StacklessRunner;
//! # use std::error::Error;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! // Parse a JSONPath query from string.
//! let query = JsonPathQuery::parse("$..person..number")?;
//! let contents = r#"
//! {
//!   "person": {
//!     "name": "John",
//!     "surname": "Doe",
//!     "phoneNumbers": [
//!       {
//!         "type": "Home",
//!         "number": "111-222-333"
//!       },
//!       {
//!         "type": "Work",
//!         "number": "123-456-789"
//!       }
//!     ]
//!   }
//! }
//! "#;
//! // Remove whitespace from the JSON - limitation of the current version.
//! let mut stripped_contents = contents.chars().filter(|c| !c.is_whitespace()).collect::<String>();
//! // Convert the contents to the Input type required by the Runners.
//! let input = Input::new(&mut stripped_contents);
//! // Compile the query. The runner can be reused to run the same query on different contents.
//! let runner = StacklessRunner::compile_query(&query)?;
//! // Count the number of occurrences of elements satisfying the query.
//! let count = runner.run::<CountResult>(&input)?.get();
//!
//! assert_eq!(2, count);
//! # Ok(())
//! # }
//! ```
//! # Input JSON assumptions
//!
//! 1. The JSON must be a syntactically valid JSON encoded in UTF-8 as defined by [RFC4627](https://datatracker.ietf.org/doc/html/rfc4627).
//! 2. The JSON must not contain any whitespace outside of string values. This is a known limitation that will be lifted in future versions.
//!
//! If the assumptions are violated the algorithm's behavior is undefined. It might panic or it might return nonsensical results.
//! No validation is performed for maximum performance. Asserting the assumptions falls on the user of this library.
//!
//! # JSONPath language
//!
//! The library implements the JSONPath syntax as established by Stefan Goessner in <https://goessner.net/articles/JsonPath/>.
//! That implementation does not describe its semantics. There is no guarantee that this library has the same semantics
//! as Goessner's implementation. The semantics used by rsonpath are described below.
//!
//! ## Grammar
//!
//! ```ebnf
//! query = [root_expr] , { expr }
//! expr = root_expr | descendant_expr | label_expr
//! root_expr = "$"
//! descendant_expr = ".."
//! label_expr = simple_label | explicit_label
//! simple_label = { ALPHANUMERIC | "_" }
//! explicit_label = "['" , JSON_LABEL , "']"
//!
//! ALPHANUMERIC = [A-Z][a-z][0-9]
//! ```
//! `JSON_LABEL` is the string defined by [RFC4627](https://datatracker.ietf.org/doc/html/rfc4627#page-5).
//!
//! ## Semantics
//!
//! The query is executed from left to right, expression by expression. When a value is found that matches
//! the current expression, the execution advances to the next expression and evaluates it recursively within
//! the context of that value.
//!
//! ### Root expression
//! The root expression may only appear at the beginning of the query and is implicit if not specified.
//! It matches the root object or array. Thus the query "$" gives either 1 or 0 results, if the JSON
//! is empty or non-empty, respectively.
//!
//! ### Label expression
//! Matches any value under a specified key in the current object or array and then executes the rest of the query on that value.
//!
//! ### Descendant expression
//! Switches the engine into a recursive descent mode. The remainder of the query is executed recursively on every value
//! nested in the current object or array.
//!
//! ## Limitations
//!
//! The only type of query supported as of now is a sequence of descendant-label expressions.
//! ```json
//! $..label_1..label_2..[...]..label_n
//! ```

// Documentation lints, enabled only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc,)
)]
#![cfg_attr(not(debug_assertions), warn(rustdoc::missing_crate_level_docs))]
// Generic pedantic lints.
#![warn(
    explicit_outlives_requirements,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes
)]
// Clippy pedantic lints.
#![warn(
    clippy::allow_attributes_without_reason,
    clippy::cargo_common_metadata,
    clippy::cast_lossless,
    clippy::cloned_instead_of_copied,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::fallible_impl_from,
    clippy::flat_map_option,
    clippy::if_then_some_else_none,
    clippy::inconsistent_struct_constructor,
    clippy::large_digit_groups,
    clippy::let_underscore_must_use,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_inline_in_public_items,
    clippy::mod_module_files,
    clippy::must_use_candidate,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::ptr_as_ptr,
    clippy::redundant_closure_for_method_calls,
    clippy::ref_binding_to_reference,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::undocumented_unsafe_blocks,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::use_self
)]
// Panic-free lint.
#![warn(
    clippy::exit,
)]
// Panic-free lints (disabled for tests).
#![cfg_attr(
    not(test),
    warn(
        clippy::expect_used,
        clippy::panic,
        clippy::panic_in_result_fn,
        clippy::unwrap_used
    )
)]
// IO hygene, only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(clippy::print_stderr, clippy::print_stdout, clippy::todo)
)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod classify;
pub mod depth;
pub mod engine;
pub mod error;
pub mod query;
pub mod quotes;
pub mod stack_based;
pub mod stackless;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(simd = "avx2")] {
        /// Default alignment required out of input blocks for the purpose
        /// of classification with [`quotes`](crate::quotes) and [`classify`](crate::classify).
        pub type BlockAlignment = aligners::alignment::TwoTo<5>;
    }
    else {
        /// Default alignment required out of input blocks for the purpose
        /// of classification with [`quotes`](crate::quotes) and [`classify`](crate::classify).
        pub type BlockAlignment = aligners::alignment::TwoTo<5>;
    }
}

/// Macro for debug logging. Evaluates to [`log::debug`], if debug assertions are enabled.
/// Otherwise it's an empty statement.
///
/// Use this instead of plain [`log::debug`], since this is automatically removed in
/// release mode and incurs no performance penalties.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => (log::debug!(target: $target, $($arg)+));
    ($($arg:tt)+) => (log::debug!($($arg)+))
}

/// Macro for debug logging. Evaluates to [`log::debug`], if debug assertions are enabled.
/// Otherwise it's an empty statement.
///
/// Use this instead of plain [`log::debug`], since this is automatically removed in
/// release mode and incurs no performance penalties.
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

/// Debug log the given u64 expression by its full 64-bit binary string representation.
#[macro_export]
macro_rules! bin {
    ($name:expr, $e:expr) => {
        $crate::debug!(
            "{: >24}: {:064b} ({})",
            $name,
            {
                let mut res = 0_u64;
                for i in 0..64 {
                    let bit = (($e) & (1 << i)) >> i;
                    res |= bit << (63 - i);
                }
                res
            },
            $e
        );
    };
}
