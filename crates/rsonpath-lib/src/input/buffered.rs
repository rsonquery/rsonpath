use super::{error::InputError, in_slice, Input, InputBlock, InputBlockIterator, MAX_BLOCK_SIZE};
use crate::error::InternalRsonpathError;
use crate::repr_align_block_size;
use crate::{query::JsonString, FallibleIterator};
#[cfg(feature = "head-skip")]
use std::cmp;
use std::{cell::RefCell, io::Read, ops::Deref, slice};

const BUF_SIZE: usize = 64 * 1024;

static_assertions::const_assert!(BUF_SIZE >= MAX_BLOCK_SIZE);

pub struct BufferedInput<R>(RefCell<InternalBuffer<R>>);

struct InternalBuffer<R> {
    source: R,
    bytes: Vec<BufferedChunk>,
    eof: bool,
}

repr_align_block_size! {
    struct BufferedChunk([u8; BUF_SIZE]);
}

pub struct BufferedInputBlockIterator<'a, R, const N: usize> {
    input: &'a BufferedInput<R>,
    idx: usize,
}

pub struct BufferedInputBlock<const N: usize>([u8; N]);

impl<R: Read> InternalBuffer<R> {
    fn as_slice(&self) -> &[u8] {
        let len = self.len();
        let ptr = self.bytes.as_slice().as_ptr().cast();

        // SAFETY: BufferedChunk has the same layout as an array of bytes due to repr(C).
        // `BUF_SIZE >= MAX_BLOCK_SIZE` (static assert at the top), so [BufferedChunk; N]
        // has the same repr as [[u8; BUF_SIZE]; N],
        // which in turn is guaranteed to have the same repr as [u8; BUF_SIZE * N].
        // https://doc.rust-lang.org/reference/type-layout.html#array-layout
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    fn len(&self) -> usize {
        self.bytes.len() * BUF_SIZE
    }

    fn read_more(&mut self) -> Result<bool, InputError> {
        if self.eof {
            return Ok(false);
        }

        self.bytes.push(BufferedChunk([0; BUF_SIZE]));
        let buf = &mut self
            .bytes
            .last_mut()
            .ok_or(InternalRsonpathError::from_expectation("empty vec after push"))?
            .0;
        let mut total = 0;

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
    #[inline]
    pub fn new(source: R) -> Self {
        Self(RefCell::new(InternalBuffer {
            source,
            bytes: vec![],
            eof: false,
        }))
    }
}

impl<R: Read> Input for BufferedInput<R> {
    type BlockIterator<'a, const N: usize> = BufferedInputBlockIterator<'a, R, N> where Self: 'a;

    #[inline(always)]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N> {
        BufferedInputBlockIterator { input: self, idx: 0 }
    }

    #[inline(always)]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let buf = self.0.borrow();
        let slice = buf.as_slice();
        in_slice::seek_backward(slice, from, needle)
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

    #[cfg(feature = "head-skip")]
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

impl<'a, R: Read, const N: usize> FallibleIterator for BufferedInputBlockIterator<'a, R, N> {
    type Item = BufferedInputBlock<N>;
    type Error = InputError;

    #[inline]
    fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
        let buf = self.input.0.borrow();

        if self.idx + N < buf.len() {
            let slice = &buf.as_slice()[self.idx..self.idx + N];
            let block: [u8; N] = slice
                .try_into()
                .map_err(|err| InternalRsonpathError::from_error(err, "slice of size N is not of size N"))?;
            self.idx += N;

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

impl<'a, R: Read, const N: usize> InputBlockIterator<'a, N> for BufferedInputBlockIterator<'a, R, N> {
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
