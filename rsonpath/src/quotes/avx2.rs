use super::*;
use crate::BlockAlignment;
use aligners::{alignment::*, AlignedBlock, AlignedBlockIterator, AlignedSlice};

use crate::bin;
use crate::debug;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
pub(crate) struct Avx2QuoteClassifier<'a> {
    iter: AlignedBlockIterator<'a, Twice<BlockAlignment>>,
    offset: usize,
    classifier: BlockAvx2Classifier,
}

impl<'a> Avx2QuoteClassifier<'a> {
    #[inline]
    pub(crate) fn new(bytes: &'a AlignedSlice<Twice<BlockAlignment>>) -> Self {
        Self {
            iter: bytes.iter_blocks(),
            offset: 0,
            classifier: unsafe { BlockAvx2Classifier::new() },
        }
    }
}

impl<'a> Iterator for Avx2QuoteClassifier<'a> {
    type Item = QuoteClassifiedBlock<'a>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(block) => {
                let mask = unsafe { self.classifier.classify(block) };
                let classified_block = QuoteClassifiedBlock {
                    block,
                    offset: self.offset,
                    within_quotes_mask: mask,
                };
                self.offset += Twice::<BlockAlignment>::size();
                Some(classified_block)
            }
            None => None,
        }
    }
}

impl std::iter::FusedIterator for Avx2QuoteClassifier<'_> {}

impl len_trait::Empty for Avx2QuoteClassifier<'_> {
    fn is_empty(&self) -> bool {
        self.iter.len() == 0
    }
}

impl<'a> QuoteClassifiedIterator<'a> for Avx2QuoteClassifier<'a> {}

struct BlockAvx2Classifier {
    prev_block_mask: u8,
    quote_mask: __m256i,
    slash_mask: __m256i,
    all_ones128: __m128i,
}

struct BlockClassification {
    slashes: u32,
    quotes: u32,
}

impl BlockAvx2Classifier {
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
            quote_mask: _mm256_set1_epi8(b'"' as i8),
            slash_mask: _mm256_set1_epi8(b'\\' as i8),
            all_ones128: _mm_set1_epi8(0xFFu8 as i8),
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify(&mut self, two_blocks: &AlignedBlock<Twice<BlockAlignment>>) -> u64 {
        let (block1, block2) = two_blocks.halves();
        let classification1 = self.classify_block(block1);
        let classification2 = self.classify_block(block2);

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
            slashes & !(ends_of_even_starts | ends_of_odd_starts),
            within_quotes,
        );

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
        bin!("slashes", slashes);
        bin!("quotes", quotes);
        bin!("prev_slash_bit", self.get_prev_slash_mask());
        bin!("prev_quote_bit", self.get_prev_quote_mask());
        bin!("escaped", escaped);
        bin!("quotes & !escaped", quotes & !escaped);
        bin!("nonescaped_quotes", nonescaped_quotes);
        bin!("within_quotes", within_quotes);

        within_quotes
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification {
        let byte_vector = _mm256_loadu_si256(block.as_ptr() as *const __m256i);

        let slash_cmp = _mm256_cmpeq_epi8(byte_vector, self.slash_mask);
        let slashes = _mm256_movemask_epi8(slash_cmp) as u32;

        let quote_cmp = _mm256_cmpeq_epi8(byte_vector, self.quote_mask);
        let quotes = _mm256_movemask_epi8(quote_cmp) as u32;

        BlockClassification { slashes, quotes }
    }
}

#[cfg(test)]
mod tests {
    use aligners::{alignment::Twice, AlignedBytes};
    use test_case::test_case;

    use crate::BlockAlignment;

    use super::Avx2QuoteClassifier;

    #[test_case("" => None)]
    #[test_case("abcd" => Some(0))]
    #[test_case(r#""abcd""# => Some(0b011111))]
    #[test_case(r#""number": 42, "string": "something" "# => Some(0b001111111111000111111100000001111111))]
    #[test_case(r#"abc\"abc\""# => Some(0b0000000000))]
    #[test_case(r#"abc\\"abc\\""# => Some(0b011111100000))]
    fn single_block(str: &str) -> Option<u64> {
        let bytes: AlignedBytes<Twice<BlockAlignment>> = AlignedBytes::new_padded(str.as_bytes());
        let mut classifier = Avx2QuoteClassifier::new(&bytes);
        classifier.next().map(|x| x.within_quotes_mask)
    }
}
