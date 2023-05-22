//! JSON depth calculations on byte streams.
//!
//! Used for iterating over the document while keeping track of depth.
//! This is heavily optimized for skipping irrelevant parts of a JSON document.
//! For example, to quickly skip to the end of the currently opened object one can set the
//! depth to `1` and then advance until it reaches `0`.
//!
//! It also supports stopping and resuming with [`ResumeClassifierState`].
//!
//! # Examples
//!
//! Illustrating how the skipping works:
//! ```rust
//! use rsonpath_lib::classification::quotes::classify_quoted_sequences;
//! use rsonpath_lib::classification::depth::{
//!     classify_depth, DepthIterator, DepthBlock
//! };
//! use rsonpath_lib::classification::structural::BracketType;
//! use rsonpath_lib::input::OwnedBytes;
//!
//! let json = r#"[42, {"b":[[]],"c":{}}, 44]}"#.to_owned();
//! //                        ^^            ^
//! //                        AB            C
//! let input = OwnedBytes::try_from(json).unwrap();
//! let quote_classifier = classify_quoted_sequences(&input);
//! // Goal: skip through the document until the end of the current list.
//! // We pass Square as the opening bracket type
//! // to tell the classifier to consider only '[' and ']' characters.
//! let mut depth_classifier = classify_depth(quote_classifier, BracketType::Square);
//! let mut depth_block = depth_classifier.next().unwrap();
//!
//! assert_eq!(depth_block.get_depth(), 0);
//! assert!(depth_block.advance_to_next_depth_decrease()); // Advances to A.
//! assert_eq!(depth_block.get_depth(), 2);
//! assert!(depth_block.advance_to_next_depth_decrease()); // Advances to B.
//! assert_eq!(depth_block.get_depth(), 1);
//! assert!(depth_block.advance_to_next_depth_decrease()); // Advances to C.
//! assert_eq!(depth_block.get_depth(), 0);
//! // Skipping complete.
//! ```
//!
//! Idiomatic usage for a high-performance skipping loop:
//! ```rust
//! use rsonpath_lib::classification::depth::{classify_depth, DepthBlock, DepthIterator};
//! use rsonpath_lib::classification::quotes::classify_quoted_sequences;
//! use rsonpath_lib::classification::structural::BracketType;
//! use rsonpath_lib::input::OwnedBytes;
//!
//! let json = r#"
//!     "a": [
//!         42,
//!         {
//!             "b": {},
//!             "c": {}
//!         },
//!         44
//!     ],
//!     "b": {
//!         "c": "value"
//!     }
//! }{"target":true}"#.to_owned();
//! // We expect to reach the newline before the opening brace of the second object.
//! let expected_idx = json.len() - 15;
//! let input = OwnedBytes::try_from(json).unwrap();
//! let quote_classifier = classify_quoted_sequences(&input);
//! let mut depth_classifier = classify_depth(quote_classifier, BracketType::Curly);
//! let mut current_depth = 1;
//!
//! while let Some(mut vector) = depth_classifier.next() {
//!     vector.add_depth(current_depth);
//!
//!     if vector.estimate_lowest_possible_depth() <= 0 {
//!         while vector.advance_to_next_depth_decrease() {
//!             if vector.get_depth() == 0 {
//!                 let stop_state = depth_classifier.stop(Some(vector));
//!                 assert_eq!(stop_state.get_idx(), expected_idx);
//!                 return;
//!             }
//!         }
//!     }
//!
//!     current_depth = vector.depth_at_end();
//! }
//! unreachable!();
//! ```
//!
use super::structural::BracketType;
use crate::{
    classification::{quotes::QuoteClassifiedIterator, ResumeClassifierState},
    input::Input,
    BLOCK_SIZE,
};
use cfg_if::cfg_if;

/// Common trait for structs that enrich a byte block with JSON depth information.
#[allow(clippy::len_without_is_empty)]
pub trait DepthBlock<'a>: Sized {
    /// Add depth to the block.
    /// This is usually done at the start of a block to carry any accumulated
    /// depth over.
    fn add_depth(&mut self, depth: isize);

    /// Returns depth at the current position.
    fn get_depth(&self) -> isize;

    /// A lower bound on the depth that can be reached when advancing.
    ///
    /// It is guaranteed that [`get_depth`](`DepthBlock::get_depth`)
    /// will always return something greater or equal to this return value, but it is not guaranteed to be a depth that
    /// is actually achievable within the block. In particular, an implementation always returning
    /// [`isize::MIN`] is a correct implementation. This is meant to be a tool for performance improvements,
    /// not reliably checking the actual minimal depth within the block.
    fn estimate_lowest_possible_depth(&self) -> isize;

    /// Returns exact depth at the end of the decorated slice.
    fn depth_at_end(&self) -> isize;

    /// Advance to the next position at which depth may decrease.
    ///
    /// # Returns
    /// `false` if the end of the block was reached without any depth decrease,
    /// `true` otherwise.
    fn advance_to_next_depth_decrease(&mut self) -> bool;
}

/// Trait for depth iterators, i.e. finite iterators returning depth information
/// about JSON documents.
pub trait DepthIterator<'a, I: Input, Q, const N: usize>: Iterator<Item = Self::Block> + 'a {
    /// Type of the [`DepthBlock`] implementation used by this iterator.
    type Block: DepthBlock<'a>;

    /// Resume classification from a state retrieved by a previous
    /// [`DepthIterator::stop`] or [`StructuralIterator::stop`](`crate::classification::structural::StructuralIterator::stop`) invocation.
    fn resume(state: ResumeClassifierState<'a, I, Q, N>, opening: BracketType) -> (Option<Self::Block>, Self);

    /// Stop classification and return a state object that can be used to resume
    /// a classifier from the place in which the current one was stopped.
    fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'a, I, Q, N>;
}

/// The result of resuming a [`DepthIterator`] &ndash; the first block and the rest of the iterator.
pub struct DepthIteratorResumeOutcome<'a, I, Q, D, const N: usize>(pub Option<D::Block>, pub D)
where
    I: Input,
    D: DepthIterator<'a, I, Q, N>;

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        type IteratorImpl<'a, I, Q, const N: usize> = nosimd::VectorIterator<'a, I, Q, N>;
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        type IteratorImpl<'a, I, Q> = avx2::VectorIterator<'a, I, Q>;
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}

/// Enrich quote classified blocks with depth information.
#[inline(always)]
pub fn classify_depth<'a, I, Q>(iter: Q, opening: BracketType) -> impl DepthIterator<'a, I, Q, BLOCK_SIZE>
where
    I: Input + 'a,
    Q: QuoteClassifiedIterator<'a, I, BLOCK_SIZE>,
{
    IteratorImpl::new(iter, opening)
}

/// Resume classification using a state retrieved from a previously
/// used classifier via the `stop` function.
#[inline(always)]
pub fn resume_depth_classification<'a, I, Q>(
    state: ResumeClassifierState<'a, I, Q, BLOCK_SIZE>,
    opening: BracketType,
) -> DepthIteratorResumeOutcome<'a, I, Q, impl DepthIterator<'a, I, Q, BLOCK_SIZE>, BLOCK_SIZE>
where
    I: Input + 'a,
    Q: QuoteClassifiedIterator<'a, I, BLOCK_SIZE>,
{
    let (first_block, iter) = IteratorImpl::resume(state, opening);
    DepthIteratorResumeOutcome(first_block, iter)
}
