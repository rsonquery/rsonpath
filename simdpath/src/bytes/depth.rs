#[allow(dead_code)]
const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

#[cfg(any(feature = "nosimd", not(target_feature = "avx2")))]
pub fn decorate_depth(bytes: &[u8]) -> nosimd::Vector {
    nosimd::Vector::new(bytes)
}

#[cfg(all(not(feature = "nosimd"), target_feature = "avx2"))]
pub fn decorate_depth(bytes: &[u8]) -> simd::Vector {
    simd::Vector::new(bytes)
}

pub trait DepthBlock<'a> {
    fn new(bytes: &'a [u8]) -> Self;
    fn len(&self) -> usize;
    fn advance(&mut self) -> bool;
    fn is_depth_greater_or_equal_to(&mut self, depth: isize) -> bool;
    fn depth_at_end(self) -> isize;
}

pub mod nosimd {
    use super::*;

    pub struct Vector<'a> {
        bytes: &'a [u8],
        depth: isize,
        idx: usize,
    }

    impl<'a> DepthBlock<'a> for Vector<'a> {
        #[inline]
        fn new(bytes: &'a [u8]) -> Self {
            let mut vector = Self {
                bytes,
                depth: 0,
                idx: 0,
            };
            vector.advance();
            vector
        }

        #[inline]
        fn len(&self) -> usize {
            self.bytes.len()
        }

        #[inline]
        fn advance(&mut self) -> bool {
            if self.idx >= self.bytes.len() {
                return false;
            }
            self.depth += match self.bytes[self.idx] {
                b'{' => 1,
                b'[' => 1,
                b'}' => -1,
                b']' => -1,
                _ => 0,
            };
            self.idx += 1;

            true
        }

        #[inline]
        fn is_depth_greater_or_equal_to(&mut self, depth: isize) -> bool {
            self.depth >= depth
        }

        #[inline]
        fn depth_at_end(mut self) -> isize {
            while self.advance() {}
            self.depth
        }
    }
}

#[cfg(all(not(feature = "nosimd"), target_feature = "avx2"))]
pub mod simd {
    use super::*;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    pub struct Vector {
        depth_bytes: [i8; BYTES_IN_AVX2_REGISTER],
        len: usize,
        idx: usize,
    }

    pub struct LazyVector {
        depth_bytes: Option<[i8; BYTES_IN_AVX2_REGISTER]>,
        opening_vector: Option<__m256i>,
        closing_vector: Option<__m256i>,
        opening_count: i8,
        closing_count: i8,
        len: usize,
        idx: usize,
    }

    impl LazyVector {
        fn new_sequential(bytes: &[u8]) -> Self {
            let mut sum: i8 = 0;
            let mut vector = LazyVector {
                depth_bytes: None,
                opening_vector: None,
                closing_vector: None,
                opening_count: 0,
                closing_count: 0,
                len: bytes.len(),
                idx: 0,
            };
            let mut depth_bytes = [0; BYTES_IN_AVX2_REGISTER];

            for i in 0..bytes.len() {
                match bytes[i] {
                    b'{' => {
                        vector.opening_count += 1;
                        sum += 1;
                    }
                    b'[' => {
                        vector.opening_count += 1;
                        sum += 1
                    }
                    b'}' => {
                        vector.closing_count += 1;
                        sum -= 1
                    }
                    b']' => {
                        vector.closing_count += 1;
                        sum -= 1
                    }
                    _ => (),
                };
                depth_bytes[i] = sum;
            }

            vector.depth_bytes = Some(depth_bytes);
            vector
        }

        #[target_feature(enable = "avx2")]
        #[inline]
        unsafe fn new_avx2(bytes: &[u8]) -> Self {
            let byte_vector = _mm256_loadu_si256(bytes.as_ptr() as *const __m256i);
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

            let opening_count = _mm256_movemask_epi8(opening_cmp).count_ones() as i8;
            let closing_count = _mm256_movemask_epi8(closing_cmp).count_ones() as i8;

            LazyVector {
                depth_bytes: None,
                opening_vector: Some(opening_cmp),
                closing_vector: Some(closing_cmp),
                opening_count,
                closing_count,
                len: BYTES_IN_AVX2_REGISTER,
                idx: 0,
            }
        }

