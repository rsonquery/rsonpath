use super::*;
use crate::{input::InputBlockIterator, FallibleIterator};

pub(crate) struct SequentialQuoteClassifier<'a, 'r, I: Input + 'a, R: InputRecorder, const N: usize>
where
    R: 'r,
{
    iter: I::BlockIterator<'a, 'r, N, R>,
    escaped: bool,
    in_quotes: bool,
    offset: Option<usize>,
}

impl<'a, 'r, I: Input, R: InputRecorder, const N: usize> SequentialQuoteClassifier<'a, 'r, I, R, N> {
    #[inline(always)]
    pub(crate) fn new(input: &'a I, recorder: &'r R) -> Self {
        Self {
            iter: input.iter_blocks(recorder),
            escaped: false,
            in_quotes: false,
            offset: None,
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
            Some(block) => {
                let mut mask = 0_u64;
                let mut idx_mask = 1;

                if let Some(offset) = self.offset {
                    self.offset = Some(offset + N);
                } else {
                    self.offset = Some(0);
                }

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

                Ok(Some(QuoteClassifiedBlock {
                    block,
                    within_quotes_mask: mask,
                }))
            }
            None => Ok(None),
        }
    }
}

impl<'a, 'r, I: Input, R: InputRecorder, const N: usize> QuoteClassifiedIterator<'a, I, N>
    for SequentialQuoteClassifier<'a, 'r, I, R, N>
where
    Self: 'a,
{
    fn get_offset(&self) -> usize {
        self.offset.unwrap_or(0)
    }

    fn offset(&mut self, count: isize) {
        if count == 0 {
            return;
        }

        self.iter.offset(count);
        self.offset = Some(match self.offset {
            None => (count as usize - 1) * BLOCK_SIZE,
            Some(offset) => offset + (count as usize) * BLOCK_SIZE,
        });
    }

    fn flip_quotes_bit(&mut self) {
        self.in_quotes = !self.in_quotes;
    }
}
