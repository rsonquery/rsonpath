#[cfg(all(target_arch = "x86", target_feature = "avx2"))]
use core::arch::x86::*;
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
use core::arch::x86_64::*;

#[allow(dead_code)]
const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

#[cfg(all(not(feature = "nosimd"), target_feature = "avx2"))]
#[inline]
pub fn decorate_depth(bytes: &[u8]) -> BytesWithDepth<simd::Vector> {
    simd::decorate_depth(bytes)
}

#[cfg(any(feature = "nosimd", not(target_feature = "avx2")))]
#[inline]
pub fn decorate_depth(bytes: &[u8]) -> BytesWithDepth<nosimd::Vector> {
    nosimd::decorate_depth(bytes)
}

pub struct BytesWithDepth<'a, D: DepthBlock> {
    bytes: &'a [u8],
    accumulated_depth: isize,
    current_vector: D,
}

impl<'a, D: DepthBlock> BytesWithDepth<'a, D> {
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        let vector = D::new(bytes);
        Self {
            bytes,
            accumulated_depth: 0,
            current_vector: vector,
        }
    }

    #[inline]
    pub fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        let adjusted_depth = depth - self.accumulated_depth; // TODO signs
        self.current_vector
            .is_depth_greater_or_equal_to(adjusted_depth)
    }

    pub fn advance(&mut self) -> bool {
        if self.current_vector.advance() {
            return true;
        }

        if self.bytes.is_empty() {
            return false;
        }

        self.bytes = &self.bytes[self.current_vector.len()..];

        if self.bytes.is_empty() {
            return false;
        }

        self.accumulated_depth += self.current_vector.depth_at_end();
        self.current_vector = D::new(self.bytes);

        true
    }
}

pub trait DepthBlock {
    fn new(bytes: &[u8]) -> Self;
    fn len(&self) -> usize;
    fn advance(&mut self) -> bool;
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool;
    fn depth_at_end(&self) -> isize;
}

pub mod nosimd {
    use super::*;

    #[inline]
    pub fn decorate_depth(bytes: &'_ [u8]) -> BytesWithDepth<'_, nosimd::Vector> {
        BytesWithDepth::<nosimd::Vector>::new(bytes)
    }

    pub struct Vector {
        byte: u8,
    }

    impl DepthBlock for Vector {
        #[inline]
        fn new(bytes: &[u8]) -> Self {
            Self { byte: bytes[0] }
        }

        #[inline]
        fn len(&self) -> usize {
            1
        }

        #[inline]
        fn advance(&mut self) -> bool {
            false
        }

        #[inline]
        fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
            self.depth_at_end() >= depth
        }

        #[inline]
        fn depth_at_end(&self) -> isize {
            match self.byte {
                b'{' => 1,
                b'[' => 1,
                b'}' => -1,
                b']' => -1,
                _ => 0,
            }
        }
    }
}

#[cfg(all(not(feature = "nosimd"), target_feature = "avx2"))]
pub mod simd {
    use super::*;

    #[inline]
    pub fn decorate_depth(bytes: &'_ [u8]) -> BytesWithDepth<'_, simd::Vector> {
        BytesWithDepth::<simd::Vector>::new(bytes)
    }

    pub struct Vector {
        depth_bytes: [i8; BYTES_IN_AVX2_REGISTER],
        len: usize,
        idx: usize,
    }

    impl Vector {
        #[target_feature(enable = "avx2")]
        unsafe fn debug_mm256(vec: &__m256i) -> [i8; BYTES_IN_AVX2_REGISTER] {
            let array = [0; BYTES_IN_AVX2_REGISTER];
            let array_ptr = array.as_ptr() as *mut __m256i;
            _mm256_storeu_si256(array_ptr, *vec);
            array
        }

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
            let vector2 = _mm256_add_epi8(
                vector1,
                _mm256_alignr_epi8::<15>(vector1, _mm256_permute2x128_si256::<8>(vector1, vector1)),
            );
            let vector4 = _mm256_add_epi8(
                vector2,
                _mm256_alignr_epi8::<14>(vector2, _mm256_permute2x128_si256::<8>(vector2, vector2)),
            );
            let vector8 = _mm256_add_epi8(
                vector4,
                _mm256_alignr_epi8::<12>(vector4, _mm256_permute2x128_si256::<8>(vector4, vector4)),
            );
            let vector16 = _mm256_add_epi8(
                vector8,
                _mm256_alignr_epi8::<8>(vector8, _mm256_permute2x128_si256::<8>(vector8, vector8)),
            );
            let vector32 =
                _mm256_add_epi8(vector16, _mm256_permute2x128_si256::<8>(vector16, vector16));
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

    impl DepthBlock for Vector {
        #[inline]
        fn new(bytes: &[u8]) -> Self {
            if bytes.len() >= BYTES_IN_AVX2_REGISTER {
                unsafe { Self::new_avx2(bytes) }
            } else {
                Self::new_sequential(bytes)
            }
        }

        #[inline]
        fn len(&self) -> usize {
            self.len
        }

        #[inline]
        fn advance(&mut self) -> bool {
            if self.idx == BYTES_IN_AVX2_REGISTER - 1 {
                return false;
            }
            self.idx += 1;
            true
        }

        #[inline]
        fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
            (self.depth_bytes[self.idx] as isize) >= depth
        }

        #[inline]
        fn depth_at_end(&self) -> isize {
            self.depth_bytes[BYTES_IN_AVX2_REGISTER - 1] as isize
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_depth_greater_or_equal_to_correctness() {
        let json = r#"{"aaa":[{},{"b":{"c":[1,2,3]}}]}"#;
        let depths = [
            1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
            2, 1, 0,
        ];
        let bytes = json.as_bytes();
        let mut bytes_with_depth = decorate_depth(bytes);

        assert_eq!(32, json.len());
        assert_eq!(json.len(), depths.len());

        for (i, &depth) in depths.iter().enumerate() {
            assert!(bytes_with_depth.is_depth_greater_or_equal_to(depth));
            if depth > 0 {
                assert!(bytes_with_depth.is_depth_greater_or_equal_to(depth - 1));
            }
            assert!(!bytes_with_depth.is_depth_greater_or_equal_to(depth + 1));

            let expected_flag = i < bytes.len() - 1;
            let advance_flag = bytes_with_depth.advance();
            assert_eq!(expected_flag, advance_flag);
        }

        assert!(!bytes_with_depth.advance());
    }
}
