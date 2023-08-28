#[cfg(target_arch = "x86")]
use ::core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::*;

#[target_feature(enable = "avx")]
pub(crate) unsafe fn quote_mask() -> __m256i {
    _mm256_set1_epi8(b'"' as i8)
}

#[target_feature(enable = "avx")]
pub(crate) unsafe fn slash_mask() -> __m256i {
    _mm256_set1_epi8(b'\\' as i8)
}

#[target_feature(enable = "avx2")]
pub(crate) unsafe fn classify_block(block: &[u8]) -> BlockClassification256 {
    let byte_vector = _mm256_loadu_si256(block.as_ptr().cast::<__m256i>());

    let slash_cmp = _mm256_cmpeq_epi8(byte_vector, slash_mask());
    let slashes = _mm256_movemask_epi8(slash_cmp) as u32;

    let quote_cmp = _mm256_cmpeq_epi8(byte_vector, quote_mask());
    let quotes = _mm256_movemask_epi8(quote_cmp) as u32;

    BlockClassification256 { slashes, quotes }
}

pub(crate) struct BlockClassification256 {
    pub(crate) slashes: u32,
    pub(crate) quotes: u32,
}
