use aligners::{alignment::Eight, alignment::TwoTo, AlignedSlice};

pub fn discrepancy_size8(a: &[u8], b: &[u8]) -> Option<usize> {
    assert_eq!(a.len(), b.len());

    for i in 0..a.len() {
        if a[i] != b[i] {
            return Some(i);
        }
    }

    None
}

pub fn discrepancy_size64(a: &AlignedSlice<Eight>, b: &AlignedSlice<Eight>) -> Option<usize> {
    const SIZE: usize = 8;

    for (i, (a_block, b_block)) in a.iter_blocks().zip(b.iter_blocks()).enumerate() {
        let a_vec = unsafe { *(a_block.as_ptr() as *const u64) };
        let b_vec = unsafe { *(b_block.as_ptr() as *const u64) };
        let xor = a_vec ^ b_vec;
        if xor != 0 {
            let idx = i * SIZE + ((xor.trailing_zeros() as usize) / SIZE);
            return Some(idx);
        }
    }
    None
}

pub fn discrepancy_size128(
    a: &AlignedSlice<TwoTo<4>>,
    b: &AlignedSlice<TwoTo<4>>,
) -> Option<usize> {
    if !is_x86_feature_detected!("sse2") {
        panic!("discrepancy_size128 requires SSE2");
    }

    unsafe { discrepancy_size128_impl(a, b) }
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn discrepancy_size128_impl(
    a: &AlignedSlice<TwoTo<4>>,
    b: &AlignedSlice<TwoTo<4>>,
) -> Option<usize> {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    const SIZE: usize = 16;

    for (i, (a_block, b_block)) in a.iter_blocks().zip(b.iter_blocks()).enumerate() {
        let a_vec = _mm_load_si128(a_block.as_ptr() as *const __m128i);
        let b_vec = _mm_load_si128(b_block.as_ptr() as *const __m128i);
        let xor = _mm_cmpeq_epi8(a_vec, b_vec);
        let mask = _mm_movemask_epi8(xor) as u32;

        if mask != 0xFFFF {
            let idx = i * SIZE + (mask.trailing_ones() as usize);
            return Some(idx);
        }
    }

    None
}

pub fn discrepancy_size256(
    a: &AlignedSlice<TwoTo<5>>,
    b: &AlignedSlice<TwoTo<5>>,
) -> Option<usize> {
    if !is_x86_feature_detected!("avx2") {
        panic!("discrepancy_size256 requires AVX2");
    }

    unsafe { discrepancy_size256_impl(a, b) }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn discrepancy_size256_impl(
    a: &AlignedSlice<TwoTo<5>>,
    b: &AlignedSlice<TwoTo<5>>,
) -> Option<usize> {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    const SIZE: usize = 32;

    for (i, (a_block, b_block)) in a.iter_blocks().zip(b.iter_blocks()).enumerate() {
        let a_vec = _mm256_load_si256(a_block.as_ptr() as *const __m256i);
        let b_vec = _mm256_load_si256(b_block.as_ptr() as *const __m256i);
        let xor = _mm256_cmpeq_epi8(a_vec, b_vec);
        let mask = _mm256_movemask_epi8(xor) as u32;

        if mask != 0xFFFFFFFF {
            let idx = i * SIZE + (mask.trailing_ones() as usize);
            return Some(idx);
        }
    }

    None
}

#[cfg(feature = "avx512")]
pub fn discrepancy_size512(
    a: &AlignedSlice<TwoTo<6>>,
    b: &AlignedSlice<TwoTo<6>>,
) -> Option<usize> {
    if !is_x86_feature_detected!("avx512f") && !is_x86_feature_detected!("avx512bw") {
        panic!("discrepancy_size512 requires AVX512f and AVX512bw");
    }

    unsafe { discrepancy_size512_impl(a, b) }
}

#[inline]
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[cfg(feature = "avx512")]
unsafe fn discrepancy_size512_impl(
    a: &AlignedSlice<TwoTo<6>>,
    b: &AlignedSlice<TwoTo<6>>,
) -> Option<usize> {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    const SIZE: usize = 64;
    for (i, (a_block, b_block)) in a.iter_blocks().zip(b.iter_blocks()).enumerate() {
        let a_vec = _mm512_load_si512(a_block.as_ptr() as *const i32);
        let b_vec = _mm512_load_si512(b_block.as_ptr() as *const i32);
        let xor = _mm512_cmpeq_epi8_mask(a_vec, b_vec);

        if xor != 0xFFFFFFFFFFFFFFFF {
            let idx = i * SIZE + (xor.trailing_ones() as usize);
            return Some(idx);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use aligners::AlignedBytes;
    use test_case::test_case;

    #[test_case(
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd],
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd],
        None
    )]
    #[test_case(
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd],
        &[0xa0, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd],
        Some(0)
    )]
    #[test_case(
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd],
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0x0d],
        Some(7)
    )]
    #[test_case(
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, 0xaa, 0xab, 0xac, 0xad],
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, 0x00, 0xab, 0xac, 0xad],
        Some(12)
    )]
    pub fn all_sizes_test(a: &[u8], b: &[u8], expected: Option<usize>) {
        let a_bytes: AlignedBytes<TwoTo<6>> = AlignedBytes::new_padded(a);
        let b_bytes: AlignedBytes<TwoTo<6>> = AlignedBytes::new_padded(b);

        let size1 = discrepancy_size8(a, b);
        let size64 = discrepancy_size64(a_bytes.relax_alignment(), b_bytes.relax_alignment());
        let size128 = discrepancy_size128(a_bytes.relax_alignment(), b_bytes.relax_alignment());
        let size256 = discrepancy_size256(a_bytes.relax_alignment(), b_bytes.relax_alignment());

        #[cfg(feature = "avx512")]
        let size512 = discrepancy_size512(&a_bytes, &b_bytes);

        assert_eq!(expected, size1);
        assert_eq!(expected, size64);
        assert_eq!(expected, size128);
        assert_eq!(expected, size256);

        #[cfg(feature = "avx512")]
        assert_eq!(expected, size512);
    }
}
