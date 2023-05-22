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

use super::*;
use crate::bin;
use crate::debug;
use crate::input::{Input, InputBlock, InputBlockIterator};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const SIZE: usize = 64;

pub(crate) struct Avx2QuoteClassifier<'a, I: Input> {
    _input: &'a I,
    iter: I::BlockIterator<'a, SIZE>,
    offset: Option<usize>,
    classifier: BlockAvx2Classifier,
}

impl<'a, I: Input> Avx2QuoteClassifier<'a, I> {
    #[inline]
    pub(crate) fn new(input: &'a I) -> Self {
        Self {
            _input: input,
            iter: input.iter_blocks::<SIZE>(),
            offset: None,
            // SAFETY: target_feature invariant
            classifier: unsafe { BlockAvx2Classifier::new() },
        }
    }
}

impl<'a, I: Input> Iterator for Avx2QuoteClassifier<'a, I> {
    type Item = QuoteClassifiedBlock<<<I as Input>::BlockIterator<'a, 64> as InputBlockIterator<'a, 64>>::Block, 64>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(block) => {
                if let Some(offset) = self.offset {
                    self.offset = Some(offset + block.len());
                } else {
                    self.offset = Some(0);
                }

                // SAFETY: target_feature invariant
                let mask = unsafe { self.classifier.classify(&block) };
                let classified_block = QuoteClassifiedBlock {
                    block,
                    within_quotes_mask: mask,
                };
                Some(classified_block)
            }
            None => None,
        }
    }
}

impl<I: Input> std::iter::FusedIterator for Avx2QuoteClassifier<'_, I> {}

impl<'a, I: Input + 'a> QuoteClassifiedIterator<'a, I, 64> for Avx2QuoteClassifier<'a, I> {
    fn get_offset(&self) -> usize {
        self.offset.unwrap_or(0)
    }

    fn offset(&mut self, count: isize) {
        debug_assert!(count >= 0);
        debug!("Offsetting by {count}");

        if count == 0 {
            return;
        }

        self.iter.offset(count);

        self.offset = Some(match self.offset {
            None => (count as usize - 1) * BLOCK_SIZE,
            Some(offset) => offset + (count as usize) * BLOCK_SIZE,
        });
    }

    fn flip_quotes_bit(&mut self) {
        self.classifier.flip_prev_quote_mask();
    }
}

struct BlockAvx2Classifier {
    /// Compressed information about the state from the previous block.
    /// The first bit is lit iff the previous block ended with an unescaped escape character.
    /// The second bit is lit iff the previous block ended with a starting quote,
    /// meaning that it was not escaped, nor was it the closing quote of a quoted sequence.
    prev_block_mask: u8,
    /// Constant mask for comparing against the double quote character
    quote_mask: __m256i,
    /// Constant mask for comparing against the backslash character
    slash_mask: __m256i,
    /// Constant mask filled with ones for use with clmul.
    all_ones128: __m128i,
}

struct BlockClassification {
    slashes: u32,
    quotes: u32,
}

impl BlockAvx2Classifier {
    /// Bitmask selecting bits on even positions when indexing from zero.
    const ODD: u64 = 0b0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_u64;
    /// Bitmask selecting bits on odd positions when indexing from zero.
    const EVEN: u64 = 0b1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_u64;

    /// Set the inter-block state based on slash overflow and the quotes mask.
    fn update_prev_block_mask(&mut self, set_slash_mask: bool, quotes: u64) {
        let slash_mask = u8::from(set_slash_mask);
        let quote_mask = (((quotes & (1 << 63)) >> 62) as u8) & 0x02;
        self.prev_block_mask = slash_mask | quote_mask;
    }

    /// Flip the inter-block state bit representing the quote state.
    fn flip_prev_quote_mask(&mut self) {
        self.prev_block_mask ^= 0x02;
    }

    /// Returns 0x01 if the last character of the previous block was an unescaped escape character,
    /// zero otherwise.
    fn get_prev_slash_mask(&self) -> u64 {
        u64::from(self.prev_block_mask & 0x01)
    }

