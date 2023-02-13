use super::*;
use crate::classification::quotes::{
    QuoteClassifiedBlock, ResumeClassifierBlockState, ResumeClassifierState,
};
use crate::debug;

struct Block<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    idx: usize,
    are_colons_on: bool,
    are_commas_on: bool,
}

impl<'a> Block<'a> {
    fn new(
        quote_classified_block: QuoteClassifiedBlock<'a>,
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
        quote_classified_block: QuoteClassifiedBlock<'a>,
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

impl<'a> Iterator for Block<'a> {
    type Item = Structural;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.quote_classified.block.len() {
            let character = self.quote_classified.block[self.idx];
            let idx_mask = 1_u64 << self.idx;
            let is_quoted = (self.quote_classified.within_quotes_mask & idx_mask) == idx_mask;

            self.idx += 1;

            match character {
                _ if is_quoted => (),
                b':' if self.are_colons_on => return Some(Colon(self.idx - 1)),
                b'[' | b'{' => return Some(Opening(self.idx - 1)),
                b',' if self.are_commas_on => return Some(Comma(self.idx - 1)),
                b']' | b'}' => return Some(Closing(self.idx - 1)),
                _ => (),
            }
        }

        None
    }
}

pub(crate) struct SequentialClassifier<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    block: Option<Block<'a>>,
    are_colons_on: bool,
    are_commas_on: bool,
}

impl<'a, I: QuoteClassifiedIterator<'a>> SequentialClassifier<'a, I> {
    #[inline(always)]
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            block: None,
            are_colons_on: false,
            are_commas_on: false,
        }
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for SequentialClassifier<'a, I> {
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

impl<'a, I: QuoteClassifiedIterator<'a>> std::iter::FusedIterator for SequentialClassifier<'a, I> {}

impl<'a, I: QuoteClassifiedIterator<'a>> StructuralIterator<'a, I> for SequentialClassifier<'a, I> {
    fn turn_commas_on(&mut self, idx: usize) {
        if !self.are_commas_on {
            self.are_commas_on = true;
            debug!("Turning commas on at {idx}.");

            if let Some(block) = self.block.take() {
                let quote_classified_block = block.quote_classified;
                let block_idx = (idx + 1) % I::block_size();

                if block_idx != 0 {
                    let new_block = Block::from_idx(
                        quote_classified_block,
                        block_idx,
                        self.are_colons_on,
                        true,
                    );
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
                let block_idx = (idx + 1) % I::block_size();

                if block_idx != 0 {
                    let new_block = Block::from_idx(
                        quote_classified_block,
                        block_idx,
                        true,
                        self.are_commas_on,
                    );
                    self.block = Some(new_block);
                }
            }
        }
    }

    fn turn_colons_off(&mut self) {
        self.are_colons_on = false;
        debug!("Turning colons off.");
    }

    fn stop(self) -> ResumeClassifierState<'a, I> {
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

    fn resume(state: ResumeClassifierState<'a, I>) -> Self {
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
