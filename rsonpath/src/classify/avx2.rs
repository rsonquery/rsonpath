use super::*;
use aligners::{alignment, alignment::Alignment, AlignedBlock, AlignedBlockIterator, AlignedSlice};

use crate::bin;
use crate::debug;
use len_trait::Empty;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

static_assertions::assert_eq_size!(usize, u64);

struct StructuralsBlock<'a> {
    block: &'a AlignedBlock<alignment::TwoSimdBlocks>,
    structural_mask: u64,
}

impl<'a> StructuralsBlock<'a> {
    #[inline]
    fn new(block: &'a AlignedBlock<alignment::TwoSimdBlocks>, structural_mask: u64) -> Self {
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

pub(crate) struct Avx2Classifier<'a> {
    iter: AlignedBlockIterator<'a, alignment::TwoSimdBlocks>,
    offset: usize,
    classifier: BlockAvx2Classifier,
    block: Option<StructuralsBlock<'a>>,
}

impl<'a> Avx2Classifier<'a> {
    #[inline]
    pub(crate) fn new(bytes: &'a AlignedSlice<alignment::TwoSimdBlocks>) -> Self {
        Self {
            iter: bytes.iter_blocks(),
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

impl Iterator for Avx2Classifier<'_> {
    type Item = Structural;

    #[inline(always)]
    fn next(&mut self) -> Option<Structural> {
        use aligners::alignment;

        if !self.next_block() {
            return None;
        }
        self.block
            .as_mut()
            .unwrap()
            .next()
            .map(|x| x.offset(self.offset - alignment::TwoSimdBlocks::size()))
    }
}

impl std::iter::FusedIterator for Avx2Classifier<'_> {}

impl Empty for Avx2Classifier<'_> {
    fn is_empty(&self) -> bool {
        self.current_block_is_spent() && self.iter.len() == 0
    }
}

impl<'a> StructuralIterator<'a> for Avx2Classifier<'a> {}

struct BlockAvx2Classifier {
    prev_block_mask: u8,
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
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02, 0x01, 0xff, 0x01, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02, 0x01, 0xff, 0x01,
        0xff, 0xff,
    ];
    const UPPER_NIBBLE_MASK_ARRAY: [u8; 32] = [
        0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ];

    const EVEN: u64 = 0b0101010101010101010101010101010101010101010101010101010101010101u64;
    const ODD: u64 = 0b1010101010101010101010101010101010101010101010101010101010101010u64;

    fn update_prev_block_mask(&mut self, slashes: u64, quotes: u64) {
        let slash_mask = (((slashes & (1 << 63)) >> 63) as u8) & 0x01;
        let quote_mask = (((quotes & (1 << 63)) >> 62) as u8) & 0x02;
        self.prev_block_mask = slash_mask | quote_mask;
    }

    fn get_prev_slash_mask(&self) -> u64 {
        (self.prev_block_mask & 0x01) as u64
    }

    fn get_prev_quote_mask(&self) -> u64 {
        ((self.prev_block_mask & 0x02) >> 1) as u64
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new() -> Self {
        Self {
            prev_block_mask: 0,
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

        let starts = slashes & !(slashes << 1) & !self.get_prev_slash_mask();
        let even_starts = Self::EVEN & starts;
        let odd_starts = Self::ODD & starts;

        let ends_of_even_starts = (even_starts.wrapping_add(slashes)) & !slashes;
        let ends_of_odd_starts = (odd_starts.wrapping_add(slashes)) & !slashes;

        let escaped = (ends_of_even_starts & Self::ODD)
            | (ends_of_odd_starts & Self::EVEN)
            | self.get_prev_slash_mask();

        let nonescaped_quotes = (quotes & !escaped) ^ self.get_prev_quote_mask();

        let nonescaped_quotes_vector = _mm_set_epi64x(0, nonescaped_quotes as i64);
        let cumulative_xor = _mm_clmulepi64_si128::<0>(nonescaped_quotes_vector, self.all_ones128);

        let within_quotes = _mm_cvtsi128_si64(cumulative_xor) as u64;
        self.update_prev_block_mask(
            slashes & !ends_of_even_starts & !ends_of_odd_starts,
            within_quotes,
        );

        let nonquoted_structural = structural & !within_quotes;

        debug!(
            "{: >24}: {}",
            "block",
            std::str::from_utf8_unchecked(
                &two_blocks[..64]
                    .iter()
                    .map(|x| if x.is_ascii_whitespace() { b' ' } else { *x })
                    .collect::<Vec<_>>()
            )
        );
        bin!("structural", structural);
        bin!("slashes", slashes);
        bin!("quotes", quotes);
        bin!("prev_slash_bit", self.get_prev_slash_mask());
        bin!("prev_quote_bit", self.get_prev_quote_mask());
        bin!("escaped", escaped);
        bin!("quotes & !escaped", quotes & !escaped);
        bin!("nonescaped_quotes", nonescaped_quotes);
        bin!("within_quotes", within_quotes);
        bin!("nonquoted_structural", nonquoted_structural);

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
