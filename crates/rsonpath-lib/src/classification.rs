//! Classifiers working on the input stream.
//!
//! - [`quotes`] contains the low-level [`QuoteClassifiedIterator`](`quotes::QuoteClassifiedIterator`)
//! computing basic information on which characters are escaped or within quotes.
//! - [`structural`] contains the [`StructuralIterator`](`structural::StructuralIterator`)
//! that wraps over a quote classifier to extract a stream of [`Structural`](`structural::Structural`) characters.
//! - [`depth`] contains the [`DepthIterator`](`depth::DepthIterator`) that works on top of a quote classifier
//! to provide quick fast-forwarding over the stream while keeping track of the depth.
//!
//! This base module provides the [`ResumeClassifierState`] struct common between all
//! higher-level classifiers that work on top of a [`QuoteClassifiedIterator`](`quotes::QuoteClassifiedIterator`).
//! It allows saving the state of a classifier and can be later used to resume classification
//! from a, possibly different, high-level classifier. This state's index can be pushed
//! forward.
//!
//! # Examples
//! ```rust
//! use rsonpath::classification::quotes::classify_quoted_sequences;
//! use rsonpath::classification::structural::{
//!     classify_structural_characters, resume_structural_classification,
//!     BracketType, Structural, StructuralIterator,
//! };
//! use rsonpath::input::{Input, OwnedBytes};
//! use rsonpath::result::empty::EmptyRecorder;
//! use rsonpath::FallibleIterator;
//!
//! let json = r#"{"a":[42, {}, 44]}"#.to_owned();
//! let input = OwnedBytes::try_from(json).unwrap();
//! let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
//! let quote_classifier = classify_quoted_sequences(iter);
//! let mut structural_classifier = classify_structural_characters(quote_classifier);
//! structural_classifier.turn_colons_on(0);
//! structural_classifier.turn_commas_on(0);
//!
//! // Classify first two structural characters.
//! assert_eq!(
//!     structural_classifier.next().unwrap(),
//!     Some(Structural::Opening(BracketType::Curly, 0))
//! );
//! assert_eq!(
//!     structural_classifier.next().unwrap(),
//!     Some(Structural::Colon(4))
//! );
//!
//! // We stop at the first non-classified character, Opening(5).
//! let mut resume_state = structural_classifier.stop();
//! assert_eq!(resume_state.get_idx(), 5);
//!
//! // Skip to index 11.
//! resume_state.forward_to(11);
//! assert_eq!(resume_state.get_idx(), 11);
//!
//! // Resume.
//! let mut structural_classifier_2 = resume_structural_classification(resume_state);
//! assert_eq!(
//!     structural_classifier_2.next().unwrap(),
//!     Some(Structural::Closing(BracketType::Curly, 11))
//! );
//! ```
pub mod depth;
pub(crate) mod mask;
pub mod memmem;
pub mod quotes;
pub mod structural;

use crate::{
    debug,
    input::{error::InputError, InputBlockIterator},
};
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
    pub fn get_idx(&self) -> usize {
        debug!(
            "iter offset: {}, block idx: {:?}",
            self.iter.get_offset(),
            self.block.as_ref().map(|b| b.idx)
        );

        self.iter.get_offset() + self.block.as_ref().map_or(0, |b| b.idx)
    }

    /// Move the state forward to `index`.
    ///
    /// # Errors
    /// If the offset crosses block boundaries, then a new block is read from the underlying
    /// [`Input`](crate::input::Input) implementation, which can fail.
    ///
    /// # Panics
    /// If the `index` is not ahead of the current position of the state ([`get_idx`](ResumeClassifierState::get_idx)).
    #[inline]
    #[allow(clippy::panic_in_result_fn)]
    pub fn forward_to(&mut self, index: usize) -> Result<(), InputError> {
        let current_block_start = self.iter.get_offset();
        let current_block_idx = self.block.as_ref().map_or(0, |b| b.idx);
        let current_idx = current_block_start + current_block_idx;

        debug!(
            "Calling forward_to({index}) when the inner iter offset is {current_block_start} and block idx is {current_block_idx:?}"
        );

        // We want to move by this much forward, and delta > 0.
        assert!(index > current_idx);
        let delta = index - current_idx;

        // First we virtually pretend to move *backward*, setting the index of the current block to zero,
        // and adjust the delta to cover that distance. This makes calculations simpler.
        // Then we need to skip zero or more blocks and set our self.block to the last one we visit.
        let remaining = delta + current_block_idx;
        let blocks_to_skip = remaining / N;
        let remainder = remaining % N;

        match self.block.as_mut() {
            Some(b) if blocks_to_skip == 0 => {
                b.idx = remaining;
            }
            Some(_) => {
                self.block = self
                    .iter
                    .offset(blocks_to_skip as isize)?
                    .map(|b| ResumeClassifierBlockState {
                        block: b,
                        idx: remainder,
                    });
            }
            None => {
                self.block = self
                    .iter
                    .offset((blocks_to_skip + 1) as isize)?
                    .map(|b| ResumeClassifierBlockState {
                        block: b,
                        idx: remainder,
                    });
            }
        }

        debug!("forward_to({index}) results in idx moved to {}", self.get_idx());

        Ok(())
    }
}
