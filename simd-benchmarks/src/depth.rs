//! JSON depth calculations on byte streams.
//!
//! There is only one sequential implementation, [`nosimd::Vector`]. Other implementations are SIMD based.
//!
//! The recommended implementation of [`DepthBlock`] is [`avx2::LazyVector`]
//! which is optimized for the usual case where the depth does not change too sharply.
//! within a single 32-byte block.

/// Common trait for structs that enrich a byte block with JSON depth information.
#[allow(clippy::len_without_is_empty)]
pub trait DepthBlock<'a>: Sized {
    /// Return the length of the decorated block.
    ///
    /// This should be constant throughout the lifetime of a `DepthBlock`
    /// and always satisfy:
    /// ```rust
    /// # use simd_benchmarks::depth::{DepthBlock, avx2};
    /// # use aligners::{AlignedBytes, alignment::TwoTo};
    /// # let bytes: AlignedBytes<TwoTo<5>> = [0; 256].into();
    /// let (depth_block, rem) = avx2::LazyVector::new(&bytes);
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
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool;

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

pub mod avx2;
#[cfg(feature = "avx512")]
pub mod avx512;
pub mod nosimd;
pub mod sse2;

#[cfg(test)]
mod tests {
    use super::*;
    use aligners::{
        alignment::{Alignment, One, TwoTo},
        AlignedBytes, AlignedSlice,
    };

    use test_case::test_case;

    fn is_depth_greater_or_equal_to_correctness<A: Alignment, C: for<'a> Ctor<'a, A>>(
        bytes: &AlignedSlice<A>,
        depths: &[isize],
        _ctor: &C,
    ) {
        assert_eq!(bytes.len(), depths.len(), "Invalid test data.");
        let mut bytes = bytes;
        let mut depths_idx = 0;
        let mut accumulated_depth = 0;

        while !bytes.is_empty() {
            let mut vector = C::ctor(&mut bytes);

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

    trait Ctor<'a, A: Alignment> {
        type Item: DepthBlock<'a>;

        fn ctor(slice: &mut &'a AlignedSlice<A>) -> Self::Item;
    }

    struct NosimdVectorCtor();
    struct Sse2VectorCtor();
    struct LazySse2VectorCtor();
    struct Avx2VectorCtor();
    struct LazyAvx2VectorCtor();
    #[cfg(feature = "avx512")]
    struct LazyAvx512VectorCtor();

    impl<'a> Ctor<'a, One> for NosimdVectorCtor {
        type Item = nosimd::Vector<'a>;

        fn ctor(slice: &mut &'a AlignedSlice<One>) -> nosimd::Vector<'a> {
            let vector = nosimd::Vector::new(slice);
            *slice = Default::default();
            vector
        }
    }

    impl<'a> Ctor<'a, TwoTo<4>> for Sse2VectorCtor {
        type Item = sse2::Vector;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<4>>) -> sse2::Vector {
            let (vector, rem) = sse2::Vector::new(slice);
            *slice = rem;
            vector
        }
    }

    impl<'a> Ctor<'a, TwoTo<4>> for LazySse2VectorCtor {
        type Item = sse2::LazyVector;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<4>>) -> sse2::LazyVector {
            let (vector, rem) = sse2::LazyVector::new(slice);
            *slice = rem;
            vector
        }
    }

    impl<'a> Ctor<'a, TwoTo<5>> for Avx2VectorCtor {
        type Item = avx2::Vector;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<5>>) -> avx2::Vector {
            let (vector, rem) = avx2::Vector::new(slice);
            *slice = rem;
            vector
        }
    }

    impl<'a> Ctor<'a, TwoTo<5>> for LazyAvx2VectorCtor {
        type Item = avx2::LazyVector;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<5>>) -> avx2::LazyVector {
            let (vector, rem) = avx2::LazyVector::new(slice);
            *slice = rem;
            vector
        }
    }

    #[cfg(feature = "avx512")]
    impl<'a> Ctor<'a, TwoTo<6>> for LazyAvx512VectorCtor {
        type Item = avx512::LazyVector;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<6>>) -> avx512::LazyVector {
            let (vector, rem) = avx512::LazyVector::new(slice);
            *slice = rem;
            vector
        }
    }

    #[test_case(NosimdVectorCtor(); "using nosimd::Vector")]
    #[test_case(Sse2VectorCtor(); "using sse2::Vector")]
    #[test_case(LazySse2VectorCtor(); "using sse2::LazyVector")]
    #[test_case(Avx2VectorCtor(); "using avx2:Vector")]
    #[test_case(LazyAvx2VectorCtor(); "using avx2::LazyVector")]
    #[cfg_attr(feature = "avx512", test_case(LazyAvx512VectorCtor(); "using avx512::LazyVector"))]
    fn is_depth_greater_or_equal_to_correctness_suite<A: Alignment, C: for<'a> Ctor<'a, A>>(
        ctor: C,
    ) {
        let json = r#"{"aaa":[{},{"b":{"c":[1,2,3]}}]}"#;
        let depths = [
            1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
            2, 1, 0,
        ];
        let bytes: AlignedBytes<TwoTo<6>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(bytes.relax_alignment(), &depths, &ctor);

        let json = r#"{}"#;
        let depths = [1, 0];
        let bytes: AlignedBytes<TwoTo<6>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(bytes.relax_alignment(), &depths, &ctor);

        let json = r#""#;
        let depths = [];
        let bytes: AlignedBytes<TwoTo<6>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(bytes.relax_alignment(), &depths, &ctor);

        let json = r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"e":{"a":[[],[1,2,3],[{"b":[{}]}]]},"d":42}"#;
        let depths = [
            1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
            2, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 4, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 4, 5, 5, 5, 5,
            5, 6, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0,
        ];
        let bytes: AlignedBytes<TwoTo<6>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(bytes.relax_alignment(), &depths, &ctor);
    }
}
