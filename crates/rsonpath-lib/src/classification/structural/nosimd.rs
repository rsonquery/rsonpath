use super::*;
use crate::classification::{mask::Mask, quotes::QuoteClassifiedBlock, ResumeClassifierBlockState};
use crate::debug;

pub(crate) struct Constructor;

impl StructuralImpl for Constructor {
    type Classifier<'i, I, Q>
        = SequentialClassifier<'i, I, Q, BLOCK_SIZE>
    where
        I: InputBlockIterator<'i, BLOCK_SIZE>,
        Q: QuoteClassifiedIterator<'i, I, MaskType, BLOCK_SIZE>;

    #[inline(always)]
    #[allow(dead_code)]
    fn new<'i, I, Q>(iter: Q) -> Self::Classifier<'i, I, Q>
    where
        I: InputBlockIterator<'i, BLOCK_SIZE>,
        Q: QuoteClassifiedIterator<'i, I, MaskType, BLOCK_SIZE>,
    {
        Self::Classifier {
            iter,
            block: None,
            are_colons_on: false,
            are_commas_on: false,
        }
    }
}

struct Block<'i, I, const N: usize>
where
    I: InputBlockIterator<'i, N>,
{
    quote_classified: QuoteClassifiedBlock<I::Block, MaskType, N>,
    idx: usize,
    are_colons_on: bool,
    are_commas_on: bool,
}

impl<'i, I, const N: usize> Block<'i, I, N>
where
    I: InputBlockIterator<'i, N>,
{
    fn new(
        quote_classified_block: QuoteClassifiedBlock<I::Block, MaskType, N>,
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
        quote_classified_block: QuoteClassifiedBlock<I::Block, MaskType, N>,
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
            let idx_mask = 1 << self.idx;
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
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
{
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
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
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

impl<'i, I, Q, const N: usize> StructuralIterator<'i, I, Q, MaskType, N> for SequentialClassifier<'i, I, Q, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
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

    fn stop(self) -> ResumeClassifierState<'i, I, Q, MaskType, N> {
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

    fn resume(state: ResumeClassifierState<'i, I, Q, MaskType, N>) -> Self {
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

    // TODO Ricardo Structural Classifier
    fn jump_to_idx(&mut self, idx: usize, expect_quoted: bool) -> Result<(), InputError> {
        let block_idx = idx % N;
        // 2. S tells its quote classifier to jump and retrieve that new block
        if let Some(mut jump_to_block) = self.iter.jump_to_idx(idx)? {
            // 6. S needs to reclassify the new current block.
            // This is the same edge-case as in head-skipping where we might happen to jump into a block that starts
            // in the middle of a string. In that case the quote classifier will be wrong about everything.
            // Consider a block:
            //               block start    jump-to point
            //                   v               v
            // input:      ..."abcdefg": [1,2,3] }
            // quote mask:       00000111111111111
            //
            // We use the `expect_quoted` parameter to resolve this issue. The code that jumps should know if
            // it's jumping to a character that ought to be quoted or not. In the case of tail-skipping we always
            // jump to a structural closing symbol, which must be unquoted. If we detect that the quote classifier
            // is wrong, we can tell it to simply flip its state and it'll be correct.
            if jump_to_block.within_quotes_mask.is_lit(block_idx) != expect_quoted {
                debug!("Mask needs flipping!");
                jump_to_block.within_quotes_mask = !jump_to_block.within_quotes_mask;
                self.iter.flip_quotes_bit();
            }
            self.block = Some(Block::from_idx(
                jump_to_block,
                block_idx,
                self.are_colons_on,
                self.are_commas_on,
            ));
        }
        // If there was no jump then it is contained fully within the current block.
        // We need to advance the inner index.
        else if let Some(curr_block) = self.block.as_mut() {
            curr_block.idx = block_idx;
        }
        Ok(())
    }
}
