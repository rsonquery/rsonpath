use aligners::{alignment::TwoTo, AlignedSlice};

use crate::debug;
use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState};

use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use std::marker::PhantomData;

pub(crate) struct VectorIterator<'a, I: QuoteClassifiedIterator<'a>, C: DelimiterClassifier> {
    iter: I,
    phantom: PhantomData<&'a C>,
}

impl<'a, I: QuoteClassifiedIterator<'a>, C: DelimiterClassifier> VectorIterator<'a, I, C> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            phantom: PhantomData,
        }
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>, C: DelimiterClassifier> Iterator
    for VectorIterator<'a, I, C>
{
    type Item = Vector<'a, C>;

    fn next(&mut self) -> Option<Self::Item> {
        let quote_classified = self.iter.next();
        quote_classified.map(Vector::new)
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>, C: DelimiterClassifier> DepthIterator<'a, I>
    for VectorIterator<'a, I, C>
{
    type Block = Vector<'a, C>;

    fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'a, I> {
        let block_state = block.and_then(|b| {
            let idx = b.idx + 1;
            debug!("Depth iterator stopping at index {idx}");
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

pub(crate) struct Vector<'a, C: DelimiterClassifier> {
    quote_classified: QuoteClassifiedBlock<'a>,
    opening_mask: u64,
    closing_mask: u64,
    len: usize,
    idx: usize,
    depth_offset: isize,
    phantom: PhantomData<&'a C>,
}

impl<'a, C: DelimiterClassifier> Vector<'a, C> {
    #[inline]
    fn new(bytes: QuoteClassifiedBlock<'a>) -> Self {
        Self::new_from(bytes, 0)
    }

    #[inline]
    fn new_from(bytes: QuoteClassifiedBlock<'a>, idx: usize) -> Self {
        let mut vector = unsafe { Self::new_avx2(bytes, idx) };
        vector.advance();
        vector
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new_avx2(bytes: QuoteClassifiedBlock<'a>, start_idx: usize) -> Self {
        let len = bytes.len();
        let idx_mask = 0xFFFFFFFFFFFFFFFFu64 << start_idx;
        let (first_block, second_block) = bytes.block.halves();
        let (first_opening_vector, first_closing_vector) =
            C::get_opening_and_closing_vectors(first_block);
        let (second_opening_vector, second_closing_vector) =
            C::get_opening_and_closing_vectors(second_block);

        let first_opening_mask = _mm256_movemask_epi8(first_opening_vector) as u32;
        let first_closing_mask = _mm256_movemask_epi8(first_closing_vector) as u32;
        let second_opening_mask = _mm256_movemask_epi8(second_opening_vector) as u32;
        let second_closing_mask = _mm256_movemask_epi8(second_closing_vector) as u32;

        let combined_opening_mask =
            (first_opening_mask as u64) | ((second_opening_mask as u64) << 32);
        let combined_closing_mask =
            (first_closing_mask as u64) | ((second_closing_mask as u64) << 32);

        let opening_mask = combined_opening_mask & (!bytes.within_quotes_mask) & idx_mask;
        let closing_mask = combined_closing_mask & (!bytes.within_quotes_mask) & idx_mask;

        Self {
            quote_classified: bytes,
            opening_mask,
            closing_mask,
            depth_offset: 0,
            len,
            idx: start_idx,
            phantom: PhantomData,
        }
    }
}

impl<'a, C: DelimiterClassifier> DepthBlock<'a> for Vector<'a, C> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.idx == self.quote_classified.len() {
            return false;
        }
        if (self.opening_mask & (1 << self.idx)) != 0 {
            self.depth_offset += 1;
        } else if (self.closing_mask & (1 << self.idx)) != 0 {
            self.depth_offset -= 1;
        }

        self.opening_mask &= !(1 << self.idx);
        self.closing_mask &= !(1 << self.idx);
        self.idx += 1;
        true
    }

    #[inline]
    fn advance_to_next_depth_change(&mut self) -> bool {
        let next_opening = self.opening_mask.trailing_zeros() as usize;
        let next_closing = self.closing_mask.trailing_zeros() as usize;

        if next_opening == 64 && next_closing == 64 {
            return false;
        }

        if next_opening < next_closing {
            self.depth_offset += 1;
            self.opening_mask &= !(1 << next_opening);
            self.idx = next_opening + 1;
        } else {
            self.depth_offset -= 1;
            self.closing_mask &= !(1 << next_closing);
            self.idx = next_closing + 1;
        }

        true
    }

    #[inline]
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        self.depth_offset >= depth
    }

    #[inline(always)]
    fn depth_at_end(&self) -> isize {
        ((self.opening_mask.count_ones() as i32) - (self.closing_mask.count_ones() as i32)) as isize
            + self.depth_offset
    }

    fn add_depth(&mut self, depth: isize) {
        self.depth_offset += depth;
    }

    fn estimate_lowest_possible_depth(&self) -> isize {
        self.depth_offset - (self.closing_mask.count_ones() as isize)
    }
}

pub(crate) trait DelimiterClassifier {
    fn get_opening_and_closing_vectors(bytes: &AlignedSlice<TwoTo<5>>) -> (__m256i, __m256i);
}

pub(crate) struct BraceDelimiterClassifier {}

pub(crate) struct BracketDelimiterClassifier {}

impl DelimiterClassifier for BraceDelimiterClassifier {
    #[inline(always)]
    fn get_opening_and_closing_vectors(bytes: &AlignedSlice<TwoTo<5>>) -> (__m256i, __m256i) {
        unsafe { Self::get_opening_and_closing_vectors_impl(bytes) }
    }
}

impl DelimiterClassifier for BracketDelimiterClassifier {
    #[inline(always)]
    fn get_opening_and_closing_vectors(bytes: &AlignedSlice<TwoTo<5>>) -> (__m256i, __m256i) {
        unsafe { Self::get_opening_and_closing_vectors_impl(bytes) }
    }
}

impl BraceDelimiterClassifier {
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn get_opening_and_closing_vectors_impl(
        bytes: &AlignedSlice<TwoTo<5>>,
    ) -> (__m256i, __m256i) {
        let byte_vector = _mm256_load_si256(bytes.as_ptr() as *const __m256i);
        let opening_mask = _mm256_set1_epi8(b'{' as i8);
        let closing_mask = _mm256_set1_epi8(b'}' as i8);
        let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, opening_mask);
        let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, closing_mask);
        (opening_brace_cmp, closing_brace_cmp)
    }
}

impl BracketDelimiterClassifier {
    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn get_opening_and_closing_vectors_impl(
        bytes: &AlignedSlice<TwoTo<5>>,
    ) -> (__m256i, __m256i) {
        let byte_vector = _mm256_load_si256(bytes.as_ptr() as *const __m256i);
        let opening_mask = _mm256_set1_epi8(b'[' as i8);
        let closing_mask = _mm256_set1_epi8(b']' as i8);
        let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, opening_mask);
        let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, closing_mask);
        (opening_brace_cmp, closing_brace_cmp)
    }
}
