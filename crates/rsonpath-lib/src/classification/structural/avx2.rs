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

use crate::classification::structural::{BracketType, QuoteClassifiedIterator, Structural, StructuralIterator};
use crate::classification::{QuoteClassifiedBlock, ResumeClassifierBlockState, ResumeClassifierState};
use crate::input::{IBlock, Input, InputBlock};
use crate::{bin, debug};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

struct StructuralsBlock<'a, I: Input + 'a> {
    quote_classified: QuoteClassifiedBlock<IBlock<'a, I, 64>, 64>,
    structural_mask: u64,
}

impl<'a, I: Input> StructuralsBlock<'a, I> {
    #[inline(always)]
    fn new(block: QuoteClassifiedBlock<IBlock<'a, I, 64>, 64>, structural_mask: u64) -> Self {
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

impl<I: Input> Iterator for StructuralsBlock<'_, I> {
    type Item = Structural;

    #[inline]
    fn next(&mut self) -> Option<Structural> {
        use BracketType::*;
        use Structural::*;

        let idx = self.get_idx() as usize;
        (idx < 64).then(|| {
            let bit_mask = 1 << idx;

            self.structural_mask ^= bit_mask;

            // The last match being a catch-all *is important*.
            // It has major performance implications, since the jump table generated here is a hot path for the engine.
            // Changing this match must be accompanied with benchmark runs to make sure perf does not regress.
            match self.quote_classified.block[idx] {
                b':' => Colon(idx),
                b'{' => Opening(Curly, idx),
                b'[' => Opening(Square, idx),
                b',' => Comma(idx),
                b'}' => Closing(Curly, idx),
                _ => Closing(Square, idx),
            }
        })
    }
}

impl<I: Input> std::iter::FusedIterator for StructuralsBlock<'_, I> {}

impl<I: Input> ExactSizeIterator for StructuralsBlock<'_, I> {
    fn len(&self) -> usize {
        self.structural_mask.count_ones() as usize
    }
}

pub(crate) struct Avx2Classifier<'a, I: Input, Q> {
    iter: Q,
    classifier: BlockAvx2Classifier,
    block: Option<StructuralsBlock<'a, I>>,
    are_commas_on: bool,
    are_colons_on: bool,
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> Avx2Classifier<'a, I, Q> {
    #[inline]
    pub(crate) fn new(iter: Q) -> Self {
        Self {
            iter,
            // SAFETY: target_feature invariant
            classifier: unsafe { BlockAvx2Classifier::new() },
            block: None,
            are_commas_on: false,
            are_colons_on: false,
        }
    }

    #[inline(always)]
    fn current_block_is_spent(&self) -> bool {
        self.block.as_ref().map_or(true, StructuralsBlock::is_empty)
    }
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> Iterator for Avx2Classifier<'a, I, Q> {
    type Item = Structural;

    #[inline(always)]
    fn next(&mut self) -> Option<Structural> {
        while self.current_block_is_spent() {
            match self.iter.next() {
                Some(block) => {
                    // SAFETY: target_feature invariant
                    self.block = unsafe { Some(self.classifier.classify(block)) };
                }
                None => {
                    self.block = None;
                    break;
                }
            }
        }

        self.block
            .as_mut()
            .and_then(|b| b.next().map(|x| x.offset(self.iter.get_offset())))
    }
}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> std::iter::FusedIterator for Avx2Classifier<'a, I, Q> {}

