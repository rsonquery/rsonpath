//! Error types for the crate.
//!
//! The main error type is [`ParseError`], which contains
//! all syntax errors encountered during parsing.
// Dev note: the pretty-printing logic is split into the `display` and `style` submodules.
// That logic is rather involved and needs to be kept manageable.
// Any logic regarding constructing the errors from the parser or handling those errors by users
// should be added in this main module, while anything relating to how the errors are printed goes
// to the submodules.
use crate::error::display::fmt_parse_error;
use crate::{
    num::error::{JsonFloatParseError, JsonIntParseError},
    str::{self},
};
use std::fmt::{self, Display};
use thiserror::Error;

mod display;
mod formatter;
mod style;

#[derive(Debug)]
pub(crate) struct ParseErrorBuilder {
    syntax_errors: Vec<SyntaxError>,
}

/// Errors raised by the query parser.
#[derive(Debug, Error)]
pub struct ParseError {
    input: String,
    inner: InnerParseError,
}

impl Display for ParseError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt_parse_error(self, &display::empty_style(), f)
    }
}

impl ParseError {
    /// Returns whether the error was caused by exceeding the parser's nesting limit.
    #[inline]
    #[must_use]
    pub fn is_nesting_limit_exceeded(&self) -> bool {
        matches!(self.inner, InnerParseError::RecursionLimit(_))
    }
}

#[derive(Debug)]
enum InnerParseError {
    Syntax(Vec<SyntaxError>),
    RecursionLimit(usize),
}

impl ParseErrorBuilder {
    pub(crate) fn new() -> Self {
        Self { syntax_errors: vec![] }
    }

    pub(crate) fn add(&mut self, syntax_error: SyntaxError) {
        self.syntax_errors.push(syntax_error)
    }

    pub(crate) fn add_many(&mut self, mut syntax_errors: Vec<SyntaxError>) {
        self.syntax_errors.append(&mut syntax_errors)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.syntax_errors.is_empty()
    }

    pub(crate) fn build(self, str: String) -> ParseError {
        ParseError {
            input: str,
            inner: InnerParseError::Syntax(self.syntax_errors),
        }
    }

    pub(crate) fn recursion_limit_exceeded(str: String, recursion_limit: usize) -> ParseError {
        ParseError {
            input: str,
            inner: InnerParseError::RecursionLimit(recursion_limit),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct SyntaxError {
    /// Kind of the error.
    kind: SyntaxErrorKind,
    /// The byte index at which the error occurred, counting from the end of the input.
    rev_idx: usize,
    /// The number of characters that the parser recognized as invalid.
    len: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum SyntaxErrorKind {
    // Top-level errors.
    DisallowedLeadingWhitespace,
    DisallowedTrailingWhitespace,
    MissingRootIdentifier,
    // String/name parsing errors.
    InvalidUnescapedCharacter,
    InvalidEscapeSequence,
    UnpairedHighSurrogate,
    UnpairedLowSurrogate,
    InvalidHexDigitInUnicodeEscape,
    MissingClosingSingleQuote,
    MissingClosingDoubleQuote,
    // Segment errors.
    InvalidSegmentStart,
    InvalidSegmentAfterTwoPeriods,
    EmptySelector,
    InvalidSelector,
    MissingSelectorSeparator,
    MissingClosingBracket,
    InvalidNameShorthandAfterOnePeriod,
    // Number parsing errors.
    NegativeZeroInteger,
    LeadingZeros,
    NumberParseError(JsonFloatParseError),
    // Index selector.
    IndexParseError(JsonIntParseError),
    // Slice selector.
    SliceStartParseError(JsonIntParseError),
    SliceEndParseError(JsonIntParseError),
    SliceStepParseError(JsonIntParseError),
    // Filter selector.
    MissingClosingParenthesis,
    InvalidNegation,
    MissingComparisonOperator,
    InvalidComparisonOperator,
    InvalidComparable,
    NonSingularQueryInComparison,
    InvalidFilter,
}

impl SyntaxError {
    pub(crate) fn new(kind: SyntaxErrorKind, rev_idx: usize, len: usize) -> Self {
        Self { kind, rev_idx, len }
    }
}

#[derive(Debug)]
pub(crate) enum InternalParseError<'a> {
    SyntaxError(SyntaxError, &'a str),
    SyntaxErrors(Vec<SyntaxError>, &'a str),
    RecursionLimitExceeded,
    NomError(nom::error::Error<&'a str>),
}

impl<'a> nom::error::ParseError<&'a str> for InternalParseError<'a> {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        Self::NomError(nom::error::Error::from_error_kind(input, kind))
    }

    fn append(input: &'a str, kind: nom::error::ErrorKind, other: Self) -> Self {
        match other {
            Self::NomError(e) => Self::NomError(nom::error::Error::append(input, kind, e)),
            _ => other,
        }
    }
}
