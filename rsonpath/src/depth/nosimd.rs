use std::marker::PhantomData;

use crate::quotes::QuoteClassifiedBlock;

use super::*;

pub(crate) struct VectorIterator<'a, I: QuoteClassifiedIterator<'a>> {
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

impl<'a, I: QuoteClassifiedIterator<'a>> DepthIterator<'a> for VectorIterator<'a, I> {
    type Block = Vector<'a>;
}

/// Decorates a byte slice with JSON depth information.
///
/// This struct works on the entire slice and calculates the depth sequentially.
pub(crate) struct Vector<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    depth: isize,
    idx: usize,
}

impl<'a> Vector<'a> {
    /// The remainder is guaranteed to be an empty slice,
    /// since this implementation works on the entire byte
    /// slice at once.
    #[inline]
    pub(crate) fn new(bytes: QuoteClassifiedBlock<'a>) -> Self {
        let mut vector = Self {
            quote_classified: bytes,
            depth: 0,
            idx: 0,
        };
        vector.advance();
        vector
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
    fn depth_at_end(mut self) -> isize {
        while self.advance() {}
        self.depth
    }

    #[inline]
    fn set_starting_depth(&mut self, depth: isize) {
        self.depth += depth;
    }

    fn estimate_lowest_possible_depth(&self) -> isize {
        let mut current = self.depth;
        let mut lowest = self.depth;
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

                lowest = std::cmp::min(lowest, current);
            }

            offset += 1;
        }
        
        lowest
    }
}