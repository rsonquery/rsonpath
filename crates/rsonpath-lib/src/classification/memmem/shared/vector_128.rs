#[cfg(target_arch = "x86")]
use ::core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::*;

pub(crate) struct BlockClassifier128 {
    first: __m128i,
    second: __m128i,
}

#[inline(always)]
pub(crate) unsafe fn slash_mask() -> __m128i {
    _mm_set1_epi8(b'\\' as i8)
}

impl BlockClassifier128 {
    #[target_feature(enable = "sse2")]
    pub(crate) unsafe fn new(first: u8, second: u8) -> Self {
        Self {
            first: _mm_set1_epi8(first as i8),
            second: _mm_set1_epi8(second as i8),
        }
    }

    #[target_feature(enable = "sse2")]
    pub(crate) unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification128 {
        let byte_vector = _mm_loadu_si128(block.as_ptr().cast::<__m128i>());

        let first_cmp_vector = _mm_cmpeq_epi8(byte_vector, self.first);
        let second_cmp_vector = _mm_cmpeq_epi8(byte_vector, self.second);
        let slash_cmp_vector = _mm_cmpeq_epi8(byte_vector, slash_mask());

        let first = _mm_movemask_epi8(first_cmp_vector) as u16;
        let second = _mm_movemask_epi8(second_cmp_vector) as u16;
        let slashes = _mm_movemask_epi8(slash_cmp_vector) as u16;

        BlockClassification128 { first, second, slashes }
    }
}

pub(crate) struct BlockClassification128 {
    pub(crate) first: u16,
    pub(crate) second: u16,
    pub(crate) slashes: u16,
}
