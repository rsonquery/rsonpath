//! This module can only be included if the code is compiled with AVX2 support
//! and on x86/x86_64 architecture for safety.

cfg_if::cfg_if! {
    if #[cfg(not(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2")
    ))] {
        compile_error!{
            "internal error: AVX2 code included on unsupported target; \
            please report this issue at https://github.com/V0ldek/rsonpath/"
        }
    }
}

use super::*;
use crate::bin;
use crate::quotes::{QuoteClassifiedBlock, ResumeClassifierBlockState};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

struct StructuralsBlock<'a> {
    quote_classified: QuoteClassifiedBlock<'a>,
    structural_mask: u64,
}

impl<'a> StructuralsBlock<'a> {
    #[inline(always)]
    fn new(block: QuoteClassifiedBlock<'a>, structural_mask: u64) -> Self {
        Self {
            quote_classified: block,
            structural_mask,
        }
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.structural_mask == 0
    }

    #[inline(always)]
    fn get_idx(&self) -> u32 {
        self.structural_mask.trailing_zeros()
    }
}

impl Iterator for StructuralsBlock<'_> {
    type Item = Structural;

    #[inline]
    fn next(&mut self) -> Option<Structural> {
        use Structural::*;

        let idx = self.get_idx() as usize;
        (idx < 64).then(|| {
            let bit_mask = 1 << idx;

            self.structural_mask ^= bit_mask;

            match self.quote_classified.block[idx] {
                b':' => Colon(idx),
                b'[' | b'{' => Opening(idx),
                #[cfg(feature = "commas")]
                b',' => Comma(idx),
                _ => Closing(idx),
            }
        })
    }
}

impl std::iter::FusedIterator for StructuralsBlock<'_> {}

impl ExactSizeIterator for StructuralsBlock<'_> {
    fn len(&self) -> usize {
        self.structural_mask.count_ones() as usize
    }
}

pub(crate) struct Avx2Classifier<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    classifier: BlockAvx2Classifier,
    block: Option<StructuralsBlock<'a>>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> Avx2Classifier<'a, I> {
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            // SAFETY: target_feature invariant
            classifier: unsafe { BlockAvx2Classifier::new() },
            block: None,
        }
    }

    #[inline(always)]
    fn next_block(&mut self) -> bool {
        while self.current_block_is_spent() {
            match self.iter.next() {
                Some(block) => {
                    // SAFETY: target_feature invariant
                    self.block = unsafe { Some(self.classifier.classify(block)) };
                }
                None => return false,
            }
        }

        true
    }

    #[inline(always)]
    fn current_block_is_spent(&self) -> bool {
        self.block.as_ref().map_or(true, StructuralsBlock::is_empty)
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> Iterator for Avx2Classifier<'a, I> {
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
            .map(|x| x.offset(self.iter.get_offset()))
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> std::iter::FusedIterator for Avx2Classifier<'a, I> {}

impl<'a, I: QuoteClassifiedIterator<'a>> StructuralIterator<'a, I> for Avx2Classifier<'a, I> {
    fn stop(self) -> ResumeClassifierState<'a, I> {
        let block = self.block.map(|b| ResumeClassifierBlockState {
            idx: b.get_idx() as usize,
            block: b.quote_classified,
        });

        ResumeClassifierState {
            iter: self.iter,
            block,
        }
    }

    fn resume(state: ResumeClassifierState<'a, I>) -> Self {
        // SAFETY: target_feature invariant
        let mut classifier = unsafe { BlockAvx2Classifier::new() };
        let block = state.block.map(|b| {
            // SAFETY: target_feature invariant
            let mut block = unsafe { classifier.classify(b.block) };
            let idx_mask = 0xFFFF_FFFF_FFFF_FFFF << b.idx;
            block.structural_mask &= idx_mask;

            block
        });

        Self {
            iter: state.iter,
            block,
            classifier,
        }
    }
}

struct BlockAvx2Classifier {
    lower_nibble_mask: __m256i,
    upper_nibble_mask: __m256i,
    upper_nibble_zeroing_mask: __m256i,
}

struct BlockClassification {
    structural: u32,
}

impl BlockAvx2Classifier {
    cfg_if! {
        if #[cfg(feature = "commas")] {
            const LOWER_NIBBLE_MASK_ARRAY: [u8; 32] = [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x03, 0x02, 0x03, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01, 0x03, 0x02, 0x03,
                0xff, 0xff,
            ];
            const UPPER_NIBBLE_MASK_ARRAY: [u8; 32] = [
                0xfe, 0xfe, 0x02, 0x01, 0xfe, 0x03, 0xfe, 0x03, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
                0xfe, 0xfe, 0xfe, 0x02, 0x01, 0xfe, 0x03, 0xfe, 0x03, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
                0xfe, 0xfe,
            ];
        }
        else {
            const LOWER_NIBBLE_MASK_ARRAY: [u8; 32] = [
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02, 0x01, 0xff, 0x01, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02, 0x01, 0xff, 0x01,
                0xff, 0xff,
            ];
            const UPPER_NIBBLE_MASK_ARRAY: [u8; 32] = [
                0xfe, 0xfe, 0xfe, 0x02, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
                0xfe, 0xfe, 0xfe, 0xfe, 0x02, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
                0xfe, 0xfe,
            ];
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new() -> Self {
        Self {
            lower_nibble_mask: _mm256_loadu_si256(
                Self::LOWER_NIBBLE_MASK_ARRAY.as_ptr().cast::<__m256i>(),
            ),
            upper_nibble_mask: _mm256_loadu_si256(
                Self::UPPER_NIBBLE_MASK_ARRAY.as_ptr().cast::<__m256i>(),
            ),
            upper_nibble_zeroing_mask: _mm256_set1_epi8(0x0F),
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify<'a>(
        &mut self,
        quote_classified_block: QuoteClassifiedBlock<'a>,
    ) -> StructuralsBlock<'a> {
        let (block1, block2) = quote_classified_block.block.halves();
        let classification1 = self.classify_block(block1);
        let classification2 = self.classify_block(block2);

        let structural =
            u64::from(classification1.structural) | (u64::from(classification2.structural) << 32);

        let nonquoted_structural = structural & !quote_classified_block.within_quotes_mask;

        bin!("structural", structural);
        bin!("nonquoted_structural", nonquoted_structural);

        StructuralsBlock::new(quote_classified_block, nonquoted_structural)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification {
        let byte_vector = _mm256_loadu_si256(block.as_ptr().cast::<__m256i>());
        let shifted_byte_vector = _mm256_srli_epi16::<4>(byte_vector);
        let upper_nibble_byte_vector =
            _mm256_and_si256(shifted_byte_vector, self.upper_nibble_zeroing_mask);
        let lower_nibble_lookup = _mm256_shuffle_epi8(self.lower_nibble_mask, byte_vector);
        let upper_nibble_lookup =
            _mm256_shuffle_epi8(self.upper_nibble_mask, upper_nibble_byte_vector);
        let structural_vector = _mm256_cmpeq_epi8(lower_nibble_lookup, upper_nibble_lookup);
        let structural = _mm256_movemask_epi8(structural_vector) as u32;

        BlockClassification { structural }
    }
}
