#[cfg(target_arch = "aarch64")]
use ::core::arch::aarch64::*;

#[cfg(target_arch = "aarch64")]
use ::core::arch::aarch64::*;

#[inline]
#[target_feature(enable = "neon")]
unsafe fn neon_movemask_epi8(cmp_vector: uint8x16_t) -> i16 {
    let shift_values: [i8; 16] = [-7, -6, -5, -4, -3, -2, -1, 0, -7, -6, -5, -4, -3, -2, -1, 0];
    
    let vshift = vld1q_s8(shift_values.as_ptr());
    
    let vmask = vandq_u8(cmp_vector, vdupq_n_u8(0x80));
    
    let vmask = vshlq_u8(vmask, vshift);
    
    let low = vaddv_u8(vget_low_u8(vmask)) as i16;
    let high = vaddv_u8(vget_high_u8(vmask)) as i16;

    low | (high << 8)
}

pub(crate) struct BlockClassifierNeon {
    first: int8x16_t,
    second: int8x16_t,
}

impl BlockClassifierNeon {
    #[target_feature(enable = "neon")]
    pub(crate) unsafe fn new(first: u8, second: u8) -> Self {
        Self {
            first: vdupq_n_s8(first as i8),
            second: vdupq_n_s8(second as i8),
        }
    }

    #[target_feature(enable = "neon")]
    pub(crate) unsafe fn classify_block(&self, block: &[u8]) -> BlockClassificationNeon {
        // vld1q zakłada alignment, ale na nowszych powinno działać
        let byte_vector = vld1q_s8(block.as_ptr().cast::<i8>());

        let first_cmp_vector = vceqq_s8(byte_vector, self.first);
        let second_cmp_vector = vceqq_s8(byte_vector, self.second);

        let first = neon_movemask_epi8(first_cmp_vector) as u16;
        let second = neon_movemask_epi8(second_cmp_vector) as u16;

        BlockClassificationNeon { first, second }
    }
}

pub(crate) struct BlockClassificationNeon {
    pub(crate) first: u16,
    pub(crate) second: u16,
}
