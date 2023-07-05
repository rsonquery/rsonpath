//! Acquires a [`Read`](std::io::Read) instance and reads it in on-demand in a buffer.
//! All of the bytes read are kept in memory.
//!
//! Choose this implementation if:
//!
//! 1. You have a [`Read`](std::io::Read) source that might contain relatively large amounts
//! of data.
//! 2. You want to run the JSONPath query on the input and then discard it.
//!
//! ## Performance characteristics
//!
//! This is the best choice for a relatively large read-once input that is not a file
//! (or when memory maps are not supported). It is faster than first reading all of
//! the contents and then passing them to [`BorrowedBytes`](`super::BorrowedBytes`). It is, however,
//! slow compared to other choices. If you know the approximate length of input,
//! use the [`with_capacity`](`BufferedInput::with_capacity`) function to avoid
//! reallocating the internal buffers.

use super::{
    error::InputError, in_slice, repr_align_block_size, Input, InputBlock, InputBlockIterator, MAX_BLOCK_SIZE,
};
use crate::{error::InternalRsonpathError, query::JsonString, result::InputRecorder, FallibleIterator};
use std::cmp;
use std::{cell::RefCell, io::Read, ops::Deref, slice};

const BUF_SIZE: usize = 64 * 1024;

static_assertions::const_assert!(BUF_SIZE >= MAX_BLOCK_SIZE);
static_assertions::const_assert!(BUF_SIZE % MAX_BLOCK_SIZE == 0);

/// Input supporting a buffered read over a [`Read`] implementation.
pub struct BufferedInput<R>(RefCell<InternalBuffer<R>>);

struct InternalBuffer<R> {
    source: R,
    bytes: Vec<BufferedChunk>,
    chunk_idx: usize,
    eof: bool,
}

repr_align_block_size! {
    struct BufferedChunk([u8; BUF_SIZE]);
}

/// Iterator over a [`BufferedInput`].
pub struct BufferedInputBlockIterator<'a, 'r, R, IR: InputRecorder, const N: usize> {
    input: &'a BufferedInput<R>,
    idx: usize,
    current_block: Option<[u8; N]>,
    recorder: &'r IR,
}

/// Block returned from a [`BufferedInputBlockIterator`].
pub struct BufferedInputBlock<const N: usize>([u8; N]);

impl<R: Read> InternalBuffer<R> {
    fn as_slice(&self) -> &[u8] {
        let len = self.len();
        let ptr = self.bytes.as_slice().as_ptr().cast();

        // SAFETY: BufferedChunk has the same layout as an array of bytes due to repr(C).
        // `BUF_SIZE >= MAX_BLOCK_SIZE`, and `BUF_SIZE` is a multiple of `MAX_BLOCK_SIZE`
        // (static asserts at the top), so [BufferedChunk; N] has the same repr as [[u8; BUF_SIZE]; N],
        // which in turn is guaranteed to have the same repr as [u8; BUF_SIZE * N].
        // https://doc.rust-lang.org/reference/type-layout.html#array-layout
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    fn len(&self) -> usize {
        self.chunk_idx * BUF_SIZE
    }

    fn read_more(&mut self) -> Result<bool, InputError> {
        if self.eof {
            return Ok(false);
        }

        if self.chunk_idx == self.bytes.len() {
            self.bytes.push(BufferedChunk([0; BUF_SIZE]));
        }

        let buf = &mut self.bytes[self.chunk_idx].0;
        let mut total = 0;
        self.chunk_idx += 1;

        while total < BUF_SIZE && !self.eof {
            let size = self.source.read(&mut buf[total..])?;

            if size == 0 {
                self.eof = true;
            }

            total += size;
        }

        Ok(total > 0)
    }
}

impl<R: Read> BufferedInput<R> {
    /// Create a new [`BufferedInput`] reading from the given `source`.
    #[inline]
    pub fn new(source: R) -> Self {
        Self(RefCell::new(InternalBuffer {
            source,
            bytes: vec![],
            eof: false,
            chunk_idx: 0,
        }))
    }

    /// Create a new [`BufferedInput`] reading from the given `source`,
    /// preallocating at least `capacity` bytes up front.
    #[inline]
    pub fn with_capacity(source: R, capacity: usize) -> Self {
        let blocks_needed = capacity / MAX_BLOCK_SIZE + 1;
        Self(RefCell::new(InternalBuffer {
            source,
            bytes: Vec::with_capacity(blocks_needed),
            eof: false,
            chunk_idx: 0,
        }))
    }
}

impl<R: Read> Input for BufferedInput<R> {
    type BlockIterator<'a, 'r, const N: usize, IR: InputRecorder + 'r> = BufferedInputBlockIterator<'a, 'r, R, IR, N> where Self: 'a;

