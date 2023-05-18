//! Input sourced from a borrowed buffer.
use super::*;

/// Input wrapping a borrowed [`[u8]`] buffer.
pub struct BorrowedBytes<'a> {
    bytes: &'a [u8],
}

/// Iterator over blocks of [`BorrowedBytes`] of size exactly `N`.
pub struct BorrowedBytesBlockIterator<'a, const N: usize> {
    input: &'a [u8],
    idx: usize,
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
        Self { bytes }
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

impl<'a, const N: usize> BorrowedBytesBlockIterator<'a, N> {
    #[must_use]
    #[inline(always)]
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self {
            input: bytes,
            idx: 0,
        }
    }
}

impl<'a> Input for BorrowedBytes<'a> {
    type BlockIterator<'b, const N: usize> = BorrowedBytesBlockIterator<'b, N> where Self: 'b;

    #[inline(always)]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N> {
        Self::BlockIterator {
            input: self.bytes,
            idx: 0,
        }
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        in_slice::seek_backward(self.bytes, from, needle)
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)> {
        in_slice::seek_non_whitespace_forward(self.bytes, from)
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        in_slice::seek_non_whitespace_backward(self.bytes, from)
    }

    #[inline]
    #[cfg(feature = "head-skip")]
    fn find_label(&self, from: usize, label: &Label) -> Option<usize> {
        in_slice::find_label(self.bytes, from, label)
    }

    #[inline]
    fn is_label_match(&self, from: usize, to: usize, label: &Label) -> bool {
        in_slice::is_label_match(self.bytes, from, to, label)
    }
}

impl<'a, const N: usize> Iterator for BorrowedBytesBlockIterator<'a, N> {
    type Item = &'a [u8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.input.len() {
            None
        } else {
            let block = &self.input[self.idx..self.idx + N];
            self.idx += N;

            Some(block)
        }
    }
}

impl<'a, const N: usize> InputBlockIterator<'a, N> for BorrowedBytesBlockIterator<'a, N> {
    type Block = Self::Item;

    #[inline(always)]
    fn offset(&mut self, count: isize) {
        assert!(count >= 0);
        self.idx += count as usize * N;
    }
}
