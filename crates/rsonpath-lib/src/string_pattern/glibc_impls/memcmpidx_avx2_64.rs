//! Comparison between two byte slices optimized for AVX2.
//!
//! This is heavily derived from the [glibc memcmp-avx2-movbe implementation](https://sourceware.org/git/?p=glibc.git;a=blob;f=sysdeps/x86_64/multiarch/memcmp-avx2-movbe.S;h=22d5b2dbb671cfccce0869c8ced4a11a58a5a4cd;hb=HEAD),
//! but returning the first index at which a mismatch occurred,
//! and is implemented in Rust with intrinsics instead of raw assembly.
//!
//! Note the word "derived". Indeed, while it wasn't a trivial translation,
//! this is undeniably a derived work. Therefore, this source file is licensed
//! under GNU Lesser General Public License v2.1 (LGPL), as it is derived from LGPL-licensed
//! glibc. A copy of the license is located in the string_pattern module directory.

use core::arch::x86_64::*;

macro_rules! VEC_SIZE {
    () => {
        32
    };
}
macro_rules! PAGE {
    () => {
        4096
    };
}

#[inline]
#[target_feature(enable = "avx2")]
pub(crate) unsafe fn cmpeq_forward(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
    let a_ptr = a.as_ptr().cast();
    let b_ptr = b.as_ptr().cast();

    if size < VEC_SIZE!() {
        return less_than_vec(a, b, size);
    }

    let a_vec = _mm256_loadu_si256(a_ptr);
    let b_vec = _mm256_loadu_si256(b_ptr);
    let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
    let mask = _mm256_movemask_epi8(cmpeq) as u32;
    let neq_idx = mask.trailing_ones();
    if neq_idx != VEC_SIZE!() {
        return Some(neq_idx as usize);
    }
    if size <= VEC_SIZE!() * 2 {
        return last_1x_vec(a, b, size);
    }

    let a_vec = _mm256_loadu_si256(a_ptr.add(1));
    let b_vec = _mm256_loadu_si256(b_ptr.add(1));
    let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
    let mask = _mm256_movemask_epi8(cmpeq) as u32;
    let neq_idx = mask.trailing_ones();
    if neq_idx != VEC_SIZE!() {
        return Some(neq_idx as usize + VEC_SIZE!());
    }
    if size <= VEC_SIZE!() * 4 {
        return last_2x_vec(a, b, size);
    }

    let a_vec = _mm256_loadu_si256(a_ptr.add(2));
    let b_vec = _mm256_loadu_si256(b_ptr.add(2));
    let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
    let mask = _mm256_movemask_epi8(cmpeq) as u32;
    let neq_idx = mask.trailing_ones();
    if neq_idx != VEC_SIZE!() {
        return Some(neq_idx as usize + 2 * VEC_SIZE!());
    }

    let a_vec = _mm256_loadu_si256(a_ptr.add(3));
    let b_vec = _mm256_loadu_si256(b_ptr.add(3));
    let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
    let mask = _mm256_movemask_epi8(cmpeq) as u32;
    let neq_idx = mask.trailing_ones();
    if neq_idx != VEC_SIZE!() {
        return Some(neq_idx as usize + 3 * VEC_SIZE!());
    }
    if size > VEC_SIZE!() * 8 {
        return more_8x_vec(a, b, size);
    }

    let a_end: *const __m256i = a.as_ptr().add(size).cast();
    let b_end: *const __m256i = b.as_ptr().add(size).cast();

    let a_vec = _mm256_loadu_si256(a_end.offset(-4));
    let b_vec = _mm256_loadu_si256(b_end.offset(-4));
    let cmpeq_1 = _mm256_cmpeq_epi8(a_vec, b_vec);
    let a_vec = _mm256_loadu_si256(a_end.offset(-3));
    let b_vec = _mm256_loadu_si256(b_end.offset(-3));
    let cmpeq_2 = _mm256_cmpeq_epi8(a_vec, b_vec);
    let a_vec = _mm256_loadu_si256(a_end.offset(-2));
    let b_vec = _mm256_loadu_si256(b_end.offset(-2));
    let cmpeq_3 = _mm256_cmpeq_epi8(a_vec, b_vec);
    let a_vec = _mm256_loadu_si256(a_end.offset(-1));
    let b_vec = _mm256_loadu_si256(b_end.offset(-1));
    let cmpeq_4 = _mm256_cmpeq_epi8(a_vec, b_vec);

    let reduced_1 = _mm256_and_si256(cmpeq_4, cmpeq_3);
    let reduced_2 = _mm256_and_si256(cmpeq_2, cmpeq_1);
    let reduced = _mm256_and_si256(reduced_1, reduced_2);
    let mask = _mm256_movemask_epi8(reduced) as u32;
    let neq_idx = mask.trailing_ones();
    if neq_idx != VEC_SIZE!() {
        return mismatch_4x_vec(cmpeq_1, cmpeq_2, cmpeq_3, cmpeq_4, neq_idx).map(|x| x + size - 4 * VEC_SIZE!());
    }

    return None;

    #[inline(always)]
    unsafe fn less_than_vec(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
        if size <= 1 {
            return one_or_less(a, b, size);
        }
        let a_raw_addr = a.as_ptr() as usize;
        let b_raw_addr = b.as_ptr() as usize;
        let addr_mask = a_raw_addr | b_raw_addr;
        let magic = (PAGE!() - 1) & addr_mask;
        if magic > (PAGE!() - VEC_SIZE!()) {
            return page_cross_less_than_vec(a, b, size);
        }

        let a_vec = _mm256_loadu_si256(a.as_ptr().cast());
        let b_vec = _mm256_loadu_si256(b.as_ptr().cast());
        let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
        let mask = _mm256_movemask_epi8(cmpeq) as u32;
        let neq_idx = mask.trailing_ones() as usize;
        if neq_idx < size {
            Some(neq_idx)
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn page_cross_less_than_vec(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
        if size >= 16 {
            return between_16_31(a, b, size);
        } else if size >= 8 {
            return between_8_15(a, b, size);
        } else if size < 4 {
            return between_2_3(a, b, size);
        }

        // Size is in [4..=7].
        let a_code_1 = a.as_ptr().cast::<u32>().read_unaligned();
        let a_code_2 = a.as_ptr().add(size - 4).cast::<u32>().read_unaligned();
        let b_code_1 = b.as_ptr().cast::<u32>().read_unaligned();
        let b_code_2 = b.as_ptr().add(size - 4).cast::<u32>().read_unaligned();
        let a_code = (u64::from(a_code_2) << (8 * (size - 4))) | u64::from(a_code_1);
        let b_code = (u64::from(b_code_2) << (8 * (size - 4))) | u64::from(b_code_1);
        let mask = a_code ^ b_code;
        let neq_idx = mask.trailing_zeros();
        if neq_idx < 64 {
            return Some(neq_idx as usize / 8);
        } else {
            return None;
        }

        #[inline(always)]
        unsafe fn between_16_31(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
            let a_vec = _mm_loadu_si128(a.as_ptr().cast());
            let b_vec = _mm_loadu_si128(b.as_ptr().cast());
            let cmpeq = _mm_cmpeq_epi8(a_vec, b_vec);
            let mask = _mm_movemask_epi8(cmpeq) as u16;
            let neq_idx = mask.trailing_ones();
            if neq_idx != 16 {
                return Some(neq_idx as usize);
            }
            let a_vec = _mm_loadu_si128(a.as_ptr().add(size).cast::<__m128i>().offset(-1));
            let b_vec = _mm_loadu_si128(b.as_ptr().add(size).cast::<__m128i>().offset(-1));
            let cmpeq = _mm_cmpeq_epi8(a_vec, b_vec);
            let mask = _mm_movemask_epi8(cmpeq) as u16;
            let neq_idx = mask.trailing_ones();
            if neq_idx != 16 {
                return Some(neq_idx as usize + size - 16);
            } else {
                return None;
            }
        }

        #[inline(always)]
        unsafe fn between_8_15(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
            let a_code_1 = a.as_ptr().cast::<u64>().read_unaligned();
            let a_code_2 = a.as_ptr().add(size - 8).cast::<u64>().read_unaligned();
            let b_code_1 = b.as_ptr().cast::<u64>().read_unaligned();
            let b_code_2 = b.as_ptr().add(size - 8).cast::<u64>().read_unaligned();
            let mask = a_code_1 ^ b_code_1;
            let neq_idx = mask.trailing_zeros();
            if neq_idx < 64 {
                return Some(neq_idx as usize / 8);
            }
            let mask = a_code_2 ^ b_code_2;
            let neq_idx = mask.trailing_zeros();
            if neq_idx < 64 {
                return Some(neq_idx as usize / 8 + size - 8);
            } else {
                return None;
            }
        }

        #[inline(always)]
        unsafe fn between_2_3(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
            let a_code_1 = a.as_ptr().cast::<u16>().read_unaligned();
            let a_code_2 = a.as_ptr().add(size - 1).read_unaligned();
            let b_code_1 = b.as_ptr().cast::<u16>().read_unaligned();
            let b_code_2 = b.as_ptr().add(size - 1).read_unaligned();
            let a_code = (u32::from(a_code_2) << (8 * (size - 1))) | u32::from(a_code_1);
            let b_code = (u32::from(b_code_2) << (8 * (size - 1))) | u32::from(b_code_1);
            let mask = a_code ^ b_code;
            let neq_idx = mask.trailing_zeros();
            if neq_idx < 32 {
                return Some(neq_idx as usize / 8);
            } else {
                return None;
            }
        }
    }

    #[inline(always)]
    unsafe fn one_or_less(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
        if size == 0 {
            None
        } else if a.get_unchecked(0) != b.get_unchecked(0) {
            Some(0)
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn last_1x_vec(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
        let a_ptr: *const __m256i = a.as_ptr().add(size).cast();
        let b_ptr: *const __m256i = b.as_ptr().add(size).cast();
        let a_vec = _mm256_loadu_si256(a_ptr.offset(-1));
        let b_vec = _mm256_loadu_si256(b_ptr.offset(-1));
        let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
        let mask = _mm256_movemask_epi8(cmpeq) as u32;
        let neq_idx = mask.trailing_ones() as usize;
        if neq_idx != VEC_SIZE!() {
            Some(size - VEC_SIZE!() + neq_idx)
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn last_2x_vec(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
        let a_ptr: *const __m256i = a.as_ptr().add(size).cast();
        let b_ptr: *const __m256i = b.as_ptr().add(size).cast();
        let a_vec = _mm256_loadu_si256(a_ptr.offset(-2));
        let b_vec = _mm256_loadu_si256(b_ptr.offset(-2));
        let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
        let mask = _mm256_movemask_epi8(cmpeq) as u32;
        let neq_idx = mask.trailing_ones() as usize;
        if neq_idx != VEC_SIZE!() {
            return Some(size - 2 * VEC_SIZE!() + neq_idx);
        }
        let a_vec = _mm256_loadu_si256(a_ptr.offset(-1));
        let b_vec = _mm256_loadu_si256(b_ptr.offset(-1));
        let cmpeq = _mm256_cmpeq_epi8(a_vec, b_vec);
        let mask = _mm256_movemask_epi8(cmpeq) as u32;
        let neq_idx = mask.trailing_ones() as usize;
        if neq_idx != VEC_SIZE!() {
            Some(size - VEC_SIZE!() + neq_idx)
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn more_8x_vec(a: &[u8], b: &[u8], size: usize) -> Option<usize> {
        let mut a_ptr: *const __m256i = a.as_ptr().cast::<__m256i>().add(4);
        let mut b_ptr = b.as_ptr().cast::<__m256i>().add(4);
        let mut idx = 4 * VEC_SIZE!();

        while idx + 4 * VEC_SIZE!() < size {
            let a_vec = _mm256_loadu_si256(a_ptr);
            let b_vec = _mm256_loadu_si256(b_ptr);
            let cmpeq_1 = _mm256_cmpeq_epi8(a_vec, b_vec);
            let a_vec = _mm256_loadu_si256(a_ptr.add(1));
            let b_vec = _mm256_loadu_si256(b_ptr.add(1));
            let cmpeq_2 = _mm256_cmpeq_epi8(a_vec, b_vec);
            let a_vec = _mm256_loadu_si256(a_ptr.add(2));
            let b_vec = _mm256_loadu_si256(b_ptr.add(2));
            let cmpeq_3 = _mm256_cmpeq_epi8(a_vec, b_vec);
            let a_vec = _mm256_loadu_si256(a_ptr.add(3));
            let b_vec = _mm256_loadu_si256(b_ptr.add(3));
            let cmpeq_4 = _mm256_cmpeq_epi8(a_vec, b_vec);
            let reduced_1 = _mm256_and_si256(cmpeq_4, cmpeq_3);
            let reduced_2 = _mm256_and_si256(cmpeq_2, cmpeq_1);
            let reduced = _mm256_and_si256(reduced_1, reduced_2);
            let mask = _mm256_movemask_epi8(reduced) as u32;
            let neq_idx = mask.trailing_ones();
            if neq_idx != VEC_SIZE!() {
                return mismatch_4x_vec(cmpeq_1, cmpeq_2, cmpeq_3, cmpeq_4, neq_idx).map(|x| x + idx);
            }

            a_ptr = a_ptr.add(4);
            b_ptr = b_ptr.add(4);
            idx += 4 * VEC_SIZE!();
        }

        let rem_size = size - idx;
        let a_end: *const __m256i = a.as_ptr().add(size).cast();
        let b_end: *const __m256i = b.as_ptr().add(size).cast();
        let a_vec = _mm256_loadu_si256(a_end.offset(-1));
        let b_vec = _mm256_loadu_si256(b_end.offset(-1));
        let cmpeq_4 = _mm256_cmpeq_epi8(a_vec, b_vec);

        if rem_size <= VEC_SIZE!() {
            let mask = _mm256_movemask_epi8(cmpeq_4) as u32;
            let neq_idx = mask.trailing_ones() as usize;
            if neq_idx != VEC_SIZE!() {
                return Some(size - VEC_SIZE!() + neq_idx);
            } else {
                return None;
            }
        }

        let a_vec = _mm256_loadu_si256(a_end.offset(-2));
        let b_vec = _mm256_loadu_si256(b_end.offset(-2));
        let cmpeq_3 = _mm256_cmpeq_epi8(a_vec, b_vec);

        if rem_size <= 2 * VEC_SIZE!() {
            let mask = _mm256_movemask_epi8(cmpeq_3) as u32;
            let neq_idx = mask.trailing_ones() as usize;
            if neq_idx != VEC_SIZE!() {
                return Some(size - VEC_SIZE!() * 2 + neq_idx);
            }
            let mask = _mm256_movemask_epi8(cmpeq_4) as u32;
            let neq_idx = mask.trailing_ones() as usize;
            if neq_idx != VEC_SIZE!() {
                return Some(size - VEC_SIZE!() + neq_idx);
            } else {
                return None;
            }
        }

        let a_vec = _mm256_loadu_si256(a_end.offset(-3));
        let b_vec = _mm256_loadu_si256(b_end.offset(-3));
        let cmpeq_2 = _mm256_cmpeq_epi8(a_vec, b_vec);
        let a_vec = _mm256_loadu_si256(a_end.offset(-4));
        let b_vec = _mm256_loadu_si256(b_end.offset(-4));
        let cmpeq_1 = _mm256_cmpeq_epi8(a_vec, b_vec);

        let reduced_1 = _mm256_and_si256(cmpeq_4, cmpeq_3);
        let reduced_2 = _mm256_and_si256(cmpeq_2, cmpeq_1);
        let reduced = _mm256_and_si256(reduced_1, reduced_2);
        let mask = _mm256_movemask_epi8(reduced) as u32;
        let neq_idx = mask.trailing_ones();
        if neq_idx != VEC_SIZE!() {
            mismatch_4x_vec(cmpeq_1, cmpeq_2, cmpeq_3, cmpeq_4, neq_idx).map(|x| x + size - 4 * VEC_SIZE!())
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn mismatch_4x_vec(
        cmpeq_1: __m256i,
        cmpeq_2: __m256i,
        cmpeq_3: __m256i,
        _cmpeq_4: __m256i,
        neq_idx: u32,
    ) -> Option<usize> {
        let mask_check = 1_u32 << neq_idx;
        let mask = _mm256_movemask_epi8(cmpeq_1) as u32;
        if (mask & mask_check) == 0 {
            return Some(neq_idx as usize);
        }
        let mask = _mm256_movemask_epi8(cmpeq_2) as u32;
        if (mask & mask_check) == 0 {
            return Some(neq_idx as usize + VEC_SIZE!());
        }
        let mask = _mm256_movemask_epi8(cmpeq_3) as u32;
        if (mask & mask_check) == 0 {
            return Some(neq_idx as usize + 2 * VEC_SIZE!());
        }
        Some(neq_idx as usize + 3 * VEC_SIZE!())
    }
}

#[cfg(test)]
mod tests {
    use super::cmpeq_forward;
    use test_case::test_case;

    #[test]
    fn test_cmpeq_forward() {
        for size in 0..=1024 {
            test_all_indices(size);
        }

        for big_size in [2048, 4096, 8192] {
            test_all_indices(big_size);
        }
    }

    fn test_all_indices(size: usize) {
        let a = vec![b'a'; size];
        let mut b = vec![b'a'; size];

        // First test the same slices.
        let res = unsafe { cmpeq_forward(&a, &b, size) };
        assert_eq!(res, None, "with size {size}");

        // Now difference at every point.
        for i in 0..size {
            b[i] = b'b';
            let res = unsafe { cmpeq_forward(&a, &b, size) };
            assert_eq!(res, Some(i), "with size {size} and idx {i}");
            b[i] = b'a';
        }
    }

    #[test]
    fn test_cmpeq_forward_page_cross() {
        // To definitely trigger the page-cross code handling we need to check
        // all slices of a big enough buffer.
        let a_base = vec![b'a'; 8192];
        let mut b_base = vec![b'a'; 8192];

        // Only these sizes are handled by the page-cross code.
        for size in 2..=31 {
            for start_idx in 0..(a_base.len() - size) {
                let a = &a_base[start_idx..start_idx + size];
                let b = &mut b_base[start_idx..start_idx + size];

                // First test the same slices.
                let res = unsafe { cmpeq_forward(&a, &b, size) };
                assert_eq!(res, None, "with size {size}");

                // Now difference at every point.
                for i in 0..size {
                    b[i] = b'b';
                    let res = unsafe { cmpeq_forward(&a, &b, size) };
                    assert_eq!(res, Some(i), "with size {size} and idx {i}");
                    b[i] = b'a';
                }
            }
        }
    }
}
