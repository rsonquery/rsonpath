use super::*;
use crate::{
    classification::{QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
};
use std::marker::PhantomData;

pub(crate) struct Constructor;

impl DepthImpl for Constructor {
    type Classifier<'i, I, Q>
        = VectorIterator<'i, I, Q, BLOCK_SIZE>
    where
        I: InputBlockIterator<'i, BLOCK_SIZE>,
        Q: QuoteClassifiedIterator<'i, I, MaskType, BLOCK_SIZE>;
}

pub(crate) struct VectorIterator<'i, I, Q, const N: usize> {
    iter: Q,
    opening: BracketType,
    were_commas_on: bool,
    were_colons_on: bool,
    phantom: PhantomData<(&'i (), I)>,
}

impl<'i, I, Q, const N: usize> FallibleIterator for VectorIterator<'i, I, Q, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
{
    type Item = Vector<'i, I, N>;
    type Error = InputError;

    fn next(&mut self) -> Result<Option<Self::Item>, InputError> {
        let quote_classified = self.iter.next()?;
        Ok(quote_classified.map(|b| Vector::new(b, self.opening)))
    }
}

impl<'i, I, Q, const N: usize> DepthIterator<'i, I, Q, MaskType, N> for VectorIterator<'i, I, Q, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
{
    type Block = Vector<'i, I, N>;

    fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'i, I, Q, MaskType, N> {
        let block_state = block.map(|b| {
            let idx = b.idx;
            debug!("Depth iterator stopping at index {idx}");
            ResumeClassifierBlockState {
                block: b.quote_classified,
                idx,
            }
        });

        ResumeClassifierState {
            iter: self.iter,
            block: block_state,
            are_commas_on: self.were_commas_on,
            are_colons_on: self.were_colons_on,
        }
    }

    fn resume(
        state: ResumeClassifierState<'i, I, Q, MaskType, N>,
        opening: BracketType,
    ) -> (Option<Self::Block>, Self) {
        let first_block = state.block.map(|b| Vector::new_from(b.block, opening, b.idx));

        (
            first_block,
            VectorIterator {
                iter: state.iter,
                opening,
                phantom: PhantomData,
                were_commas_on: state.are_commas_on,
                were_colons_on: state.are_colons_on,
            },
        )
    }
}

pub(crate) struct Vector<'i, I, const N: usize>
where
    I: InputBlockIterator<'i, N>,
{
    quote_classified: QuoteClassifiedBlock<I::Block, MaskType, N>,
    depth: isize,
    idx: usize,
    bracket_type: BracketType,
}

impl<'i, I, const N: usize> Vector<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    #[inline]
    pub(crate) fn new(bytes: QuoteClassifiedBlock<I::Block, MaskType, N>, opening: BracketType) -> Self {
        Self::new_from(bytes, opening, 0)
    }

    #[inline]
    fn new_from(bytes: QuoteClassifiedBlock<I::Block, MaskType, N>, opening: BracketType, idx: usize) -> Self {
        Self {
            quote_classified: bytes,
            depth: 0,
            idx,
            bracket_type: opening,
        }
    }

    #[inline]
    fn map_depths<F: FnMut(isize)>(&self, mut f: F) {
        let mut current = self.depth;
        f(current);
        let mut offset = 0;

        while self.idx + offset < self.quote_classified.len() {
            if let Some(character) = self.get_char(self.idx + offset) {
                current += match character {
                    b'{' | b'[' => 1,
                    b'}' | b']' => -1,
                    _ => 0,
                };
            }

            f(current);
            offset += 1;
        }
    }

    #[inline(always)]
    fn get_char(&self, idx: usize) -> Option<u8> {
        let idx_mask = 1 << idx;
        let is_quoted = (self.quote_classified.within_quotes_mask & idx_mask) == idx_mask;

        if is_quoted {
            None
        } else {
            let character = self.quote_classified.block[idx];
            Some(character)
        }
    }
}

impl<'i, I, const N: usize> DepthBlock<'i> for Vector<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
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
        let (opening, closing) = match self.bracket_type {
            BracketType::Square => (b'[', b']'),
            BracketType::Curly => (b'{', b'}'),
        };
        while self.idx < self.quote_classified.len() {
            let character = self.get_char(self.idx);
            self.idx += 1;

            if character == Some(opening) {
                self.depth += 1;
            } else if character == Some(closing) {
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
}
