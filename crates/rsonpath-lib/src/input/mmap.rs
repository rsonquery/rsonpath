use super::{borrowed::BorrowedBytesBlockIterator, error::InputError, in_slice, Input, MAX_BLOCK_SIZE};
use crate::query::JsonString;
use memmap2::Mmap;
use std::fs::File;

pub struct MmapInput {
    mmap: Mmap,
    last_block: [u8; MAX_BLOCK_SIZE],
}

impl MmapInput {
    pub fn map_file(file: &File) -> Result<Self, InputError> {
        match unsafe { Mmap::map(file) } {
            Ok(mmap) => {
                let last_block = in_slice::pad_last_block(&mmap);
                Ok(Self {
                    mmap,
                    last_block,
                })
            }
            Err(err) => Err(err.into()),
        }
    }
}

impl Input for MmapInput {
    type BlockIterator<'a, const N: usize> = BorrowedBytesBlockIterator<'a, N>;

    #[inline(always)]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N> {
        BorrowedBytesBlockIterator::new(&self.mmap, &self.last_block)
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        in_slice::seek_backward(&self.mmap, from, needle)
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
    #[cfg(feature = "head-skip")]
    fn find_member(&self, from: usize, label: &JsonString) -> Result<Option<usize>, InputError> {
        Ok(in_slice::find_member(&self.mmap, from, label))
    }

    #[inline]
    fn is_member_match(&self, from: usize, to: usize, label: &JsonString) -> bool {
        in_slice::is_member_match(&self.mmap, from, to, label)
    }
}
