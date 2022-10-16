use super::*;
use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState};
use std::marker::PhantomData;

#[derive(Clone, Copy)]
struct LabelData {
    byte1: u8,
    byte2: u8,
    byte_offset1: u8,
    byte_offset2: u8,
}

impl LabelData {
    fn new(label: &Label) -> Self {
        let (byte_offset1, byte_offset2) = label.rare_bytes().as_rare_ordered_u8();
        Self {
            byte1: label.bytes_with_quotes()[byte_offset1 as usize],
            byte2: label.bytes_with_quotes()[byte_offset2 as usize],
            byte_offset1,
            byte_offset2,
        }
    }
}

pub(crate) struct VectorIterator<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    label_data: LabelData,
    opening: u8,
    prev_block: Option<QuoteClassifiedBlock<'a>>,
    phantom: PhantomData<&'a I>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> VectorIterator<'a, I> {
    pub(crate) fn new(iter: I, label: &Label, opening: u8) -> Self {
        Self {
            iter,
            label_data: LabelData::new(label),
            opening,
            prev_block: None,
            phantom: PhantomData,
        }
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for VectorIterator<'a, I> {
    type Item = Vector<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let quote_classified = self.iter.next();
        let vector = quote_classified.map(|b| {
            Vector::new(
                b,
                self.iter.get_offset(),
                self.opening,
                self.label_data,
                self.prev_block,
            )
        });
        self.prev_block = quote_classified;

        vector
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> DepthSeekIterator<'a, I> for VectorIterator<'a, I> {
    type Block = Vector<'a>;

    fn stop(
        self,
        block: Option<Self::Block>,
        result: Option<usize>,
    ) -> ResumeClassifierState<'a, I> {
        let mut raw_state = {
            let block_state = block.and_then(|b| {
                if b.match_idx >= b.quote_classified.len() {
                    None
                } else {
                    Some(ResumeClassifierBlockState {
                        block: b.quote_classified,
                        idx: b.match_idx,
                    })
                }
            });

            ResumeClassifierState {
                iter: self.iter,
                block: block_state,
            }
        };

        if let Some(result) = result {
            let difference = result - raw_state.get_idx();
            raw_state.offset_bytes(difference as isize);
        }

        raw_state
    }

    fn resume(
        state: ResumeClassifierState<'a, I>,
        label: &Label,
        opening: u8,
    ) -> (Option<Self::Block>, Self) {
        let label_data = LabelData::new(label);
        let first_block = state.block.as_ref().map(|b| {
            Vector::new_from(
                b.block,
                state.iter.get_offset(),
                opening,
                label_data,
                None,
                b.idx,
            )
        });

        (
            first_block,
            VectorIterator {
                iter: state.iter,
                prev_block: state.block.map(|b| b.block),
                label_data,
                opening,
                phantom: PhantomData,
            },
        )
    }
}

pub(crate) struct Vector<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    depth: isize,
    depth_idx: usize,
    match_idx: usize,
    offset: usize,
    opening: u8,
    label_data: LabelData,
    prev_block: Option<QuoteClassifiedBlock<'a>>,
}

impl<'a> Vector<'a> {
    #[inline]
    fn new(
        bytes: QuoteClassifiedBlock<'a>,
        offset: usize,
        opening: u8,
        label_data: LabelData,
        prev_block: Option<QuoteClassifiedBlock<'a>>,
    ) -> Self {
        Self::new_from(bytes, offset, opening, label_data, prev_block, 0)
    }

    #[inline]
    fn new_from(
        bytes: QuoteClassifiedBlock<'a>,
        offset: usize,
        opening: u8,
        label_data: LabelData,
        prev_block: Option<QuoteClassifiedBlock<'a>>,
        idx: usize,
    ) -> Self {
        Self {
            quote_classified: bytes,
            depth: 0,
            match_idx: idx,
            depth_idx: idx,
            opening,
            offset,
            label_data,
            prev_block,
        }
    }

    #[inline]
    fn map_depths<F: FnMut(isize)>(&self, mut f: F) {
        let closing = self.opening + 2;
        let mut current = self.depth;
        f(current);
        let mut offset = 0;

        while self.depth_idx + offset < self.quote_classified.len() {
            let idx = self.depth_idx + offset;
            let character = self.quote_classified.block[idx];
            let idx_mask = 1u64 << idx;
            let is_quoted = (self.quote_classified.within_quotes_mask & idx_mask) == idx_mask;

            if !is_quoted {
                if character == self.opening {
                    current += 1;
                } else if character == closing {
                    current -= 1;
                }
            }

            f(current);
            offset += 1;
        }
    }
}

impl<'a> DepthSeekBlock<'a> for Vector<'a> {
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
    fn advance_to_next_depth_decrease(&mut self) -> Option<usize> {
        let closing = self.opening + 2;
        while self.depth_idx < self.quote_classified.len() {
            let character = self.quote_classified.block[self.depth_idx];
            let idx_mask = 1u64 << self.depth_idx;
            let is_quoted = (self.quote_classified.within_quotes_mask & idx_mask) == idx_mask;
            self.depth_idx += 1;

            if !is_quoted {
                if character == self.opening {
                    self.depth += 1;
                } else if character == closing {
                    self.depth -= 1;
                    return Some(self.depth_idx);
                }
            }
        }

        None
    }

    #[inline]
    fn add_depth(&mut self, depth: isize) {
        self.depth += depth;
    }

    fn estimate_lowest_possible_depth(&self) -> isize {
        let mut lowest = isize::MAX;
        self.map_depths(|x| lowest = std::cmp::min(lowest, x));

        lowest
    }

    fn get_depth_idx(&self) -> usize {
        self.depth_idx + self.offset
    }

    fn advance_to_next_possible_match(&mut self) -> Option<usize> {
        while self.match_idx < self.quote_classified.len() {
            let idx = self.match_idx;
            self.match_idx += 1;

            let is_possible_match = if self.quote_classified.block[idx] == self.label_data.byte2 {
                debug_assert!(self.label_data.byte_offset2 > self.label_data.byte_offset1);
                let relative_offset =
                    (self.label_data.byte_offset2 - self.label_data.byte_offset1) as usize;

                if relative_offset > idx {
                    if let Some(prev_block) = self.prev_block {
                        let idx_in_the_prev_block = prev_block.len() - (relative_offset - idx);
                        prev_block.block[idx_in_the_prev_block] == self.label_data.byte1
                    } else {
                        false
                    }
                } else {
                    let idx_in_this_block = idx - relative_offset;
                    self.quote_classified.block[idx_in_this_block] == self.label_data.byte1
                }
            } else {
                false
            };

            if is_possible_match && idx >= (self.label_data.byte_offset2 as usize) {
                let match_idx = idx - (self.label_data.byte_offset2 as usize);
                return Some(match_idx + self.offset);
            }
        }

        None
    }

    fn advance_to_end(&mut self) {
        self.depth = self.depth_at_end();
        self.depth_idx = self.quote_classified.len();
        self.match_idx = self.quote_classified.len();
    }
}
