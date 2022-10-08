//! Classification of bytes withing JSON quote sequences.
//!
//! Provides the [`QuoteClassifiedBlock`] struct and [`QuoteClassifiedIterator`] trait
//! that allow effectively enriching JSON inputs with quote sequence information.
use aligners::{alignment::Twice, AlignedBlock, AlignedSlice};
use cfg_if::cfg_if;

use crate::{debug, BlockAlignment};

/// Input block with a bitmask signifying which characters are within quotes.
///
/// Characters within quotes in the input are guaranteed to have their corresponding
/// bit in `within_quotes_mask` set. The $0$-th bit of the mask corresponds to the
/// last character in `block`, the $1$-st bit to the second-to-last character, etc.
///
/// There is no guarantee on how the boundary quote characters are classified,
/// their bits might be lit or not lit depending on the implementation.
pub struct QuoteClassifiedBlock<'a> {
    /// The block that was classified.
    pub block: &'a AlignedBlock<Twice<BlockAlignment>>,
    /// Mask marking characters within a quoted sequence.
    pub within_quotes_mask: u64,
}

impl<'a> QuoteClassifiedBlock<'a> {
    /// Returns the length of the classified block.
    pub fn len(&self) -> usize {
        self.block.len()
    }

    /// Whether the classified block is empty.
    pub fn is_empty(&self) -> bool {
        self.block.is_empty()
    }
}

/// Trait for quote classifier iterators, i.e. finite iterators
/// enriching blocks of input with quote bitmasks.
/// Iterator is allowed to hold a reference to the JSON document valid for `'a`.
pub trait QuoteClassifiedIterator<'a>:
    Iterator<Item = QuoteClassifiedBlock<'a>> + len_trait::Empty + 'a
{
    /// Get size of a single quote classified block returned by this iterator.
    fn block_size() -> usize;

    /// Get the total offset in bytes from the beginning of input.
    fn get_offset(&self) -> usize;

    /// Move the iterator `count` blocks forward.
    /// Effectively skips `count * Twice<BlockAlignment>::size()` bytes.
    fn offset(&mut self, count: isize);

    /// Flip the bit representing whether the last block ended with a nonescaped quote.
    ///
    /// This should be done only in very specific circumstances where the previous-block
    /// state could have been damaged due to stopping and resuming the classification at a later point.
    fn flip_quotes_bit(&mut self);
}

/// State allowing resumption of a classifier from a particular place
/// in the input along with the stopped [`QuoteClassifiedIterator`].
pub struct ResumeClassifierState<'a, I: QuoteClassifiedIterator<'a>> {
    /// The stopped iterator.
    pub iter: I,
    /// The block at which classification was stopped.
    pub block: Option<ResumeClassifierBlockState<'a>>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> ResumeClassifierState<'a, I> {
    /// Get the index in the original bytes input at which classification has stopped.
    pub fn get_idx(&self) -> usize {
        debug!(
            "iter offset: {}, block idx: {:?}",
            self.iter.get_offset(),
            self.block.as_ref().map(|b| b.idx)
        );

        self.iter.get_offset() + self.block.as_ref().map_or(0, |b| b.idx)
    }

    /// Move the state forward by `count` bytes.
    pub fn offset_bytes(&mut self, count: isize) {
        debug_assert!(count > 0);
        let count = count as usize;
        let remaining_in_block = self.block.as_ref().map_or(0, |b| b.block.len() - b.idx);

        if remaining_in_block > count {
            self.block.as_mut().unwrap().idx += count;
        } else {
            let blocks_to_advance = (count - remaining_in_block) / I::block_size();
            
            let remainder = (self.block.as_ref().map_or(0, |b| b.idx) + count
                - blocks_to_advance * I::block_size())
                % I::block_size();

            self.iter.offset(blocks_to_advance as isize);
            let next_block = self.iter.next();

            self.block = next_block.map(|b| ResumeClassifierBlockState {
                block: b,
                idx: remainder,
            });
        }

        debug!(
            "offset_bytes({count}) results in idx moved to {}",
            self.get_idx()
        );
    }
}

/// State of the block at which classification was stopped.
pub struct ResumeClassifierBlockState<'a> {
    /// Quote classified information about the block.
    pub block: QuoteClassifiedBlock<'a>,
    /// The index at which classification was stopped.
    pub idx: usize,
}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        use nosimd::SequentialQuoteClassifier;
        use aligners::alignment;

        /// Walk through the JSON document represented by `bytes`
        /// and classify quoted sequences.
        #[inline(always)]
        pub fn classify_quoted_sequences(
            bytes: &AlignedSlice<alignment::Twice<BlockAlignment>>,
        ) -> impl QuoteClassifiedIterator {
            SequentialQuoteClassifier::new(bytes)
        }
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        use avx2::Avx2QuoteClassifier;
        use aligners::alignment;

        /// Walk through the JSON document represented by `bytes`
        /// and classify quoted sequences.
        #[inline(always)]
        pub fn classify_quoted_sequences(
            bytes: &AlignedSlice<alignment::Twice<BlockAlignment>>,
        ) -> impl QuoteClassifiedIterator {
            Avx2QuoteClassifier::new(bytes)
        }
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}
