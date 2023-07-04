//! Takes ownership of the input data.
//!
//! Choose this implementation if:
//!
//! 1. You already have the data loaded in-memory, but it is not properly
//! aligned, and its size is relatively small, so reallocating it is acceptable &ndash; use the
//! [`new`](`OwnedBytes::new`) function for this.
//!
//! ## Performance characteristics
//!
//! Runtime performance is the same as for [`BorrowedBytes`]. The overhead comes from
//! the input construction.
//!
//! For data of small length (around a megabyte) full copy is going to be faster still
//! than using a buffered input stream.

use super::{borrowed::BorrowedBytesBlockIterator, error::InputError, *};
use crate::query::JsonString;
use std::{alloc, ptr, slice};

/// Input into a query engine.
pub struct OwnedBytes {
    bytes_ptr: ptr::NonNull<u8>,
    len: usize,
    capacity: usize,
    last_block: LastBlock,
}

impl OwnedBytes {
    /// Finalize the initialization of bytes by computing the last_block
    /// and producing the final instance.
    ///
    /// # Safety:
    /// - `ptr` must represent an initialized block of bytes of length `cap`
    /// - `len` <= cap
    unsafe fn finalize_new(ptr: ptr::NonNull<u8>, len: usize, cap: usize) -> Self {
        let slice = slice::from_raw_parts(ptr.as_ptr(), len);
        let last_block = in_slice::pad_last_block(slice);

        Self {
            bytes_ptr: ptr,
            len,
            capacity: cap,
            last_block,
        }
    }

    /// Get a reference to the bytes as a slice.
    #[must_use]
    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: Pointer is not null and its validity is an internal invariant.
        unsafe { slice::from_raw_parts(self.bytes_ptr.as_ptr(), self.len) }
    }

    /// Convert to [`BorrowedBytes`].
    #[must_use]
    #[inline(always)]
    pub fn as_borrowed(&self) -> BorrowedBytes {
        // SAFETY: we satisfy both invariants of length divisible by MAX_BLOCK_SIZE
        // and alignment to MAX_BLOCK_SIZE.
        unsafe { BorrowedBytes::new(self.as_slice()) }
    }

    /// Create an instance of [`OwnedBytes`] from raw pointers.
    ///
    /// # Safety
    /// This is highly unsafe, and requires similar invariants as stdlib's [`Vec`].
    ///
    /// - `ptr` must have been allocated using the global allocator;
    /// - `ptr` must have been allocated with size equal to `size`;
    /// - first `size` values that `ptr` points to must be correctly initialized
    /// instances of [`u8`].
    /// - the `size` must be not more than [`isize::MAX`];
    /// - `ptr` must be aligned to the [`MAX_BLOCK_SIZE`] boundary;
    /// - `size` must be divisible by [`MAX_BLOCK_SIZE`].
    ///
    /// Ownership of `ptr` is transferred to the resulting instance. You must ensure
    /// there are no other mutable references to its contents and that the memory
    /// is not deallocated.
    #[must_use]
    #[inline(always)]
    pub unsafe fn from_raw_parts(ptr: ptr::NonNull<u8>, size: usize) -> Self {
        Self::finalize_new(ptr, size, size)
    }

    /// Copy a buffer of bytes and create a proper [`OwnedBytes`] instance.
    ///
    /// The contents of the buffer will be copied and might be padded to
    /// the [`MAX_BLOCK_SIZE`] boundary.
    ///
    /// # Errors
    /// If the length of the buffer plus
    /// the padding exceeds the system limit of [`isize::MAX`], an [`InputError::AllocationSizeExceeded`]
    /// error will be raised.
    #[inline]
    pub fn new<T: AsRef<[u8]>>(src: &T) -> Result<Self, InputError> {
        let slice = src.as_ref();
        let rem = slice.len() % MAX_BLOCK_SIZE;
        let pad = if rem == 0 { 0 } else { MAX_BLOCK_SIZE - rem };
        let size = slice.len() + pad;

        if size == 0 {
            // SAFETY: For len and cap 0 the dangling ptr always works.
            return Ok(unsafe { Self::finalize_new(ptr::NonNull::dangling(), 0, 0) });
        }

        // Size overflow check happens in get_layout.
        let layout = Self::get_layout(size)?;

        // SAFETY:
        // Layout is guaranteed to be of non-zero size at this point.
        let raw_ptr = unsafe { alloc::alloc(layout) };
        let ptr = ptr::NonNull::new(raw_ptr).unwrap_or_else(|| alloc::handle_alloc_error(layout));

        // SAFETY:
        unsafe {
            ptr::copy_nonoverlapping(slice.as_ptr(), ptr.as_ptr(), slice.len());
            ptr::write_bytes(ptr.as_ptr().add(slice.len()), 0, pad);
        };

        // SAFETY: At this point we allocated and initialized exactly `size` bytes.
        Ok(unsafe { Self::finalize_new(ptr, size, size) })
    }

    /// Create a new instance of [`OwnedBytes`] from a buffer satisfying
    /// all invariants.
    ///
    /// # Safety
    /// The invariants are assumed, not checked. You must ensure that the
    /// buffer passed to this function:
    /// - has length not exceeding the system cap of [isize::MAX];
    /// - is aligned to the [`MAX_BLOCK_SIZE`] boundary;
    /// - has length divisible by [`MAX_BLOCK_SIZE`].
    #[inline]
    #[must_use]
    pub unsafe fn new_unchecked<T: AsRef<[u8]>>(src: &T) -> Self {
        let slice = src.as_ref();
        let size = slice.len();

        if size == 0 {
            return Self::finalize_new(ptr::NonNull::dangling(), 0, 0);
        }

        let layout = Self::get_layout(size).unwrap_unchecked();
        let raw_ptr = alloc::alloc(layout);
        let ptr = ptr::NonNull::new(raw_ptr).unwrap_or_else(|| alloc::handle_alloc_error(layout));
        ptr::copy_nonoverlapping(slice.as_ptr(), ptr.as_ptr(), slice.len());

        Self::finalize_new(ptr, size, size)
    }

    #[inline(always)]
    fn get_layout(size: usize) -> Result<alloc::Layout, InputError> {
        alloc::Layout::from_size_align(size, MAX_BLOCK_SIZE).map_err(|_err| InputError::AllocationSizeExceeded)
    }
}

