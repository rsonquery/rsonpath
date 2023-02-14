//! Overflow-safe utilities for tracking JSON document depth.
use super::error::DepthError;
use std::{fmt::Display, ops::Deref};

/// Overflow-safe thin wrapper for a [`u8`] depth counter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Depth {
    value: u8,
}

impl Depth {
    /// Depth of 0.
    pub(crate) const ZERO: Self = Self { value: 0 };

    /// Add `1` to the depth, or raise an error if the maximum
    /// supported value is reached.
    pub(crate) fn increment(&mut self) -> Result<(), DepthError> {
        self.value = self
            .value
            .checked_add(1)
            .ok_or(DepthError::AboveLimit(u8::MAX as usize))?;
        Ok(())
    }

    /// Subtract `1` from the depth, or raise an error if the depth
    /// is zero.
    pub(crate) fn decrement(&mut self) -> Result<(), DepthError> {
        self.value = self.value.checked_sub(1).ok_or(DepthError::BelowZero)?;
        Ok(())
    }
}

impl Display for Depth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Deref for Depth {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
