use crate::classification::simd::neon::neon_movemask_epi8;
use ::core::arch::aarch64::*;

#[inline]
#[target_feature(enable = "neon")]
unsafe fn neon_shuffle(data: int8x16_t, mask: int8x16_t) -> int8x16_t {
    let low = vget_low_s8(data);
    let high = vget_high_s8(data);
    let recombined = int8x8x2_t(low, high);

    vcombine_s8(
        vtbl2_s8(recombined, vand_s8(vdup_n_s8(0x0F), vget_low_s8(mask))),
        vtbl2_s8(recombined, vand_s8(vdup_n_s8(0x0F), vget_high_s8(mask))),
    )
}

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

#[target_feature(enable = "neon")]
#[inline]
pub(crate) unsafe fn upper_nibble_zeroing_mask() -> int8x16_t {
    vdupq_n_s8(0x0F)
}

#[target_feature(enable = "neon")]
#[inline]
pub(crate) unsafe fn lower_nibble_mask() -> int8x16_t {
    vreinterpretq_s8_u8(vld1q_u8(LOWER_NIBBLE_MASK_ARRAY.as_ptr()))
}

#[target_feature(enable = "neon")]
#[inline]
pub(crate) unsafe fn commas_toggle_mask() -> int8x16_t {
    vreinterpretq_s8_u8(vld1q_u8(COMMAS_TOGGLE_MASK_ARRAY.as_ptr()))
}

#[target_feature(enable = "neon")]
#[inline]
pub(crate) unsafe fn colons_toggle_mask() -> int8x16_t {
    vreinterpretq_s8_u8(vld1q_u8(COLON_TOGGLE_MASK_ARRAY.as_ptr()))
}

#[target_feature(enable = "neon")]
#[inline]
pub(crate) unsafe fn colons_and_commas_toggle_mask() -> int8x16_t {
    vorrq_s8(colons_toggle_mask(), commas_toggle_mask())
}

pub(crate) struct BlockClassifierNeon {
    upper_nibble_mask: int8x16_t,
}

impl BlockClassifierNeon {
    #[target_feature(enable = "neon")]
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self {
            upper_nibble_mask: vreinterpretq_s8_u8(vld1q_u8(UPPER_NIBBLE_MASK_ARRAY.as_ptr())),
        }
    }

    #[target_feature(enable = "neon")]
    #[inline]
    pub(crate) unsafe fn toggle_commas(&mut self) {
        self.upper_nibble_mask = veorq_s8(self.upper_nibble_mask, commas_toggle_mask());
    }

    #[target_feature(enable = "neon")]
    #[inline]
    pub(crate) unsafe fn toggle_colons(&mut self) {
        self.upper_nibble_mask = veorq_s8(self.upper_nibble_mask, colons_toggle_mask());
    }

    #[target_feature(enable = "neon")]
    #[inline]
    pub(crate) unsafe fn toggle_colons_and_commas(&mut self) {
        self.upper_nibble_mask = veorq_s8(self.upper_nibble_mask, colons_and_commas_toggle_mask());
    }

    #[target_feature(enable = "neon")]
    #[inline]
    pub(crate) unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification128 {
        let byte_vector = vreinterpretq_s16_u8(vld1q_u8(block.as_ptr()));
        let shifted_byte_vector = vreinterpretq_s8_s16(vshrq_n_s16(byte_vector, 4));
        let upper_nibble_byte_vector = vandq_s8(shifted_byte_vector, upper_nibble_zeroing_mask());
        let lower_nibble_lookup = neon_shuffle(lower_nibble_mask(), vreinterpretq_s8_s16(byte_vector));
        let upper_nibble_lookup = neon_shuffle(self.upper_nibble_mask, upper_nibble_byte_vector);
        let structural_vector = vceqq_s8(lower_nibble_lookup, upper_nibble_lookup);
        let structural = neon_movemask_epi8(structural_vector) as u16;

        BlockClassification128 { structural }
    }
}

pub(crate) struct BlockClassification128 {
    pub(crate) structural: u16,
}
