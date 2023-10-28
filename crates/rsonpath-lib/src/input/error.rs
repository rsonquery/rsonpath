//! Error types for the [`input`](`crate::input`) module.

use crate::error::InternalRsonpathError;
use thiserror::Error;

/// Errors raised when constructing [`Input`](super::Input) implementations.
#[derive(Debug, Error)]
pub enum InputError {
    /// Error that occurs when an unbounded-sized implementation
    /// (e.g. [`BufferedInput`](super::BufferedInput)) would allocate more than the global limit of [isize::MAX].
    #[error("owned buffer size exceeded the hard system limit of isize::MAX")]
    AllocationSizeExceeded,
    /// Error when reading input from an underlying IO handle.
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    /// Irrecoverable error due to a broken invariant or assumption.
    /// Preferred over panicking.
    #[error("InputError: {0}")]
    InternalError(
        #[source]
        #[from]
        InternalRsonpathError,
    ),
}

pub(crate) trait InputErrorConvertible<T>: Sized {
    fn e(self) -> Result<T, InputError>;
}

impl<T, E: Into<InputError>> InputErrorConvertible<T> for Result<T, E> {
    #[inline(always)]
    fn e(self) -> Result<T, InputError> {
        self.map_err(std::convert::Into::into)
    }
}

/// Error type for [`Input`](`super::Input`) implementations that never fail
/// when reading more input.
#[derive(Debug, Error)]
pub enum Infallible {}

impl From<Infallible> for InputError {
    #[inline(always)]
    fn from(_value: Infallible) -> Self {
        unreachable!()
    }
}