impl TryFrom<String> for OwnedBytes {
    type Error = InputError;

    #[inline]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.into_bytes())
    }
}

impl TryFrom<Vec<u8>> for OwnedBytes {
    type Error = InputError;

    #[inline]
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl TryFrom<&[u8]> for OwnedBytes {
    type Error = InputError;

    #[inline(always)]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::new(&value)
    }
}

impl<'a> From<&BorrowedBytes<'a>> for OwnedBytes {
    #[inline(always)]
    fn from(value: &BorrowedBytes) -> Self {
        // SAFETY: BorrowedBytes satisfies all preconditions by its invariants.
        unsafe { Self::new_unchecked(value) }
    }
}

impl TryFrom<&str> for OwnedBytes {
    type Error = InputError;

    #[inline(always)]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.as_bytes())
    }
}

impl Drop for OwnedBytes {
    #[inline]
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }

        // This should never happen and if it did it would cause a memory leak.
        #[allow(clippy::expect_used)]
        let layout = Self::get_layout(self.capacity).expect("layout for existing OwnedBytes must never change");

        // SAFETY:
        // `ptr` is allocated in `new` and layout is constructed using the same function
        // and size.
        // This relies on self.capacity not being mutated ever.
        unsafe { alloc::dealloc(self.bytes_ptr.as_ptr(), layout) }
    }
}

impl Input for OwnedBytes {
    type BlockIterator<'a, 'r, const N: usize, R: InputRecorder + 'r> = BorrowedBytesBlockIterator<'a, 'r, N, R>;

    type Block<'a, const N: usize> = &'a [u8];

    #[inline(always)]
    fn iter_blocks<'a, 'r, R: InputRecorder, const N: usize>(
        &'a self,
        recorder: &'r R,
    ) -> Self::BlockIterator<'a, 'r, N, R> {
        BorrowedBytesBlockIterator::new(&self.as_slice(), &self.last_block, recorder)
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        in_slice::seek_backward(self.as_slice(), from, needle)
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, InputError> {
        Ok(in_slice::seek_forward(self.as_slice(), from, needles))
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, InputError> {
        Ok(in_slice::seek_non_whitespace_forward(self.as_slice(), from))
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        in_slice::seek_non_whitespace_backward(self.as_slice(), from)
    }

    #[inline]
    fn find_member(&self, from: usize, label: &JsonString) -> Result<Option<usize>, InputError> {
        Ok(in_slice::find_member(self.as_slice(), from, label))
    }

    #[inline]
    fn is_member_match(&self, from: usize, to: usize, label: &JsonString) -> bool {
        in_slice::is_member_match(self.as_slice(), from, to, label)
    }
}
