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
//! use rsonpath_lib::query::JsonPathQuery;
//! use rsonpath_lib::query::error::ParserError;
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
use super::NonNegativeArrayIndex;
use std::{
    fmt::{self, Display},
    num::TryFromIntError,
};
use thiserror::Error;

/// Errors raised by the query parser.
#[derive(Debug, Error)]
pub enum ParserError {
    /// Parsing error that occurred due to invalid input.
    #[error("one or more parsing errors occurred:\n{}", .report)]
    SyntaxError {
        /// Error report.
        report: ParseErrorReport,
    },

    /// Internal parser error. This is not expected to happen,
    /// and signifies a bug in [`query`](`crate::query`).
    #[error(
        "unexpected error in the parser; please report this issue at {}",
        crate::error::BUG_REPORT_URL
    )]
    InternalNomError {
        /// Source error from the [`nom`] crate.
        #[from]
        #[source]
        source: nom::error::Error<String>,
    },

    /// An invalid array index was specified in the query.
    #[error(transparent)]
    ArrayIndexError(#[from] ArrayIndexError),
}

///Errors raised trying to parse array indices.
#[derive(Debug, Error)]
pub enum ArrayIndexError {
    /// A value in excess of the permitted size was specified.
    #[error(
        "Array index {0} exceeds maximum specification value of {}.",
        NonNegativeArrayIndex::MAX
    )]
    ExceedsUpperLimitError(String),

    /// An error occurred while calculating the maximum possible length of array indices.
    #[error("There was an error determining the maximum possible length of array indices.")]
    UpperLimitLengthCalculationError,
}

/// Error report created during the parser's run over a single input string.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseErrorReport {
    errors: Vec<ParseError>,
}

impl Display for ParseErrorReport {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in self.errors() {
            writeln!(f, "{error}\n")?;
        }

        Ok(())
    }
}

/// Single error raised during parsing, defined as the
/// contiguous sequence of characters that caused the error.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParseError {
    /// The index at which the error occurred.
    pub start_idx: usize,
    /// The number of characters that the parser recognized as invalid.
    pub len: usize,
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid tokens of length {} at position {} ",
            self.len, self.start_idx
        )
    }
}

impl ParseError {
    fn end_idx(&self) -> usize {
        self.start_idx + self.len - 1
    }
}

impl ParseErrorReport {
    pub(crate) fn new() -> Self {
        Self { errors: vec![] }
    }

    pub(crate) fn record_at(&mut self, idx: usize) {
        match self.errors.last_mut() {
            Some(last_error) if last_error.end_idx() + 1 == idx => last_error.len += 1,
            _ => self.add_new(idx),
        }
    }

    /// Retrieves an [`Iterator`] over all [`ParseErrors`](`ParseError`)
    /// in the report.
    #[inline]
    pub fn errors(&self) -> impl Iterator<Item = &ParseError> {
        self.errors.iter()
    }

    fn add_new(&mut self, idx: usize) {
        self.errors.push(ParseError { start_idx: idx, len: 1 })
    }
}

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
