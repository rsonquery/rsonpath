//! Borrows a slice of bytes of the input document.
//!
//! Choose this implementation if:
//!
//! 1. You already have the data loaded in-memory and can borrow it while
//!    using the engine.
//!
//! ## Performance characteristics
//!
//! This type of input is the fastest to process for the engine,
//! since there is no additional overhead from loading anything to memory.
//! It is on par with [`OwnedBytes`](`super::OwnedBytes`), but doesn't take ownership
//! of the bytes.

use super::{
    align_to,
    error::Infallible,
    padding::{EndPaddedInput, PaddedBlock, TwoSidesPaddedInput},
    Input, InputBlockIterator, SeekableBackwardsInput, SliceSeekable, MAX_BLOCK_SIZE,
};
use crate::{debug, result::InputRecorder};
use rsonpath_syntax::str::JsonString;

/// Input wrapping a borrowed [`[u8]`] buffer.
pub struct BorrowedBytes<'a> {
    middle_bytes: &'a [u8],
    first_block: PaddedBlock,
    last_block: PaddedBlock,
}

/// Iterator over blocks of [`BorrowedBytes`] of size exactly `N`.
pub struct BorrowedBytesBlockIterator<'r, I, R, const N: usize> {
    input: I,
    idx: usize,
    recorder: &'r R,
}

impl<'a> BorrowedBytes<'a> {
    /// Create a new instance of [`BorrowedBytes`] wrapping the given buffer.
    ///
    /// The input will be automatically padded internally, incurring at most
    /// two times [`MAX_BLOCK_SIZE`] of memory overhead.
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

impl<'a> From<&'a [u8]> for BorrowedBytes<'a> {
    #[inline(always)]
    fn from(value: &'a [u8]) -> Self {
        BorrowedBytes::new(value)
    }
}

impl<'a> From<&'a str> for BorrowedBytes<'a> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        BorrowedBytes::new(value.as_bytes())
    }
}

impl<'a, 'r, I, R, const N: usize> BorrowedBytesBlockIterator<'r, I, R, N>
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
        }
    }
}

impl Input for BorrowedBytes<'_> {
    type BlockIterator<'b, 'r, R, const N: usize> = BorrowedBytesBlockIterator<'r, TwoSidesPaddedInput<'b>, R, N>
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
        }
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, Infallible> {
        return Ok(
            if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() + MAX_BLOCK_SIZE {
                match self.middle_bytes.seek_forward(from - MAX_BLOCK_SIZE, needles) {
                    Some((x, y)) => Some((x + MAX_BLOCK_SIZE, y)),
                    None => handle_last(&self.last_block, MAX_BLOCK_SIZE + self.middle_bytes.len(), needles),
                }
            } else {
                self.as_padded_input().seek_forward(from, needles)
            },
        );

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
            if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() + MAX_BLOCK_SIZE {
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

    #[inline(always)]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> Result<bool, Self::Error> {
        debug_assert!(from < to);
        // The hot path is when we're checking fully within the middle section.
        // This has to be as fast as possible, so the "cold" path referring to the TwoSidesPaddedInput
        // impl is explicitly marked with #[cold].
        if from > MAX_BLOCK_SIZE && to < self.middle_bytes.len() + MAX_BLOCK_SIZE {
            // This is the hot path -- do the bounds check and memcmp.
            let bytes = self.middle_bytes;
            let from = from - MAX_BLOCK_SIZE;
            let to = to - MAX_BLOCK_SIZE;
            let slice = &bytes[from..to];
            Ok(member.quoted().as_bytes() == slice && (from == 0 || bytes[from - 1] != b'\\'))
        } else {
            // This is a very expensive, cold path.
            Ok(self.as_padded_input().is_member_match(from, to, member))
        }
    }
}

impl SeekableBackwardsInput for BorrowedBytes<'_> {
    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        return if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() + MAX_BLOCK_SIZE {
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
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        return if from >= MAX_BLOCK_SIZE && from < self.middle_bytes.len() + MAX_BLOCK_SIZE {
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
}

impl<'a, 'r, R, const N: usize> InputBlockIterator<'a, N>
    for BorrowedBytesBlockIterator<'r, TwoSidesPaddedInput<'a>, R, N>
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
            // SAFETY: Bounds check above.
            // self.idx >= MBS => start >= 0, and self.idx < middle.len + MBS => self.idx < middle.len
            // By construction, middle has length divisible by N.
            let block = unsafe { self.input.middle().get_unchecked(start..start + N) };
            self.recorder.record_block_start(block);
            self.idx += N;
            Ok(Some(block))
        } else {
            Ok(cold_path(self))
        };

        #[cold]
        fn cold_path<'a, 'r, R, const N: usize>(
            iter: &mut BorrowedBytesBlockIterator<'r, TwoSidesPaddedInput<'a>, R, N>,
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

impl<'a, 'r, R, const N: usize> InputBlockIterator<'a, N> for BorrowedBytesBlockIterator<'r, EndPaddedInput<'a>, R, N>
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
            // SAFETY: Bounds check above.
            // self.idx >= MBS => start >= 0, and self.idx < middle.len + MBS => self.idx < middle.len
            // By construction, middle has length divisible by N.
            let block = unsafe { self.input.middle().get_unchecked(start..start + N) };
            self.recorder.record_block_start(block);
            self.idx += N;
            Ok(Some(block))
        } else {
            Ok(cold_path(self))
        };

        #[cold]
        fn cold_path<'a, 'r, R, const N: usize>(
            iter: &mut BorrowedBytesBlockIterator<'r, EndPaddedInput<'a>, R, N>,
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
