//! Structures providing guarantees on byte sequence alignment.
//!
//! Loading block-aligned bytes into SIMD is generally faster than unaligned.
//! For some crucial data it might be beneficial to align them to page boundaries
//! for better cache performance.
//!
//! # Examples
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment};
//! let possibly_unaligned = [1, 2, 3];
//! let aligned = AlignedBytes::<alignment::Block>::from(possibly_unaligned);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % alignment::BLOCK_SIZE, 0);
//! assert_eq!(aligned, possibly_unaligned);
//! ```
//!
//! Using the [`page_size`](https://crates.io/crates/page_size) crate to get the page size.
//!
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment};
//! let possibly_unaligned = [1, 2, 3];
//! let aligned = AlignedBytes::<alignment::Page>::from(possibly_unaligned);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % page_size::get(), 0);
//! assert_eq!(aligned, possibly_unaligned);
//! ```
//!
//! If not created from existing bytes, new [`AlignedBytes`]
//! contains garbage memory.
//!
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment};
//! let aligned = AlignedBytes::<alignment::Page>::new(1024);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % page_size::get(), 0);
//! // We cannot assert anything else, `aligned` can contain arbitrary bytes.
//! ```
//!
//! For zero-initialisation use `new_zeroed`.
//!
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment};
//! let aligned = AlignedBytes::<alignment::Page>::new_zeroed(1024);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % page_size::get(), 0);
//! assert!(aligned.iter().all(|&x| x == 0));
//! ```

use static_assertions as sa;
use std::mem;
use std::ops::{Deref, DerefMut};

/// Common trait for [`AlignedBytes`] for all different alignments.
pub trait Aligned {
    /// Create new uninitialized block of bytes of given length.
    fn new(len: usize) -> Self;

    /// Create new block of bytes of given length and initialize
    /// to all-zeroes.
    fn new_zeroed(len: usize) -> Self;
}

/// Bytes aligned to a boundary represented by `A`.
///
/// # Guarantees
///
/// It is guaranteed that the bytes allocated in this structure are aligned
/// to an [`A::size()`](`alignment::Alignment::size`) byte boundary. Therefore the integer representation
/// of the pointer obtained by the [`as_ptr`](`std::slice::[]::as_ptr`) (or
/// [`as_mut_ptr`](`std::slice::[]::as_mut_ptr`)) will be divisible by
/// [`A::size()`](`alignment::Alignment::size`).
///
/// # Notes
///
/// The only safe implementations are for the alignment markers in [`alignment`]
/// provided by this crate.
#[derive(Debug)]
pub struct AlignedBytes<A: alignment::Alignment> {
    vec: Vec<u8>,
    a: std::marker::PhantomData<A>,
}

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
))]
#[repr(C, align(32))]
struct BlockAlignment {
    x: [u8; 32],
}

#[cfg(not(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature = "avx2"
)))]
#[repr(C, align(1))]
struct BlockAlignment {
    x: [u8; 1],
}

#[repr(C, align(1024))]
struct MBAlignment {
    x: [u8; 1024],
}

#[repr(C, align(2048))]
struct TwoMBAlignment {
    x: [u8; 2048],
}

#[repr(C, align(4096))]
struct FourMBAlignment {
    x: [u8; 4096],
}

#[repr(C, align(8192))]
struct EightMBAlignment {
    x: [u8; 8192],
}

sa::const_assert!(mem::size_of::<MBAlignment>() % mem::size_of::<BlockAlignment>() == 0);

/// Types of possible alignment type arguments for [`AlignedBytes`](`AlignedBytes`).
pub mod alignment {
    /// Size of the [`Block`] alignment.
    pub const BLOCK_SIZE: usize = std::mem::size_of::<super::BlockAlignment>();

    /// Trait for all alignment types that provides its size.
    pub trait Alignment {
        /// Size of the alignment.
        fn size() -> usize;
    }

    /// Alignment to a SIMD block guarantee.
    ///
    /// It is guaranteed that this alignment's [`size`] is a multiplicity
    /// of the size of a SIMD register of the target architecture.
    /// In `nosimd` feature mode the size is one.
    ///
    /// # Alignments
    ///
    /// The alignment size will be the first entry in the below table
    /// that is supported by the target CPU.
    ///
    /// | CPU feature     | Alignment (bytes) |
    /// |-----------------|------------------:|
    /// | AVX2            | 32                |
    /// | none (`nosimd`) | 1                 |
    #[derive(Debug)]
    pub struct Block {}

    /// Alignment to page boundary.
    ///
    /// Size is the size of a single page in the OS as returned by the
    /// [`page_size`] crate.
    #[derive(Debug)]
    pub struct Page {}

    impl Alignment for Block {
        fn size() -> usize {
            std::mem::size_of::<super::BlockAlignment>()
        }
    }

    impl Alignment for Page {
        fn size() -> usize {
            page_size::get()
        }
    }
}

use alignment::Alignment;

