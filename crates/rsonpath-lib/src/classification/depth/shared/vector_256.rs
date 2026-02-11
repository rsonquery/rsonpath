use crate::classification::structural::BracketType;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

pub(crate) struct DelimiterClassifierImpl256 {
    opening: i8,
}

impl DelimiterClassifierImpl256 {
    pub(crate) fn new(opening: BracketType) -> Self {
        let opening = match opening {
            BracketType::Square => b'[',
            BracketType::Curly => b'{',
        };

        Self { opening: opening as i8 }
    }

    #[inline(always)]
    unsafe fn opening_mask(&self) -> __m256i {
        _mm256_set1_epi8(self.opening)
    }

    #[inline(always)]
    unsafe fn closing_mask(&self) -> __m256i {
        _mm256_set1_epi8(self.opening + 2)
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    pub(crate) unsafe fn get_opening_and_closing_masks(&self, bytes: &[u8]) -> (u32, u32) {
        assert_eq!(32, bytes.len(), "vector_256 requires 32 bytes");
        // SAFETY: target_feature invariant
        unsafe {
            let byte_vector = _mm256_loadu_si256(bytes.as_ptr().cast::<__m256i>());
            let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, self.opening_mask());
            let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, self.closing_mask());
            let opening_mask = _mm256_movemask_epi8(opening_brace_cmp) as u32;
            let closing_mask = _mm256_movemask_epi8(closing_brace_cmp) as u32;

            (opening_mask, closing_mask)
        }
    }
}
