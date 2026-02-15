use crate::classification::simd::neon::neon_movemask_epi8;
use ::core::arch::aarch64::*;

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
