#[cfg(target_arch = "aarch64")]
use ::core::arch::aarch64::*;

#[inline]
#[target_feature(enable = "neon")]
unsafe fn neon_movemask_epi8(cmp_vector: uint8x16_t) -> i16 {
    // Tablica shiftów
    let shift_values: [i8; 16] = [-7, -6, -5, -4, -3, -2, -1, 0, -7, -6, -5, -4, -3, -2, -1, 0];
    
    // Załaduj shift do NEON
    let vshift = vld1q_s8(shift_values.as_ptr());
    
    // Zamaskowanie bitu 7 (0x80)
    let vmask = vandq_u8(cmp_vector, vdupq_n_u8(0x80));
    
    // Przesunięcie bitów
    let vmask = vshlq_u8(vmask, vshift);
    
    // Sumowanie dolnej i górnej połowy
    let low = vaddv_u8(vget_low_u8(vmask)) as i16;
    let high = vaddv_u8(vget_high_u8(vmask)) as i16;

    low | (high << 8)
}

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