    type Block<'a, const N: usize> = BufferedInputBlock<N> where Self: 'a;

    #[inline(always)]
    fn iter_blocks<'a, 'r, IR: InputRecorder, const N: usize>(
        &'a self,
        recorder: &'r IR,
    ) -> Self::BlockIterator<'a, 'r, N, IR> {
        BufferedInputBlockIterator {
            input: self,
            idx: 0,
            current_block: None,
            recorder,
        }
    }

    #[inline(always)]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let buf = self.0.borrow();
        let slice = buf.as_slice();
        in_slice::seek_backward(slice, from, needle)
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, InputError> {
        let mut buf = self.0.borrow_mut();
        let mut moving_from = from;

        loop {
            let res = {
                let slice = buf.as_slice();
                in_slice::seek_forward(slice, moving_from, needles)
            };

            moving_from = buf.len();

            if res.is_some() {
                return Ok(res);
            } else if !buf.read_more()? {
                return Ok(None);
            }
        }
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, InputError> {
        let mut buf = self.0.borrow_mut();
        let mut moving_from = from;

        loop {
            let res = {
                let slice = buf.as_slice();
                in_slice::seek_non_whitespace_forward(slice, moving_from)
            };

            moving_from = buf.len();

            if res.is_some() {
                return Ok(res);
            } else if !buf.read_more()? {
                return Ok(None);
            }
        }
    }

    #[inline(always)]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        let buf = self.0.borrow();
        let slice = buf.as_slice();
        in_slice::seek_non_whitespace_backward(slice, from)
    }

    #[inline]
    fn find_member(&self, from: usize, member: &JsonString) -> Result<Option<usize>, InputError> {
        let mut buf = self.0.borrow_mut();
        let mut moving_from = from;

        loop {
            let res = {
                let slice = buf.as_slice();
                in_slice::find_member(slice, moving_from, member)
            };

            moving_from = cmp::min(from, buf.len().saturating_sub(member.bytes_with_quotes().len() - 1));

            if res.is_some() {
                return Ok(res);
            } else if !buf.read_more()? {
                return Ok(None);
            }
        }
    }

    #[inline(always)]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> bool {
        let buf = self.0.borrow();
        let slice = buf.as_slice();
        in_slice::is_member_match(slice, from, to, member)
    }
}

impl<'a, 'r, R: Read, IR: InputRecorder, const N: usize> FallibleIterator
    for BufferedInputBlockIterator<'a, 'r, R, IR, N>
{
    type Item = BufferedInputBlock<N>;
    type Error = InputError;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(block) = self.current_block.take() {
            self.recorder.record_block_end(&block);
        }

        let buf = self.input.0.borrow();

        if self.idx + N < buf.len() {
            let slice = &buf.as_slice()[self.idx..self.idx + N];
            let block: [u8; N] = slice
                .try_into()
                .map_err(|err| InternalRsonpathError::from_error(err, "slice of size N is not of size N"))?;
            self.idx += N;
            self.current_block = Some(block);

            Ok(Some(BufferedInputBlock(block)))
        } else {
            drop(buf);
            let mut buf_mut = self.input.0.borrow_mut();

            if !buf_mut.read_more()? {
                Ok(None)
            } else {
                drop(buf_mut);
                self.next()
            }
        }
    }
}

impl<'a, 'r, R: Read, IR: InputRecorder, const N: usize> InputBlockIterator<'a, N>
    for BufferedInputBlockIterator<'a, 'r, R, IR, N>
{
    type Block = BufferedInputBlock<N>;

    #[inline(always)]
    fn offset(&mut self, count: isize) {
        assert!(count >= 0);
        self.idx += count as usize * N;
    }
}

impl<const N: usize> Deref for BufferedInputBlock<N> {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, const N: usize> InputBlock<'a, N> for BufferedInputBlock<N> {
    #[inline(always)]
    fn halves(&self) -> (&[u8], &[u8]) {
        assert_eq!(N % 2, 0);
        (&self[..N / 2], &self[N / 2..])
    }
}

impl<'a, 'r, R, IR: InputRecorder, const N: usize> Drop for BufferedInputBlockIterator<'a, 'r, R, IR, N> {
    #[inline]
    fn drop(&mut self) {
        if let Some(block) = self.current_block.take() {
            self.recorder.record_block_end(&block);
        }
    }
}
