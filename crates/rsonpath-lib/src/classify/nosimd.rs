use super::*;
use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState, ResumeClassifierState};

struct Block<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    idx: usize,
}

impl<'a> Block<'a> {
    fn new(quote_classified_block: QuoteClassifiedBlock<'a>) -> Self {
        Self {
            quote_classified: quote_classified_block,
            idx: 0,
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

            if !is_quoted {
                match character {
                    b':' => return Some(Colon(self.idx - 1)),
                    b'[' | b'{' => return Some(Opening(self.idx - 1)),
                    b',' => return Some(Comma(self.idx - 1)),
                    b']' | b'}' => return Some(Closing(self.idx - 1)),
                    _ => (),
                }
            }
        }

        None
    }
}

pub(crate) struct SequentialClassifier<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    block: Option<Block<'a>>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> SequentialClassifier<'a, I> {
    #[inline(always)]
    pub(crate) fn new(iter: I) -> Self {
        Self { iter, block: None }
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
                    let mut block = Block::new(block);
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
    fn stop(self) -> ResumeClassifierState<'a, I> {
        let block = self.block.map(|b| ResumeClassifierBlockState {
            block: b.quote_classified,
            idx: b.idx,
        });
        ResumeClassifierState {
            iter: self.iter,
            block,
        }
    }

    fn resume(state: ResumeClassifierState<'a, I>) -> Self {
        Self {
            iter: state.iter,
            block: state.block.map(|b| Block {
                quote_classified: b.block,
                idx: b.idx,
            }),
        }
    }
}
