use crate::{
    query::Label,
    quotes::{QuoteClassifiedIterator, ResumeClassifierState},
};
use cfg_if::cfg_if;

/// Common trait for structs that enrich a byte block with JSON depth information.
#[allow(clippy::len_without_is_empty)]
pub trait DepthSeekBlock<'a>: Sized {
    /// Add depth to the block.
    /// This is usually done at the start of a block to carry any accumulated
    /// depth over.
    fn add_depth(&mut self, depth: isize);

    /// Returns depth at the current position.
    fn get_depth(&self) -> isize;

    /// Returns the index to which depth has been queried already.
    fn get_depth_idx(&self) -> usize;

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
    fn advance_to_next_depth_decrease(&mut self) -> Option<usize>;

    /// Advance to the next position at which a label match may occur.
    fn advance_to_next_possible_match(&mut self) -> Option<usize>;

    /// Set the vector indices to the end of the current block.
    fn advance_to_end(&mut self);
}

/// Trait for depth iterators, i.e. finite iterators returning depth information
/// about JSON documents.
pub trait DepthSeekIterator<'a, I: QuoteClassifiedIterator<'a>>:
    Iterator<Item = Self::Block> + 'a
{
    /// Type of the [`DepthBlock`] implementation used by this iterator.
    type Block: DepthSeekBlock<'a>;

    /// Resume classification from a state retrieved by a previous
    /// [`DepthIterator::stop`] or [`StructuralIterator::stop`](`crate::classify::StructuralIterator::stop`) invocation.
    fn resume(state: ResumeClassifierState<'a, I>, label: &Label, opening: u8) -> (Option<Self::Block>, Self);

    /// Stop classification and return a state object that can be used to resume
    /// a classifier from the place in which the current one was stopped.
    fn stop(self, block: Option<Self::Block>, result: Option<usize>) -> ResumeClassifierState<'a, I>;
}

/// The result of resuming a [`DepthSeekIterator`] &ndash; the first block and the rest of the iterator.
pub struct DepthSeekIteratorResumeOutcome<
    'a,
    I: QuoteClassifiedIterator<'a>,
    D: DepthSeekIterator<'a, I>,
>(pub Option<D::Block>, pub D);

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;

        /// Enrich quote classified blocks with depth information.
        #[inline(always)]
        pub fn classify_depth_seek<'a, I: QuoteClassifiedIterator<'a>>(iter: I, label: &Label, opening: u8) -> impl DepthSeekIterator<'a, I> {
            nosimd::VectorIterator::new(iter, label, opening)
        }

        /// Resume classification using a state retrieved from a previously
        /// used classifier via the `stop` function.
        #[inline(always)]
        pub fn resume_depth_seek_classification<'a, I: QuoteClassifiedIterator<'a>>(
            state: ResumeClassifierState<'a, I>, label: &Label, opening: u8
        ) -> DepthSeekIteratorResumeOutcome<'a, I, impl DepthSeekIterator<'a, I>> {
            let (first_block, iter) = nosimd::VectorIterator::resume(state, label, opening);
            DepthSeekIteratorResumeOutcome(first_block, iter)
        }
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;

        /// Enrich quote classified blocks with depth information.
        #[inline(always)]
        pub fn classify_depth_seek<'a, I: QuoteClassifiedIterator<'a>>(iter: I, label: &Label, opening: u8) -> impl DepthIterator<'a, I> {
            avx2::VectorIterator::new(iter, label, opening)
        }

        /// Resume classification using a state retrieved from a previously
        /// used classifier via the `stop` function.
        #[inline(always)]
        pub fn resume_depth_seek_classification<'a, I: QuoteClassifiedIterator<'a>>(
            state: ResumeClassifierState<'a, I>, label: &Label, opening: u8
        ) -> DepthSeekIteratorResumeOutcome<'a, I, impl DepthSeekIterator<'a, I>> {
            let (first_block, iter) = avx2::VectorIterator::resume(state, label, opening);
            DepthSeekIteratorResumeOutcome(first_block, iter)
        }
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}
