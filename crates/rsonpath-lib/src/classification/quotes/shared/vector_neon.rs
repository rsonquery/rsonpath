use crate::classification::simd::neon::neon_movemask_epi8;
use ::core::arch::aarch64::*;

#[inline(always)]
pub(crate) unsafe fn quote_mask() -> int8x16_t {
    vdupq_n_s8(b'"' as i8)
}

#[inline(always)]
pub(crate) unsafe fn slash_mask() -> int8x16_t {
    vdupq_n_s8(b'\\' as i8)
}

#[target_feature(enable = "neon")]
pub(crate) unsafe fn classify_block(block: &[u8]) -> BlockClassificationNeon {
    let byte_vector = vreinterpretq_s8_u8(vld1q_u8(block.as_ptr()));

    let slash_cmp = vceqq_s8(byte_vector, slash_mask());
    let slashes = neon_movemask_epi8(slash_cmp) as u16;

    let quote_cmp = vceqq_s8(byte_vector, quote_mask());
    let quotes = neon_movemask_epi8(quote_cmp) as u16;

    BlockClassificationNeon { slashes, quotes }
}

pub(crate) struct BlockClassificationNeon {
    pub(crate) slashes: u16,
    pub(crate) quotes: u16,
}
