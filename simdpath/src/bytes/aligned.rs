use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct AlignedBytes<A> {
    vec: Vec<u8>,
    a: std::marker::PhantomData<A>,
}

impl<A, B> PartialEq<AlignedBytes<B>> for AlignedBytes<A> {
    fn eq(&self, other: &AlignedBytes<B>) -> bool {
        self.vec == other.vec
    }
}

impl<_T> PartialEq<AlignedBytes<_T>> for [u8] {
    fn eq(&self, other: &AlignedBytes<_T>) -> bool {
        self == other.vec
    }
}

impl<_T> PartialEq<AlignedBytes<_T>> for &[u8] {
    fn eq(&self, other: &AlignedBytes<_T>) -> bool {
        *self == other.vec
    }
}

impl<_T> PartialEq<[u8]> for AlignedBytes<_T> {
    fn eq(&self, other: &[u8]) -> bool {
        self.vec == other
    }
}

impl<_T> PartialEq<&[u8]> for AlignedBytes<_T> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.vec == *other
    }
}

impl<_T> PartialEq<Vec<u8>> for AlignedBytes<_T> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.vec == *other
    }
}

impl<_T> PartialEq<AlignedBytes<_T>> for Vec<u8> {
    fn eq(&self, other: &AlignedBytes<_T>) -> bool {
        *self == other.vec
    }
}

impl<A> Eq for AlignedBytes<A> {}

impl<A, B> PartialOrd<AlignedBytes<B>> for AlignedBytes<A> {
    fn partial_cmp(&self, other: &AlignedBytes<B>) -> Option<std::cmp::Ordering> {
        self.vec.partial_cmp(&other.vec)
    }
}

impl<A> Ord for AlignedBytes<A> {
    fn cmp(&self, other: &AlignedBytes<A>) -> std::cmp::Ordering {
        self.vec.cmp(&other.vec)
    }
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
mod block_align {
    #[repr(C, align(32))]
    pub struct BlockAlignment {
        x: [u8; 32],
    }
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
)))]
mod block_align {
    #[repr(C, align(1))]
    pub struct BlockAlignment {
        x: [u8; 1],
    }
}

use block_align::*;

#[repr(C, align(1024))]
pub struct MBAlignment {
    x: [u8; 1024],
}

#[repr(C, align(2048))]
pub struct TwoMBAlignment {
    x: [u8; 2048],
}

#[repr(C, align(4096))]
pub struct FourMBAlignment {
    x: [u8; 4096],
}

#[repr(C, align(8192))]
pub struct EightMBAlignment {
    x: [u8; 8192],
}

pub mod alignment {
    #[derive(Debug)]
    pub struct Block {}
    #[derive(Debug)]
    pub struct Page {}
}

impl<_T> AlignedBytes<_T> {
    unsafe fn new_for<T>(len: usize) -> Self {
        let intermediate_capacity = (len + mem::size_of::<T>() - 1) / mem::size_of::<T>();

        let mut aligned: Vec<T> = Vec::with_capacity(intermediate_capacity);

        let ptr = aligned.as_mut_ptr() as *mut u8;
        let capacity = aligned.capacity() * mem::size_of::<T>();

        mem::forget(aligned);

        let vec = Vec::from_raw_parts(ptr, len, capacity);

        Self {
            vec,
            a: std::marker::PhantomData {},
        }
    }
}

impl AlignedBytes<alignment::Block> {
    pub fn new(len: usize) -> Self {
        unsafe { Self::new_for::<BlockAlignment>(len) }
    }
}

impl AlignedBytes<alignment::Page> {
    pub fn new(len: usize) -> Self {
        match page_size::get() {
            1024 => unsafe { Self::new_for::<MBAlignment>(len) },
            2048 => unsafe { Self::new_for::<TwoMBAlignment>(len) },
            4096 => unsafe { Self::new_for::<FourMBAlignment>(len) },
            8192 => unsafe { Self::new_for::<EightMBAlignment>(len) },
            _ => unimplemented!(
                "Unimplemented page size for alignment: {}",
                page_size::get()
            ),
        }
    }
}

impl<_T> Deref for AlignedBytes<_T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.vec.deref()
    }
}

impl<_T> DerefMut for AlignedBytes<_T> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.vec.deref_mut()
    }
}

impl<T: AsRef<[u8]>> From<T> for AlignedBytes<alignment::Block> {
    fn from(s: T) -> Self {
        let slice = s.as_ref();
        let mut bytes = Self::new(slice.len());
        bytes.copy_from_slice(slice);
        bytes
    }
}

impl<T: AsRef<[u8]>> From<T> for AlignedBytes<alignment::Page> {
    fn from(s: T) -> Self {
        let slice = s.as_ref();
        let mut bytes = Self::new(slice.len());
        bytes.copy_from_slice(slice);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_block_aligned_when_created_from_unaligned_slice() {
        let alignment_size = mem::size_of::<BlockAlignment>();
        let slice: &[u8] = &std::iter::repeat(42)
            .take(alignment_size)
            .collect::<Vec<_>>();
        let misalignment = slice.as_ptr() as usize % alignment_size;
        let source = if misalignment > 0 { slice } else { &slice[1..] };
        let bytes = AlignedBytes::<alignment::Block>::from(source);

        assert_eq!(bytes.as_ptr() as usize % alignment_size, 0);
    }

    #[test]
    fn contains_same_bytes_when_block_aligned_from_slice() {
        let slice = (0..=47).collect::<Vec<u8>>();
        let bytes = AlignedBytes::<alignment::Block>::from(&slice);

        assert_eq!(bytes, slice);
    }

    #[test]
    fn creates_empty_bytes_when_given_zero_length_for_block() {
        let bytes = AlignedBytes::<alignment::Block>::new(0);

        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn is_page_aligned_when_created_from_unaligned_slice() {
        let alignment_size = page_size::get();
        let slice: &[u8] = &std::iter::repeat(42)
            .take(alignment_size)
            .collect::<Vec<_>>();
        let misalignment = slice.as_ptr() as usize % alignment_size;
        let source = if misalignment > 0 { slice } else { &slice[1..] };
        let bytes = AlignedBytes::<alignment::Page>::from(source);

        assert_eq!(bytes.as_ptr() as usize % alignment_size, 0);
    }

    #[test]
    fn contains_same_bytes_when_page_aligned_from_slice() {
        let slice = (0..=47).collect::<Vec<u8>>();
        let bytes = AlignedBytes::<alignment::Page>::from(&slice);

        assert_eq!(bytes, slice);
    }

    #[test]
    fn creates_empty_bytes_when_given_zero_length_for_page() {
        let bytes = AlignedBytes::<alignment::Page>::new(0);

        assert_eq!(bytes.len(), 0);
    }
}
