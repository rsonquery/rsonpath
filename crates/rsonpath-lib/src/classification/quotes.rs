//! Classification of bytes withing JSON quote sequences.
//!
//! Provides the [`QuoteClassifiedBlock`] struct and [`QuoteClassifiedIterator`] trait
//! that allow effectively enriching JSON inputs with quote sequence information.
//!
//! The output of quote classification is an iterator of [`QuoteClassifiedBlock`]
//! which contain bitmasks whose lit bits signify characters that are within quotes
//! in the source document. These characters need to be ignored.
//!
//! Note that the actual quote characters are not guaranteed to be classified
//! as "within themselves" or otherwise. In particular the current implementation
//! marks _opening_ quotes with lit bits, but _closing_ quotes are always unmarked.
//! This behavior should not be presumed to be stable, though, and can change
//! without a major semver bump.
//!
//! # Examples
//! ```
//! use rsonpath::classification::quotes::{classify_quoted_sequences, QuoteClassifiedIterator};
//! use rsonpath::input::{Input, OwnedBytes};
//! use rsonpath::result::empty::EmptyRecorder;
//! use rsonpath::FallibleIterator;
//!
//! let json = r#"{"x": "string", "y": {"z": "\"escaped\""}}"#.to_owned();
//! //            011000111111100011000011000111111111111000
//! // The mask below appears reversed due to endianness.
//! let expd = 0b000111111111111000110000110001111111000110;
//! let input = OwnedBytes::try_from(json).unwrap();
//! let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
//! let mut quote_classifier = classify_quoted_sequences(iter);
//!
//! let block = quote_classifier.next().unwrap().unwrap();
//! assert_eq!(expd, block.within_quotes_mask);
//! ```

use crate::{
    input::{error::InputError, InputBlock, InputBlockIterator},
    FallibleIterator, MaskType, BLOCK_SIZE,
};
use cfg_if::cfg_if;

/// Input block with a bitmask signifying which characters are within quotes.
///
/// Characters within quotes in the input are guaranteed to have their corresponding
/// bit in `within_quotes_mask` set. The $0$-th bit of the mask corresponds to the
/// last character in `block`, the $1$-st bit to the second-to-last character, etc.
///
/// There is no guarantee on how the boundary quote characters are classified,
/// their bits might be lit or not lit depending on the implementation.
pub struct QuoteClassifiedBlock<B, M, const N: usize> {
    /// The block that was classified.
    pub block: B,
    /// Mask marking characters within a quoted sequence.
    pub within_quotes_mask: M,
}

/// Trait for quote classifier iterators, i.e. finite iterators
/// enriching blocks of input with quote bitmasks.
/// Iterator is allowed to hold a reference to the JSON document valid for `'a`.
pub trait QuoteClassifiedIterator<'i, I: InputBlockIterator<'i, N>, M, const N: usize>:
    FallibleIterator<Item = QuoteClassifiedBlock<I::Block, M, N>, Error = InputError>
{
    /// Get the total offset in bytes from the beginning of input.
    fn get_offset(&self) -> usize;

    /// Move the iterator `count` blocks forward.
    /// Effectively skips `count * Twice<BlockAlignment>::size()` bytes.
    fn offset(&mut self, count: isize);

    /// Flip the bit representing whether the last block ended with a nonescaped quote.
    ///
    /// This should be done only in very specific circumstances where the previous-block
    /// state could have been damaged due to stopping and resuming the classification at a later point.
    fn flip_quotes_bit(&mut self);
}

/// Higher-level classifier that can be consumed to retrieve the inner
/// [`Input::BlockIterator`](crate::input::Input::BlockIterator).
pub trait InnerIter<I> {
    /// Consume `self` and return the wrapped [`Input::BlockIterator`](crate::input::Input::BlockIterator).
    fn into_inner(self) -> I;
}

impl<'i, B, M, const N: usize> QuoteClassifiedBlock<B, M, N>
where
    B: InputBlock<'i, N>,
{
    /// Returns the length of the classified block.
    #[must_use]
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.block.len()
    }

    /// Whether the classified block is empty.
    #[must_use]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.block.is_empty()
    }
}

mod avx2_32;
mod avx2_64;
mod nosimd;
mod shared;
mod ssse3_32;
mod ssse3_64;

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        type ClassifierImpl<'i, I, const N: usize> = nosimd::SequentialQuoteClassifier<'i, I, N>;
    }
    else if #[cfg(simd = "avx2_64")] {
        type ClassifierImpl<'i, I> = avx2_64::Avx2QuoteClassifier64<'i, I>;
    }
    else if #[cfg(simd = "avx2_32")] {
        type ClassifierImpl<'i, I> = avx2_32::Avx2QuoteClassifier32<'i, I>;
    }
    else if #[cfg(simd = "ssse3_64")] {
        type ClassifierImpl<'i, I> = ssse3_64::Ssse3QuoteClassifier64<'i, I>;
    }
    else if #[cfg(simd = "ssse3_32")] {
        type ClassifierImpl<'i, I> = ssse3_32::Ssse3QuoteClassifier32<'i, I>;
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}

/// Walk through the JSON document represented by `bytes`
/// and classify quoted sequences.
#[must_use]
#[inline(always)]
pub fn classify_quoted_sequences<'i, I>(
    iter: I,
) -> impl QuoteClassifiedIterator<'i, I, MaskType, BLOCK_SIZE> + InnerIter<I>
where
    I: InputBlockIterator<'i, BLOCK_SIZE>,
{
    ClassifierImpl::new(iter)
}

pub(crate) fn resume_quote_classification<'i, I>(
    iter: I,
    first_block: Option<I::Block>,
) -> (
    impl QuoteClassifiedIterator<'i, I, MaskType, BLOCK_SIZE> + InnerIter<I>,
    Option<QuoteClassifiedBlock<I::Block, MaskType, BLOCK_SIZE>>,
)
where
    I: InputBlockIterator<'i, BLOCK_SIZE>,
{
    ClassifierImpl::resume(iter, first_block)
}
