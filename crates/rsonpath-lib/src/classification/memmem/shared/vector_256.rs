#[cfg(target_arch = "x86")]
use ::core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::*;

pub(crate) struct BlockClassifier256 {
    first: __m256i,
    second: __m256i,
}

#[inline(always)]
pub(crate) unsafe fn slash_mask() -> __m256i {
    _mm256_set1_epi8(b'\\' as i8)
}

#[inline(always)]
pub(crate) unsafe fn quote_mask() -> __m256i {
    _mm256_set1_epi8(b'"' as i8)
}

impl BlockClassifier256 {
    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn new(first: u8, second: u8) -> Self {
        Self {
            first: _mm256_set1_epi8(first as i8),
            second: _mm256_set1_epi8(second as i8),
        }
    }

    #[target_feature(enable = "avx2")]
    pub(crate) unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification256 {
        let byte_vector = _mm256_loadu_si256(block.as_ptr().cast::<__m256i>());

        let first_cmp_vector = _mm256_cmpeq_epi8(byte_vector, self.first);
        let second_cmp_vector = _mm256_cmpeq_epi8(byte_vector, self.second);
        let slash_cmp_vector = _mm256_cmpeq_epi8(byte_vector, slash_mask());
        let quote_cmp_vector = _mm256_cmpeq_epi8(byte_vector, quote_mask());

        let first = _mm256_movemask_epi8(first_cmp_vector) as u32;
        let second = _mm256_movemask_epi8(second_cmp_vector) as u32;
        let slashes = _mm256_movemask_epi8(slash_cmp_vector) as u32;
        let quotes = _mm256_movemask_epi8(quote_cmp_vector) as u32;

        BlockClassification256 {
            first,
            second,
            slashes,
            quotes,
        }
    }
}

pub(crate) struct BlockClassification256 {
    pub(crate) first: u32,
    pub(crate) second: u32,
    pub(crate) slashes: u32,
    pub(crate) quotes: u32,
}
