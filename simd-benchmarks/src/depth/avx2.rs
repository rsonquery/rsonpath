use aligners::{alignment::TwoTo, AlignedSlice};

use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

/// Works on a 32-byte slice, calculating the depth vectorially
/// at construction.
#[cfg_attr(
    docsrs,
    doc(cfg(all(
        target_feature = "avx2",
        any(target_arch = "x86", target_arch = "x86_64")
    )))
)]
pub struct Vector {
    depth_bytes: [i8; BYTES_IN_AVX2_REGISTER],
    len: usize,
    idx: usize,
}

/// Works on a 32-byte slice, but uses a heuristic to quickly
/// respond to queries and not count the depth exactly unless
/// needed.
///
/// The heuristic checks if it is possible to achieve the queried
/// depth within the block by counting the number of opening
/// and closing structural characters. This can be done much
/// more quickly than precise depth calculation.
#[cfg_attr(
    docsrs,
    doc(cfg(all(
        target_feature = "avx2",
        any(target_arch = "x86", target_arch = "x86_64")
    )))
)]
pub struct LazyVector {
    opening_mask: u32,
    closing_mask: u32,
    len: usize,
    rev_idx: usize,
}

impl LazyVector {
    fn new_sequential(bytes: &AlignedSlice<TwoTo<5>>) -> (Self, &AlignedSlice<TwoTo<5>>) {
        let mut vector = Self {
            opening_mask: 0,
            closing_mask: 0,
            len: bytes.len(),
            rev_idx: bytes.len() - 1,
        };
        for byte in bytes.iter() {
            vector.opening_mask >>= 1;
            vector.closing_mask >>= 1;
            match byte {
                b'{' => {
                    vector.opening_mask |= 1 << 31;
                }
                b'[' => {
                    vector.opening_mask |= 1 << 31;
                }
                b'}' => {
                    vector.closing_mask |= 1 << 31;
                }
                b']' => {
                    vector.closing_mask |= 1 << 31;
                }
                _ => (),
            };
        }

        (vector, Default::default())
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn new_avx2(bytes: &AlignedSlice<TwoTo<5>>) -> (Self, &AlignedSlice<TwoTo<5>>) {
        let (opening_vector, closing_vector) = get_opening_and_closing_vectors(bytes);

        let opening_mask = _mm256_movemask_epi8(opening_vector) as u32;
        let closing_mask = _mm256_movemask_epi8(closing_vector) as u32;

        let vector = Self {
            opening_mask,
            closing_mask,
            len: BYTES_IN_AVX2_REGISTER,
            rev_idx: BYTES_IN_AVX2_REGISTER - 1,
        };

        (vector, bytes.offset(1))
    }
}

impl LazyVector {
    #[inline]
    pub fn new(bytes: &AlignedSlice<TwoTo<5>>) -> (Self, &AlignedSlice<TwoTo<5>>) {
        if bytes.len() >= BYTES_IN_AVX2_REGISTER {
            unsafe { Self::new_avx2(bytes) }
        } else {
            Self::new_sequential(bytes)
        }
    }
}

impl<'a> DepthBlock<'a> for LazyVector {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.rev_idx == 0 {
            return false;
        }
        self.rev_idx -= 1;
        true
    }

    #[inline]
    fn advance_by(&mut self, i: usize) -> usize {
        let j = std::cmp::min(i, self.rev_idx);
        self.rev_idx -= j;
        j
    }

    #[inline]
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        let closing_count = self.closing_mask.count_ones() as isize;
        if depth <= -closing_count {
            return true;
        }

        let actual_depth = ((self.opening_mask << self.rev_idx).count_ones() as i32)
            - ((self.closing_mask << self.rev_idx).count_ones() as i32);
        actual_depth as isize >= depth
    }

    #[inline(always)]
    fn depth_at_end(self) -> isize {
        ((self.opening_mask.count_ones() as i32) - (self.closing_mask.count_ones() as i32)) as isize
    }
}

