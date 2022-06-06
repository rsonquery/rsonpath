use aligners::{alignment::TwoTo, AlignedSlice};

pub fn find_byte_nosimd(byte: u8, slice: &[u8]) -> Option<usize> {
    for (i, &b) in slice.iter().enumerate() {
        if b == byte {
            return Some(i);
        }
    }
    None
}

pub fn find_byte_rust_nosimd(byte: u8, slice: &[u8]) -> Option<usize> {
    slice.iter().position(|&x| x == byte)
}

pub fn find_byte_size128(a: u8, b: &AlignedSlice<TwoTo<4>>) -> Option<usize> {
    if !is_x86_feature_detected!("sse2") {
        panic!("find_byte_size128 requires SSE2");
    }

    unsafe { find_byte_size128_impl(a, b) }
}

#[target_feature(enable = "sse2")]
unsafe fn find_byte_size128_impl(byte: u8, bytes: &AlignedSlice<TwoTo<4>>) -> Option<usize> {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    const SIZE: usize = 16;

    let byte_mask = _mm_set1_epi8(byte as i8);

    for (i, block) in bytes.iter_blocks().enumerate() {
        let vec = _mm_load_si128(block.as_ptr() as *const __m128i);
        let cmp_vector = _mm_cmpeq_epi8(vec, byte_mask);
        let cmp_packed = _mm_movemask_epi8(cmp_vector);

        if cmp_packed != 0 {
            return Some(i * SIZE + (cmp_packed.trailing_zeros() as usize));
        }
    }

    None
}

pub fn find_byte_size256(a: u8, b: &AlignedSlice<TwoTo<5>>) -> Option<usize> {
    if !is_x86_feature_detected!("avx2") {
        panic!("find_byte_size256 requires AVX2");
    }

    unsafe { find_byte_size256_impl(a, b) }
}

#[target_feature(enable = "avx2")]
unsafe fn find_byte_size256_impl(byte: u8, bytes: &AlignedSlice<TwoTo<5>>) -> Option<usize> {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    const SIZE: usize = 32;

    let byte_mask = _mm256_set1_epi8(byte as i8);

    for (i, block) in bytes.iter_blocks().enumerate() {
        let vec = _mm256_load_si256(block.as_ptr() as *const __m256i);
        let cmp_vector = _mm256_cmpeq_epi8(vec, byte_mask);
        let cmp_packed = _mm256_movemask_epi8(cmp_vector);

        if cmp_packed != 0 {
            return Some(i * SIZE + (cmp_packed.trailing_zeros() as usize));
        }
    }

    None
}

#[cfg(feature = "avx512")]
pub fn find_byte_size512(a: u8, b: &AlignedSlice<TwoTo<6>>) -> Option<usize> {
    if !is_x86_feature_detected!("avx512f") && !is_x86_feature_detected!("avx512bw") {
        panic!("discrepancy_size512 requires AVX512f and AVX512bw");
    }

    unsafe { find_byte_size512_impl(a, b) }
}

#[inline]
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
#[cfg(feature = "avx512")]
unsafe fn find_byte_size512_impl(byte: u8, bytes: &AlignedSlice<TwoTo<6>>) -> Option<usize> {
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;
    const SIZE: usize = 64;

    let byte_mask = _mm512_set1_epi8(byte as i8);

    for (i, block) in bytes.iter_blocks().enumerate() {
        let vec = _mm512_load_si512(block.as_ptr() as *const i32);
        let cmp_packed = _mm512_cmpeq_epi8_mask(vec, byte_mask);

        if cmp_packed != 0 {
            return Some(i * SIZE + (cmp_packed.trailing_zeros() as usize));
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
        0xee,
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd],
        None
    )]
    #[test_case(
        0xa0,
        &[0xa0, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd],
        Some(0)
    )]
    #[test_case(
        0x0d,
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0x0d],
        Some(7)
    )]
    #[test_case(
        0x00,
        &[0xaa, 0xbb, 0xcc, 0xdd, 0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x22, 0x33, 0x44, 0x00, 0xab, 0xac, 0xad],
        Some(12)
    )]
    pub fn all_sizes_test(byte: u8, bytes: &[u8], expected: Option<usize>) {
        let aligned_bytes: AlignedBytes<TwoTo<6>> = AlignedBytes::new_padded(bytes);

        let size1 = find_byte_nosimd(byte, &aligned_bytes);
        let size64 = find_byte_rust_nosimd(byte, &aligned_bytes);
        let size128 = find_byte_size128(byte, aligned_bytes.relax_alignment());
        let size256 = find_byte_size256(byte, aligned_bytes.relax_alignment());

        #[cfg(feature = "avx512")]
        let size512 = find_byte_size512(byte, &aligned_bytes);

        assert_eq!(expected, size1);
        assert_eq!(expected, size64);
        assert_eq!(expected, size128);
        assert_eq!(expected, size256);

        #[cfg(feature = "avx512")]
        assert_eq!(expected, size512);
    }
}
