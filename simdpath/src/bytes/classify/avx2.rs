use super::common::*;
use align::{
    alignment, alignment::Alignment, AlignedBlock, AlignedBlockIterator, AlignedSlice,
};

use len_trait::Empty;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

static_assertions::assert_eq_size!(usize, u64);

pub struct StructuralsBlock<'a> {
    block: &'a AlignedBlock<alignment::TwoSimdBlocks>,
    structural_mask: u64,
}

impl<'a> StructuralsBlock<'a> {
    #[inline(always)]
    pub fn new(block: &'a AlignedBlock<alignment::TwoSimdBlocks>, structural_mask: u64) -> Self {
        Self {
            block,
            structural_mask,
        }
    }
}

impl<'a> Iterator for StructuralsBlock<'a> {
    type Item = Structural;

    #[inline(always)]
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
            b']' | b'}' => Closing(idx),
            b'[' | b'{' => Opening(idx),
            _ => Colon(idx),
        };

        Some(character)
    }
}

impl<'a> std::iter::FusedIterator for StructuralsBlock<'a> {}

impl<'a> ExactSizeIterator for StructuralsBlock<'a> {
    fn len(&self) -> usize {
        self.structural_mask.count_ones() as usize
    }
}

impl<'a> Empty for StructuralsBlock<'a> {
    fn is_empty(&self) -> bool {
        self.structural_mask == 0
    }
}

pub struct Avx2Classifier<'a> {
    iter: AlignedBlockIterator<'a, alignment::TwoSimdBlocks>,
    offset: usize,
    classifier: BlockAvx2Classifier,
    block: Option<StructuralsBlock<'a>>
}

impl<'a> Avx2Classifier<'a> {
    #[inline(always)]
    pub fn new(bytes: &'a AlignedSlice<alignment::TwoSimdBlocks>) -> Self {
        Self {
            iter: bytes.iter_blocks(),
            offset: 0,
            classifier: unsafe { BlockAvx2Classifier::new() },
            block: None
        }
    }

