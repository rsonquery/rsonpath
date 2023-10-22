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
use std::marker::PhantomData;

use super::{
    error::Infallible,
    padding::{EndPaddedInput, PaddedBlock, TwoSidesPaddedInput},
    Input, InputBlockIterator, InputError, SliceSeekable, MAX_BLOCK_SIZE,
};
use crate::{debug, query::JsonString, result::InputRecorder, FallibleIterator};

/// Input wrapping a borrowed [`[u8]`] buffer.
pub struct BorrowedBytes<'a> {
    middle_bytes: &'a [u8],
    first_block: PaddedBlock,
    last_block: PaddedBlock,
}

/// Iterator over blocks of [`BorrowedBytes`] of size exactly `N`.
pub struct BorrowedBytesBlockIterator<'a, 'r, I, R, const N: usize> {
    input: I,
    idx: usize,
    recorder: &'r R,
    phantom: PhantomData<&'a I>,
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
    pub fn new(bytes: &'a [u8]) -> Self {
        let (first, middle, last) = align_to::<MAX_BLOCK_SIZE>(bytes);
        let first_block = PaddedBlock::pad_first_block(first);
        let last_block = PaddedBlock::pad_last_block(last);

        Self {
            middle_bytes: middle,
            first_block,
            last_block,
        }
    }

    pub(super) fn as_padded_input(&self) -> TwoSidesPaddedInput {
        TwoSidesPaddedInput::new(&self.first_block, self.middle_bytes, &self.last_block)
    }
}

impl<'a, 'r, I, R, const N: usize> BorrowedBytesBlockIterator<'a, 'r, I, R, N>
where
    R: InputRecorder<&'a [u8]>,
{
    #[must_use]
    #[inline(always)]
    pub(super) fn new(input: I, recorder: &'r R) -> Self {
        Self {
            idx: 0,
            input,
            recorder,
            phantom: PhantomData,
        }
    }
}

impl<'a> Input for BorrowedBytes<'a> {
    type BlockIterator<'b, 'r, R, const N: usize> = BorrowedBytesBlockIterator<'b, 'r, TwoSidesPaddedInput<'b>, R, N>
    where Self: 'b,
          R: InputRecorder<&'b [u8]> + 'r;

    type Error = Infallible;
    type Block<'b, const N: usize> = &'b [u8] where Self: 'b;

    #[inline(always)]
    fn leading_padding_len(&self) -> usize {
        self.first_block.padding_len()
    }

    #[inline(always)]
    fn trailing_padding_len(&self) -> usize {
        self.last_block.padding_len()
    }

    #[inline(always)]
    fn len_hint(&self) -> Option<usize> {
        Some(self.middle_bytes.len() + self.first_block.len() + self.last_block.len())
    }

