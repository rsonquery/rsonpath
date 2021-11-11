//! JSON depth calculations on byte streams.
//!
//! The recommended implementation of [`DepthBlock`] is [`simd::LazyAvx2Vector`],
//! which is optimised for the usual case where the depth does not change too sharply
//! within a single 32-byte block.

#[allow(dead_code)]
const BYTES_IN_AVX2_REGISTER: usize = 256 / 8;

/// Common trait for structs that enrich a byte block with JSON depth information.
#[allow(clippy::len_without_is_empty)]
pub trait DepthBlock<'a>: Sized {
    /// Decorate a byte block with depth information,
    /// returning an instance and the remaining portion of the
    /// byte slice that did not get decorated.
    fn new(bytes: &'a [u8]) -> (Self, &'a [u8]);

    /// Return the length of the decorated block.
    ///
    /// This should be constant throughout the lifetime of a `DepthBlock`
    /// and always satisfy:
    /// ```rust
    /// # use simdpath::bytes::nosimd::depth::Vector as Impl;
    /// # use simdpath::bytes::DepthBlock;
    /// # let bytes = &[0; 256];
    /// let (depth_block, rem) = Impl::new(bytes);
    /// let expected_len = bytes.len() - rem.len();
    ///
    /// assert_eq!(expected_len, depth_block.len());
    /// ```
    fn len(&self) -> usize;

    /// Advance to the next position in the decorated slice.
    /// Returns `true` if the position changed, `false` if
    /// the end of the decorated slice was reached.
    fn advance(&mut self) -> bool;

    /// Check whether the depth at current position of the slice is
    /// greater than or equal to `depth`.
    ///
    /// Implementing structs should start at the first position in the
    /// decorated slice. To change the position, call
    /// [`advance`](`DepthBlock::advance`) or [`advance_by`](`DepthBlock::advance_by`).
    fn is_depth_greater_or_equal_to(&mut self, depth: isize) -> bool;

    /// Returns exact depth at the end of the decorated slice,
    /// consuming the block.
    fn depth_at_end(self) -> isize;

    /// Advance by `i` positions in the decorated slice.
    /// Returns the number of positions by which the block advanced.
    /// If it is less than `i` then the end of the decorated slice was reached.
    fn advance_by(&mut self, i: usize) -> usize {
        let mut j = 0;
        while j < i {
            if !self.advance() {
                break;
            }
            j += 1;
        }
        j
    }
}

/// Sequential depth computation.
pub mod nosimd {
    use super::*;

    /// Decorates a byte slice with JSON depth information.
    ///
    /// This struct works on the entire slice and calculates the depth sequentially.
    pub struct Vector<'a> {
        bytes: &'a [u8],
        depth: isize,
        idx: usize,
    }

    impl<'a> DepthBlock<'a> for Vector<'a> {
        /// The remainder is guaranteed to be an empty slice,
        /// since this implementation works on the entire byte
        /// slice at once.
        #[inline]
        fn new(bytes: &'a [u8]) -> (Self, &'a [u8]) {
            let mut vector = Self {
                bytes,
                depth: 0,
                idx: 0,
            };
            vector.advance();
            (vector, &[])
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

/// Depth computation powered by SIMD.
#[cfg(not(feature = "nosimd"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "nosimd"))))]
pub mod simd {
    #[cfg(any(
        all(
            target_feature = "avx2",
            any(target_arch = "x86", target_arch = "x86_64")
        ),
        doc
    ))]
    #[doc(inline)]
    pub use super::avx2_simd::*;
}