impl<_T: Alignment> AlignedBytes<_T> {
    unsafe fn new_for<T>(len: usize, write_zeros: bool) -> Self {
        let intermediate_capacity = (len + mem::size_of::<T>() - 1) / mem::size_of::<T>();

        let mut aligned: Vec<T> = Vec::with_capacity(intermediate_capacity);

        let ptr = aligned.as_mut_ptr() as *mut u8;
        let capacity = aligned.capacity() * mem::size_of::<T>();

        mem::forget(aligned);

        if write_zeros {
            std::ptr::write_bytes(ptr, 0, capacity);
        }

        let vec = Vec::from_raw_parts(ptr, len, capacity);

        Self {
            vec,
            a: std::marker::PhantomData {},
        }
    }
}

impl Aligned for AlignedBytes<alignment::Block> {
    fn new(len: usize) -> Self {
        unsafe { Self::new_for::<BlockAlignment>(len, false) }
    }

    fn new_zeroed(len: usize) -> Self {
        unsafe { Self::new_for::<BlockAlignment>(len, true) }
    }
}

impl AlignedBytes<alignment::Page> {
    fn new_internal(len: usize, write_zeros: bool) -> Self {
        match page_size::get() {
            1024 => unsafe { Self::new_for::<MBAlignment>(len, write_zeros) },
            2048 => unsafe { Self::new_for::<TwoMBAlignment>(len, write_zeros) },
            4096 => unsafe { Self::new_for::<FourMBAlignment>(len, write_zeros) },
            8192 => unsafe { Self::new_for::<EightMBAlignment>(len, write_zeros) },
            _ => unimplemented!(
                "Unimplemented page size for alignment: {}",
                page_size::get()
            ),
        }
    }
}

impl Aligned for AlignedBytes<alignment::Page> {
    fn new(len: usize) -> Self {
        Self::new_internal(len, false)
    }

    fn new_zeroed(len: usize) -> Self {
        Self::new_internal(len, true)
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

impl From<AlignedBytes<alignment::Page>> for AlignedBytes<alignment::Block> {
    fn from(s: AlignedBytes<alignment::Page>) -> Self {
        sa::assert_eq_size!(
            AlignedBytes::<alignment::Page>,
            AlignedBytes::<alignment::Block>,
        );
        debug_assert!(alignment::Page::size() % alignment::Block::size() == 0);

        unsafe { mem::transmute(s) }
    }
}

impl<_T: Alignment> Deref for AlignedBytes<_T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.vec.deref()
    }
}

impl<_T: Alignment> DerefMut for AlignedBytes<_T> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.vec.deref_mut()
    }
}

impl<A: Alignment, B: Alignment> PartialEq<AlignedBytes<B>> for AlignedBytes<A> {
    fn eq(&self, other: &AlignedBytes<B>) -> bool {
        self.vec == other.vec
    }
}

impl<_T: Alignment> PartialEq<AlignedBytes<_T>> for [u8] {
    fn eq(&self, other: &AlignedBytes<_T>) -> bool {
        self == other.vec
    }
}

impl<_T: Alignment> PartialEq<AlignedBytes<_T>> for &[u8] {
    fn eq(&self, other: &AlignedBytes<_T>) -> bool {
        *self == other.vec
    }
}

impl<_T: Alignment> PartialEq<[u8]> for AlignedBytes<_T> {
    fn eq(&self, other: &[u8]) -> bool {
        self.vec == other
    }
}

impl<_T: Alignment, const N: usize> PartialEq<[u8; N]> for AlignedBytes<_T> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.vec == other
    }
}

impl<_T: Alignment> PartialEq<&[u8]> for AlignedBytes<_T> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.vec == *other
    }
}

impl<_T: Alignment> PartialEq<Vec<u8>> for AlignedBytes<_T> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.vec == *other
    }
}

impl<_T: Alignment> PartialEq<AlignedBytes<_T>> for Vec<u8> {
    fn eq(&self, other: &AlignedBytes<_T>) -> bool {
        *self == other.vec
    }
}

impl<A: Alignment> Eq for AlignedBytes<A> {}

impl<A: Alignment, B: Alignment> PartialOrd<AlignedBytes<B>> for AlignedBytes<A> {
    fn partial_cmp(&self, other: &AlignedBytes<B>) -> Option<std::cmp::Ordering> {
        self.vec.partial_cmp(&other.vec)
    }
}

impl<A: Alignment> Ord for AlignedBytes<A> {
    fn cmp(&self, other: &AlignedBytes<A>) -> std::cmp::Ordering {
        self.vec.cmp(&other.vec)
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

    #[test]
    fn block_alignment_from_page_alignment_is_identity() {
        let slice = (0..=47).collect::<Vec<u8>>();
        let page_aligned = AlignedBytes::<alignment::Page>::from(&slice);
        let block_aligned: AlignedBytes<alignment::Block> = page_aligned.into();

        assert_eq!(block_aligned, slice);
    }
}
