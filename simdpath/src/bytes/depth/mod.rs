//! JSON depth calculations on byte streams.
//!
#![cfg_attr(
    not(feature = "simd"),
    doc = "There is only one sequential implementation, [`Vector`]. Other implementations are SIMD based and behind the default `simd` feature."
)]
#![cfg_attr(
    all(
        feature = "simd",
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2",
    ),
    doc = "The recommended implementation of [`DepthBlock`] is [`LazyAvx2Vector`]"
)]
#![cfg_attr(
    all(
        feature = "simd",
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2",
    ),
    doc = "which is optimized for the usual case where the depth does not change too sharply"
)]
#![cfg_attr(feature = "simd", doc = "within a single 32-byte block.")]
use cfg_if::cfg_if;

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
    /// # use simdpath::bytes::depth::{DepthBlock, DepthBlockImpl} ;
    /// # let bytes = &[0; 256];
    /// let (depth_block, rem) = DepthBlockImpl::new(bytes);
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

mod nosimd;
pub use nosimd::*;

cfg_if! {
    if #[cfg(doc)] {
        #[cfg_attr(docsrs, doc(cfg(all(
            feature = "simd",
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "avx2",
        ))))]
        mod avx2;

        #[cfg_attr(docsrs, doc(cfg(all(
            feature = "simd",
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "avx2",
        ))))]
        #[doc(inline)]
        pub use avx2::*;

        /// Default [`DepthBlock`] implementation for `simd` disabled.
        pub type DepthBlockImpl<'a> = Vector<'a>;
    }
    else if #[cfg(not(feature = "simd"))] {
        /// Default [`DepthBlock`] implementation for `simd` disabled.
        pub type DepthBlockImpl<'a> = Vector<'a>;
    }
    else if #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        target_feature = "avx2",
    ))] {
        mod avx2;
        pub use avx2::*;
        /// Default [`DepthBlock`] implementation for AVX2 SIMD.
        pub type DepthBlockImpl = LazyAvx2Vector;
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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

    #[cfg_attr(
        all(feature = "simd", target_feature = "avx2"),
        test_case(Avx2Vector::new; "using simd::Avx2Vector::new"),
        test_case(LazyAvx2Vector::new; "using simd::LazyAvx2Vector::new"),
    )]
    #[cfg_attr(
        not(feature = "simd"),
        test_case(Vector::new; "using nosimd::Vector::new")
    )]
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
}
