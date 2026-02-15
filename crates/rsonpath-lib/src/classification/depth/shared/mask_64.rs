use crate::{
    bin_u64,
    classification::{depth::DepthBlock, quotes::QuoteClassifiedBlock},
    debug,
    input::InputBlock,
};
use std::marker::PhantomData;

const SIZE: usize = 64;

/// Works on a 64-byte slice, but uses a heuristic to quickly
/// respond to queries and not count the depth exactly unless
/// needed.
///
/// The heuristic checks if it is possible to achieve the queried
/// depth within the block by counting the number of opening
/// and closing structural characters. This can be done much
/// more quickly than precise depth calculation.
pub(crate) struct DepthVector64<'a, B: InputBlock<'a, SIZE>> {
    pub(crate) quote_classified: QuoteClassifiedBlock<B, u64, SIZE>,
    pub(crate) opening_mask: u64,
    pub(crate) opening_count: u32,
    pub(crate) closing_mask: u64,
    pub(crate) idx: usize,
    pub(crate) depth: i32,
    pub(crate) phantom: PhantomData<&'a ()>,
}

impl<'a, B: InputBlock<'a, SIZE>> DepthBlock<'a> for DepthVector64<'a, B> {
    #[inline(always)]
    fn advance_to_next_depth_decrease(&mut self) -> bool {
        let next_closing = self.closing_mask.trailing_zeros() as usize;

        if next_closing == SIZE {
            return false;
        }

        bin_u64!("opening_mask", self.opening_mask);
        bin_u64!("closing_mask", self.closing_mask);

        self.opening_mask >>= next_closing;
        self.closing_mask >>= next_closing;
        self.opening_mask >>= 1;
        self.closing_mask >>= 1;

        bin_u64!("new opening_mask", self.opening_mask);
        bin_u64!("new closing_mask", self.closing_mask);

        let new_opening_count = self.opening_mask.count_ones() as i32;
        let delta = (self.opening_count as i32) - new_opening_count - 1;
        self.opening_count = new_opening_count as u32;

        debug!("next_closing: {next_closing}");
        debug!("new_opening_count: {new_opening_count}");
        debug!("delta: {delta}");

        self.depth += delta;
        self.idx += next_closing + 1;

        true
    }

    #[inline(always)]
    fn get_depth(&self) -> isize {
        self.depth as isize
    }

    #[inline(always)]
    fn depth_at_end(&self) -> isize {
        (((self.opening_count as i32) - self.closing_mask.count_ones() as i32) + self.depth) as isize
    }

    #[inline(always)]
    fn add_depth(&mut self, depth: isize) {
        self.depth += depth as i32;
    }

    #[inline(always)]
    fn estimate_lowest_possible_depth(&self) -> isize {
        (self.depth - self.closing_mask.count_ones() as i32) as isize
    }
}
