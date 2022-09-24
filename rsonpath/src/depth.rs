//! JSON depth calculations on byte streams.
//!
//! There is only one sequential implementation, [`nosimd::Vector`]. Other implementations are SIMD based.
//!
//! The recommended implementation of [`DepthBlock`] is [`avx2::LazyVector`]
//! which is optimized for the usual case where the depth does not change too sharply.
//! within a single 32-byte block.

use crate::quotes::{QuoteClassifiedIterator, ResumeClassifierState};
use cfg_if::cfg_if;

/// Common trait for structs that enrich a byte block with JSON depth information.
#[allow(clippy::len_without_is_empty)]
pub trait DepthBlock<'a>: Sized {
    /// Return the length of the decorated block.
    ///
    /// This should be constant throughout the lifetime of a `DepthBlock`
    /// and always satisfy:
    ///
    /// # use rsonpath::depth::{DepthBlock, avx2};
    /// # use aligners::{AlignedBytes, alignment::TwoTo};
    /// # let bytes: AlignedBytes<TwoTo<5>> = [0; 256].into();
    /// let (depth_block, rem) = avx2::LazyVector::new(&bytes);
    /// let expected_len = bytes.len() - rem.len();
    ///
    /// assert_eq!(expected_len, depth_block.len());
    ///
    fn len(&self) -> usize;

    /// Set the depth the block considers as initial.
    /// All depth calculations will be done with this amount added.
    fn set_starting_depth(&mut self, depth: isize);

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

    /// A lower bound on the depth that can be reached when advancing.
    ///
    /// It is guaranteed that [`is_depth_greater_or_equal_to`](`DepthBlock::is_depth_greater_or_equal_to`)
    /// will always return `false` when given this depth, but it is not guaranteed to be a depth that
    /// is actually achievable within the block. In particular, an implementation always returning
    /// [`isize::MIN`] is a correct implementation. This is meant to be a tool for performance improvements,
    /// not reliably checking the actual minimal depth within the block.
    fn estimate_lowest_possible_depth(&self) -> isize;

    /// Returns exact depth at the end of the decorated slice.
    fn depth_at_end(&self) -> isize;

    fn advance_to_next_depth_change(&mut self) -> bool;

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

/// Trait for depth iterators, i.e. finite iterators returning depth information
/// about JSON documents.
pub trait DepthIterator<'a, I: QuoteClassifiedIterator<'a>>:
    Iterator<Item = Self::Block> + 'a
{
    /// Type of the [`DepthBlock`] implementation used by this iterator.
    type Block: DepthBlock<'a>;

    fn resume(state: ResumeClassifierState<'a, I>) -> (Option<Self::Block>, Self);

    fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'a, I>;
}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        pub mod nosimd;

        /// Enrich quote classified blocks with depth information.
        pub fn classify_depth<'a, I: QuoteClassifiedIterator<'a>>(iter: I) -> impl DepthIterator<'a, I> {
            nosimd::VectorIterator::new(iter)
        }
        
        pub fn resume_depth_classification<'a, I: QuoteClassifiedIterator<'a>>(
            state: ResumeClassifierState<'a, I>,
        ) -> (Option<nosimd::Vector>, nosimd::VectorIterator<'a, I>) {
            nosimd::VectorIterator::resume(state)
        }
    }
    else if #[cfg(simd = "avx2")] {
        pub mod avx2;

        /// Enrich quote classified blocks with depth information.
        pub fn classify_depth<'a, I: QuoteClassifiedIterator<'a>>(iter: I) -> impl DepthIterator<'a, I> {
            avx2::VectorIterator::new(iter)
        }
        
        pub fn resume_depth_classification<'a, I: QuoteClassifiedIterator<'a>>(
            state: ResumeClassifierState<'a, I>,
        ) -> (Option<avx2::Vector>, avx2::VectorIterator<'a, I>) {
            avx2::VectorIterator::resume(state)
        }
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}
#[cfg(test)]
mod tests {
    use crate::quotes::classify_quoted_sequences;

    use super::*;
    use aligners::AlignedBytes;
    use test_case::test_case;

    #[test_case("", &[]; "empty")]
    #[test_case("{}", &[1, 0]; "just braces")]
    #[test_case("abcdefghijklmnopqrstuvwxyz", &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]; "no structural")]
    #[test_case(r#"{"aaa":[{},{"b":{"c":[1,2,3]}}]}"#, &[
        1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
        2, 1, 0,
    ]; "simple json")]
    #[test_case(r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"e":{"a":[[],[1,2,3],[{"b":[{}]}]]},"d":42}"#, &[
        1, 1, 1, 1, 1, 1, 1, 2, 3, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 4, 3,
        2, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 4, 3, 3, 4, 4, 4, 4, 4, 4, 3, 3, 4, 5, 5, 5, 5,
        5, 6, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 1, 0,]; "long json")]
    fn depth_at_each_step(input: &str, depths: &[isize]) {
        assert_eq!(input.len(), depths.len(), "Invalid test data.");

        let bytes = AlignedBytes::new_padded(input.as_bytes());
        let quote_iter = classify_quoted_sequences(&bytes);
        let depth_iter = classify_depth(quote_iter);
        let mut depths_idx = 0;
        let mut accumulated_depth = 0;

        for mut vector in depth_iter {
            vector.set_starting_depth(accumulated_depth);
            loop {
                let depth = if depths.len() <= depths_idx {
                    depths.last().copied().unwrap_or(0)
                } else {
                    depths[depths_idx]
                };
                assert!(
                    vector.is_depth_greater_or_equal_to(depth),
                    "Failed for exact depth: '{}' at index '{}'",
                    depth,
                    depths_idx
                );
                assert!(
                    vector.is_depth_greater_or_equal_to(depth - 1),
                    "Failed for depth one below: '{}' at index '{}'",
                    depth,
                    depths_idx
                );
                assert!(
                    !vector.is_depth_greater_or_equal_to(depth + 1),
                    "Failed for depth one above: '{}' at index '{}'",
                    depth,
                    depths_idx
                );
                depths_idx += 1;
                if !vector.advance() {
                    break;
                }
            }
            accumulated_depth = vector.depth_at_end();
        }

        assert_eq!(bytes.len(), depths_idx);
    }
}
