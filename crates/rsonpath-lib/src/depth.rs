//! Overflow-safe utilities for tracking JSON document depth.
use super::error::DepthError;
use std::{
    fmt::Display,
    ops::{Add, Deref, Sub},
};

/// Overflow-safe thin wrapper for a [`u8`] depth counter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Depth(u8);

impl Depth {
    /// Depth of 0.
    pub(crate) const ZERO: Self = Self(0);

    /// Add `1` to the depth, or raise an error if the maximum
    /// supported value is reached.
    pub(crate) fn increment(&mut self) -> Result<(), DepthError> {
        *self = (*self + 1)?;
        Ok(())
    }

    /// Subtract `1` from the depth, or raise an error if the depth
    /// is zero.
    pub(crate) fn decrement(&mut self) -> Result<(), DepthError> {
        *self = (*self - 1)?;
        Ok(())
    }
}

macro_rules! impl_add {
    ($t:ty) => {
        impl Add<u8> for $t {
            type Output = Result<Depth, DepthError>;

            #[inline]
            fn add(self, rhs: u8) -> Self::Output {
                self.0
                    .checked_add(rhs)
                    .ok_or(DepthError::AboveLimit(u8::MAX as usize))
                    .map(Depth)
            }
        }
    };
}

macro_rules! impl_sub {
    ($t:ty) => {
        impl Sub<u8> for $t {
            type Output = Result<Depth, DepthError>;

            #[inline]
            fn sub(self, rhs: u8) -> Self::Output {
                self.0.checked_sub(rhs).ok_or(DepthError::BelowZero).map(Depth)
            }
        }
    };
}

impl_add!(Depth);
impl_add!(&Depth);
impl_add!(&mut Depth);

impl_sub!(Depth);
impl_sub!(&Depth);
impl_sub!(&mut Depth);

impl Display for Depth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Deref for Depth {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
