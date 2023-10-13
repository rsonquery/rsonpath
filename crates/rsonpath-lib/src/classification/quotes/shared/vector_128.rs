#[cfg(target_arch = "x86")]
use ::core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::*;

#[inline(always)]
pub(crate) unsafe fn quote_mask() -> __m128i {
    _mm_set1_epi8(b'"' as i8)
}

#[inline(always)]
pub(crate) unsafe fn slash_mask() -> __m128i {
    _mm_set1_epi8(b'\\' as i8)
}

#[target_feature(enable = "sse2")]
pub(crate) unsafe fn classify_block(block: &[u8]) -> BlockClassification128 {
    let byte_vector = _mm_loadu_si128(block.as_ptr().cast::<__m128i>());

    let slash_cmp = _mm_cmpeq_epi8(byte_vector, slash_mask());
    let slashes = _mm_movemask_epi8(slash_cmp) as u16;

    let quote_cmp = _mm_cmpeq_epi8(byte_vector, quote_mask());
    let quotes = _mm_movemask_epi8(quote_cmp) as u16;

    BlockClassification128 { slashes, quotes }
}

pub(crate) struct BlockClassification128 {
    pub(crate) slashes: u16,
    pub(crate) quotes: u16,
}
