use crate::classification::structural::BracketType;

#[cfg(target_arch = "wasm32")]
use core::arch::wasm32::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

pub(crate) struct DelimiterClassifierImpl128 {
    opening: i8,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl DelimiterClassifierImpl128 {
    pub(crate) fn new(opening: BracketType) -> Self {
        let opening = match opening {
            BracketType::Square => b'[',
            BracketType::Curly => b'{',
        };

        Self { opening: opening as i8 }
    }

    #[inline(always)]
    unsafe fn opening_mask(&self) -> __m128i {
        _mm_set1_epi8(self.opening)
    }

    #[inline(always)]
    unsafe fn closing_mask(&self) -> __m128i {
        _mm_set1_epi8(self.opening + 2)
    }

    #[target_feature(enable = "sse2")]
    #[inline]
    pub(crate) unsafe fn get_opening_and_closing_masks(&self, bytes: &[u8]) -> (u16, u16) {
        assert_eq!(16, bytes.len());
        // SAFETY: target_feature invariant
        unsafe {
            let byte_vector = _mm_loadu_si128(bytes.as_ptr().cast::<__m128i>());
            let opening_brace_cmp = _mm_cmpeq_epi8(byte_vector, self.opening_mask());
            let closing_brace_cmp = _mm_cmpeq_epi8(byte_vector, self.closing_mask());
            let opening_mask = _mm_movemask_epi8(opening_brace_cmp) as u16;
            let closing_mask = _mm_movemask_epi8(closing_brace_cmp) as u16;

            (opening_mask, closing_mask)
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl DelimiterClassifierImpl128 {
    pub(crate) fn new(opening: BracketType) -> Self {
        let opening = match opening {
            BracketType::Square => b'[',
            BracketType::Curly => b'{',
        };
        Self { opening: opening as i8 }
    }

    #[inline(always)]
    unsafe fn opening_mask(&self) -> v128 {
        i8x16_splat(self.opening)
    }

    #[inline(always)]
    unsafe fn closing_mask(&self) -> v128 {
        i8x16_splat(self.opening + 2)
    }

    #[target_feature(enable = "simd128")]
    #[inline]
    pub(crate) unsafe fn get_opening_and_closing_masks(&self, bytes: &[u8]) -> (u16, u16) {
        assert_eq!(16, bytes.len());

        let byte_vector = v128_load(bytes.as_ptr() as *const v128);

        let opening_brace_cmp = i8x16_eq(byte_vector, self.opening_mask());
        let closing_brace_cmp = i8x16_eq(byte_vector, self.closing_mask());

        let opening_mask = i8x16_bitmask(opening_brace_cmp) as u16;
        let closing_mask = i8x16_bitmask(closing_brace_cmp) as u16;

        (opening_mask, closing_mask)
    }
}
