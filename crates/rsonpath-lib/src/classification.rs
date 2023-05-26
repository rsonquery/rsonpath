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
//! use rsonpath_lib::classification::quotes::classify_quoted_sequences;
//! use rsonpath_lib::classification::structural::{
//!     classify_structural_characters, resume_structural_classification,
//!     BracketType, Structural, StructuralIterator,
//! };
//! use rsonpath_lib::input::OwnedBytes;
//!
//! let json = r#"{"a":[42, {}, 44]}"#.to_owned();
//! let input = OwnedBytes::try_from(json).unwrap();
//! let quote_classifier = classify_quoted_sequences(&input);
//! let mut structural_classifier = classify_structural_characters(quote_classifier);
//! structural_classifier.turn_colons_on(0);
//! structural_classifier.turn_commas_on(0);
//!
//! // Classify first two structural characters.
//! assert_eq!(
//!     structural_classifier.next(),
//!     Some(Structural::Opening(BracketType::Curly, 0))
//! );
//! assert_eq!(
//!     structural_classifier.next(),
//!     Some(Structural::Colon(4))
//! );
//!
//! // We stop at the first non-classified character, Opening(5).
//! let mut resume_state = structural_classifier.stop();
//! assert_eq!(resume_state.get_idx(), 5);
//!
//! // Skip 6 bytes.
//! resume_state.offset_bytes(6);
//! assert_eq!(resume_state.get_idx(), 11);
//!
//! // Resume.
//! let mut structural_classifier_2 = resume_structural_classification(resume_state);
//! assert_eq!(
//!     structural_classifier_2.next(),
//!     Some(Structural::Closing(BracketType::Curly, 11))
//! );
//! ```
pub mod depth;
pub mod quotes;
pub mod structural;

use crate::{
    debug,
    input::{IBlock, Input},
};
use quotes::{QuoteClassifiedBlock, QuoteClassifiedIterator};

/// State allowing resumption of a classifier from a particular place
/// in the input along with the stopped [`QuoteClassifiedIterator`].
pub struct ResumeClassifierState<'a, I: Input, Q, const N: usize> {
    /// The stopped iterator.
    pub iter: Q,
    /// The block at which classification was stopped.
    pub block: Option<ResumeClassifierBlockState<'a, I, N>>,
    /// Was comma classification turned on when the classification was stopped.
    pub are_commas_on: bool,
    /// Was colon classification turned on when the classification was stopped.
    pub are_colons_on: bool,
}

/// State of the block at which classification was stopped.
pub struct ResumeClassifierBlockState<'a, I: Input + 'a, const N: usize> {
    /// Quote classified information about the block.
    pub block: QuoteClassifiedBlock<IBlock<'a, I, N>, N>,
    /// The index at which classification was stopped.
    pub idx: usize,
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, N>, const N: usize> ResumeClassifierState<'a, I, Q, N> {
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

    /// Move the state forward by `count` bytes.
    ///
    /// # Panics
    /// If the `count` is not positive.
    #[inline]
    pub fn offset_bytes(&mut self, count: isize) {
        assert!(count > 0);
        let count = count as usize;

        let remaining_in_block = self.block.as_ref().map_or(0, |b| b.block.len() - b.idx);

        match self.block.as_mut() {
            Some(b) if b.block.len() - b.idx > count => {
                b.idx += count;
            }
            _ => {
                let blocks_to_advance = (count - remaining_in_block) / N;

                let remainder = (self.block.as_ref().map_or(0, |b| b.idx) + count - blocks_to_advance * N) % N;

                self.iter.offset(blocks_to_advance as isize);
                let next_block = self.iter.next();

                self.block = next_block.map(|b| ResumeClassifierBlockState {
                    block: b,
                    idx: remainder,
                });
            }
        }

        debug!("offset_bytes({count}) results in idx moved to {}", self.get_idx());
    }
}
