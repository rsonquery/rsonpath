use super::*;
use aligners::AlignedBlockIterator;
use len_trait::Empty;

pub(crate) struct SequentialQuoteClassifier<'a> {
    iter: AlignedBlockIterator<'a, alignment::Twice<BlockAlignment>>,
    escaped: bool,
    in_quotes: bool,
}

impl<'a> SequentialQuoteClassifier<'a> {
    #[inline(always)]
    pub(crate) fn new(bytes: &'a AlignedSlice<alignment::Twice<BlockAlignment>>) -> Self {
        Self {
            iter: bytes.iter_blocks(),
            escaped: false,
            in_quotes: false,
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

impl<'a> Empty for SequentialQuoteClassifier<'a> {
    fn is_empty(&self) -> bool {
        self.iter.len() == 0
    }
}

impl<'a> QuoteClassifiedIterator<'a> for SequentialQuoteClassifier<'a> {}
