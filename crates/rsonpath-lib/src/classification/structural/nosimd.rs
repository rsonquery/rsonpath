use super::*;
use crate::classification::{quotes::QuoteClassifiedBlock, ResumeClassifierBlockState, ResumeClassifierState};
use crate::debug;

struct Block<'i, I, const N: usize>
where
    I: InputBlockIterator<'i, N>,
{
    quote_classified: QuoteClassifiedBlock<I::Block, usize, N>,
    idx: usize,
    are_colons_on: bool,
    are_commas_on: bool,
}

impl<'i, I, const N: usize> Block<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    fn new(
        quote_classified_block: QuoteClassifiedBlock<I::Block, usize, N>,
        are_colons_on: bool,
        are_commas_on: bool,
    ) -> Self {
        Self {
            quote_classified: quote_classified_block,
            idx: 0,
            are_colons_on,
            are_commas_on,
        }
    }

    fn from_idx(
        quote_classified_block: QuoteClassifiedBlock<I::Block, usize, N>,
        idx: usize,
        are_colons_on: bool,
        are_commas_on: bool,
    ) -> Self {
        Self {
            quote_classified: quote_classified_block,
            idx,
            are_colons_on,
            are_commas_on,
        }
    }
}

impl<'i, I, const N: usize> Iterator for Block<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    type Item = Structural;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.quote_classified.block.len() {
            let character = self.quote_classified.block[self.idx];
            let idx_mask = 1_usize << self.idx;
            let is_quoted = (self.quote_classified.within_quotes_mask & idx_mask) == idx_mask;

            let structural = match character {
                _ if is_quoted => None,
                b':' if self.are_colons_on => Some(Colon(self.idx)),
                b'{' => Some(Opening(BracketType::Curly, self.idx)),
                b'[' => Some(Opening(BracketType::Square, self.idx)),
                b',' if self.are_commas_on => Some(Comma(self.idx)),
                b'}' => Some(Closing(BracketType::Curly, self.idx)),
                b']' => Some(Closing(BracketType::Square, self.idx)),
                _ => None,
            };

            self.idx += 1;

            if structural.is_some() {
                return structural;
            };
        }

        None
    }
}

pub(crate) struct SequentialClassifier<'i, I, Q, const N: usize>
where
    I: InputBlockIterator<'i, N>,
{
    iter: Q,
    block: Option<Block<'i, I, N>>,
    are_colons_on: bool,
    are_commas_on: bool,
}

impl<'i, I, Q, const N: usize> SequentialClassifier<'i, I, Q, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, usize, N>,
{
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn new(iter: Q) -> Self {
        Self {
            iter,
            block: None,
            are_colons_on: false,
            are_commas_on: false,
        }
    }

    #[inline]
    fn reclassify(&mut self, idx: usize) {
        if let Some(block) = self.block.take() {
            let quote_classified_block = block.quote_classified;
            let relevant_idx = idx + 1;
            let block_idx = (idx + 1) % N;
            debug!("relevant_idx is {relevant_idx}.");

            if block_idx != 0 || relevant_idx == self.iter.get_offset() {
                let new_block = Block::from_idx(
                    quote_classified_block,
                    block_idx,
                    self.are_colons_on,
                    self.are_commas_on,
                );
                self.block = Some(new_block);
            }
        }
    }
}

impl<'i, I, Q, const N: usize> FallibleIterator for SequentialClassifier<'i, I, Q, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, usize, N>,
{
    type Item = Structural;
    type Error = InputError;

    #[inline(always)]
    fn next(&mut self) -> Result<Option<Structural>, InputError> {
        let mut item = self.block.as_mut().and_then(Iterator::next);

        while item.is_none() {
            match self.iter.next()? {
                Some(block) => {
                    let mut block = Block::new(block, self.are_colons_on, self.are_commas_on);
                    item = block.next();
                    self.block = Some(block);
                }
                None => return Ok(None),
            }
        }

        Ok(item.map(|x| x.offset(self.iter.get_offset())))
    }
}

impl<'i, I, Q, const N: usize> StructuralIterator<'i, I, Q, usize, N> for SequentialClassifier<'i, I, Q, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, usize, N>,
{
    fn turn_colons_and_commas_on(&mut self, idx: usize) {
        if !self.are_commas_on && !self.are_colons_on {
            self.are_commas_on = true;
            self.are_colons_on = true;
            debug!("Turning both commas and colons on at {idx}.");

            self.reclassify(idx);
        } else if !self.are_commas_on {
            self.turn_commas_on(idx);
        } else if !self.are_colons_on {
            self.turn_colons_on(idx);
        }
    }

    fn turn_colons_and_commas_off(&mut self) {
        if self.are_commas_on && self.are_colons_on {
            self.are_commas_on = false;
            self.are_colons_on = false;
            debug!("Turning both commas and colons off.");
        } else if self.are_commas_on {
            self.turn_commas_off();
        } else if self.are_colons_on {
            self.turn_colons_off();
        }
    }

    fn turn_commas_on(&mut self, idx: usize) {
        if !self.are_commas_on {
            self.are_commas_on = true;
            debug!("Turning commas on at {idx}.");

            self.reclassify(idx);
        }
    }

    fn turn_commas_off(&mut self) {
        self.are_commas_on = false;
        debug!("Turning commas off.");
    }

    fn turn_colons_on(&mut self, idx: usize) {
        if !self.are_colons_on {
            self.are_colons_on = true;
            debug!("Turning colons on at {idx}.");

            self.reclassify(idx);
        }
    }

    fn turn_colons_off(&mut self) {
        self.are_colons_on = false;
        debug!("Turning colons off.");
    }

    fn stop(self) -> ResumeClassifierState<'i, I, Q, usize, N> {
        let block = self.block.map(|b| ResumeClassifierBlockState {
            block: b.quote_classified,
            idx: b.idx,
        });
        ResumeClassifierState {
            iter: self.iter,
            block,
            are_colons_on: self.are_colons_on,
            are_commas_on: self.are_commas_on,
        }
    }

    fn resume(state: ResumeClassifierState<'i, I, Q, usize, N>) -> Self {
        Self {
            iter: state.iter,
            block: state.block.map(|b| Block {
                quote_classified: b.block,
                idx: b.idx,
                are_commas_on: state.are_commas_on,
                are_colons_on: state.are_colons_on,
            }),
            are_commas_on: state.are_commas_on,
            are_colons_on: state.are_colons_on,
        }
    }
}