#[cfg(any(
    all(
        not(feature = "nosimd"),
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2"
    ),
    doc
))]
mod avx2_simd {
    use super::*;
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    /// Works on a 32-byte slice, calculating the depth vectorially
    /// at construction.
    #[cfg_attr(
        docsrs,
        doc(cfg(all(
            target_feature = "avx2",
            any(target_arch = "x86", target_arch = "x86_64")
        )))
    )]
    pub struct Avx2Vector {
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
    pub struct LazyAvx2Vector {
        depth_bytes: Option<[i8; BYTES_IN_AVX2_REGISTER]>,
        opening_vector: Option<__m256i>,
        closing_vector: Option<__m256i>,
        opening_count: i8,
        closing_count: i8,
        len: usize,
        idx: usize,
    }

    impl LazyAvx2Vector {
        fn new_sequential(bytes: &[u8]) -> (Self, &[u8]) {
            let mut sum: i8 = 0;
            let mut vector = Self {
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
            (vector, &[])
        }

        #[target_feature(enable = "avx2")]
        #[inline]
        unsafe fn new_avx2(bytes: &[u8]) -> (Self, &[u8]) {
            let (opening_vector, closing_vector) = get_opening_and_closing_vectors(bytes);

            let opening_count = _mm256_movemask_epi8(opening_vector).count_ones() as i8;
            let closing_count = _mm256_movemask_epi8(closing_vector).count_ones() as i8;

            let vector = Self {
                depth_bytes: None,
                opening_vector: Some(opening_vector),
                closing_vector: Some(closing_vector),
                opening_count,
                closing_count,
                len: BYTES_IN_AVX2_REGISTER,
                idx: 0,
            };

            (vector, &bytes[BYTES_IN_AVX2_REGISTER..])
        }

        #[target_feature(enable = "avx2")]
        #[inline]
        unsafe fn depth_bytes(&mut self) -> &[i8; BYTES_IN_AVX2_REGISTER] {
            if let Some(ref depth_bytes) = self.depth_bytes {
                return depth_bytes;
            }

            let depth_bytes = calculate_depth_from_vectors(
                self.opening_vector.unwrap(),
                self.closing_vector.unwrap(),
            );
            self.depth_bytes = Some(depth_bytes);

            self.depth_bytes.as_ref().unwrap()
        }
    }

    impl<'a> DepthBlock<'a> for LazyAvx2Vector {
        #[inline]
        fn new(bytes: &'a [u8]) -> (Self, &'a [u8]) {
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
        fn advance_by(&mut self, i: usize) -> usize {
            let j = std::cmp::min(i, self.len() - self.idx + 1);
            self.idx += j;
            j
        }

        #[inline]
        fn is_depth_greater_or_equal_to(&mut self, depth: isize) -> bool {
            if depth <= -(self.closing_count as isize) {
                return true;
            }
            if depth > (self.opening_count as isize) {
                return false;
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

    impl Avx2Vector {
        #[inline]
        fn new_sequential(bytes: &[u8]) -> (Self, &[u8]) {
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

            (vector, &[])
        }

        #[inline]
        #[target_feature(enable = "avx2")]
        unsafe fn new_avx2(bytes: &[u8]) -> (Self, &[u8]) {
            let (opening_vector, closing_vector) = get_opening_and_closing_vectors(bytes);
            let depth_bytes = calculate_depth_from_vectors(opening_vector, closing_vector);

            let vector = Self {
                depth_bytes,
                len: BYTES_IN_AVX2_REGISTER,
                idx: 0,
            };
            (vector, &bytes[BYTES_IN_AVX2_REGISTER..])
        }
    }

    impl<'a> DepthBlock<'a> for Avx2Vector {
        #[inline]
        fn new(bytes: &'a [u8]) -> (Self, &[u8]) {
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
        fn advance_by(&mut self, i: usize) -> usize {
            let j = std::cmp::min(i, self.len() - self.idx + 1);
            self.idx += j;
            j
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

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn get_opening_and_closing_vectors(bytes: &[u8]) -> (__m256i, __m256i) {
        debug_assert!(bytes.len() >= BYTES_IN_AVX2_REGISTER);

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
        (opening_cmp, closing_cmp)
    }

    #[inline]
    #[target_feature(enable = "avx2")]
    unsafe fn calculate_depth_from_vectors(
        opening_vector: __m256i,
        closing_vector: __m256i,
    ) -> [i8; BYTES_IN_AVX2_REGISTER] {
        let array = [0; BYTES_IN_AVX2_REGISTER];

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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_depth_greater_or_equal_to_correctness<
        'a,
        F: Fn(&'a [u8]) -> (D, &[u8]),
        D: DepthBlock<'a>,
    >(
        build: &F,
        bytes: &'a [u8],
        depths: &[isize],
    ) {
        assert_eq!(bytes.len(), depths.len(), "Invalid test data.");
        let mut bytes = bytes;
        let mut depths_idx = 0;
        let mut accumulated_depth = 0;

        while !bytes.is_empty() {
            let (mut vector, rem) = build(bytes);
            bytes = rem;

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
        F: Fn(&'a [u8]) -> (D, &'a [u8]),
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
    fn is_depth_greater_or_equal_to_correctness_for_nosimd_vector() {
        is_depth_greater_or_equal_to_correctness_suite(nosimd::Vector::new);
    }

    #[test]
    #[cfg(all(not(feature = "nosimd"), target_feature = "avx2"))]
    fn is_depth_greater_or_equal_to_correctness_for_simd_vector() {
        is_depth_greater_or_equal_to_correctness_suite(simd::Avx2Vector::new);
    }

    #[test]
    #[cfg(all(not(feature = "nosimd"), target_feature = "avx2"))]
    fn is_depth_greater_or_equal_to_correctness_for_simd_lazy_vectpr() {
        is_depth_greater_or_equal_to_correctness_suite(simd::LazyAvx2Vector::new);
    }
}
