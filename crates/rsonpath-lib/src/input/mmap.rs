//! Uses [`Mmap`](memmap2) to map a file into memory with kernel support.
//!
//! Choose this implementation if:
//!
//! 1. Your platform supports memory maps.
//! 2. The input data is in a file or comes from standard input:
//!   a) if from a file, then you can guarantee that the file is not going to be modified
//!      in or out of process while the input is alive;
//!   b) if from stdin, then that the input lives in memory (for example comes via a pipe);
//!      input from a tty is not memory-mappable.
//!
//! ## Performance characteristics
//!
//! A memory map is by far the fastest way to process a file. For some queries it is faster
//! by an order of magnitude to execute the query on a memory map than it is to simply read the
//! file into main memory.

use super::{
    borrowed::BorrowedBytesBlockIterator,
    error::{Infallible, InputError},
    padding::PaddedBlock,
    Input, SliceSeekable, MAX_BLOCK_SIZE,
};
use crate::{input::padding::EndPaddedInput, query::JsonString, result::InputRecorder};
use memmap2::{Mmap, MmapAsRawDesc};

/// Input wrapping a memory mapped file.
pub struct MmapInput {
    mmap: Mmap,
    last_block_start: usize,
    last_block: PaddedBlock,
}

impl MmapInput {
    /// Map a file to memory.
    ///
    /// # Safety
    ///
    /// This operation is inherently unsafe, since the file can be modified
    /// in or out of process. See [Mmap documentation](https://docs.rs/memmap2/latest/memmap2/struct.Mmap.html).
    ///
    /// # Errors
    ///
    /// Calling mmap might result in an IO error.
    #[inline]
    pub unsafe fn map_file<D: MmapAsRawDesc>(file_desc: D) -> Result<Self, InputError> {
        match Mmap::map(file_desc) {
            Ok(mmap) => {
                let last_block_start = (mmap.len() / MAX_BLOCK_SIZE) * MAX_BLOCK_SIZE;
                let last_block = PaddedBlock::pad_last_block(&mmap[last_block_start..]);
                Ok(Self {
                    mmap,
                    last_block_start,
                    last_block,
                })
            }
            Err(err) => Err(err.into()),
        }
    }

    pub(super) fn as_padded_input(&self) -> EndPaddedInput {
        let middle = &self.mmap.as_ref()[..self.last_block_start];
        EndPaddedInput::new(middle, &self.last_block)
    }
}

impl Input for MmapInput {
    type BlockIterator<'a, 'r, R, const N: usize> = BorrowedBytesBlockIterator<'a, 'r, EndPaddedInput<'a>, R, N>
    where
        R: InputRecorder<&'a [u8]> + 'r;

    type Error = Infallible;
    type Block<'a, const N: usize> = &'a [u8];

    #[inline(always)]
    fn leading_padding_len(&self) -> usize {
        0
    }

    #[inline(always)]
    fn trailing_padding_len(&self) -> usize {
        self.last_block.padding_len()
    }

    #[inline(always)]
    fn len_hint(&self) -> Option<usize> {
        Some((self.mmap.len() / MAX_BLOCK_SIZE + 1) * MAX_BLOCK_SIZE)
    }

    #[inline(always)]
    fn iter_blocks<'a, 'r, R, const N: usize>(&'a self, recorder: &'r R) -> Self::BlockIterator<'a, 'r, R, N>
    where
        R: InputRecorder<&'a [u8]>,
    {
        let padded_input = EndPaddedInput::new(&self.mmap[..self.last_block_start], &self.last_block);

        BorrowedBytesBlockIterator::new(padded_input, recorder)
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        self.as_padded_input().seek_backward(from, needle)
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, Infallible> {
        Ok(self.as_padded_input().seek_forward(from, needles))
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, Infallible> {
        Ok(self.as_padded_input().seek_non_whitespace_forward(from))
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        self.as_padded_input().seek_non_whitespace_backward(from)
    }

    #[inline]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> Result<bool, Self::Error> {
        Ok(self.as_padded_input().is_member_match(from, to, member))
    }
}
