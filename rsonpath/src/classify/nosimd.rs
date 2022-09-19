use super::*;
use crate::quotes::QuoteClassifiedBlock;
use std::iter::Peekable;

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
            let idx_mask = 1u64 << (self.quote_classified.block.len() - self.idx - 1);
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
    block: Option<Peekable<Block<'a>>>,
    offset: usize,
}

impl<'a, I: QuoteClassifiedIterator<'a>> SequentialClassifier<'a, I> {
    #[inline(always)]
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            block: None,
            offset: 0,
        }
    }

    #[inline(always)]
    fn next_block(&mut self) -> bool {
        while self.current_block_is_spent() {
            match self.iter.next() {
                Some(block) => {
                    if self.block.is_some() {
                        self.offset += block.len();
                    }
                    self.block = Some(Block::new(block).peekable());
                }
                None => return false,
            }
        }

        true
    }

    #[inline(always)]
    fn current_block_is_spent(&mut self) -> bool {
        self.block.as_mut().map_or(true, |x| x.peek().is_none())
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for SequentialClassifier<'a, I> {
    type Item = Structural;

    #[inline(always)]
    fn next(&mut self) -> Option<Structural> {
        if !self.next_block() {
            return None;
        }
        self.block
            .as_mut()
            .unwrap()
            .next()
            .map(|x| x.offset(self.offset))
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> std::iter::FusedIterator for SequentialClassifier<'a, I> {}

impl<'a, I: QuoteClassifiedIterator<'a>> StructuralIterator<'a> for SequentialClassifier<'a, I> {}