impl<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, 64>> StructuralIterator<'a, I, Q, 64>
    for Avx2Classifier<'a, I, Q>
{
    fn turn_commas_on(&mut self, idx: usize) {
        if !self.are_commas_on {
            self.are_commas_on = true;
            debug!("Turning commas on at {idx}.");
            // SAFETY: target_feature invariant
            unsafe { self.classifier.toggle_commas() }

            if let Some(block) = self.block.take() {
                let quote_classified_block = block.quote_classified;
                let block_idx = (idx + 1) % 64;

                if block_idx != 0 {
                    let mask = u64::MAX << block_idx;
                    // SAFETY: target_feature invariant
                    let mut new_block = unsafe { self.classifier.classify(quote_classified_block) };
                    new_block.structural_mask &= mask;
                    self.block = Some(new_block);
                }
            }
        }
    }

    fn turn_commas_off(&mut self) {
        if self.are_commas_on {
            self.are_commas_on = false;
            debug!("Turning commas off.");
            // SAFETY: target_feature invariant
            unsafe { self.classifier.toggle_commas() }
        }
    }

    fn turn_colons_on(&mut self, idx: usize) {
        if !self.are_colons_on {
            self.are_colons_on = true;
            debug!("Turning colons on at {idx}.");
            // SAFETY: target_feature invariant
            unsafe { self.classifier.toggle_colons() }

            if let Some(block) = self.block.take() {
                let quote_classified_block = block.quote_classified;
                let block_idx = (idx + 1) % 64;

                if block_idx != 0 {
                    let mask = u64::MAX << block_idx;
                    // SAFETY: target_feature invariant
                    let mut new_block = unsafe { self.classifier.classify(quote_classified_block) };
                    new_block.structural_mask &= mask;
                    self.block = Some(new_block);
                }
            }
        }
    }

    fn turn_colons_off(&mut self) {
        if self.are_colons_on {
            self.are_colons_on = false;
            debug!("Turning colons off.");
            // SAFETY: target_feature invariant
            unsafe { self.classifier.toggle_colons() }
        }
    }

    fn stop(self) -> ResumeClassifierState<'a, I, Q, 64> {
        let block = self.block.map(|b| ResumeClassifierBlockState {
            idx: b.get_idx() as usize,
            block: b.quote_classified,
        });

        ResumeClassifierState {
            iter: self.iter,
            block,
            are_commas_on: self.are_commas_on,
            are_colons_on: self.are_colons_on,
        }
    }

    fn resume(state: ResumeClassifierState<'a, I, Q, 64>) -> Self {
        // SAFETY: target_feature invariant
        let mut classifier = unsafe { BlockAvx2Classifier::new() };

        // SAFETY: target_feature invariant
        unsafe {
            if state.are_commas_on {
                classifier.toggle_commas();
            }
            if state.are_colons_on {
                classifier.toggle_colons();
            }
        }

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
            are_commas_on: state.are_commas_on,
            are_colons_on: state.are_colons_on,
        }
    }
}

struct BlockAvx2Classifier {
    lower_nibble_mask: __m256i,
    upper_nibble_mask: __m256i,
    upper_nibble_zeroing_mask: __m256i,
    commas_toggle_mask: __m256i,
    colons_toggle_mask: __m256i,
}

struct BlockClassification {
    structural: u32,
}

impl BlockAvx2Classifier {
    const LOWER_NIBBLE_MASK_ARRAY: [u8; 32] = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x01, 0x02, 0x01, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x01, 0x02, 0x01, 0xff, 0xff,
    ];
    const UPPER_NIBBLE_MASK_ARRAY: [u8; 32] = [
        0xfe, 0xfe, 0x10, 0x10, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
        0x10, 0x10, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
    ];
    const COMMAS_TOGGLE_MASK_ARRAY: [u8; 32] = [
        0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    const COLON_TOGGLE_MASK_ARRAY: [u8; 32] = [
        0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new() -> Self {
        Self {
            lower_nibble_mask: _mm256_loadu_si256(Self::LOWER_NIBBLE_MASK_ARRAY.as_ptr().cast::<__m256i>()),
            upper_nibble_mask: _mm256_loadu_si256(Self::UPPER_NIBBLE_MASK_ARRAY.as_ptr().cast::<__m256i>()),
            upper_nibble_zeroing_mask: _mm256_set1_epi8(0x0F),
            commas_toggle_mask: _mm256_loadu_si256(Self::COMMAS_TOGGLE_MASK_ARRAY.as_ptr().cast::<__m256i>()),
            colons_toggle_mask: _mm256_loadu_si256(Self::COLON_TOGGLE_MASK_ARRAY.as_ptr().cast::<__m256i>()),
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn toggle_commas(&mut self) {
        self.upper_nibble_mask = _mm256_xor_si256(self.upper_nibble_mask, self.commas_toggle_mask);
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn toggle_colons(&mut self) {
        self.upper_nibble_mask = _mm256_xor_si256(self.upper_nibble_mask, self.colons_toggle_mask);
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify<'a, I: Input>(
        &mut self,
        quote_classified_block: QuoteClassifiedBlock<IBlock<'a, I, 64>, 64>,
    ) -> StructuralsBlock<'a, I> {
        let (block1, block2) = quote_classified_block.block.halves();
        let classification1 = self.classify_block(block1);
        let classification2 = self.classify_block(block2);

        let structural = u64::from(classification1.structural) | (u64::from(classification2.structural) << 32);

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
        let upper_nibble_byte_vector = _mm256_and_si256(shifted_byte_vector, self.upper_nibble_zeroing_mask);
        let lower_nibble_lookup = _mm256_shuffle_epi8(self.lower_nibble_mask, byte_vector);
        let upper_nibble_lookup = _mm256_shuffle_epi8(self.upper_nibble_mask, upper_nibble_byte_vector);
        let structural_vector = _mm256_cmpeq_epi8(lower_nibble_lookup, upper_nibble_lookup);
        let structural = _mm256_movemask_epi8(structural_vector) as u32;

        BlockClassification { structural }
    }
}
