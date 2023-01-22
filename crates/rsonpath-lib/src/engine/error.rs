//! Error definitions and utilities for engine execution.

use thiserror::Error;

/// Error enum for all types of errors that can be reported
/// during engine execution.
#[derive(Debug, Error)]
pub enum EngineError {
    /// Document depth fell below zero, which can only happen
    /// if there are more closing than opening braces.
    #[error("Mismatched closing character in the input JSON.")]
    DepthBelowZero,
    /// The depth limit was reached -- the document is too nested.
    /// The inner [`usize`] value should be set to the actual limit.
    #[error(
        "Maximum depth of {0} exceeded. \
        Larger depths are currently unsupported. \
        If this feature is important to you, \
        please raise an issue at {}",
        crate::error::FEATURE_REQUEST_URL
    )]
    DepthAboveLimit(usize),
    /// An error occurred when trying to parse a label terminated by a particular colon character.
    /// The inner [`usize`] value should be set to the byte index of the colon.
    #[error(
        "Malformed label in the input JSON. \
        The colon at position {0} must be preceded by a string, but \
        the engine could not match the appropriate double quote characters."
    )]
    MalformedLabelQuotes(usize),
}