        #[target_feature(enable = "avx2")]
        #[inline]
        unsafe fn depth_bytes(&mut self) -> &[i8; BYTES_IN_AVX2_REGISTER] {
            if let Some(ref depth_bytes) = self.depth_bytes {
                return depth_bytes;
            }

            self.depth_bytes = Some([0; BYTES_IN_AVX2_REGISTER]);

            let vector1 =
                _mm256_sub_epi8(self.closing_vector.unwrap(), self.opening_vector.unwrap());
            let vector2 = _mm256_add_epi8(vector1, _mm256_slli_si256::<1>(vector1));
            let vector4 = _mm256_add_epi8(vector2, _mm256_slli_si256::<2>(vector2));
            let vector8 = _mm256_add_epi8(vector4, _mm256_slli_si256::<4>(vector4));
            let vector16 = _mm256_add_epi8(vector8, _mm256_slli_si256::<8>(vector8));

            let halfway = _mm256_extract_epi8::<15>(vector16);
            let halfway_vector = _mm256_set1_epi8(halfway as i8);
            let halfway_vector_second_lane_only =
                _mm256_permute2x128_si256::<8>(halfway_vector, halfway_vector);

            let vector32 = _mm256_add_epi8(vector16, halfway_vector_second_lane_only);

            let array_ptr = self.depth_bytes.as_ref().unwrap().as_ptr() as *mut __m256i;
            _mm256_storeu_si256(array_ptr, vector32);

            self.depth_bytes.as_ref().unwrap()
        }
    }

    impl<'a> DepthBlock<'a> for LazyVector {
        #[inline]
        fn new(bytes: &'a [u8]) -> Self {
            if bytes.len() >= BYTES_IN_AVX2_REGISTER {
                unsafe { Self::new_avx2(bytes) }
            } else {
                Self::new_sequential(bytes)
            }
        }

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
        fn is_depth_greater_or_equal_to(&mut self, depth: isize) -> bool {
            if depth <= -(self.closing_count as isize) {
                return true;
            }

            let idx = self.idx;
            let actual_depth = unsafe { self.depth_bytes()[idx] as isize };
            actual_depth >= depth
        }

        #[inline(always)]
        fn depth_at_end(self) -> isize {
            (self.opening_count - self.closing_count) as isize
        }
    }

    impl Vector {
        #[inline]
        fn new_sequential(bytes: &[u8]) -> Self {
            let mut sum: i8 = 0;
            let mut vector = Vector {
                depth_bytes: [0; BYTES_IN_AVX2_REGISTER],
                len: bytes.len(),
                idx: 0,
            };

            for i in 0..vector.len {
                sum += match bytes[i] {
                    b'{' => 1,
                    b'[' => 1,
                    b'}' => -1,
                    b']' => -1,
                    _ => 0,
                };
                vector.depth_bytes[i] = sum;
            }

            vector
        }

        #[target_feature(enable = "avx2")]
        #[inline]
        unsafe fn new_avx2(bytes: &[u8]) -> Self {
            let byte_vector = _mm256_loadu_si256(bytes.as_ptr() as *const __m256i);
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
            let vector1 = _mm256_sub_epi8(closing_cmp, opening_cmp);
            let vector2 = _mm256_add_epi8(vector1, _mm256_slli_si256::<1>(vector1));
            let vector4 = _mm256_add_epi8(vector2, _mm256_slli_si256::<2>(vector2));
            let vector8 = _mm256_add_epi8(vector4, _mm256_slli_si256::<4>(vector4));
            let vector16 = _mm256_add_epi8(vector8, _mm256_slli_si256::<8>(vector8));

            let halfway = _mm256_extract_epi8::<15>(vector16);
            let halfway_vector = _mm256_set1_epi8(halfway as i8);
            let halfway_vector_second_lane_only =
                _mm256_permute2x128_si256::<8>(halfway_vector, halfway_vector);

            let vector32 = _mm256_add_epi8(vector16, halfway_vector_second_lane_only);

            let mut vector = Self {
                depth_bytes: [0; BYTES_IN_AVX2_REGISTER],
                len: BYTES_IN_AVX2_REGISTER,
                idx: 0,
            };
            let array_ptr = vector.depth_bytes.as_ptr() as *mut __m256i;
            _mm256_storeu_si256(array_ptr, vector32);
            vector
        }
    }

