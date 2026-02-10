#[cfg(target_arch = "wasm32")]
use ::core::arch::wasm32::*;
#[cfg(target_arch = "x86")]
use ::core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::core::arch::x86_64::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(crate) struct BlockClassifier128 {
    first: __m128i,
    second: __m128i,
}

#[cfg(target_arch = "wasm32")]
pub(crate) struct BlockClassifier128 {
    first: v128,
    second: v128,
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
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

        let first = _mm_movemask_epi8(first_cmp_vector) as u16;
        let second = _mm_movemask_epi8(second_cmp_vector) as u16;

        BlockClassification128 { first, second }
    }
}

#[cfg(target_arch = "wasm32")]
impl BlockClassifier128 {
    #[target_feature(enable = "simd128")]
    pub(crate) unsafe fn new(first: u8, second: u8) -> Self {
        Self {
            first: i8x16_splat(first as i8),
            second: i8x16_splat(second as i8),
        }
    }

    #[target_feature(enable = "simd128")]
    pub(crate) unsafe fn classify_block(&self, block: &[u8]) -> BlockClassification128 {
        debug_assert_eq!(block.len(), 16);
        let byte_vector = v128_load(block.as_ptr() as *const v128);

        let first_cmp_vector = i8x16_eq(byte_vector, self.first);
        let second_cmp_vector = i8x16_eq(byte_vector, self.second);

        let first = i8x16_bitmask(first_cmp_vector) as u16;
        let second = i8x16_bitmask(second_cmp_vector) as u16;

        BlockClassification128 { first, second }
    }
}

pub(crate) struct BlockClassification128 {
    pub(crate) first: u16,
    pub(crate) second: u16,
}