    #[inline(always)]
    fn next_block(&mut self) -> bool {
        while self.current_block_is_spent() {
            match self.iter.next() {
                Some(block) => {
                    self.block = unsafe { Some(self.classifier.classify(block)) };
                    self.offset += alignment::TwoSimdBlocks::size();
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

impl<'a> Iterator for Avx2Classifier<'a> {
    type Item = Structural;

    #[inline(always)]
    fn next(&mut self) -> Option<Structural> {
        use crate::bytes::simd::BLOCK_SIZE;

        if !self.next_block() {
            return None;
        }
        self.block
            .as_mut()
            .unwrap()
            .next()
            .map(|x| x.offset(self.offset - 2 * BLOCK_SIZE))
    }
}

impl<'a> std::iter::FusedIterator for Avx2Classifier<'a> {}

impl<'a> Empty for Avx2Classifier<'a> {
    fn is_empty(&self) -> bool {
        self.current_block_is_spent() && self.iter.len() == 0
    }
}

impl<'a> StructuralIterator<'a> for Avx2Classifier<'a> {}

pub struct BlockAvx2Classifier {
    prev_quote_bit: u64,
    prev_slash_bit: u64,
    lower_nibble_mask: __m256i,
    upper_nibble_mask: __m256i,
    upper_nibble_zeroing_mask: __m256i,
    quote_mask: __m256i,
    slash_mask: __m256i,
    all_ones128: __m128i,
}

struct BlockClassification {
    slashes: u32,
    quotes: u32,
    structural: u32,
}

impl BlockAvx2Classifier {
    const LOWER_NIBBLE_MASK_ARRAY: [u8; 32] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x02,
        0x00, 0x00,
    ];
    const UPPER_NIBBLE_MASK_ARRAY: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0x01, 0xFF, 0x02, 0xFF, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0xFF, 0x02, 0xFF, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF,
    ];

    const EVEN: u64 = 0b0101010101010101010101010101010101010101010101010101010101010101u64;
    const ODD: u64 = 0b1010101010101010101010101010101010101010101010101010101010101010u64;

    #[target_feature(enable = "avx2")]
    #[inline]
    pub unsafe fn new() -> Self {
        Self {
            prev_quote_bit: 0,
            prev_slash_bit: 0,
            lower_nibble_mask: _mm256_loadu_si256(
                Self::LOWER_NIBBLE_MASK_ARRAY.as_ptr() as *const __m256i
            ),
            upper_nibble_mask: _mm256_loadu_si256(
                Self::UPPER_NIBBLE_MASK_ARRAY.as_ptr() as *const __m256i
            ),
            upper_nibble_zeroing_mask: _mm256_set1_epi8(0x0F),
            quote_mask: _mm256_set1_epi8(b'"' as i8),
            slash_mask: _mm256_set1_epi8(b'\\' as i8),
            all_ones128: _mm_set1_epi8(0xFFu8 as i8),
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify<'a>(
        &mut self,
        two_blocks: &'a AlignedBlock<alignment::TwoSimdBlocks>,
    ) -> StructuralsBlock<'a> {
        let (block1, block2) = two_blocks.blocks();
        let classification1 = self.classify_block(block1);
        let classification2 = self.classify_block(block2);

        let structural =
            (classification1.structural as u64) | ((classification2.structural as u64) << 32);
        let slashes = (classification1.slashes as u64) | ((classification2.slashes as u64) << 32);
        let quotes = (classification1.quotes as u64) | ((classification2.quotes as u64) << 32);

        let starts = slashes & !(slashes << 1) & !self.prev_slash_bit;
        let even_starts = Self::EVEN & starts;
        let odd_starts = Self::ODD & starts;

        let ends_of_even_starts = (even_starts.wrapping_add(slashes)) & !slashes;
        let ends_of_odd_starts = (odd_starts.wrapping_add(slashes)) & !slashes;

        let escaped = (ends_of_even_starts & Self::ODD)
            | (ends_of_odd_starts & Self::EVEN)
            | self.prev_slash_bit;
        self.prev_slash_bit =
            (slashes & !ends_of_even_starts & !ends_of_odd_starts & (1 << 63)) >> 63;

        let nonescaped_quotes = (quotes & !escaped) ^ self.prev_quote_bit;

        let nonescaped_quotes_vector = _mm_set_epi64x(0, nonescaped_quotes as i64);
        let cumulative_xor = _mm_clmulepi64_si128::<0>(nonescaped_quotes_vector, self.all_ones128);

        let within_quotes = _mm_cvtsi128_si64(cumulative_xor) as u64;
        self.prev_quote_bit = (within_quotes & (1 << 63)) >> 63;

        let nonquoted_structural = structural & !within_quotes;

        /*log::debug!(
            "{: >24}: {}",
            "block",
            std::str::from_utf8_unchecked(
                &block1[..64]
                    .iter()
                    .map(|x| if x.is_ascii_whitespace() { b' ' } else { *x })
                    .collect::<Vec<_>>()
            )
        );
        bin!("structural", structural);
        bin!("slashes", slashes);
        bin!("quotes", quotes);
        bin!("prev_slash_bit", self.prev_slash_bit);
        bin!("prev_quote_bit", self.prev_quote_bit);
        bin!("escaped", escaped);
        bin!("quotes & !escaped", quotes & !escaped);
        bin!("nonescaped_quotes", nonescaped_quotes);
        bin!("within_quotes", within_quotes);
        bin!("nonquoted_structural", nonquoted_structural);*/

        StructuralsBlock::new(two_blocks, nonquoted_structural)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification {
        let byte_vector = _mm256_load_si256(block.as_ptr() as *const __m256i);
        let shifted_byte_vector = _mm256_srli_epi16::<4>(byte_vector);
        let upper_nibble_byte_vector =
            _mm256_and_si256(shifted_byte_vector, self.upper_nibble_zeroing_mask);
        let lower_nibble_lookup = _mm256_shuffle_epi8(self.lower_nibble_mask, byte_vector);
        let upper_nibble_lookup =
            _mm256_shuffle_epi8(self.upper_nibble_mask, upper_nibble_byte_vector);
        let structural_vector = _mm256_cmpeq_epi8(lower_nibble_lookup, upper_nibble_lookup);
        let structural = _mm256_movemask_epi8(structural_vector) as u32;

        let slash_cmp = _mm256_cmpeq_epi8(byte_vector, self.slash_mask);
        let slashes = _mm256_movemask_epi8(slash_cmp) as u32;

        let quote_cmp = _mm256_cmpeq_epi8(byte_vector, self.quote_mask);
        let quotes = _mm256_movemask_epi8(quote_cmp) as u32;

        BlockClassification {
            structural,
            slashes,
            quotes,
        }
    }
}
