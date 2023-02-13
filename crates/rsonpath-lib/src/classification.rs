//! TODO

pub mod depth;
pub mod quotes;
pub mod structural;

use crate::debug;
use quotes::{QuoteClassifiedBlock, QuoteClassifiedIterator};

/// State allowing resumption of a classifier from a particular place
/// in the input along with the stopped [`QuoteClassifiedIterator`].
pub struct ResumeClassifierState<'a, I: QuoteClassifiedIterator<'a>> {
    /// The stopped iterator.
    pub iter: I,
    /// The block at which classification was stopped.
    pub block: Option<ResumeClassifierBlockState<'a>>,
    /// Was comma classification turned on when the classification was stopped.
    pub are_commas_on: bool,
    /// Was colon classification turned on when the classification was stopped.
    pub are_colons_on: bool,
}

/// State of the block at which classification was stopped.
pub struct ResumeClassifierBlockState<'a> {
    /// Quote classified information about the block.
    pub block: QuoteClassifiedBlock<'a>,
    /// The index at which classification was stopped.
    pub idx: usize,
}

impl<'a, I: QuoteClassifiedIterator<'a>> ResumeClassifierState<'a, I> {
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
    #[inline]
    pub fn offset_bytes(&mut self, count: isize) {
        debug_assert!(count > 0);
        let count = count as usize;

        let remaining_in_block = self.block.as_ref().map_or(0, |b| b.block.len() - b.idx);

        match self.block.as_mut() {
            Some(b) if b.block.len() - b.idx > count => {
                b.idx += count;
            }
            _ => {
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
        }

        debug!(
            "offset_bytes({count}) results in idx moved to {}",
            self.get_idx()
        );
    }
}
