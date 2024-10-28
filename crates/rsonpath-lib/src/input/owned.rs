//! Takes ownership of bytes of the input document.
//!
//! Choose this implementation if:
//!
//! 1. You already have the data loaded in-memory.
//! 2. You don't want to deal with ownership and would rather have the input
//!    take ownership of the bytes.
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
    Input, SeekableBackwardsInput, SliceSeekable, MAX_BLOCK_SIZE,
};
use crate::result::InputRecorder;
use rsonpath_syntax::str::JsonString;
use std::borrow::Borrow;
use crate::input;

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

impl<'b, 'r, B, R, const N: usize> Input<'b, 'r, R, N> for OwnedBytes<B>
where
    B: Borrow<[u8]> + 'b,
    R: InputRecorder<&'b [u8]> + 'r,
{
    type BlockIterator = BorrowedBytesBlockIterator<'r, TwoSidesPaddedInput<'b>, R, N>;

    type Error = Infallible;

    type Block = &'b [u8];

    #[inline(always)]
    fn leading_padding_len(&self) -> usize {
        self.first_block.padding_len()
    }

    #[inline(always)]
    fn trailing_padding_len(&self) -> usize {
        self.last_block.padding_len()
    }

    #[inline]
    fn iter_blocks(&'b self, recorder: &'r R) -> Self::BlockIterator
    where
        R: InputRecorder<&'b [u8]>,
    {
        let (_, middle, _) = align_to::<MAX_BLOCK_SIZE>(self.bytes.borrow());
        assert_eq!(middle.len(), self.middle_len);

        let padded = TwoSidesPaddedInput::new(&self.first_block, middle, &self.last_block);

        BorrowedBytesBlockIterator::new(padded, recorder)
    }

    #[inline]
    fn seek_forward<const M: usize>(&self, from: usize, needles: [u8; M]) -> Result<Option<(usize, u8)>, Self::Error> {
        let offset = <OwnedBytes<B> as Input<'_, '_, R, N>>::leading_padding_len(self);
        let from = from.saturating_sub(offset);

        Ok(self
            .bytes
            .borrow()
            .seek_forward(from, needles)
            .map(|(x, y)| (x + <OwnedBytes<B> as Input<'_, '_, R, N>>::leading_padding_len(self), y)))
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, Self::Error> {
        let offset = <OwnedBytes<B> as input::Input<'_, '_, R, N>>::leading_padding_len(self);
        let from = from.saturating_sub(offset);

        Ok(self
            .bytes
            .borrow()
            .seek_non_whitespace_forward(from)
            .map(|(x, y)| (x + <OwnedBytes<B> as input::Input<'_, '_, R, N>>::leading_padding_len(self), y)))
    }

    #[inline]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> Result<bool, Self::Error> {
        let offset = <OwnedBytes<B> as input::Input<'_, '_, R, N>>::leading_padding_len(self);
        let Some(from) = from.checked_sub(offset) else {
            return Ok(false);
        };

        Ok(self.bytes.borrow().is_member_match(from, to - offset, member))
    }
}

impl<'b, 'r, B, R, const N: usize> SeekableBackwardsInput<'b, 'r, R, N> for OwnedBytes<B>
where
    B: Borrow<[u8]> + 'b,
    R: InputRecorder<&'b [u8]> + 'r,
{
    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let offset = <OwnedBytes<B> as input::Input<'_, '_, R, N>>::leading_padding_len(self);
        let from = from.checked_sub(offset)?;

        self.bytes.borrow().seek_backward(from, needle).map(|x| x + offset)
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        let offset = <OwnedBytes<B> as input::Input<'_, '_, R, N>>::leading_padding_len(self);
        let from = from.checked_sub(offset)?;

        self.bytes
            .borrow()
            .seek_non_whitespace_backward(from)
            .map(|(x, y)| (x + <OwnedBytes<B> as input::Input<'_, '_, R, N>>::leading_padding_len(self), y))
    }
}
