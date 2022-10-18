use super::*;
use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState};
use std::marker::PhantomData;

pub(crate) struct VectorIterator<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    opening: u8,
    phantom: PhantomData<&'a I>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> VectorIterator<'a, I> {
    pub(crate) fn new(iter: I, opening: u8) -> Self {
        Self {
            iter,
            opening,
            phantom: PhantomData,
        }
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for VectorIterator<'a, I> {
    type Item = Vector<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let quote_classified = self.iter.next();
        quote_classified.map(|b| Vector::new(b, self.opening))
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

    fn resume(state: ResumeClassifierState<'a, I>, opening: u8) -> (Option<Self::Block>, Self) {
        let first_block = state
            .block
            .map(|b| Vector::new_from(b.block, opening, b.idx));

        (
            first_block,
            VectorIterator {
                iter: state.iter,
                opening,
                phantom: PhantomData,
            },
        )
    }
}

pub(crate) struct Vector<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    depth: isize,
    idx: usize,
    opening: u8,
}

impl<'a> Vector<'a> {
    #[inline]
    pub(crate) fn new(bytes: QuoteClassifiedBlock<'a>, opening: u8) -> Self {
        Self::new_from(bytes, opening, 0)
    }

    #[inline]
    fn new_from(bytes: QuoteClassifiedBlock<'a>, opening: u8, idx: usize) -> Self {
        Self {
            quote_classified: bytes,
            depth: 0,
            idx,
            opening,
        }
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
    fn get_depth(&self) -> isize {
        self.depth
    }

    #[inline]
    fn depth_at_end(&self) -> isize {
        let mut current = 0;
        self.map_depths(|x| current = x);

        current
    }

    #[inline]
    fn advance_to_next_depth_decrease(&mut self) -> bool {
        let closing = self.opening + 2;
        while self.idx < self.quote_classified.len() {
            let character = self.quote_classified.block[self.idx];
            self.idx += 1;

            if character == self.opening {
                self.depth += 1;
            } else if character == closing {
                self.depth -= 1;
                return true;
            }
        }

        false
    }

    #[inline]
    fn add_depth(&mut self, depth: isize) {
        self.depth += depth;
    }

    fn estimate_lowest_possible_depth(&self) -> isize {
        let mut lowest = 0;
        self.map_depths(|x| lowest = std::cmp::min(lowest, x));

        lowest
    }

    fn remaining_depth_increase(&self) -> isize {
        todo!()
    }
}
