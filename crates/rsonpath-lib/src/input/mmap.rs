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

use super::{borrowed::BorrowedBytesBlockIterator, error::InputError, in_slice, Input, LastBlock};
use crate::{query::JsonString, result::InputRecorder};
use memmap2::{Mmap, MmapAsRawDesc};

/// Input wrapping a memory mapped file.
pub struct MmapInput {
    mmap: Mmap,
    last_block: LastBlock,
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
                let last_block = in_slice::pad_last_block(&mmap);
                Ok(Self { mmap, last_block })
            }
            Err(err) => Err(err.into()),
        }
    }
}

impl Input for MmapInput {
    type BlockIterator<'a, 'r, const N: usize, R: InputRecorder + 'r> = BorrowedBytesBlockIterator<'a, 'r, N, R>;

    type Block<'a, const N: usize> = &'a [u8];

    #[inline(always)]
    fn iter_blocks<'a, 'r, R: InputRecorder, const N: usize>(
        &'a self,
        recorder: &'r R,
    ) -> Self::BlockIterator<'a, 'r, N, R> {
        BorrowedBytesBlockIterator::new(&self.mmap, &self.last_block, recorder)
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        in_slice::seek_backward(&self.mmap, from, needle)
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, InputError> {
        Ok(in_slice::seek_forward(&self.mmap, from, needles))
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, InputError> {
        Ok(in_slice::seek_non_whitespace_forward(&self.mmap, from))
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        in_slice::seek_non_whitespace_backward(&self.mmap, from)
    }

    #[inline]
    fn find_member(&self, from: usize, label: &JsonString) -> Result<Option<usize>, InputError> {
        Ok(in_slice::find_member(&self.mmap, from, label))
    }

    #[inline]
    fn is_member_match(&self, from: usize, to: usize, label: &JsonString) -> bool {
        in_slice::is_member_match(&self.mmap, from, to, label)
    }
}
