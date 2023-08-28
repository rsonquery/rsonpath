use super::*;
use crate::{debug, input::InputBlockIterator, FallibleIterator};
use std::marker::PhantomData;

pub(crate) struct SequentialQuoteClassifier<'i, I, const N: usize>
where
    I: InputBlockIterator<'i, N>,
{
    iter: I,
    escaped: bool,
    in_quotes: bool,
    phantom: PhantomData<&'i ()>,
}

impl<'i, I, const N: usize> SequentialQuoteClassifier<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            escaped: false,
            in_quotes: false,
            phantom: PhantomData,
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn resume(
        iter: I,
        first_block: Option<I::Block>,
    ) -> (Self, Option<QuoteClassifiedBlock<I::Block, usize, N>>) {
        let mut s = Self {
            iter,
            escaped: false,
            in_quotes: false,
            phantom: PhantomData,
        };

        let block = first_block.map(|b| s.classify_block(b));

        (s, block)
    }

    fn classify_block(&mut self, block: I::Block) -> QuoteClassifiedBlock<I::Block, usize, N> {
        let mut mask = 0_usize;
        let mut idx_mask = 1;

        for character in block.iter().copied() {
            if !self.escaped && character == b'"' {
                self.in_quotes = !self.in_quotes;
            }

            if character == b'\\' {
                self.escaped = !self.escaped;
            } else {
                self.escaped = false;
            }

            if self.in_quotes {
                mask |= idx_mask;
            }

            idx_mask <<= 1;
        }

        QuoteClassifiedBlock {
            block,
            within_quotes_mask: mask,
        }
    }
}

impl<'i, I, const N: usize> FallibleIterator for SequentialQuoteClassifier<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    type Item = QuoteClassifiedBlock<I::Block, usize, N>;
    type Error = InputError;

    #[inline(always)]
    fn next(&mut self) -> Result<Option<Self::Item>, InputError> {
        match self.iter.next()? {
            Some(block) => Ok(Some(self.classify_block(block))),
            None => Ok(None),
        }
    }
}

impl<'i, I, const N: usize> InnerIter<I> for SequentialQuoteClassifier<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    fn into_inner(self) -> I {
        self.iter
    }
}

impl<'i, I, const N: usize> QuoteClassifiedIterator<'i, I, usize, N> for SequentialQuoteClassifier<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    fn get_offset(&self) -> usize {
        self.iter.get_offset() - 64
    }

    fn offset(&mut self, count: isize) {
        debug_assert!(count >= 0);
        debug!("Offsetting by {count}");

        if count == 0 {
            return;
        }

        self.iter.offset(count);
    }

    fn flip_quotes_bit(&mut self) {
        self.in_quotes = !self.in_quotes;
    }
}
