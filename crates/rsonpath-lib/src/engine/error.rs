//! Error definitions and utilities for engine execution.
use crate::error::InternalRsonpathError;
use thiserror::Error;

/// Error enum for all types of errors that can be reported
/// during engine execution.
///
/// **NOTE**: These error are _not_ guaranteed to be raised for every
/// JSON document that is malformed in the respective manner.
/// The engine may ignore such errors and simply produce incorrect results
/// for invalid documents.
#[derive(Debug, Error)]
pub enum EngineError {
    /// Document depth fell below zero, which can only happen
    /// if there are more closing than opening braces.
    /// The inner [`usize`] value indicates the position of the mismatched closing character.
    #[error("Mismatched closing character in the input JSON at position {0}.")]
    DepthBelowZero(usize, #[source] DepthError),
    /// The depth limit was reached -- the document is too nested.
    /// The inner [`usize`] value indicates the position of the opening character
    /// which caused the overflow.
    #[error("Opening character at position {0} caused depth overflow.")]
    DepthAboveLimit(usize, #[source] DepthError),
    /// The engine reached end of the document while depth was positive.
    /// This means that some of the opening characters do not have matching
    /// closing characters.
    #[error("Malformed input JSON; end of input was reached, but unmatched opening characters remained.")]
    MissingClosingCharacter(),
    /// An error occurred when trying to parse a member name terminated by a particular colon character.
    /// The inner [`usize`] value should be set to the byte index of the colon.
    #[error(
        "Malformed member name in the input JSON; \
        the colon at position {0} must be preceded by a string, but \
        there are no matching double quote characters."
    )]
    MalformedStringQuotes(usize),
    /// Engine error that occurred due to a known limitation.
    #[error(transparent)]
    NotSupported(#[from] crate::error::UnsupportedFeatureError),
    /// Irrecoverable error due to a broken invariant or assumption.
    /// The engine returns these instead of panicking.
    #[error("EngineError: {0}")]
    InternalError(
        #[source]
        #[from]
        InternalRsonpathError,
    ),
}

/// Errors in internal depth tracking of execution engines.
#[derive(Error, Debug)]
pub enum DepthError {
    /// The engine's maximum depth limit was exceeded.
    /// The inner [`usize`] indicates that limit.
    #[error("Maximum depth of {0} exceeded.")]
    AboveLimit(usize),
    /// The document has unmatched closing characters
    /// and is malformed.
    #[error("Depth fell below zero.")]
    BelowZero,
}
