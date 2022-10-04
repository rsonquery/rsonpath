use aligners::{alignment::TwoTo, AlignedSlice};

use crate::debug;
use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState};

use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use std::marker::PhantomData;

pub(crate) struct VectorIterator<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    classifier: DelimiterClassifierImpl,
    phantom: PhantomData<&'a I>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> VectorIterator<'a, I> {
    pub(crate) fn new(iter: I, opening: u8) -> Self {
        Self {
            iter,
            classifier: DelimiterClassifierImpl::new(opening),
            phantom: PhantomData,
        }
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for VectorIterator<'a, I> {
    type Item = Vector<'a>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let quote_classified = self.iter.next();
        quote_classified.map(|q| Vector::new(q, &self.classifier))
    }
}

impl<'a, I: QuoteClassifiedIterator<'a> + 'a> DepthIterator<'a, I> for VectorIterator<'a, I> {
    type Block = Vector<'a>;

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

    fn resume(state: ResumeClassifierState<'a, I>, opening: u8) -> (Option<Self::Block>, Self) {
        let classifier = DelimiterClassifierImpl::new(opening);
        let first_block = state
            .block
            .map(|b| Vector::new_from(b.block, &classifier, b.idx));

        (
            first_block,
            VectorIterator {
                iter: state.iter,
                classifier,
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

pub(crate) struct Vector<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    opening_mask: u64,
    closing_mask: u64,
    idx: usize,
    depth: isize,
}

impl<'a> Vector<'a> {
    #[inline]
    fn new(bytes: QuoteClassifiedBlock<'a>, classifier: &DelimiterClassifierImpl) -> Self {
        Self::new_from(bytes, classifier, 0)
    }

    #[inline]
    fn new_from(
        bytes: QuoteClassifiedBlock<'a>,
        classifier: &DelimiterClassifierImpl,
        idx: usize,
    ) -> Self {
        unsafe { Self::new_avx2(bytes, classifier, idx) }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new_avx2(
        bytes: QuoteClassifiedBlock<'a>,
        classifier: &DelimiterClassifierImpl,
        start_idx: usize,
    ) -> Self {
        let idx_mask = 0xFFFFFFFFFFFFFFFFu64 << start_idx;
        let (first_block, second_block) = bytes.block.halves();
        let (first_opening_vector, first_closing_vector) =
            classifier.get_opening_and_closing_vectors(first_block);
        let (second_opening_vector, second_closing_vector) =
            classifier.get_opening_and_closing_vectors(second_block);

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
            depth: 0,
            idx: start_idx,
        }
    }
}

impl<'a> DepthBlock<'a> for Vector<'a> {
    #[inline]
    fn advance_to_next_depth_decrease(&mut self) -> bool {
        let next_closing = self.closing_mask.trailing_zeros() as usize;

        if next_closing == 64 {
            return false;
        }

        let remainder_mask = 0xFFFFFFFFFFFFFFFFu64 << next_closing;

        let delta = ((self.opening_mask & !remainder_mask).count_ones() as i32) - 1;

        self.depth += delta as isize;
        self.opening_mask &= remainder_mask;
        self.closing_mask &= remainder_mask ^ (1 << next_closing);
        self.idx = next_closing;

        true
    }

    #[inline]
    fn get_depth(&self) -> isize {
        self.depth
    }

    #[inline(always)]
    fn depth_at_end(&self) -> isize {
        ((self.opening_mask.count_ones() as i32) - (self.closing_mask.count_ones() as i32)) as isize
            + self.depth
    }

    fn add_depth(&mut self, depth: isize) {
        self.depth += depth;
    }

    fn estimate_lowest_possible_depth(&self) -> isize {
        self.depth - (self.closing_mask.count_ones() as isize)
    }
}

struct DelimiterClassifierImpl {
    opening_mask: __m256i,
    closing_mask: __m256i,
}

impl DelimiterClassifierImpl {
    #[inline(always)]
    fn new(opening: u8) -> Self {
        let closing = opening + 2;

        unsafe {
            let opening_mask = _mm256_set1_epi8(opening as i8);
            let closing_mask = _mm256_set1_epi8(closing as i8);

            Self {
                opening_mask,
                closing_mask,
            }
        }
    }

    #[inline(always)]
    fn get_opening_and_closing_vectors(
        &self,
        bytes: &AlignedSlice<TwoTo<5>>,
    ) -> (__m256i, __m256i) {
        unsafe {
            let byte_vector = _mm256_load_si256(bytes.as_ptr() as *const __m256i);
            let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, self.opening_mask);
            let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, self.closing_mask);
            (opening_brace_cmp, closing_brace_cmp)
        }
    }
}
