//! Error types for the [`query`](`crate::query`) module.

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
