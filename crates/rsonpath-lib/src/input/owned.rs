use super::{
    borrowed::{BorrowedBytes, BorrowedBytesBlockIterator},
    Input,
};
use crate::query::Label;
use std::{alloc, ptr, slice, mem};

/// Input into a query engine.
pub struct OwnedBytes {
    bytes_ptr: ptr::NonNull<u8>,
    len: usize,
    capacity: usize,
}

impl OwnedBytes {
    const ALIGNMENT: usize = 8 * 1024;

    #[must_use]
    #[inline(always)]
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: Pointer is not null and its validity is an internal invariant.
        unsafe { slice::from_raw_parts(self.bytes_ptr.as_ptr(), self.len) }
    }

    #[must_use]
    #[inline(always)]
    pub fn as_borrowed(&self) -> BorrowedBytes {
        BorrowedBytes::new(self.as_slice())
    }

    #[inline(always)]
    fn get_layout(size: usize) -> alloc::Layout {
        alloc::Layout::from_size_align(size, Self::ALIGNMENT).unwrap()
    }

    #[must_use]
    #[inline(always)]
    pub unsafe fn from_raw_parts(ptr: ptr::NonNull<u8>, size: usize) -> Self {
        Self {
            bytes_ptr: ptr,
            len: size,
            capacity: size,
        }
    }

    /// Transmute a buffer into an input.
    #[must_use]
    #[inline]
    pub fn new<T: AsRef<[u8]>>(src: &T) -> Self {
        let slice = src.as_ref();
        let rem = slice.len() % Self::ALIGNMENT;
        let pad = if rem == 0 { 0 } else { Self::ALIGNMENT - rem };
        let size = slice.len() + pad;

        if size == 0 {
            return Self {
                bytes_ptr: ptr::NonNull::dangling(),
                len: 0,
                capacity: 0,
            };
        }

        let layout = Self::get_layout(size);

        // SAFETY:
        // Layout is guaranteed to be of non-zero size at this point.
        let raw_ptr = unsafe { alloc::alloc(layout) };
        let ptr = ptr::NonNull::new(raw_ptr).unwrap_or_else(|| alloc::handle_alloc_error(layout));

        // SAFETY:
        unsafe {
            ptr::copy_nonoverlapping(slice.as_ptr(), ptr.as_ptr(), slice.len());
            ptr::write_bytes(
                ptr.as_ptr().offset(isize::try_from(slice.len()).unwrap()),
                0,
                pad,
            );
        };

        Self {
            bytes_ptr: ptr,
            len: size,
            capacity: size,
        }
    }
}

impl From<String> for OwnedBytes {
    #[inline]
    #[must_use]
    fn from(mut value: String) -> Self {
        let rem = value.len() % Self::ALIGNMENT;
        let pad = if rem == 0 { 0 } else { Self::ALIGNMENT - rem };
        for _ in 0..pad {
            value.push('\0');
        }

        let ptr = ptr::NonNull::new(value.as_mut_ptr()).unwrap();
        let len = value.len();
        let capacity = value.capacity();

        mem::forget(value);

        Self {
            bytes_ptr: ptr,
            len,
            capacity
        }
    }
}

impl From<Vec<u8>> for OwnedBytes {
    fn from(mut value: Vec<u8>) -> Self {   
        let rem = value.len() % Self::ALIGNMENT;
        let pad = if rem == 0 { 0 } else { Self::ALIGNMENT - rem };
        for _ in 0..pad {
            value.push(0);
        }

        let ptr = ptr::NonNull::new(value.as_mut_ptr()).unwrap();
        let len = value.len();
        let capacity = value.capacity();

        mem::forget(value);

        Self {
            bytes_ptr: ptr,
            len,
            capacity
        }
    }
}

impl<T: AsRef<[u8]>> From<&T> for OwnedBytes {
    fn from(value: &T) -> Self {
        Self::new(value)
    }
}

impl From<&str> for OwnedBytes {
    fn from(value: &str) -> Self {
        Self::from(&value.as_bytes())
    }
}

impl Drop for OwnedBytes {
    #[inline]
    fn drop(&mut self) {
        if self.len == 0 {
            return;
        }

        let layout = Self::get_layout(self.capacity);

        // SAFETY:
        // `ptr` is allocated in `new` and layout is constructed using the same function
        // and size.
        // This relies on self.size not being mutated ever.
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
    fn find_label(&self, from: usize, label: &Label) -> Option<usize> {
        self.as_borrowed().find_label(from, label)
    }

    #[inline]
    fn is_label_match(&self, from: usize, to: usize, label: &Label) -> bool {
        self.as_borrowed().is_label_match(from, to, label)
    }
}
