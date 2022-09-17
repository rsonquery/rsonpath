use super::*;
use crate::quotes::QuoteClassifiedBlock;
use crate::BlockAlignment;
use aligners::{alignment::*, AlignedBlock};

use crate::bin;
use len_trait::Empty;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

struct StructuralsBlock<'a> {
    block: &'a AlignedBlock<Twice<BlockAlignment>>,
    structural_mask: u64,
}

impl<'a> StructuralsBlock<'a> {
    #[inline]
    fn new(block: &'a AlignedBlock<Twice<BlockAlignment>>, structural_mask: u64) -> Self {
        Self {
            block,
            structural_mask,
        }
    }
}

impl Iterator for StructuralsBlock<'_> {
    type Item = Structural;

    #[inline]
    fn next(&mut self) -> Option<Structural> {
        use Structural::*;
        let next_character_idx = self.structural_mask.trailing_zeros();

        if next_character_idx == 64 {
            return None;
        }

        let bit_mask = 1 << next_character_idx;

        self.structural_mask ^= bit_mask;

        let idx = next_character_idx as usize;
        let character = match self.block[idx] {
            b':' => Colon(idx),
            b'[' | b'{' => Opening(idx),
            b',' => Comma(idx),
            _ => Closing(idx),
        };

        Some(character)
    }
}

impl std::iter::FusedIterator for StructuralsBlock<'_> {}

impl ExactSizeIterator for StructuralsBlock<'_> {
    fn len(&self) -> usize {
        self.structural_mask.count_ones() as usize
    }
}

impl Empty for StructuralsBlock<'_> {
    fn is_empty(&self) -> bool {
        self.structural_mask == 0
    }
}

pub(crate) struct Avx2Classifier<'a, I: QuoteClassifiedIterator<'a>> {
    iter: I,
    offset: usize,
    classifier: BlockAvx2Classifier,
    block: Option<StructuralsBlock<'a>>,
}

impl<'a, I: QuoteClassifiedIterator<'a>> Avx2Classifier<'a, I> {
    #[inline]
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            offset: 0,
            classifier: unsafe { BlockAvx2Classifier::new() },
            block: None,
        }
    }

    #[inline(always)]
    fn next_block(&mut self) -> bool {
        while self.current_block_is_spent() {
            match self.iter.next() {
                Some(block) => {
                    self.block = unsafe { Some(self.classifier.classify(block)) };
                    self.offset += Twice::<BlockAlignment>::size();
                }
                None => return false,
            }
        }

        true
    }

    #[inline(always)]
    fn current_block_is_spent(&self) -> bool {
        self.block.as_ref().map_or(true, Empty::is_empty)
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
            .map(|x| x.offset(self.offset - Twice::<BlockAlignment>::size()))
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> std::iter::FusedIterator for Avx2Classifier<'a, I> {}

impl<'a, I: QuoteClassifiedIterator<'a>> Empty for Avx2Classifier<'a, I> {
    fn is_empty(&self) -> bool {
        self.current_block_is_spent() && self.iter.is_empty()
    }
}

impl<'a, I: QuoteClassifiedIterator<'a>> StructuralIterator<'a> for Avx2Classifier<'a, I> {}

struct BlockAvx2Classifier {
    lower_nibble_mask: __m256i,
    upper_nibble_mask: __m256i,
    upper_nibble_zeroing_mask: __m256i,
}

struct BlockClassification {
    structural: u32,
}

impl BlockAvx2Classifier {
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

    const EVEN: u64 = 0b0101010101010101010101010101010101010101010101010101010101010101u64;
    const ODD: u64 = 0b1010101010101010101010101010101010101010101010101010101010101010u64;

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new() -> Self {
        Self {
            lower_nibble_mask: _mm256_loadu_si256(
                Self::LOWER_NIBBLE_MASK_ARRAY.as_ptr() as *const __m256i
            ),
            upper_nibble_mask: _mm256_loadu_si256(
                Self::UPPER_NIBBLE_MASK_ARRAY.as_ptr() as *const __m256i
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
            (classification1.structural as u64) | ((classification2.structural as u64) << 32);

        let nonquoted_structural = structural & !quote_classified_block.within_quotes_mask;

        bin!("structural", structural);
        bin!("nonquoted_structural", nonquoted_structural);

        StructuralsBlock::new(quote_classified_block.block, nonquoted_structural)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification {
        let byte_vector = _mm256_loadu_si256(block.as_ptr() as *const __m256i);
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
