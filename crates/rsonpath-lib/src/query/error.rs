//! Error types for the [`query`](`crate::query`) module.
//!
//! The main error type is [`ParseErrorReport`], which contains
//! all [`ParseErrors`](`ParseError`) encountered during parsing.
//!
//! # Examples
//!
//! Retrieving the part of input that caused a parse error:
//!
//! ```rust
//! use rsonpath::query::JsonPathQuery;
//! use rsonpath::query::error::ParserError;
//!
//! let query_str =
//!     "$.prop..invalid$chars.this_is_fine";
//! //                  ^     ^
//! //                  |_____|________________ start_idx of the error
//! //                        |________________ at this point the parser recovers
//! //                  ^^^^^^_________________ error length of 6
//! let result = JsonPathQuery::parse(query_str);
//!
//! match result {
//!     Err(ParserError::SyntaxError { report }) => {
//!         assert_eq!(report.errors().count(), 1);
//!         let parse_error = report.errors().next().unwrap();
//!         assert_eq!(parse_error.start_idx, 15);
//!         assert_eq!(parse_error.len, 6);
//!         let start = parse_error.start_idx;
//!         let end = parse_error.start_idx + parse_error.len;
//!         let invalid_tokens = &query_str[start..end];
//!         assert_eq!(invalid_tokens, "$chars");
//!     },
//!     _ => unreachable!(),
//! }
//! ```
use std::num::TryFromIntError;
use thiserror::Error;

/// Errors raised by the query compiler.
#[derive(Debug, Error)]
pub enum CompilerError {
    /// Max automaton size was exceeded during compilation of the query.
    #[error("Max automaton size was exceeded. Query is too complex.")]
    QueryTooComplex(#[source] Option<TryFromIntError>),

    /// Compiler error that occurred due to a known limitation.
    #[error(transparent)]
    NotSupported(#[from] crate::error::UnsupportedFeatureError),
}
