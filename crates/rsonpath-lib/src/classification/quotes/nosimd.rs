use super::*;
use crate::{debug, input::error::InputErrorConvertible as _};
use std::marker::PhantomData;

pub(crate) struct Constructor;

impl QuotesImpl for Constructor {
    type Classifier<'i, I>
        = SequentialQuoteClassifier<'i, I, BLOCK_SIZE>
    where
        I: InputBlockIterator<'i, BLOCK_SIZE>;

    #[inline(always)]
    fn new<'i, I>(iter: I) -> Self::Classifier<'i, I>
    where
        I: InputBlockIterator<'i, BLOCK_SIZE>,
    {
        SequentialQuoteClassifier {
            iter,
            escaped: false,
            in_quotes: false,
            phantom: PhantomData,
        }
    }

    fn resume<'i, I>(
        iter: I,
        first_block: Option<I::Block>,
    ) -> ResumedQuoteClassifier<Self::Classifier<'i, I>, I::Block, MaskType, BLOCK_SIZE>
    where
        I: InputBlockIterator<'i, BLOCK_SIZE>,
    {
        let mut s = SequentialQuoteClassifier {
            iter,
            escaped: false,
            in_quotes: false,
            phantom: PhantomData,
        };

        let block = first_block.map(|b| s.classify_block(b));

        ResumedQuoteClassifier {
            classifier: s,
            first_block: block,
        }
    }
}

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
    fn classify_block(&mut self, block: I::Block) -> QuoteClassifiedBlock<I::Block, MaskType, N> {
        let mut mask: MaskType = 0;
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
    type Item = QuoteClassifiedBlock<I::Block, MaskType, N>;
    type Error = InputError;

    #[inline(always)]
    fn next(&mut self) -> Result<Option<Self::Item>, InputError> {
        match self.iter.next().e()? {
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

impl<'i, I, const N: usize> QuoteClassifiedIterator<'i, I, MaskType, N> for SequentialQuoteClassifier<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    fn get_offset(&self) -> usize {
        self.iter.get_offset() - N
    }

    fn offset(&mut self, count: isize) -> QuoteIterResult<I::Block, MaskType, N> {
        debug_assert!(count > 0, "must not offset by zero");
        debug!("Offsetting by {count}");

        // This cannot overflow due to the assertion above.
        for _ in 0..count - 1 {
            self.iter.next().e()?;
        }

        self.next()
    }

    fn flip_quotes_bit(&mut self) {
        self.in_quotes = !self.in_quotes;
    }
}
