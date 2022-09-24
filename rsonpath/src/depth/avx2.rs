use aligners::{alignment::TwoTo, AlignedSlice};

use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState};

use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use std::marker::PhantomData;

pub struct VectorIterator<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    phantom: PhantomData<&'a I>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> VectorIterator<'a, I> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            phantom: PhantomData,
        }
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for VectorIterator<'a, I> {
    type Item = Vector<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let quote_classified = self.iter.next();
        quote_classified.map(Vector::new)
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> DepthIterator<'a, I> for VectorIterator<'a, I> {
    type Block = Vector<'a>;

    fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'a, I> {
        let block_state = block.and_then(|b| {
            let idx = b.idx() + 1;
            if idx >= b.quote_classified.len() {
                None
            } else {
                Some(ResumeClassifierBlockState {
                    block: b.quote_classified,
                    idx,
                })
            }
        });

        ResumeClassifierState {
            iter: self.iter,
            block: block_state,
        }
    }

    fn resume(state: ResumeClassifierState<'a, I>) -> (Option<Self::Block>, Self) {
        let first_block = state.block.map(|b| Vector::new_from(b.block, b.idx));

        (
            first_block,
            VectorIterator {
                iter: state.iter,
                phantom: PhantomData,
            },
        )
    }
}

/// Works on a 32-byte slice, but uses a heuristic to quickly
/// respond to queries and not count the depth exactly unless
/// needed.
///
/// The heuristic checks if it is possible to achieve the queried
/// depth within the block by counting the number of opening
/// and closing structural characters. This can be done much
/// more quickly than precise depth calculation.
#[cfg_attr(
    docsrs,
    doc(cfg(all(
        target_feature = "avx2",
        any(target_arch = "x86", target_arch = "x86_64")
    )))
)]
pub struct Vector<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    opening_mask: u64,
    closing_mask: u64,
    len: usize,
    rev_idx: usize,
    depth_offset: isize,
}

impl<'a> Vector<'a> {
    #[inline]
    pub fn new(bytes: QuoteClassifiedBlock<'a>) -> Self {
        unsafe { Self::new_avx2(bytes, 0) }
    }

    #[inline]
    fn new_from(bytes: QuoteClassifiedBlock<'a>, idx: usize) -> Self {
        unsafe { Self::new_avx2(bytes, idx) }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new_avx2(bytes: QuoteClassifiedBlock<'a>, start_idx: usize) -> Self {
        let len = bytes.len();
        let idx_mask = 0xFFFFFFFFFFFFFFFFu64 << start_idx;
        let (first_block, second_block) = bytes.block.halves();
        let (first_opening_vector, first_closing_vector) =
            get_opening_and_closing_vectors(first_block);
        let (second_opening_vector, second_closing_vector) =
            get_opening_and_closing_vectors(second_block);

        let first_opening_mask = _mm256_movemask_epi8(first_opening_vector) as u32;
        let first_closing_mask = _mm256_movemask_epi8(first_closing_vector) as u32;
        let second_opening_mask = _mm256_movemask_epi8(second_opening_vector) as u32;
        let second_closing_mask = _mm256_movemask_epi8(second_closing_vector) as u32;

        let combined_opening_mask = (first_opening_mask as u64)
            | ((second_opening_mask as u64) << 32);
        let combined_closing_mask = (first_closing_mask as u64)
            | ((second_closing_mask as u64) << 32);

        let opening_mask = combined_opening_mask & (!bytes.within_quotes_mask) & idx_mask;
        let closing_mask = combined_closing_mask & (!bytes.within_quotes_mask) & idx_mask;

        Self {
            quote_classified: bytes,
            opening_mask,
            closing_mask,
            depth_offset: 0,
            len,
            rev_idx: len - 1 - start_idx,
        }
    }

    fn idx(&self) -> usize {
        self.quote_classified.len() - 1 - self.rev_idx
    }
}

impl<'a> DepthBlock<'a> for Vector<'a> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.rev_idx == 0 {
            return false;
        }
        self.rev_idx -= 1;
        true
    }

    #[inline]
    fn advance_by(&mut self, i: usize) -> usize {
        let j = std::cmp::min(i, self.rev_idx);
        self.rev_idx -= j;
        j
    }

    #[inline]
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        let depth = depth - self.depth_offset;
        let closing_count = self.closing_mask.count_ones() as isize;
        if depth <= -closing_count {
            return true;
        }

        let actual_depth = ((self.opening_mask << self.rev_idx).count_ones() as i32)
            - ((self.closing_mask << self.rev_idx).count_ones() as i32);
        actual_depth as isize >= depth
    }

    #[inline(always)]
    fn depth_at_end(&self) -> isize {
        ((self.opening_mask.count_ones() as i32) - (self.closing_mask.count_ones() as i32)) as isize
            + self.depth_offset
    }

    fn set_starting_depth(&mut self, depth: isize) {
        self.depth_offset = depth;
    }

    fn estimate_lowest_possible_depth(&self) -> isize {
        self.depth_offset - (self.closing_mask.count_ones() as isize)
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_opening_and_closing_vectors(bytes: &AlignedSlice<TwoTo<5>>) -> (__m256i, __m256i) {
    let byte_vector = _mm256_load_si256(bytes.as_ptr() as *const __m256i);
    let opening_brace_mask = _mm256_set1_epi8(b'{' as i8);
    let opening_bracket_mask = _mm256_set1_epi8(b'[' as i8);
    let closing_brace_mask = _mm256_set1_epi8(b'}' as i8);
    let closing_bracket_mask = _mm256_set1_epi8(b']' as i8);
    let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, opening_brace_mask);
    let opening_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, opening_bracket_mask);
    let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, closing_brace_mask);
    let closing_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, closing_bracket_mask);
    let opening_cmp = _mm256_or_si256(opening_brace_cmp, opening_bracket_cmp);
    let closing_cmp = _mm256_or_si256(closing_brace_cmp, closing_bracket_cmp);
    (opening_cmp, closing_cmp)
}
