//! Classifiers working on the input stream.
//!
//! - [`quotes`] contains the low-level [`QuoteClassifiedIterator`](`quotes::QuoteClassifiedIterator`)
//!   computing basic information on which characters are escaped or within quotes.
//! - [`structural`] contains the [`StructuralIterator`](`structural::StructuralIterator`)
//!   that wraps over a quote classifier to extract a stream of [`Structural`](`structural::Structural`) characters.
//! - [`depth`] contains the [`DepthIterator`](`depth::DepthIterator`) that works on top of a quote classifier
//!   to provide quick fast-forwarding over the stream while keeping track of the depth.
//!
//! This base module provides the [`ResumeClassifierState`] struct common between all
//! higher-level classifiers that work on top of a [`QuoteClassifiedIterator`](`quotes::QuoteClassifiedIterator`).
//! It allows saving the state of a classifier and can be later used to resume classification
//! from a, possibly different, high-level classifier. This state's index can be pushed
//! forward.
#[cfg(test)]
mod classifier_correctness_tests;
pub mod depth;
pub(crate) mod mask;
pub mod memmem;
pub mod quotes;
pub(crate) mod simd;
pub mod structural;

use std::fmt::Display;

use crate::{debug, input::InputBlockIterator};
use quotes::{QuoteClassifiedBlock, QuoteClassifiedIterator};

/// State allowing resumption of a classifier from a particular place
/// in the input along with the stopped [`QuoteClassifiedIterator`].
pub struct ResumeClassifierState<'i, I, Q, M, const N: usize>
where
    I: InputBlockIterator<'i, N>,
{
    /// The stopped iterator.
    pub iter: Q,
    /// The block at which classification was stopped.
    pub block: Option<ResumeClassifierBlockState<'i, I, M, N>>,
    /// Was comma classification turned on when the classification was stopped.
    pub are_commas_on: bool,
    /// Was colon classification turned on when the classification was stopped.
    pub are_colons_on: bool,
}

/// State of the block at which classification was stopped.
pub struct ResumeClassifierBlockState<'i, I, M, const N: usize>
where
    I: InputBlockIterator<'i, N>,
{
    /// Quote classified information about the block.
    pub block: QuoteClassifiedBlock<I::Block, M, N>,
    /// The index at which classification was stopped.
    pub idx: usize,
}

impl<'i, I, Q, M, const N: usize> ResumeClassifierState<'i, I, Q, M, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, M, N>,
{
    /// Get the index in the original bytes input at which classification has stopped.
    #[inline(always)]
    pub(crate) fn get_idx(&self) -> usize {
        debug!(
            "iter offset: {}, block idx: {:?}",
            self.iter.get_offset(),
            self.block.as_ref().map(|b| b.idx)
        );

        self.iter.get_offset() + self.block.as_ref().map_or(0, |b| b.idx)
    }
}

/// Get a human-readable description of SIMD capabilities supported by rsonpath
/// on the current machine.
#[doc(hidden)]
#[inline]
#[must_use]
pub fn describe_simd() -> impl Display {
    simd::configure()
}