    impl<'a> DepthBlock<'a> for Vector {
        #[inline]
        fn new(bytes: &'a [u8]) -> Self {
            if bytes.len() >= BYTES_IN_AVX2_REGISTER {
                unsafe { Self::new_avx2(bytes) }
            } else {
                Self::new_sequential(bytes)
            }
        }

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

        #[inline(always)]
        fn is_depth_greater_or_equal_to(&mut self, depth: isize) -> bool {
            (self.depth_bytes[self.idx] as isize) >= depth
        }

        #[inline(always)]
        fn depth_at_end(self) -> isize {
            self.depth_bytes[self.len() - 1] as isize
        }
    }

    #[target_feature(enable = "avx2")]
    unsafe fn debug_mm256(vec: &__m256i) -> [i8; BYTES_IN_AVX2_REGISTER] {
        let array = [0; BYTES_IN_AVX2_REGISTER];
        let array_ptr = array.as_ptr() as *mut __m256i;
        _mm256_storeu_si256(array_ptr, *vec);
        array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_depth_greater_or_equal_to_correctness<'a, F: Fn(&'a [u8]) -> D, D: DepthBlock<'a>>(
        build: &F,
        bytes: &'a [u8],
        depths: &[isize],
    ) {
        assert_eq!(bytes.len(), depths.len(), "Invalid test data.");
        let mut bytes = bytes;
        let mut depths_idx = 0;
        let mut accumulated_depth = 0;

        while !bytes.is_empty() {
            let mut vector = build(bytes);
            bytes = &bytes[vector.len()..];

            loop {
                let depth = depths[depths_idx];
                let adjusted_depth = depth - accumulated_depth;
                assert!(
                    vector.is_depth_greater_or_equal_to(adjusted_depth),
                    "Failed for exact depth: '{}' at index '{}'",
                    adjusted_depth,
                    depths_idx
                );
                assert!(
                    vector.is_depth_greater_or_equal_to(adjusted_depth - 1),
                    "Failed for depth one below: '{}' at index '{}'",
                    adjusted_depth,
                    depths_idx
                );
                assert!(
                    !vector.is_depth_greater_or_equal_to(adjusted_depth + 1),
                    "Failed for depth one above: '{}' at index '{}'",
                    adjusted_depth,
                    depths_idx
                );
                depths_idx += 1;
                if !vector.advance() {
                    break;
                }
            }
            accumulated_depth += vector.depth_at_end();
        }

        assert_eq!(depths.len(), depths_idx);
    }

    fn is_depth_greater_or_equal_to_correctness_suite<
        'a,
        F: Fn(&'a [u8]) -> D,
        D: DepthBlock<'a>,
    >(
        build: F,
    ) {
        let json = r#"{"aaa":[{},{"b":{"c":[1,2,3]}}]}"#;
        let depths = [
            1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
            2, 1, 0,
        ];

        is_depth_greater_or_equal_to_correctness(&build, json.as_bytes(), &depths);

        let json = r#"{}"#;
        let depths = [1, 0];

        is_depth_greater_or_equal_to_correctness(&build, json.as_bytes(), &depths);

        let json = r#""#;
        let depths = [];

        is_depth_greater_or_equal_to_correctness(&build, json.as_bytes(), &depths);

        let json = r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"e":{"a":[[],[1,2,3],[{"b":[{}]}]]},"d":42}"#;
        let depths = [
            1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
            2, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 4, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 4, 5, 5, 5, 5,
            5, 6, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0,
        ];

        is_depth_greater_or_equal_to_correctness(&build, json.as_bytes(), &depths);
    }

    #[test]
    fn is_depth_greater_or_equal_to_correctness_for_decorate_depth() {
        is_depth_greater_or_equal_to_correctness_suite(decorate_depth);
    }

    #[test]
    #[cfg(all(not(feature = "nosimd"), target_feature = "avx2"))]
    fn is_depth_greater_or_equal_to_correctness_for_decorate_depth_lazy() {
        is_depth_greater_or_equal_to_correctness_suite(simd::LazyVector::new);
    }
}
