use super::*;
use crate::classification::{quotes::QuoteClassifiedBlock, ResumeClassifierBlockState, ResumeClassifierState};
use crate::debug;
use crate::input::IBlock;

struct Block<'a, I: Input + 'a, const N: usize> {
    quote_classified: QuoteClassifiedBlock<IBlock<'a, I, N>, N>,
    idx: usize,
    are_colons_on: bool,
    are_commas_on: bool,
}

impl<'a, I: Input, const N: usize> Block<'a, I, N> {
    fn new(
        quote_classified_block: QuoteClassifiedBlock<IBlock<'a, I, N>, N>,
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
        quote_classified_block: QuoteClassifiedBlock<IBlock<'a, I, N>, N>,
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

impl<'a, I: Input, const N: usize> Iterator for Block<'a, I, N> {
    type Item = Structural;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.quote_classified.block.len() {
            let character = self.quote_classified.block[self.idx];
            let idx_mask = 1_u64 << self.idx;
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

pub(crate) struct SequentialClassifier<'a, I: Input, Q, const N: usize> {
    iter: Q,
    block: Option<Block<'a, I, N>>,
    are_colons_on: bool,
    are_commas_on: bool,
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, N>, const N: usize> SequentialClassifier<'a, I, Q, N> {
    #[inline(always)]
    pub(crate) fn new(iter: Q) -> Self {
        Self {
            iter,
            block: None,
            are_colons_on: false,
            are_commas_on: false,
        }
    }
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, N>, const N: usize> Iterator
    for SequentialClassifier<'a, I, Q, N>
{
    type Item = Structural;

    #[inline(always)]
    fn next(&mut self) -> Option<Structural> {
        let mut item = self.block.as_mut().and_then(Iterator::next);

        while item.is_none() {
            match self.iter.next() {
                Some(block) => {
                    let mut block = Block::new(block, self.are_colons_on, self.are_commas_on);
                    item = block.next();
                    self.block = Some(block);
                }
                None => return None,
            }
        }

        item.map(|x| x.offset(self.iter.get_offset()))
    }
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, N>, const N: usize> std::iter::FusedIterator
    for SequentialClassifier<'a, I, Q, N>
{
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, N>, const N: usize> StructuralIterator<'a, I, Q, N>
    for SequentialClassifier<'a, I, Q, N>
{
    fn turn_commas_on(&mut self, idx: usize) {
        if !self.are_commas_on {
            self.are_commas_on = true;
            debug!("Turning commas on at {idx}.");

            if let Some(block) = self.block.take() {
                let quote_classified_block = block.quote_classified;
                let block_idx = (idx + 1) % N;

                if block_idx != 0 {
                    let new_block = Block::from_idx(quote_classified_block, block_idx, self.are_colons_on, true);
                    self.block = Some(new_block);
                }
            }
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

            if let Some(block) = self.block.take() {
                let quote_classified_block = block.quote_classified;
                let block_idx = (idx + 1) % N;

                if block_idx != 0 {
                    let new_block = Block::from_idx(quote_classified_block, block_idx, true, self.are_commas_on);
                    self.block = Some(new_block);
                }
            }
        }
    }

    fn turn_colons_off(&mut self) {
        self.are_colons_on = false;
        debug!("Turning colons off.");
    }

    fn stop(self) -> ResumeClassifierState<'a, I, Q, N> {
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

    fn resume(state: ResumeClassifierState<'a, I, Q, N>) -> Self {
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
