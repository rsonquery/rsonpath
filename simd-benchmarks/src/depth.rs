//! JSON depth calculations on byte streams.
//!
//! There is only one sequential implementation, [`nosimd::Vector`]. Other implementations are SIMD based.
//!
//! The recommended implementation of [`DepthBlock`] is [`avx2::LazyAvx2Vector`]
//! which is optimized for the usual case where the depth does not change too sharply.
//! within a single 32-byte block.

use aligners::{alignment::TwoTo, AlignedSlice};

/// Common trait for structs that enrich a byte block with JSON depth information.
#[allow(clippy::len_without_is_empty)]
pub trait DepthBlock<'a>: Sized {
    /// Return the length of the decorated block.
    ///
    /// This should be constant throughout the lifetime of a `DepthBlock`
    /// and always satisfy:
    /// ```rust
    /// # use simd_benchmarks::depth::{DepthBlock, avx2} ;
    /// # let bytes = &[0; 256];
    /// let (depth_block, rem) = avx2::LazyAvx2Vector::new(bytes);
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
pub mod nosimd;

#[cfg(test)]
mod tests {
    use super::*;
    use aligners::AlignedBytes;
    use test_case::test_case;

    fn is_depth_greater_or_equal_to_correctness<C: for<'a> Ctor<'a>>(
        bytes: &AlignedSlice<TwoTo<5>>,
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

    trait Ctor<'a> {
        type Item: DepthBlock<'a>;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<5>>) -> Self::Item;
    }

    struct NosimdVectorCtor();

    struct Avx2VectorCtor();

    struct LazyAvx2VectorCtor();

    impl<'a> Ctor<'a> for NosimdVectorCtor {
        type Item = nosimd::Vector<'a>;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<5>>) -> nosimd::Vector<'a> {
            let vector = nosimd::Vector::new(slice);
            *slice = Default::default();
            vector
        }
    }

    impl<'a> Ctor<'a> for Avx2VectorCtor {
        type Item = avx2::Avx2Vector;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<5>>) -> avx2::Avx2Vector {
            let (vector, rem) = avx2::Avx2Vector::new(slice);
            *slice = rem;
            vector
        }
    }

    impl<'a> Ctor<'a> for LazyAvx2VectorCtor {
        type Item = avx2::LazyAvx2Vector;

        fn ctor(slice: &mut &'a AlignedSlice<TwoTo<5>>) -> avx2::LazyAvx2Vector {
            let (vector, rem) = avx2::LazyAvx2Vector::new(slice);
            *slice = rem;
            vector
        }
    }

    #[test_case(Avx2VectorCtor(); "using avx2::Avx2Vector")]
    #[test_case(LazyAvx2VectorCtor(); "using avx2::LazyAvx2Vector")]
    #[test_case(NosimdVectorCtor(); "using nosimd::Vector")]
    fn is_depth_greater_or_equal_to_correctness_suite<C: for<'a> Ctor<'a>>(ctor: C) {
        let json = r#"{"aaa":[{},{"b":{"c":[1,2,3]}}]}"#;
        let depths = [
            1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
            2, 1, 0,
        ];
        let bytes: AlignedBytes<TwoTo<5>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(&bytes, &depths, &ctor);

        let json = r#"{}"#;
        let depths = [1, 0];
        let bytes: AlignedBytes<TwoTo<5>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(&bytes, &depths, &ctor);

        let json = r#""#;
        let depths = [];
        let bytes: AlignedBytes<TwoTo<5>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(&bytes, &depths, &ctor);

        let json = r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"e":{"a":[[],[1,2,3],[{"b":[{}]}]]},"d":42}"#;
        let depths = [
            1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
            2, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 4, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 4, 5, 5, 5, 5,
            5, 6, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0,
        ];
        let bytes: AlignedBytes<TwoTo<5>> = json.as_bytes().into();

        is_depth_greater_or_equal_to_correctness(&bytes, &depths, &ctor);
    }

    // #[test]
    // fn is_depth_greater_or_equal_to_avx2_Avx2Vector() {
    //     is_depth_greater_or_equal_to_correctness_suite::<avx2::Avx2Vector>();
    // }

    // #[test]
    // fn is_depth_greater_or_equal_to_avx2_LazyAvx2Vector() {
    //     is_depth_greater_or_equal_to_correctness_suite::<avx2::LazyAvx2Vector>();
    // }

    // #[test]
    // fn is_depth_greater_or_equal_to_nosimd_Vector() {
    //     is_depth_greater_or_equal_to_correctness_suite::<nosimd::Vector>();
    // }
}
