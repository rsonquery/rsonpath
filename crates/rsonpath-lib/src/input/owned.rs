//! Input sourced from an owned buffer of bytes, without growing.
use super::{
    borrowed::{BorrowedBytes, BorrowedBytesBlockIterator},
    error::InputError,
    Input, MAX_BLOCK_SIZE,
};
use crate::query::JsonString;
use std::{alloc, ptr, slice};

/// Input into a query engine.
pub struct OwnedBytes {
    bytes_ptr: ptr::NonNull<u8>,
    len: usize,
    capacity: usize,
}

impl OwnedBytes {
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
        Self {
            bytes_ptr: ptr,
            len: size,
            capacity: size,
        }
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
            return Ok(Self {
                bytes_ptr: ptr::NonNull::dangling(),
                len: 0,
                capacity: 0,
            });
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

        Ok(Self {
            bytes_ptr: ptr,
            len: size,
            capacity: size,
        })
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
            return Self {
                bytes_ptr: ptr::NonNull::dangling(),
                len: 0,
                capacity: 0,
            };
        }

        let layout = Self::get_layout(size).unwrap_unchecked();
        let raw_ptr = alloc::alloc(layout);
        let ptr = ptr::NonNull::new(raw_ptr).unwrap_or_else(|| alloc::handle_alloc_error(layout));
        ptr::copy_nonoverlapping(slice.as_ptr(), ptr.as_ptr(), slice.len());

        Self {
            bytes_ptr: ptr,
            len: size,
            capacity: size,
        }
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
    type BlockIterator<'a, const N: usize> = BorrowedBytesBlockIterator<'a, N>;

    #[inline(always)]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N> {
        BorrowedBytesBlockIterator::new(self.as_slice())
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        self.as_borrowed().seek_backward(from, needle)
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)> {
        self.as_borrowed().seek_non_whitespace_forward(from)
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        self.as_borrowed().seek_non_whitespace_backward(from)
    }

    #[inline]
    #[cfg(feature = "head-skip")]
    fn find_member(&self, from: usize, member_name: &JsonString) -> Option<usize> {
        self.as_borrowed().find_member(from, member_name)
    }

    #[inline]
    fn is_member_match(&self, from: usize, to: usize, member_name: &JsonString) -> bool {
        self.as_borrowed().is_member_match(from, to, member_name)
    }
}
