use super::*;
use crate::input::InputBlockIterator;

pub(crate) struct SequentialQuoteClassifier<'a, I: Input + 'a, const N: usize> {
    iter: I::BlockIterator<'a, N>,
    escaped: bool,
    in_quotes: bool,
    offset: Option<usize>,
}

impl<'a, I: Input, const N: usize> SequentialQuoteClassifier<'a, I, N> {
    #[inline(always)]
    pub(crate) fn new(input: &'a I) -> Self {
        Self {
            iter: input.iter_blocks(),
            escaped: false,
            in_quotes: false,
            offset: None,
        }
    }
}

impl<'a, I: Input, const N: usize> Iterator for SequentialQuoteClassifier<'a, I, N> {
    type Item = QuoteClassifiedBlock<IBlock<'a, I, N>, N>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
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

                Some(QuoteClassifiedBlock {
                    block,
                    within_quotes_mask: mask,
                })
            }
            None => None,
        }
    }
}

impl<'a, I: Input, const N: usize> std::iter::FusedIterator for SequentialQuoteClassifier<'a, I, N> {}

impl<'a, I: Input, const N: usize> QuoteClassifiedIterator<'a, I, N> for SequentialQuoteClassifier<'a, I, N> {
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
