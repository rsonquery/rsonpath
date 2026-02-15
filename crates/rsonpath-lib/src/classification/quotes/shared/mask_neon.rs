use crate::bin_u64;
use ::core::arch::aarch64::*;

/// Bitmask selecting bits on even positions when indexing from zero.
pub(crate) const ODD: u64 = 0b0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_0101_u64;
/// Bitmask selecting bits on odd positions when indexing from zero.
pub(crate) const EVEN: u64 = 0b1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_1010_u64;

unsafe fn all_ones64() -> u64 {
    0xFFFF_FFFF_FFFF_FFFF
}

pub(crate) struct BlockClassifierNeon {
    /// Compressed information about the state from the previous block.
    /// The first bit is lit iff the previous block ended with an unescaped escape character.
    /// The second bit is lit iff the previous block ended with a starting quote,
    /// meaning that it was not escaped, nor was it the closing quote of a quoted sequence.
    prev_block_mask: u8,
}

impl BlockClassifierNeon {
    pub(crate) fn new() -> Self {
        Self { prev_block_mask: 0 }
    }

    /// Set the inter-block state based on slash overflow and the quotes mask.
    fn update_prev_block_mask(&mut self, set_slash_mask: bool, quotes: u64) {
        let slash_mask = u8::from(set_slash_mask);
        let quote_mask = (((quotes & (1 << 63)) >> 62) as u8) & 0x02;
        self.prev_block_mask = slash_mask | quote_mask;
    }

    /// Flip the inter-block state bit representing the quote state.
    pub(crate) fn flip_prev_quote_mask(&mut self) {
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

    #[target_feature(enable = "neon")]
    #[target_feature(enable = "aes")]
    pub(crate) unsafe fn classify(&mut self, slashes: u64, quotes: u64) -> u64 {
        let (escaped, set_prev_slash_mask) = if slashes == 0 {
            (self.get_prev_slash_mask(), false)
        } else {
            let slashes_excluding_escaped_first = slashes & !self.get_prev_slash_mask();
            let starts = slashes_excluding_escaped_first & !(slashes_excluding_escaped_first << 1);
            let odd_starts = ODD & starts;
            let even_starts = EVEN & starts;

            let odd_starts_carry = odd_starts.wrapping_add(slashes);
            let (even_starts_carry, set_prev_slash_mask) = even_starts.overflowing_add(slashes);

            let ends_of_odd_starts = odd_starts_carry & !slashes;
            let ends_of_even_starts = even_starts_carry & !slashes;

            let escaped = (ends_of_odd_starts & EVEN) | (ends_of_even_starts & ODD) | self.get_prev_slash_mask();

            (escaped, set_prev_slash_mask)
        };

        let nonescaped_quotes = (quotes & !escaped) ^ self.get_prev_quote_mask();

        let cumulative_xor = vmull_p64(nonescaped_quotes, all_ones64());

        let within_quotes = cumulative_xor as u64;
        self.update_prev_block_mask(set_prev_slash_mask, within_quotes);

        bin_u64!("slashes", slashes);
        bin_u64!("quotes", quotes);
        bin_u64!("prev_slash_bit", self.get_prev_slash_mask());
        bin_u64!("prev_quote_bit", self.get_prev_quote_mask());
        bin_u64!("escaped", escaped);
        bin_u64!("quotes & !escaped", quotes & !escaped);
        bin_u64!("nonescaped_quotes", nonescaped_quotes);
        bin_u64!("within_quotes", within_quotes);

        within_quotes
    }
}
