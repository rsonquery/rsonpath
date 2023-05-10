//! Error types for the [`input`](`crate::input`) module.

use thiserror::Error;
use crate::error::InternalRsonpathError;

/// Errors raised when constructing [`Input`](super::Input) implementations.
#[derive(Debug, Error)]
pub enum InputError {
    /// Error that occurs when an unbounded-sized implementation
    /// (e.g. [`OwnedBytes`](super::OwnedBytes)) would allocate more than the global limit of [isize::MAX].
    #[error("owned buffer size exceeded the hard system limit of isize::MAX")]
    AllocationSizeExceeded,
    /// Irrecoverable error due to a broken invariant or assumption.
    /// Preferred over panicking.
    #[error("InputError: {0}")]
    InternalError(
        #[source]
        #[from]
        InternalRsonpathError,
    ),
}