    /// Returns 0x01 if the last character of the previous block was an unescaped quote, zero otherwise.
    fn get_prev_quote_mask(&self) -> u64 {
        u64::from((self.prev_block_mask & 0x02) >> 1)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new() -> Self {
        Self {
            prev_block_mask: 0,
            quote_mask: _mm256_set1_epi8(b'"' as i8),
            slash_mask: _mm256_set1_epi8(b'\\' as i8),
            all_ones128: _mm_set1_epi8(0xFF_u8 as i8),
        }
    }

    #[target_feature(enable = "avx2")]
    #[target_feature(enable = "pclmulqdq")]
    #[inline]
    unsafe fn classify<'a, B: InputBlock<'a, 64>>(&mut self, two_blocks: &B) -> u64 {
        /* For a 64-bit architecture, we classify two adjacent 32-byte blocks and combine their masks
         * into a single 64-bit mask, which is significantly more performant.
         *
         * The step-by-step algorithm for determining which characters are within quotes is as follows:
         *   I. Determine which characters are escaped.
         *      1. Find all backslashes '\' and produce a 64-bit bitmask marking their positions.
         *      2. Identify backslashes not preceded by any other backslashes, the "starts".
         *      3. Find the "ends", positions right after a contiguous sequences of backslashes.
         *          a) Use the "add-carry trick".
         *          b) Do this separately for "starts" at even and odd positions.
         *      4. If an "end" of an even-position "start" occurs at an odd position, it is escaped.
         *         Analogously for "ends" of odd-position "starts" occurring at even positions.
         *   II. Determine quoted sequences.
         *      1. Find all quotes '"' and produce a 64-bit bitmask marking their positions.
         *      2. Exclude escaped quotes based on step I.
         *      3. Mark all characters between quotes by running a cumulative XOR on the bitmask.
         */

        // Steps I.1., II.1.
        let (block1, block2) = two_blocks.halves();
        let classification1 = self.classify_block(block1);
        let classification2 = self.classify_block(block2);

        // Masks are combined by shifting the latter block's 32-bit masks left by 32 bits.
        // From now on when we refer to a "block" we mean the combined 64 bytes of the input.
        let slashes = u64::from(classification1.slashes) | (u64::from(classification2.slashes) << 32);
        let quotes = u64::from(classification1.quotes) | (u64::from(classification2.quotes) << 32);

        let (escaped, set_prev_slash_mask) = if slashes == 0 {
            // If there are no slashes in the input steps I.2, I.3, I.4 can be skipped.
            (self.get_prev_slash_mask(), false)
        } else {
            /* Step I.2.
             *
             * A character is a start of the sequence if it is not preceded by a backslash.
             * We also check whether the last character of the previous block was an unescaped backslash
             * to correctly classify the first character in the block.
             *
             * Visualization for 8-byte-long blocks:
             *                  | prev bl.|curr bl. |
             *  bitmask index   | 76543210 76543210 |
             *  input           | \x\\\\x\ \x\\\x\\ |
             *  slashes         | 10111101 10111011 |
             *  slashes << 1    | 01011110 01011101 |
             *  prev_slash      | 00000000 10000000 |
             *  starts          | 10100001 00100010 |
             *  even_starts     | 00000001 00000000 |
             *  odd_starts      | 10100000 00100010 |
             */

            let slashes_excluding_escaped_first = slashes & !self.get_prev_slash_mask();
            let starts = slashes_excluding_escaped_first & !(slashes_excluding_escaped_first << 1);
            let odd_starts = Self::ODD & starts;
            let even_starts = Self::EVEN & starts;

            /* Step I.3.
             *
             * Recall that in binary arithmetic an addition of two ones at the same place
             * causes a carry - the result bit is set to zero, and the one is carried forward to the next place.
             * To find an end of a contiguous sequence of ones we can use an "add-carry trick" - by adding a number
             * with a bit set exactly at the start of the sequence and adding it to the original mask we cause a carry
             * that propagates up until the end of the sequence.
             *
             * This can overflow, so we use `wrapping_add` to ignore that. In case of the slashes starting at even
             * positions we want to explicitly check for that overflow - if it occurs, it means that all the bits
             * from some even position `i` up to the position `0` were lit, and thus the backslash at position `0`
             * is _not_ escaped (since there was an even number of backslashes preceding it).
             * We should therefore set the `prev_slash` mask if and only if an overflow occurs here.
             *
             * Visualization for 8-byte-long blocks:
             *                    | prev bl.|curr bl. |
             *  bitmask index     | 76543210 76543210 |
             *  input             | \x\\\\x\ \x\\\x\\ |
             *  slashes           | 10111101 10111011 |
             *  even_starts       | 00000001 00000000 |
             *  even_starts_carry | 10111100 10111011 | <-- Overflow occurs!
             *  slashes           | 10111101 10111011 |
             *  odd_starts        | 10100000 00100010 |
             *  odd_starts_carry  | 01000011 10000100 | <-- Overflow occurs, but is inconsequential.
             */

            let odd_starts_carry = odd_starts.wrapping_add(slashes);
            let (even_starts_carry, set_prev_slash_mask) = even_starts.overflowing_add(slashes);

            // We need to exclude `slashes`, as the ones from the opposite-parity positions
            // cause noise in the mask. Note in the above how `even_starts_carry` contains
            // almost all bits copied over from slashes that did not cause a carry, but
            // in actuality the only "end of an even start" is the one lost to overflow.
            let ends_of_odd_starts = odd_starts_carry & !slashes;
            let ends_of_even_starts = even_starts_carry & !slashes;

            /* Step I.4.
             *
             * Find the characters preceded by a contiguous sequence of backslashes of odd length.
             * Note that the `escaped` mask is completely arbitrary for the backslash characters themselves,
             * but that is irrelevant to any further processing steps.
             *
             * Visualization for 8-byte-long blocks:
             *                      | prev bl.|curr bl. |
             *  bitmask index       | 76543210 76543210 |
             *  input               | \x\\\\x\ \x\\\x\\ |
             *  ends_of_odd_starts  | 01000010 00000100 |
             *  ends_of_even_starts | 00000001 00000000 |
             *  prev_slash          | 00000000 10000000 |
             *  escaped             | 01000000 10000100 |
             */
            let escaped =
                (ends_of_odd_starts & Self::EVEN) | (ends_of_even_starts & Self::ODD) | self.get_prev_slash_mask();

            (escaped, set_prev_slash_mask)
        };

        /* Step II.2.
         *
         * Select only unescaped quotes.
         *
         * We also check whether the last character of the previous block was still within quotes
         * and flip the first bit if it was. Assume that is the case - then there are two possibilities:
         *  1. The first character of the current block was a quote.
         *     That quote is then not marked as an unescaped quote, but clearly it was a closing quote,
         *     so it can be safely ignored.
         *  2. The first character of the current block was not a quote.
         *     As it follows from the clmul operation, the first character in the current block will then
         *     correctly be marked as quoted.
         */
        let nonescaped_quotes = (quotes & !escaped) ^ self.get_prev_quote_mask();

        /*
         * Step II.3.
         *
         * The clmul operation's semantics when given a 128-bit vector `a` as the first operand and
         * an all-ones 128-bit vector `b` as the second operand are the same as a cumulative XOR.
         * Therefore, a lit bit of `nonescaped_quotes` will be "spread" up until a pairing lit bit
         * occurs in the mask, which exactly corresponds to marking all characters after a quote
         * up until the pairing closing quote is found.
         *
         * We only use the lower 64 bits of the vector, so we first copy the mask with `_mm_set_epi64x`
         * and then extract the 64-bit result with `_mm_cvtsi128_si64`.
         *
         * Again, note that the quoted classification for the delimiting quotes themselves can be arbitrary.
         *
         * Visualization for 8-byte-long blocks:
         *                      | prev bl.|curr bl. |
         *  bitmask index       | 76543210 76543210 |
         *  input               | "xx"xxx" xx"xx"xx |
         *  quotes              | 10010001 00100100 |
         *  prev_quote          | 00000000 10000000 |
         *  nonescaped_quotes   | 10010001 10100100 |
         *  cumulative_xor      | 11100001 11000111 |
         */
        let nonescaped_quotes_vector = _mm_set_epi64x(0, nonescaped_quotes as i64);
        let cumulative_xor = _mm_clmulepi64_si128::<0>(nonescaped_quotes_vector, self.all_ones128);

        let within_quotes = _mm_cvtsi128_si64(cumulative_xor) as u64;
        self.update_prev_block_mask(set_prev_slash_mask, within_quotes);

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
        let byte_vector = _mm256_loadu_si256(block.as_ptr().cast::<__m256i>());

        let slash_cmp = _mm256_cmpeq_epi8(byte_vector, self.slash_mask);
        let slashes = _mm256_movemask_epi8(slash_cmp) as u32;

        let quote_cmp = _mm256_cmpeq_epi8(byte_vector, self.quote_mask);
        let quotes = _mm256_movemask_epi8(quote_cmp) as u32;

        BlockClassification { slashes, quotes }
    }
}

#[cfg(test)]
mod tests {
    use super::Avx2QuoteClassifier;
    use crate::input::OwnedBytes;
    use test_case::test_case;

    #[test_case("" => None)]
    #[test_case("abcd" => Some(0))]
    #[test_case(r#""abcd""# => Some(0b01_1111))]
    #[test_case(r#""number": 42, "string": "something" "# => Some(0b0011_1111_1111_0001_1111_1100_0000_0111_1111))]
    #[test_case(r#"abc\"abc\""# => Some(0b00_0000_0000))]
    #[test_case(r#"abc\\"abc\\""# => Some(0b0111_1110_0000))]
    #[test_case(r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"e":{"a":[[],[1,2,3],"# => Some(0b0_0000_0000_0000_0110_0011_0000_0000_0000_0110_0011_0000_0001_1110))]
    fn single_block(str: &str) -> Option<u64> {
        let owned_str = str.to_owned();
        let input = OwnedBytes::new(&owned_str).unwrap();
        let mut classifier = Avx2QuoteClassifier::new(&input);
        classifier.next().map(|x| x.within_quotes_mask)
    }
}
