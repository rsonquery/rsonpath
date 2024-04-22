//! Takes ownership of bytes of the input document.
//!
//! Choose this implementation if:
//!
//! 1. You already have the data loaded in-memory.
//! 2. You don't want to deal with ownership and would rather have the input
//! take ownership of the bytes.
//!
//! ## Performance characteristics
//!
//! This is as fast as [`BorrowedBytes`](`super::BorrowedBytes`), unless
//! the [`Borrow`] implementation of the underlying byte structure is weird
//! and costly.
// === Design note ===
// This struct appears to be basically the same as BorrowedBytes, just with different
// ownership mechanics. It appears that it should be possible to have a single struct
// that achieves the API of both, taking either ownership or a borrow, but this leads to
// lifetime issues around the current padding impl.

use super::{
    align_to,
    borrowed::BorrowedBytesBlockIterator,
    error::Infallible,
    padding::{PaddedBlock, TwoSidesPaddedInput},
    Input, SliceSeekable, MAX_BLOCK_SIZE,
};
use crate::{
    classification::simd::Simd,
    result::InputRecorder,
    string_pattern::{matcher::StringPatternMatcher, StringPattern},
};
use std::borrow::Borrow;

/// Input wrapping a buffer borrowable as a slice of bytes.
pub struct OwnedBytes<B> {
    bytes: B,
    middle_len: usize,
    first_block: PaddedBlock,
    last_block: PaddedBlock,
}

impl<B> OwnedBytes<B>
where
    B: Borrow<[u8]>,
{
    /// Create a new instance of [`OwnedBytes`] taking over the given buffer.
    ///
    /// The input will be automatically padded internally, incurring at most
    /// two times [`MAX_BLOCK_SIZE`] of memory overhead.
    #[inline(always)]
    pub fn new(bytes: B) -> Self {
        let (first, middle, last) = align_to::<MAX_BLOCK_SIZE>(bytes.borrow());
        let first_block = PaddedBlock::pad_first_block(first);
        let last_block = PaddedBlock::pad_last_block(last);

        Self {
            middle_len: middle.len(),
            bytes,
            first_block,
            last_block,
        }
    }
}

impl<B> From<B> for OwnedBytes<B>
where
    B: Borrow<[u8]>,
{
    #[inline(always)]
    fn from(value: B) -> Self {
        Self::new(value)
    }
}

impl From<String> for OwnedBytes<Vec<u8>> {
    #[inline(always)]
    fn from(value: String) -> Self {
        Self::new(value.into_bytes())
    }
}

impl<B> Input for OwnedBytes<B>
where
    B: Borrow<[u8]>,
{
    type BlockIterator<'i, 'r, R, const N: usize> = BorrowedBytesBlockIterator<'r, TwoSidesPaddedInput<'i>, R, N>
    where
        Self: 'i,
        R: InputRecorder<Self::Block<'i, N>> + 'r;

    type Error = Infallible;

    type Block<'i, const N: usize> = &'i [u8]
    where
        Self: 'i;

    #[inline(always)]
    fn leading_padding_len(&self) -> usize {
        self.first_block.padding_len()
    }

    #[inline(always)]
    fn trailing_padding_len(&self) -> usize {
        self.last_block.padding_len()
    }

    #[inline]
    fn iter_blocks<'i, 'r, R, const N: usize>(&'i self, recorder: &'r R) -> Self::BlockIterator<'i, 'r, R, N>
    where
        R: InputRecorder<Self::Block<'i, N>>,
    {
        let (_, middle, _) = align_to::<MAX_BLOCK_SIZE>(self.bytes.borrow());
        assert_eq!(middle.len(), self.middle_len);

        let padded = TwoSidesPaddedInput::new(&self.first_block, middle, &self.last_block);

        BorrowedBytesBlockIterator::new(padded, recorder)
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let offset = self.leading_padding_len();
        let from = from.checked_sub(offset)?;

        self.bytes.borrow().seek_backward(from, needle).map(|x| x + offset)
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, Self::Error> {
        let offset = self.leading_padding_len();
        let from = from.saturating_sub(offset);

        Ok(self
            .bytes
            .borrow()
            .seek_forward(from, needles)
            .map(|(x, y)| (x + self.leading_padding_len(), y)))
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, Self::Error> {
        let offset = self.leading_padding_len();
        let from = from.saturating_sub(offset);

        Ok(self
            .bytes
            .borrow()
            .seek_non_whitespace_forward(from)
            .map(|(x, y)| (x + self.leading_padding_len(), y)))
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        let offset = self.leading_padding_len();
        let from = from.checked_sub(offset)?;

        self.bytes
            .borrow()
            .seek_non_whitespace_backward(from)
            .map(|(x, y)| (x + self.leading_padding_len(), y))
    }

    #[inline]
    fn pattern_match_from<M: StringPatternMatcher>(
        &self,
        from: usize,
        pattern: &StringPattern,
    ) -> Result<Option<usize>, Self::Error> {
        let offset = self.leading_padding_len();
        let Some(from) = from.checked_sub(offset) else {
            return Ok(None);
        };

        Ok(self
            .bytes
            .borrow()
            .pattern_match_from::<M>(from, pattern)
            .map(|x| x + offset))
    }

    #[inline]
    fn pattern_match_to<M: StringPatternMatcher>(
        &self,
        to: usize,
        pattern: &StringPattern,
    ) -> Result<Option<usize>, Self::Error> {
        let offset = self.leading_padding_len();
        Ok(self
            .bytes
            .borrow()
            .pattern_match_to::<M>(to - offset, pattern)
            .map(|x| x + offset))
    }
}
