//! Error types for the [`input`](`crate::input`) module.

use crate::error::InternalRsonpathError;
use thiserror::Error;

/// Errors raised when constructing [`Input`](super::Input) implementations.
#[derive(Debug, Error)]
pub enum InputError {
    /// Error that occurs when an unbounded-sized implementation
    /// (e.g. [`BufferedInput`](super::BufferedInput)) would allocate more than the global limit of [`isize::MAX`].
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

/// Hack to convert errors to [`InputError`] easily.
///
/// The bound on errors in [`Input`](super::Input) is [`Into<InputError>`].
/// This doesn't work with the usual `?` Rust operator, as that requires the reverse
/// bound (for [`InputError`] to be `From` the source). This is not easily expressible
/// as a bound on [`Input`](super::Input). Instead we use this small function to perform
/// the same conversion.
pub(crate) trait InputErrorConvertible<T>: Sized {
    /// Convert to [`InputError`] result.
    ///
    /// Instead of
    /// ```rust,ignore
    /// err.map_err(|x| x.into())?;
    /// ```
    /// you can write
    /// ```rust,ignore
    /// err.e()?;
    /// ```
    /// as a shorthand.
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
