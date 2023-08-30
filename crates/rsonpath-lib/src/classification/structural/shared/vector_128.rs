#[cfg(target_arch = "x86")]
use ::core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::*;

const LOWER_NIBBLE_MASK_ARRAY: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x01, 0x02, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x01, 0x02, 0x01, 0xff, 0xff,
];
const UPPER_NIBBLE_MASK_ARRAY: [u8; 32] = [
    0xfe, 0xfe, 0x10, 0x10, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0x10,
    0x10, 0xfe, 0x01, 0xfe, 0x01, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe, 0xfe,
];
const COMMAS_TOGGLE_MASK_ARRAY: [u8; 32] = [
    0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const COLON_TOGGLE_MASK_ARRAY: [u8; 32] = [
    0x00, 0x00, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[target_feature(enable = "sse2")]
pub(crate) unsafe fn upper_nibble_zeroing_mask() -> __m128i {
    _mm_set1_epi8(0x0F)
}

#[target_feature(enable = "sse2")]
pub(crate) unsafe fn lower_nibble_mask() -> __m128i {
    _mm_loadu_si128(LOWER_NIBBLE_MASK_ARRAY.as_ptr().cast::<__m128i>())
}

#[target_feature(enable = "sse2")]
pub(crate) unsafe fn commas_toggle_mask() -> __m128i {
    _mm_loadu_si128(COMMAS_TOGGLE_MASK_ARRAY.as_ptr().cast::<__m128i>())
}

#[target_feature(enable = "sse2")]
pub(crate) unsafe fn colons_toggle_mask() -> __m128i {
    _mm_loadu_si128(COLON_TOGGLE_MASK_ARRAY.as_ptr().cast::<__m128i>())
}

#[target_feature(enable = "sse2")]
pub(crate) unsafe fn colons_and_commas_toggle_mask() -> __m128i {
    _mm_or_si128(colons_toggle_mask(), commas_toggle_mask())
}

pub(crate) struct BlockClassifier128 {
    upper_nibble_mask: __m128i,
}

impl BlockClassifier128 {
    #[target_feature(enable = "sse2")]
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self {
            upper_nibble_mask: _mm_loadu_si128(UPPER_NIBBLE_MASK_ARRAY.as_ptr().cast::<__m128i>()),
        }
    }

    #[target_feature(enable = "sse2")]
    #[inline]
    pub(crate) unsafe fn toggle_commas(&mut self) {
        self.upper_nibble_mask = _mm_xor_si128(self.upper_nibble_mask, commas_toggle_mask());
    }

    #[target_feature(enable = "sse2")]
    #[inline]
    pub(crate) unsafe fn toggle_colons(&mut self) {
        self.upper_nibble_mask = _mm_xor_si128(self.upper_nibble_mask, colons_toggle_mask());
    }

    #[target_feature(enable = "sse2")]
    #[inline]
    pub(crate) unsafe fn toggle_colons_and_commas(&mut self) {
        self.upper_nibble_mask = _mm_xor_si128(self.upper_nibble_mask, colons_and_commas_toggle_mask());
    }

    #[target_feature(enable = "ssse3")]
    #[inline]
    pub(crate) unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification128 {
        let byte_vector = _mm_loadu_si128(block.as_ptr().cast::<__m128i>());
        let shifted_byte_vector = _mm_srli_epi16::<4>(byte_vector);
        let upper_nibble_byte_vector = _mm_and_si128(shifted_byte_vector, upper_nibble_zeroing_mask());
        let lower_nibble_lookup = _mm_shuffle_epi8(lower_nibble_mask(), byte_vector);
        let upper_nibble_lookup = _mm_shuffle_epi8(self.upper_nibble_mask, upper_nibble_byte_vector);
        let structural_vector = _mm_cmpeq_epi8(lower_nibble_lookup, upper_nibble_lookup);
        let structural = _mm_movemask_epi8(structural_vector) as u16;

        BlockClassification128 { structural }
    }
}

pub(crate) struct BlockClassification128 {
    pub(crate) structural: u16,
}
