//! Borrows a slice of bytes of the input document.
//!
//! Choose this implementation if:
//!
//! 1. You already have the data loaded in-memory and it is properly aligned.
//!
//! ## Performance characteristics
//!
//! This type of input is the fastest to process for the engine,
//! since there is no additional overhead from loading anything to memory.

use log::debug;

use super::*;
use crate::{query::JsonString, result::InputRecorder};

/// Input wrapping a borrowed [`[u8]`] buffer.
pub struct BorrowedBytes<'a> {
    bytes: &'a [u8],
    last_block: LastBlock,
}

/// Iterator over blocks of [`BorrowedBytes`] of size exactly `N`.
pub struct BorrowedBytesBlockIterator<'a, 'r, const N: usize, R: InputRecorder> {
    input: &'a [u8],
    current_block: Option<&'a [u8]>,
    last_block: &'a LastBlock,
    idx: usize,
    recorder: &'r R,
}

impl<'a> BorrowedBytes<'a> {
    /// Create a new instance of [`BorrowedBytes`] wrapping the given buffer.
    ///
    /// # Safety
    /// The buffer must satisfy all invariants of [`BorrowedBytes`],
    /// since it is not copied or modified. It must:
    /// - have length divisible by [`MAX_BLOCK_SIZE`] (the function checks this);
    /// - be aligned to [`MAX_BLOCK_SIZE`].
    ///
    /// The latter condition cannot be reliably checked.
    /// Violating it may result in memory errors where the engine relies
    /// on proper alignment.
    ///
    /// # Panics
    ///
    /// If `bytes.len()` is not divisible by [`MAX_BLOCK_SIZE`].
    #[must_use]
    #[inline(always)]
    pub unsafe fn new(bytes: &'a [u8]) -> Self {
        assert_eq!(bytes.len() % MAX_BLOCK_SIZE, 0);
        let last_block = in_slice::pad_last_block(bytes);
        Self { bytes, last_block }
    }

    /// Get a reference to the bytes as a slice.
    #[must_use]
    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        self.bytes
    }

    /// Copy the bytes to an [`OwnedBytes`] instance.
    #[must_use]
    #[inline(always)]
    pub fn to_owned(&self) -> OwnedBytes {
        OwnedBytes::from(self)
    }
}

impl<'a> AsRef<[u8]> for BorrowedBytes<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        self.bytes
    }
}

impl<'a, 'r, const N: usize, R: InputRecorder> BorrowedBytesBlockIterator<'a, 'r, N, R> {
    #[must_use]
    #[inline(always)]
    pub(super) fn new(bytes: &'a [u8], last_block: &'a LastBlock, recorder: &'r R) -> Self {
        Self {
            input: bytes,
            idx: 0,
            current_block: None,
            last_block,
            recorder,
        }
    }
}

impl<'a> Input for BorrowedBytes<'a> {
    type BlockIterator<'b, 'r, const N: usize, R: InputRecorder + 'r> = BorrowedBytesBlockIterator<'b, 'r, N, R> where Self: 'b;

    type Block<'b, const N: usize> = &'b [u8] where Self: 'b;

    #[inline(always)]
    fn iter_blocks<'b, 'r, R: InputRecorder, const N: usize>(
        &'b self,
        recorder: &'r R,
    ) -> Self::BlockIterator<'b, 'r, N, R> {
        Self::BlockIterator {
            input: self.bytes,
            idx: 0,
            current_block: None,
            last_block: &self.last_block,
            recorder,
        }
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        in_slice::seek_backward(self.bytes, from, needle)
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, InputError> {
        Ok(in_slice::seek_forward(self.as_slice(), from, needles))
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, InputError> {
        Ok(in_slice::seek_non_whitespace_forward(self.bytes, from))
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        in_slice::seek_non_whitespace_backward(self.bytes, from)
    }

    #[inline]
    fn find_member(&self, from: usize, member: &JsonString) -> Result<Option<usize>, InputError> {
        Ok(in_slice::find_member(self.bytes, from, member))
    }

    #[inline]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> bool {
        in_slice::is_member_match(self.bytes, from, to, member)
    }
}

impl<'a, 'r, const N: usize, R: InputRecorder> FallibleIterator for BorrowedBytesBlockIterator<'a, 'r, N, R> {
    type Item = &'a [u8];
    type Error = InputError;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        debug!("next!");
        if let Some(block) = self.current_block.take() {
            self.recorder.record_block_end(block);
        }

        if self.idx >= self.input.len() {
            Ok(None)
        } else if self.idx >= self.last_block.absolute_start {
            let i = self.idx - self.last_block.absolute_start;
            self.idx += N;
            let block = &self.last_block.bytes[i..i + N];
            self.current_block = Some(block);

            Ok(Some(block))
        } else {
            let block = &self.input[self.idx..self.idx + N];
            self.idx += N;
            self.current_block = Some(block);

            Ok(Some(block))
        }
    }
}

impl<'a, 'r, const N: usize, R: InputRecorder> InputBlockIterator<'a, N> for BorrowedBytesBlockIterator<'a, 'r, N, R> {
    type Block = &'a [u8];

    #[inline(always)]
    fn offset(&mut self, count: isize) {
        assert!(count >= 0);
        debug!("offsetting input iter by {count}");
        self.idx += count as usize * N;
    }

    #[inline(always)]
    fn get_offset(&self) -> usize {
        debug!("getting input iter {}", self.idx);
        self.idx
    }
}

impl<'a, 'r, const N: usize, R: InputRecorder> Drop for BorrowedBytesBlockIterator<'a, 'r, N, R> {
    #[inline]
    fn drop(&mut self) {
        if let Some(block) = self.current_block.take() {
            self.recorder.record_block_end(block);
        }
    }
}