    #[inline(always)]
    fn iter_blocks<'b, 'r, R, const N: usize>(&'b self, recorder: &'r R) -> Self::BlockIterator<'b, 'r, R, N>
    where
        R: InputRecorder<&'b [u8]>,
    {
        let padded_input = TwoSidesPaddedInput::new(&self.first_block, self.middle_bytes, &self.last_block);

        Self::BlockIterator {
            idx: 0,
            input: padded_input,
            recorder,
            phantom: PhantomData,
        }
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        return if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() {
            match self.middle_bytes.seek_backward(from - MAX_BLOCK_SIZE, needle) {
                Some(x) => Some(x + MAX_BLOCK_SIZE),
                None => handle_first(&self.first_block, needle),
            }
        } else {
            self.as_padded_input().seek_backward(from, needle)
        };

        #[cold]
        #[inline(never)]
        fn handle_first(first_block: &PaddedBlock, needle: u8) -> Option<usize> {
            first_block.bytes().seek_backward(first_block.len() - 1, needle)
        }
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, Infallible> {
        return Ok(if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() {
            match self.middle_bytes.seek_forward(from - MAX_BLOCK_SIZE, needles) {
                Some((x, y)) => Some((x + MAX_BLOCK_SIZE, y)),
                None => handle_last(&self.last_block, MAX_BLOCK_SIZE + self.middle_bytes.len(), needles),
            }
        } else {
            self.as_padded_input().seek_forward(from, needles)
        });

        #[cold]
        #[inline(never)]
        fn handle_last<const N: usize>(
            last_block: &PaddedBlock,
            offset: usize,
            needles: [u8; N],
        ) -> Option<(usize, u8)> {
            last_block
                .bytes()
                .seek_forward(0, needles)
                .map(|(x, y)| (x + offset, y))
        }
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, Infallible> {
        return Ok(
            // The hot path is when we start and end within the middle section.
            // We use the regular slice path for that scenario, and fall back to the very expensive
            // TwoSidesPaddedInput with all bells and whistles only when that doesn't work.
            if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() {
                match self.middle_bytes.seek_non_whitespace_forward(from - MAX_BLOCK_SIZE) {
                    Some((x, y)) => Some((x + MAX_BLOCK_SIZE, y)),
                    None => handle_last(&self.last_block, MAX_BLOCK_SIZE + self.middle_bytes.len()),
                }
            } else {
                self.as_padded_input().seek_non_whitespace_forward(from)
            },
        );

        #[cold]
        #[inline(never)]
        fn handle_last(last_block: &PaddedBlock, offset: usize) -> Option<(usize, u8)> {
            last_block
                .bytes()
                .seek_non_whitespace_forward(0)
                .map(|(x, y)| (x + offset, y))
        }
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        return if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() {
            match self.middle_bytes.seek_non_whitespace_backward(from - MAX_BLOCK_SIZE) {
                Some((x, y)) => Some((x + MAX_BLOCK_SIZE, y)),
                None => handle_first(&self.first_block),
            }
        } else {
            self.as_padded_input().seek_non_whitespace_backward(from)
        };

        #[cold]
        #[inline(never)]
        fn handle_first(first_block: &PaddedBlock) -> Option<(usize, u8)> {
            first_block.bytes().seek_non_whitespace_backward(first_block.len() - 1)
        }
    }

    #[inline(always)]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> bool {
        debug_assert!(from <= to);
        // The hot path is when we're checking fully within the middle section.
        // This has to be as fast as possible, so the "cold" path referring to the TwoSidesPaddedInput
        // impl is explicitly marked with #[cold].
        if from > MAX_BLOCK_SIZE && to < self.middle_bytes.len() + MAX_BLOCK_SIZE {
            // This is the hot path -- do the bounds check and memcmp.
            let bytes = self.middle_bytes;
            let from = from - MAX_BLOCK_SIZE;
            let to = to - MAX_BLOCK_SIZE + 1;
            if to > bytes.len() {
                return false;
            }
            let slice = &bytes[from..to];
            member.bytes_with_quotes() == slice && (from == 0 || bytes[from - 1] != b'\\')
        } else {
            // This is a very expensive, cold path.
            self.as_padded_input().is_member_match(from, to, member)
        }
    }
}

impl<'a, 'r, R, const N: usize> InputBlockIterator<'a, N>
    for BorrowedBytesBlockIterator<'a, 'r, TwoSidesPaddedInput<'a>, R, N>
where
    R: InputRecorder<&'a [u8]> + 'r,
{
    type Block = &'a [u8];
    type Error = Infallible;

    #[inline(always)]
    fn next(&mut self) -> Result<Option<Self::Block>, Self::Error> {
        debug!("next!");
        return if self.idx >= MAX_BLOCK_SIZE && self.idx < self.input.middle().len() + MAX_BLOCK_SIZE {
            let start = self.idx - MAX_BLOCK_SIZE;
            let block = unsafe { self.input.middle().get_unchecked(start..start + N) };
            self.recorder.record_block_start(block);
            self.idx += N;
            Ok(Some(block))
        } else {
            Ok(cold_path(self))
        };

        #[cold]
        fn cold_path<'a, 'r, R, const N: usize>(
            iter: &mut BorrowedBytesBlockIterator<'a, 'r, TwoSidesPaddedInput<'a>, R, N>,
        ) -> Option<&'a [u8]>
        where
            R: InputRecorder<&'a [u8]>,
        {
            let block = iter.input.try_slice(iter.idx, N);

            if let Some(b) = block {
                iter.recorder.record_block_start(b);
                iter.idx += N;
            }

            block
        }
    }

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

impl<'a, 'r, R, const N: usize> InputBlockIterator<'a, N>
    for BorrowedBytesBlockIterator<'a, 'r, EndPaddedInput<'a>, R, N>
where
    R: InputRecorder<&'a [u8]> + 'r,
{
    type Block = &'a [u8];
    type Error = Infallible;

    #[inline(always)]
    fn next(&mut self) -> Result<Option<Self::Block>, Self::Error> {
        debug!("next!");
        return if self.idx < self.input.middle().len() {
            let start = self.idx;
            let block = unsafe { self.input.middle().get_unchecked(start..start + N) };
            self.recorder.record_block_start(block);
            self.idx += N;
            Ok(Some(block))
        } else {
            Ok(cold_path(self))
        };

        #[cold]
        fn cold_path<'a, 'r, R, const N: usize>(
            iter: &mut BorrowedBytesBlockIterator<'a, 'r, EndPaddedInput<'a>, R, N>,
        ) -> Option<&'a [u8]>
        where
            R: InputRecorder<&'a [u8]>,
        {
            let block = iter.input.try_slice(iter.idx, N);

            if let Some(b) = block {
                iter.recorder.record_block_start(b);
                iter.idx += N;
            }

            block
        }
    }

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

// This is mostly adapted from [slice::align_to](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to).
fn align_to<const N: usize>(bytes: &[u8]) -> (&[u8], &[u8], &[u8]) {
    let ptr = bytes.as_ptr();
    let offset = ptr.align_offset(N);
    if offset > bytes.len() {
        (bytes, &[], &[])
    } else {
        let (left, rest) = bytes.split_at(offset);
        let middle_len = (rest.len() / N) * N;
        let (middle, right) = rest.split_at(middle_len);

        (left, middle, right)
    }
}

#[cfg(test)]
mod tests {
    use crate::input::MAX_BLOCK_SIZE;

    use super::align_to;

    // Run all tests for the actual alignment we use.
    const N: usize = MAX_BLOCK_SIZE;
    const SIZE: usize = 1024;

    #[repr(align(128))]
    struct Aligned {
        bytes: [u8; SIZE],
    }

    #[test]
    fn test_all_alignments() {
        // We construct a byte array that is already aligned,
        // and then take all suffixes for all possible misalignments
        // and small sizes.
        let aligned = Aligned { bytes: get_bytes() };
        let slice = &aligned.bytes;

        for i in 0..slice.len() {
            let misalignment = i % N;
            test_with_misalignment(misalignment, &slice[i..]);
        }
    }

    fn get_bytes() -> [u8; SIZE] {
        let mut bytes = [0; SIZE];

        for (i, b) in bytes.iter_mut().enumerate() {
            let x = i % (u8::MAX as usize);
            *b = x as u8;
        }

        bytes
    }

    fn test_with_misalignment(misalignment: usize, slice: &[u8]) {
        let expected_left_len = (N - misalignment) % N;
        let expected_rem_len = slice.len() - expected_left_len;
        let expected_middle_len = (expected_rem_len / N) * N;
        let expected_right_len = expected_rem_len - expected_middle_len;

        let (left, middle, right) = align_to::<N>(slice);
        let glued_back: Vec<_> = [left, middle, right].into_iter().flatten().copied().collect();

        assert_eq!(left.len(), expected_left_len, "misalignment = {misalignment}");
        assert_eq!(middle.len(), expected_middle_len, "misalignment = {misalignment}");
        assert_eq!(right.len(), expected_right_len, "misalignment = {misalignment}");
        assert_eq!(glued_back, slice, "misalignment = {misalignment}");
    }
}
