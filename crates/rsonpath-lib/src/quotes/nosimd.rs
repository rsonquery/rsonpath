use super::*;
use aligners::alignment::Alignment;
use aligners::{AlignedBlockIterator, AlignedSlice};

pub(crate) struct SequentialQuoteClassifier<'a> {
    iter: AlignedBlockIterator<'a, alignment::Twice<BlockAlignment>>,
    escaped: bool,
    in_quotes: bool,
    offset: Option<usize>,
}

impl<'a> SequentialQuoteClassifier<'a> {
    #[inline(always)]
    pub(crate) fn new(bytes: &'a AlignedSlice<alignment::Twice<BlockAlignment>>) -> Self {
        Self {
            iter: bytes.iter_blocks(),
            escaped: false,
            in_quotes: false,
            offset: None,
        }
    }
}

impl<'a> Iterator for SequentialQuoteClassifier<'a> {
    type Item = QuoteClassifiedBlock<'a>;

    #[inline(always)]
    fn next(&mut self) -> Option<QuoteClassifiedBlock<'a>> {
        match self.iter.next() {
            Some(block) => {
                let mut mask = 0u64;
                let mut idx_mask = 1;

                if let Some(offset) = self.offset {
                    self.offset = Some(offset + Self::block_size());
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

impl<'a> std::iter::FusedIterator for SequentialQuoteClassifier<'a> {}

impl<'a> QuoteClassifiedIterator<'a> for SequentialQuoteClassifier<'a> {
    fn block_size() -> usize {
        Twice::<BlockAlignment>::size()
    }

    fn is_empty(&self) -> bool {
        self.iter.len() == 0
    }

    fn get_offset(&self) -> usize {
        self.offset.unwrap_or(0)
    }

    fn offset(&mut self, count: isize) {
        if count == 0 {
            return;
        }

        self.iter.offset(count);
        self.offset = Some(match self.offset {
            None => (count as usize - 1) * Self::block_size(),
            Some(offset) => offset + (count as usize) * Self::block_size(),
        });
    }

    fn flip_quotes_bit(&mut self) {
        self.in_quotes = !self.in_quotes;
    }
}