impl Vector {
    #[inline]
    fn new_sequential(bytes: &AlignedSlice<TwoTo<5>>) -> (Self, &AlignedSlice<TwoTo<5>>) {
        let mut sum: i8 = 0;
        let mut vector = Self {
            depth_bytes: [0; BYTES_IN_AVX2_REGISTER],
            len: bytes.len(),
            idx: 0,
        };

        for (i, byte) in bytes.iter().enumerate() {
            sum += match byte {
                b'{' => 1,
                b'[' => 1,
                b'}' => -1,
                b']' => -1,
                _ => 0,
            };
            vector.depth_bytes[i] = sum;
        }

        (vector, Default::default())
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn new_avx2(bytes: &AlignedSlice<TwoTo<5>>) -> (Self, &AlignedSlice<TwoTo<5>>) {
        let (opening_vector, closing_vector) = get_opening_and_closing_vectors(bytes);
        let depth_bytes = calculate_depth_from_vectors(opening_vector, closing_vector);

        let vector = Self {
            depth_bytes,
            len: BYTES_IN_AVX2_REGISTER,
            idx: 0,
        };
        (vector, bytes.offset(1))
    }
}

impl Vector {
    #[inline]
    pub fn new(bytes: &AlignedSlice<TwoTo<5>>) -> (Self, &AlignedSlice<TwoTo<5>>) {
        if bytes.len() >= BYTES_IN_AVX2_REGISTER {
            unsafe { Self::new_avx2(bytes) }
        } else {
            Self::new_sequential(bytes)
        }
    }
}

impl<'a> DepthBlock<'a> for Vector {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.idx + 1 >= self.len() {
            return false;
        }
        self.idx += 1;
        true
    }

    #[inline]
    fn advance_by(&mut self, i: usize) -> usize {
        let j = std::cmp::min(i, self.len() - self.idx + 1);
        self.idx += j;
        j
    }

    #[inline(always)]
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        (self.depth_bytes[self.idx] as isize) >= depth
    }

    #[inline(always)]
    fn depth_at_end(self) -> isize {
        self.depth_bytes[self.len() - 1] as isize
    }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn get_opening_and_closing_vectors(bytes: &AlignedSlice<TwoTo<5>>) -> (__m256i, __m256i) {
    debug_assert!(bytes.len() >= BYTES_IN_AVX2_REGISTER);

    let byte_vector = _mm256_load_si256(bytes.as_ptr() as *const __m256i);
    let opening_brace_mask = _mm256_set1_epi8(b'{' as i8);
    let opening_bracket_mask = _mm256_set1_epi8(b'[' as i8);
    let closing_brace_mask = _mm256_set1_epi8(b'}' as i8);
    let closing_bracket_mask = _mm256_set1_epi8(b']' as i8);
    let opening_brace_cmp = _mm256_cmpeq_epi8(byte_vector, opening_brace_mask);
    let opening_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, opening_bracket_mask);
    let closing_brace_cmp = _mm256_cmpeq_epi8(byte_vector, closing_brace_mask);
    let closing_bracket_cmp = _mm256_cmpeq_epi8(byte_vector, closing_bracket_mask);
    let opening_cmp = _mm256_or_si256(opening_brace_cmp, opening_bracket_cmp);
    let closing_cmp = _mm256_or_si256(closing_brace_cmp, closing_bracket_cmp);
    (opening_cmp, closing_cmp)
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn calculate_depth_from_vectors(
    opening_vector: __m256i,
    closing_vector: __m256i,
) -> [i8; BYTES_IN_AVX2_REGISTER] {
    /* Calculate depth as prefix sums of the closing and opening vectors.
        This is done by calculating prefix sums of length 2, 4, 8, 16
        and finally 32. This can be thought of as creating a binary tree over
        the vector.

        This is a bit more tricky with AVX2, because the vector is physically
        split into two 128-bit lanes, and they can only be bitwise
        shifted independently. This allows us to calculate two 16-byte long
        prefix sums, but to combine them we need to extract the total sum from
        the first lane and then add it to the entire second lane.
    */
    let array = [0; BYTES_IN_AVX2_REGISTER];
    let vector1 = _mm256_sub_epi8(closing_vector, opening_vector);
    let vector2 = _mm256_add_epi8(vector1, _mm256_slli_si256::<1>(vector1));
    let vector4 = _mm256_add_epi8(vector2, _mm256_slli_si256::<2>(vector2));
    let vector8 = _mm256_add_epi8(vector4, _mm256_slli_si256::<4>(vector4));
    let vector16 = _mm256_add_epi8(vector8, _mm256_slli_si256::<8>(vector8));

    let halfway = _mm256_extract_epi8::<15>(vector16);
    let halfway_vector = _mm256_set1_epi8(halfway as i8);
    let halfway_vector_second_lane_only =
        _mm256_permute2x128_si256::<8>(halfway_vector, halfway_vector);

    let vector32 = _mm256_add_epi8(vector16, halfway_vector_second_lane_only);
    let array_ptr = array.as_ptr() as *mut __m256i;
    _mm256_storeu_si256(array_ptr, vector32);

    array
}
