//! This module can only be included if the code is compiled with AVX2 support
//! and on x86/x86_64 architecture for safety.
cfg_if::cfg_if! {
    if #[cfg(not(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        simd = "avx2")
    ))] {
        compile_error!{
            "internal error: AVX2 code included on unsupported target; \
            please report this issue at https://github.com/V0ldek/rsonpath/issues/new?template=bug_report.md"
        }
    }
}

use crate::classification::{quotes::QuoteClassifiedBlock, ResumeClassifierBlockState};
use crate::input::{IBlock, Input, InputBlock};
use crate::{bin, debug};
use std::marker::PhantomData;

use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

pub(crate) struct VectorIterator<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> {
    iter: Q,
    classifier: DelimiterClassifierImpl,
    were_commas_on: bool,
    were_colons_on: bool,
    phantom: PhantomData<&'a I>,
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> VectorIterator<'a, I, Q> {
    pub(crate) fn new(iter: Q, opening: BracketType) -> Self {
        Self {
            iter,
            classifier: DelimiterClassifierImpl::new(opening),
            phantom: PhantomData,
            were_commas_on: false,
            were_colons_on: false,
        }
    }
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> Iterator for VectorIterator<'a, I, Q> {
    type Item = Vector<'a, I>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let quote_classified = self.iter.next();
        quote_classified.map(|q| Vector::new(q, &self.classifier))
    }
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> DepthIterator<'a, I, Q, 64> for VectorIterator<'a, I, Q> {
    type Block = Vector<'a, I>;

    fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'a, I, Q, 64> {
        let block_state = block.and_then(|b| {
            let idx = b.idx;
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
            are_commas_on: self.were_commas_on,
            are_colons_on: self.were_colons_on,
        }
    }

    fn resume(state: ResumeClassifierState<'a, I, Q, 64>, opening: BracketType) -> (Option<Self::Block>, Self) {
        let classifier = DelimiterClassifierImpl::new(opening);
        let first_block = state.block.and_then(|b| {
            if b.idx == 64 {
                None
            } else {
                Some(Vector::new_from(b.block, &classifier, b.idx))
            }
        });

        (
            first_block,
            VectorIterator {
                iter: state.iter,
                classifier,
                phantom: PhantomData,
                were_commas_on: state.are_commas_on,
                were_colons_on: state.are_colons_on,
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
    doc(cfg(all(target_feature = "avx2", any(target_arch = "x86", target_arch = "x86_64"))))
)]

pub(crate) struct Vector<'a, I: Input + 'a> {
    quote_classified: QuoteClassifiedBlock<IBlock<'a, I, 64>, 64>,
    opening_mask: u64,
    opening_count: u32,
    closing_mask: u64,
    idx: usize,
    depth: i32,
}

impl<'a, I: Input> Vector<'a, I> {
    #[inline]
    fn new(bytes: QuoteClassifiedBlock<IBlock<'a, I, 64>, 64>, classifier: &DelimiterClassifierImpl) -> Self {
        Self::new_from(bytes, classifier, 0)
    }

    #[inline]
    fn new_from(
        bytes: QuoteClassifiedBlock<IBlock<'a, I, 64>, 64>,
        classifier: &DelimiterClassifierImpl,
        idx: usize,
    ) -> Self {
        // SAFETY: target_feature invariant
        unsafe { Self::new_avx2(bytes, classifier, idx) }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new_avx2(
        bytes: QuoteClassifiedBlock<IBlock<'a, I, 64>, 64>,
        classifier: &DelimiterClassifierImpl,
        start_idx: usize,
    ) -> Self {
        let idx_mask = 0xFFFF_FFFF_FFFF_FFFF_u64 << start_idx;
        let (first_block, second_block) = bytes.block.halves();
        let (first_opening_vector, first_closing_vector) = classifier.get_opening_and_closing_vectors(first_block);
        let (second_opening_vector, second_closing_vector) = classifier.get_opening_and_closing_vectors(second_block);

        let first_opening_mask = _mm256_movemask_epi8(first_opening_vector) as u32;
        let first_closing_mask = _mm256_movemask_epi8(first_closing_vector) as u32;
        let second_opening_mask = _mm256_movemask_epi8(second_opening_vector) as u32;
        let second_closing_mask = _mm256_movemask_epi8(second_closing_vector) as u32;

        let combined_opening_mask = u64::from(first_opening_mask) | (u64::from(second_opening_mask) << 32);
        let combined_closing_mask = u64::from(first_closing_mask) | (u64::from(second_closing_mask) << 32);

        let opening_mask = combined_opening_mask & (!bytes.within_quotes_mask) & idx_mask;
        let closing_mask = combined_closing_mask & (!bytes.within_quotes_mask) & idx_mask;

        Self {
            quote_classified: bytes,
            opening_mask,
            closing_mask,
            opening_count: opening_mask.count_ones(),
            depth: 0,
            idx: 0,
        }
    }
}

impl<'a, I: Input> DepthBlock<'a> for Vector<'a, I> {
    #[inline(always)]
    fn advance_to_next_depth_decrease(&mut self) -> bool {
        let next_closing = self.closing_mask.trailing_zeros() as usize;

        if next_closing == 64 {
            return false;
        }

        bin!("opening_mask", self.opening_mask);
        bin!("closing_mask", self.closing_mask);

        self.opening_mask >>= next_closing;
        self.closing_mask >>= next_closing;
        self.opening_mask >>= 1;
        self.closing_mask >>= 1;

        bin!("new opening_mask", self.opening_mask);
        bin!("new closing_mask", self.closing_mask);

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
        (((self.opening_count as i32) - (self.closing_mask.count_ones() as i32)) + self.depth) as isize
    }

    #[inline(always)]
    fn add_depth(&mut self, depth: isize) {
        self.depth += depth as i32;
    }

    #[inline(always)]
    fn estimate_lowest_possible_depth(&self) -> isize {
        (self.depth - (self.closing_mask.count_ones() as i32)) as isize
    }
}

struct DelimiterClassifierImpl {
    opening_mask: __m256i,
    closing_mask: __m256i,
}

impl DelimiterClassifierImpl {
    #[inline(always)]
    fn new(opening: BracketType) -> Self {
        let opening = match opening {
            BracketType::Square => b'[',
            BracketType::Curly => b'{',
        };
        let closing = opening + 2;

        // SAFETY: target_feature invariant
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
    fn get_opening_and_closing_vectors(&self, bytes: &[u8]) -> (__m256i, __m256i) {
        assert_eq!(32, bytes.len());
        // SAFETY: target_feature invariant
        unsafe {
            let byte_vector = _mm256_loadu_si256(bytes.as_ptr().cast::<__m256i>());
            let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, self.opening_mask);
            let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, self.closing_mask);
            (opening_brace_cmp, closing_brace_cmp)
        }
    }
}
