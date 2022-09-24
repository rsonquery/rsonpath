use super::*;
use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState};
use std::marker::PhantomData;

pub struct VectorIterator<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    phantom: PhantomData<&'a I>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> VectorIterator<'a, I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            phantom: PhantomData,
        }
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for VectorIterator<'a, I> {
    type Item = Vector<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let quote_classified = self.iter.next();
        quote_classified.map(Vector::new)
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> DepthIterator<'a, I> for VectorIterator<'a, I> {
    type Block = Vector<'a>;

    fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'a, I> {
        let block_state = block.and_then(|b| {
            if b.idx >= b.quote_classified.len() {
                None
            } else {
                Some(ResumeClassifierBlockState {
                    block: b.quote_classified,
                    idx: b.idx,
                })
            }
        });

        ResumeClassifierState {
            iter: self.iter,
            block: block_state,
        }
    }

    fn resume(state: ResumeClassifierState<'a, I>) -> (Option<Self::Block>, Self) {
        let first_block = state.block.map(|b| Vector::new_from(b.block, b.idx));

        (
            first_block,
            VectorIterator {
                iter: state.iter,
                phantom: PhantomData,
            },
        )
    }
}

/// Decorates a byte slice with JSON depth information.
///
/// This struct works on the entire slice and calculates the depth sequentially.
pub struct Vector<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    depth: isize,
    idx: usize,
}

impl<'a> Vector<'a> {
    #[inline]
    pub(crate) fn new(bytes: QuoteClassifiedBlock<'a>) -> Self {
        Self::new_from(bytes, 0)
    }

    #[inline]
    fn new_from(bytes: QuoteClassifiedBlock<'a>, idx: usize) -> Self {
        let mut vector = Self {
            quote_classified: bytes,
            depth: 0,
            idx,
        };
        vector.advance();
        vector
    }

    #[inline]
    fn map_depths<F: FnMut(isize)>(&self, mut f: F) {
        let mut current = self.depth;
        f(current);
        let mut offset = 0;

        while self.idx + offset < self.quote_classified.len() {
            let character = self.quote_classified.block[self.idx];
            let idx_mask = 1u64 << self.idx;
            let is_quoted = (self.quote_classified.within_quotes_mask & idx_mask) == idx_mask;

            if !is_quoted {
                current += match character {
                    b'{' => 1,
                    b'[' => 1,
                    b'}' => -1,
                    b']' => -1,
                    _ => 0,
                };
            }

            f(current);
            offset += 1;
        }
    }
}

impl<'a> DepthBlock<'a> for Vector<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.quote_classified.len()
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.idx >= self.quote_classified.len() {
            return false;
        }

        let character = self.quote_classified.block[self.idx];
        let idx_mask = 1u64 << self.idx;
        let is_quoted = (self.quote_classified.within_quotes_mask & idx_mask) == idx_mask;

        if !is_quoted {
            self.depth += match character {
                b'{' => 1,
                b'[' => 1,
                b'}' => -1,
                b']' => -1,
                _ => 0,
            };
        }
        self.idx += 1;

        true
    }

    #[inline]
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        self.depth >= depth
    }

    #[inline]
    fn depth_at_end(&self) -> isize {
        let mut current = 0;
        self.map_depths(|x| current = x);

        current
    }

    #[inline]
    fn advance_to_next_depth_change(&mut self) -> bool {
        self.advance()
    }

    #[inline]
    fn set_starting_depth(&mut self, depth: isize) {
        self.depth += depth;
    }

    fn estimate_lowest_possible_depth(&self) -> isize {
        let mut lowest = 0;
        self.map_depths(|x| lowest = std::cmp::min(lowest, x));

        lowest
    }
}
