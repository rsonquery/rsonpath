use super::*;
use crate::{debug, input::InputBlockIterator, FallibleIterator};

pub(crate) struct SequentialQuoteClassifier<'a, 'r, I: Input + 'a, R: InputRecorder, const N: usize>
where
    R: 'r,
{
    iter: I::BlockIterator<'a, 'r, N, R>,
    escaped: bool,
    in_quotes: bool,
}

impl<'a, 'r, I: Input, R: InputRecorder, const N: usize> SequentialQuoteClassifier<'a, 'r, I, R, N> {
    #[inline(always)]
    pub(crate) fn new(input: &'a I, recorder: &'r R) -> Self {
        Self {
            iter: input.iter_blocks(recorder),
            escaped: false,
            in_quotes: false,
        }
    }

    #[inline]
    pub(crate) fn resume(
        iter: I::BlockIterator<'a, 'r, N, R>,
        first_block: Option<I::Block<'a, N>>,
    ) -> (Self, Option<QuoteClassifiedBlock<I::Block<'a, N>, N>>) {
        let mut s = Self {
            iter,
            escaped: false,
            in_quotes: false,
        };

        let block = first_block.map(|b| s.classify_block(b));

        (s, block)
    }

    fn classify_block(&mut self, block: I::Block<'a, N>) -> QuoteClassifiedBlock<I::Block<'a, N>, N> {
        let mut mask = 0_u64;
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

impl<'a, 'r, I: Input, R: InputRecorder, const N: usize> FallibleIterator
    for SequentialQuoteClassifier<'a, 'r, I, R, N>
{
    type Item = QuoteClassifiedBlock<I::Block<'a, N>, N>;
    type Error = InputError;

    #[inline(always)]
    fn next(&mut self) -> Result<Option<Self::Item>, InputError> {
        match self.iter.next()? {
            Some(block) => Ok(Some(self.classify_block(block))),
            None => Ok(None),
        }
    }
}

impl<'a, 'r, I: Input, R: InputRecorder, const N: usize> InnerIter<'a, 'r, I, R, N>
    for SequentialQuoteClassifier<'a, 'r, I, R, N>
{
    fn into_inner(self) -> I::BlockIterator<'a, 'r, N, R> {
        self.iter
    }
}

impl<'a, 'r, I: Input, R: InputRecorder, const N: usize> QuoteClassifiedIterator<'a, I, N>
    for SequentialQuoteClassifier<'a, 'r, I, R, N>
where
    Self: 'a,
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
