use crate::classification::simd::neon::neon_movemask_epi8;
use crate::classification::structural::BracketType;
use ::core::arch::aarch64::*;

pub(crate) struct DelimiterClassifierImplNeon {
    opening: i8,
}

impl DelimiterClassifierImplNeon {
    pub(crate) fn new(opening: BracketType) -> Self {
        let opening = match opening {
            BracketType::Square => b'[',
            BracketType::Curly => b'{',
        };

        Self { opening: opening as i8 }
    }

    #[inline(always)]
    unsafe fn opening_mask(&self) -> int8x16_t {
        vdupq_n_s8(self.opening)
    }

    #[inline(always)]
    unsafe fn closing_mask(&self) -> int8x16_t {
        // Difference between ASCII codes of opening and closing is always 2.
        // b'[' + 2 == b']' and b'{' + 2 == b'}'
        vdupq_n_s8(self.opening + 2)
    }

    #[target_feature(enable = "neon")]
    #[inline]
    pub(crate) unsafe fn get_opening_and_closing_masks(&self, bytes: &[u8]) -> (u16, u16) {
        assert_eq!(16, bytes.len(), "vector_neon requires 16 bytes");
        // SAFETY: target_feature invariant
        unsafe {
            let byte_vector = vreinterpretq_s8_u8(vld1q_u8(bytes.as_ptr()));
            let opening_brace_cmp = vceqq_s8(byte_vector, self.opening_mask());
            let closing_brace_cmp = vceqq_s8(byte_vector, self.closing_mask());
            let opening_mask = neon_movemask_epi8(opening_brace_cmp) as u16;
            let closing_mask = neon_movemask_epi8(closing_brace_cmp) as u16;

            (opening_mask, closing_mask)
        }
    }
}
