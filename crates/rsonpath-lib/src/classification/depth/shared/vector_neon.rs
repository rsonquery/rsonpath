use crate::classification::structural::BracketType;

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
        vdupq_n_s8(self.opening + 2)
    }

    #[target_feature(enable = "neon")]
    #[inline]
    pub(crate) unsafe fn get_opening_and_closing_masks(&self, bytes: &[u8]) -> (u16, u16) {
        assert_eq!(16, bytes.len());
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
