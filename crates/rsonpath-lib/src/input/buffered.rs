//! Acquires a [`Read`] instance and reads it in on-demand in a buffer.
//! All of the bytes read are kept in memory.
//!
//! Choose this implementation if:
//!
//! 1. You have a [`Read`] source that might contain relatively large amounts
//!    of data.
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
    error::InputError, repr_align_block_size, Input, InputBlock, InputBlockIterator, SeekableBackwardsInput,
    SliceSeekable, MAX_BLOCK_SIZE,
};
use crate::{error::InternalRsonpathError, result::InputRecorder, JSON_SPACE_BYTE};
use rsonpath_syntax::str::JsonString;
use std::{cell::RefCell, io::Read, ops::Deref, slice};

// The buffer has to be a multiple of MAX_BLOCK_SIZE.
// It could technically be as small as MAX_BLOCK_SIZE, but there is a performance consideration.
// The fewer reads we make, the smoother the pipeline of the engine can go.
// 8KB is too little and hurts performance. 64KB appears to be a good compromise.
const BUF_SIZE: usize = 64 * 1024;

static_assertions::const_assert!(BUF_SIZE >= MAX_BLOCK_SIZE);
static_assertions::const_assert!(BUF_SIZE % MAX_BLOCK_SIZE == 0);

/// Input supporting a buffered read over a [`Read`] implementation.
pub struct BufferedInput<R>(RefCell<InternalBuffer<R>>);

struct InternalBuffer<R> {
    source: R,
    bytes: Vec<BufferedChunk>,
    chunk_idx: usize,
    source_read: usize,
    eof: bool,
}

repr_align_block_size! {
    struct BufferedChunk([u8; BUF_SIZE]);
}

/// Iterator over a [`BufferedInput`].
pub struct BufferedInputBlockIterator<'a, 'r, R, IR, const N: usize> {
    input: &'a BufferedInput<R>,
    idx: usize,
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
            self.bytes.push(BufferedChunk([JSON_SPACE_BYTE; BUF_SIZE]));
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
            self.source_read += size;
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
            source_read: 0,
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
            source_read: 0,
        }))
    }
}

impl<R: Read> Input for BufferedInput<R> {
    type BlockIterator<'a, 'r, IR, const N: usize> = BufferedInputBlockIterator<'a, 'r, R, IR, N>
        where Self: 'a,
              IR: InputRecorder<BufferedInputBlock<N>> + 'r;

    type Error = InputError;
    type Block<'a, const N: usize> = BufferedInputBlock<N> where Self: 'a;

    #[inline(always)]
    fn leading_padding_len(&self) -> usize {
        0
    }

    #[inline(always)]
    fn trailing_padding_len(&self) -> usize {
        let rem = self.0.borrow().source_read % BUF_SIZE;
        if rem == 0 {
            0
        } else {
            BUF_SIZE - rem
        }
    }

    #[inline(always)]
    fn iter_blocks<'i, 'r, IR, const N: usize>(&'i self, recorder: &'r IR) -> Self::BlockIterator<'i, 'r, IR, N>
    where
        IR: InputRecorder<Self::Block<'i, N>>,
    {
        BufferedInputBlockIterator {
            input: self,
            idx: 0,
            recorder,
        }
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, InputError> {
        let mut buf = self.0.borrow_mut();
        let mut moving_from = from;

        loop {
            let res = buf.as_slice().seek_forward(moving_from, needles);

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
            let res = buf.as_slice().seek_non_whitespace_forward(moving_from);

            moving_from = buf.len();

            if res.is_some() {
                return Ok(res);
            } else if !buf.read_more()? {
                return Ok(None);
            }
        }
    }

    #[inline(always)]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> Result<bool, Self::Error> {
        let mut buf = self.0.borrow_mut();

        while buf.len() < to {
            if !buf.read_more()? {
                return Ok(false);
            }
        }

        let bytes = buf.as_slice();
        let slice = &bytes[from..to];
        Ok(member.quoted().as_bytes() == slice && (from == 0 || bytes[from - 1] != b'\\'))
    }
}

impl<R: Read> SeekableBackwardsInput for BufferedInput<R> {
    #[inline(always)]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        let buf = self.0.borrow();
        buf.as_slice().seek_non_whitespace_backward(from)
    }

    #[inline(always)]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let buf = self.0.borrow();
        buf.as_slice().seek_backward(from, needle)
    }
}

impl<'a, R: Read, IR, const N: usize> InputBlockIterator<'a, N> for BufferedInputBlockIterator<'a, '_, R, IR, N>
where
    IR: InputRecorder<BufferedInputBlock<N>>,
{
    type Block = BufferedInputBlock<N>;
    type Error = InputError;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Block>, Self::Error> {
        let buf = self.input.0.borrow();

        if self.idx + N <= buf.len() {
            let slice = &buf.as_slice()[self.idx..self.idx + N];
            let block: [u8; N] = slice
                .try_into()
                .map_err(|err| InternalRsonpathError::from_error(err, "slice of size N is not of size N"))?;
            self.idx += N;

            self.recorder.record_block_start(BufferedInputBlock(block));

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

    #[inline(always)]
    fn offset(&mut self, count: isize) {
        assert!(count >= 0);
        self.idx += count as usize * N;
    }

    #[inline(always)]
    fn get_offset(&self) -> usize {
        self.idx
    }
}

impl<const N: usize> Deref for BufferedInputBlock<N> {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> InputBlock<'_, N> for BufferedInputBlock<N> {
    #[inline(always)]
    fn halves(&self) -> (&[u8], &[u8]) {
        assert_eq!(N % 2, 0);
        (&self[..N / 2], &self[N / 2..])
    }
}
