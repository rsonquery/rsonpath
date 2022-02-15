//! Structures providing guarantees on byte sequence alignment.
//!
//! Loading block-aligned bytes into SIMD is generally faster than unaligned.
//! For some crucial data it might be beneficial to align them to page boundaries
//! for better cache performance.
//!
//! # Examples
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let possibly_unaligned = [1, 2, 3];
//! let aligned = AlignedBytes::<alignment::Block>::from(possibly_unaligned);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % alignment::Block::size(), 0);
//! assert_eq!(aligned, possibly_unaligned);
//! ```
//!
//! Using the [`page_size`](https://crates.io/crates/page_size) crate to get the page size.
//!
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let possibly_unaligned = [1, 2, 3];
//! let aligned = AlignedBytes::<alignment::Page>::from(possibly_unaligned);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % page_size::get(), 0);
//! assert_eq!(aligned, possibly_unaligned);
//! ```
//!
//! To create a new aligned block of bytes it's easiest to use [`new_zeroed`](`AlignedBytes::new_zeroed`).
//!
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let aligned = AlignedBytes::<alignment::Page>::new_zeroed(1024);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % page_size::get(), 0);
//! assert!(aligned.iter().all(|&x| x == 0));
//! ```
//!
//! You can also use [`new`](`AlignedBytes::new`) to possibly skip initialization.
//! This is `unsafe`, since the underlying memory might be uninitialized, but may be useful
//! if you immediately want to initialize the memory afterwards.
//!
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let mut aligned = unsafe { AlignedBytes::<alignment::Page>::new(1024) };
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % page_size::get(), 0);
//!
//! // We cannot assert anything else, `aligned` can contain arbitrary bytes.
//! // To be able to read anything, we must first initialize.
//!
//! for i in 0..1024 {
//!     aligned[i] = 1;
//! }
//!
//! let ones = std::iter::repeat(1).take(1024).collect::<Vec<u8>>();
//! assert_eq!(ones, aligned);
//!
//! ```
//!
//! If you want a safe way to initialize the bytes, there is [`new_initialize`](`AlignedBytes::new_initialize`)
//! that initializes all bytes with a function of their index.
//!
//! ```
//! # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let aligned = AlignedBytes::<alignment::Block>::new_initialize(8, |i| { i as u8 });
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr as usize % alignment::Block::size(), 0);
//! assert_eq!(aligned, [0, 1, 2, 3, 4, 5, 6, 7]);
//! ```

use std::borrow::{Borrow, BorrowMut};
use std::iter::FusedIterator;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// Common trait for [`AlignedBytes`] for all different alignments.
pub trait Aligned {
    /// Return the size of the alignment in bytes.
    fn alignment_size() -> usize;

    /// Return the slice of the bytes offset by `count` alignment units.
    fn offset(&self, count: isize) -> &Self;
}

/// Bytes aligned to a boundary represented by `A`.
///
/// This type owns the bytes. They are allocated when the struct is created and deallocated
/// on drop.
///
/// # Guarantees
///
/// It is guaranteed that the bytes allocated in this structure are aligned
/// to an [`A::size()`](`alignment::Alignment::size`) byte boundary. Therefore the integer representation
/// of the pointer obtained by the [`as_ptr`](`std::slice::[]::as_ptr`) (or
/// [`as_mut_ptr`](`std::slice::[]::as_mut_ptr`)) will be divisible by
/// [`A::size()`](`alignment::Alignment::size`).
pub struct AlignedBytes<A: alignment::Alignment> {
    bytes_ptr: std::ptr::NonNull<u8>,
    size: usize,
    phantom: std::marker::PhantomData<A>,
}

/// Slice of bytes aligned to a boundary represented by `A`.
///
/// # Guarantees
///
/// It is guaranteed that the bytes allocated in this structure are aligned
/// to an [`A::size()`](`alignment::Alignment::size`) byte boundary. Therefore the integer representation
/// of the pointer obtained by the [`as_ptr`](`std::slice::[]::as_ptr`) (or
/// [`as_mut_ptr`](`std::slice::[]::as_mut_ptr`)) will be divisible by
/// [`A::size()`](`alignment::Alignment::size`).
///
/// # Safety
///
/// Because the used `repr` is [`transparent`](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation),
/// it is possible to directly [`std::mem::transmute`] a [`[u8]`] into an [`AlignedSlice<A>`] (and vice-versa).
/// This is only safe if the original slice is already aligned to [`A::size()`](`alignment::Alignment::size`).
/// Using unaligned bytes in a place that requires alignment is usually undefined behavior.
#[repr(transparent)]
pub struct AlignedSlice<A: alignment::Alignment> {
    phantom: std::marker::PhantomData<A>,
    bytes: [u8],
}

/// Types of possible alignment type arguments for [`AlignedBytes`](`AlignedBytes`).
pub mod alignment {
    use std::sync::Once;

    use cfg_if::cfg_if;

    /// Trait for all alignment types that provides its size.
    ///
    /// # Safety
    /// The `size` returned must satisfy the following conditions:
    /// - it is constant between calls, i.e. two calls to `size` for the same alignment *MUST* return the same value;
    /// - the value returned is a power of two.
    ///
    /// Violating any of these constraints will cause undefined behavior when the alignment is used
    /// for [`AlignedBytes`](`super::AlignedBytes`).
    pub unsafe trait Alignment {
        /// Size of the alignment.
        fn size() -> usize;
    }

    /// Alignment to a SIMD block guarantee.
    ///
    /// It is guaranteed that this alignment's [`size`](`Alignment::size`) is a multiplicity
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

    /// Alignment to two SIMD blocks guarantee.
    ///
    /// This size is always equal to twice the size of [`Block`].
    ///
    /// # Examples
    /// ```rust
    /// use simdpath::bytes::align::alignment::{self, Alignment};
    ///
    /// assert_eq!(2 * alignment::Block::size(), alignment::TwoBlocks::size());
    /// ```
    #[derive(Debug)]
    pub struct TwoBlocks {}

    /// Alignment to page boundary.
    ///
    /// Size is the size of a single page in the OS as returned by the
    /// [`page_size`] crate.
    #[derive(Debug)]
    pub struct Page {}

    // SAFETY:
    // Always returning a const value that is a power of two.
    unsafe impl Alignment for Block {
        fn size() -> usize {
            cfg_if! {
                if #[cfg(feature="nosimd")] {
                    1
                }
                else if #[cfg(all(
                    any(target_arch = "x86", target_arch = "x86_64"),
                    target_feature = "avx2",
                ))] {
                    32
                } else {
                    1
                }
            }
        }
    }

    // SAFETY:
    // Safe as long as the impl for `Block` is safe, since we multiply by 2.
    unsafe impl Alignment for TwoBlocks {
        fn size() -> usize {
            Block::size() * 2
        }
    }

    // SAFETY:
    // We check whether the size is power of two. The [`page_size`] crate caches the result
    // of its call, so it will not change, but I prefer not to rely on an external crate not changing
    // its implementation for safety.
    //
    // No sane platform would have a page size that is not a power of two, but better not to take chances.
    // This assertion will only be called once anyway.
    unsafe impl Alignment for Page {
        fn size() -> usize {
            static INIT: Once = Once::new();
            static mut PAGE_SIZE: usize = 0;

            // SAFETY:
            // Wrapped with `Once`.
            unsafe {
                INIT.call_once(|| {
                    PAGE_SIZE = page_size::get();

                    if PAGE_SIZE == 0 || PAGE_SIZE.next_power_of_two() != PAGE_SIZE {
                        panic!(
                            "detected page size that is not a power of two, this is unsupported"
                        );
                    }
                });
                PAGE_SIZE
            }
        }
    }
}

use alignment::Alignment;
use len_trait::Empty;

impl<A: Alignment> AlignedBytes<A> {
    fn get_layout(size: usize) -> std::alloc::Layout {
        std::alloc::Layout::from_size_align(size, A::size()).unwrap()
    }

    /// Create new, possibly uninitialized, block of bytes of given length.
    ///
    /// # Safety
    /// The memory used by the bytes might not be initialized, which makes reading
    /// from them undefined behavior (yes, [even for `u8` reading uninitialized bytes is UB](https://doc.rust-lang.org/std/mem/union.MaybeUninit.html#initialization-invariant)).
    /// To use the bytes you must first initialize them manually.
    ///
    /// If you want zeroed bytes, use [`AlignedBytes::new_zeroed`] instead.
    /// If you want to initialize the bytes with custom logic, use [`AlignedBytes::new_initialize`] instead.
    /// If you want to align existing bytes, use the [`From`] trait implementations.
    #[inline]
    pub unsafe fn new(size: usize) -> Self {
        Self::new_impl(size)
    }

    // Extracted so that this fn isn't all in an `unsafe` context by default.
    fn new_impl(size: usize) -> Self {
        if size == 0 {
            return Self::default();
        }

        let layout = Self::get_layout(size);

        // SAFETY:
        // Layout is guaranteed to be of non-zero size at this point.
        let raw_ptr = unsafe { std::alloc::alloc(layout) };
        let ptr = std::ptr::NonNull::new(raw_ptr).unwrap();

        Self {
            bytes_ptr: ptr,
            size,
            phantom: std::marker::PhantomData {},
        }
    }

    /// Create new block of bytes of given length and initialize each byte to a function
    /// of its index.
    ///
    /// # Examples
    /// ```rust
    /// # use simdpath::bytes::align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
    /// let aligned = AlignedBytes::<alignment::Block>::new_initialize(8, |i| { (i % 2) as u8 });
    /// let ptr = aligned.as_ptr();
    ///
    /// assert_eq!(ptr as usize % alignment::Block::size(), 0);
    /// assert_eq!(aligned, [0, 1, 0, 1, 0, 1, 0, 1]);
    /// ```
    pub fn new_initialize<F>(size: usize, f: F) -> Self
    where
        F: Fn(usize) -> u8,
    {
        let mut block = unsafe { Self::new(size) };

        for i in 0..block.size {
            block[i] = f(i);
        }

        block
    }

    /// Create new block of bytes of given length and initialize
    /// to all-zeroes.
    pub fn new_zeroed(size: usize) -> Self {
        if size == 0 {
            return Self::default();
        }

        let layout = Self::get_layout(size);

        // SAFETY:
        // Layout is guaranteed to be of non-zero size at this point.
        let raw_ptr = unsafe { std::alloc::alloc_zeroed(layout) };
        let ptr = std::ptr::NonNull::new(raw_ptr).unwrap();

        Self {
            bytes_ptr: ptr,
            size,
            phantom: std::marker::PhantomData {},
        }
    }

    /// Return the size of the alignment in bytes.
    pub fn alignment_size() -> usize {
        A::size()
    }
}

impl<A: Alignment> AlignedSlice<A> {
    /// Returns the slice offset by `count` aligned chunks.
    /// This is equivalent to skipping `count * A::size()` bytes.
    ///
    /// # Panics
    /// If there are less than `count` chunks until end of the slice.
    pub fn offset(&self, count: isize) -> &Self {
        let offset_in_bytes = A::size() * (count as usize);

        if self.bytes.len() < offset_in_bytes {
            panic!(
                "offset {count} out of range for AlignedSlice of {} aligned chunks",
                self.bytes.len() / A::size()
            )
        }

        // SAFETY:
        // - First transmute is safe because of repr(transparent).
        // - The offset_in_bytes is guaranteed to retain alignment, since it is calculated above
        //   as a multiple of A::size() and the slice was aligned at the beginning.
        // - Second transmute is safe because of repr(transparent) and the alignment guarantee
        //   being satisfied as above.
        unsafe {
            let slice: &[u8] = std::mem::transmute(self);
            std::mem::transmute(&slice[offset_in_bytes..])
        }
    }

    /// Return an iterator over consecutive aligned chunks of the slice.
    pub fn iter_chunks(&self) -> AlignedChunkIterator<A> {
        AlignedChunkIterator { bytes: self }
    }

    /// Relax the alignment to a smaller one.
    ///
    /// # Panics
    /// If `B::size()` > `A::size()`.
    pub fn relax_alignment<B: Alignment>(&self) -> &AlignedSlice<B> {
        if A::size() < B::size() {
            panic!("target alignment is larger than source alignment, the 'relax_alignment' conversion is not valid")
        }

        // SAFETY:
        // Since all alignments are multiples of two, A::size() >= B::size() => A::size() % B::size() == 0.
        // The precedent condition is asserted above.
        unsafe { mem::transmute(self) }
    }
}

impl<A: Alignment> Drop for AlignedBytes<A> {
    fn drop(&mut self) {
        use std::alloc::dealloc;

        if self.size == 0 {
            return;
        }

        let layout = Self::get_layout(self.size);

        // SAFETY:
        // `ptr` is allocated in `new_internal` and
        // layout is constructed using the same function and will be the same.
        // This relies on `A::size()` being constant and self.size not being mutated ever.
        unsafe { dealloc(self.bytes_ptr.as_ptr(), layout) }
    }
}

impl<T: AsRef<[u8]>, A: Alignment> From<T> for AlignedBytes<A> {
    fn from(s: T) -> Self {
        let slice = s.as_ref();
        let bytes;

        // SAFETY:
        // Uninitialized `new` is safe since we immediately initialize the bytes with `s`, and `copy` is safe because:
        // - src is valid for reading `slice.len()` bytes.
        // - dst is valid for writing `slice.len()` bytes, since `Self::new` allocates that much
        //   bytes, but aligned.
        // - Both pointers are properly aligned, since proper alignment for `u8` is 1.
        unsafe {
            bytes = Self::new(slice.len());
            std::ptr::copy(slice.as_ptr(), bytes.bytes_ptr.as_ptr(), slice.len())
        };

        bytes
    }
}

impl AsRef<AlignedSlice<alignment::Block>> for AlignedSlice<alignment::Page> {
    fn as_ref(&self) -> &AlignedSlice<alignment::Block> {
        if alignment::Page::size() % alignment::Block::size() != 0 {
            panic!("page alignment is not a multiple of block alignment, the 'as_ref' conversion is not valid")
        }

        // SAFETY:
        // Transmute is safe due to repr(transparent) and the alignment guarantee is upheld
        // due to the above assertion.
        unsafe { mem::transmute(self) }
    }
}

impl<A: Alignment> AsRef<AlignedSlice<A>> for AlignedBytes<A> {
    fn as_ref(&self) -> &AlignedSlice<A> {
        self
    }
}

impl<A: Alignment> AsMut<AlignedSlice<A>> for AlignedBytes<A> {
    fn as_mut(&mut self) -> &mut AlignedSlice<A> {
        self
    }
}

impl<A: Alignment> AsRef<[u8]> for AlignedSlice<A> {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl<A: Alignment> AsMut<[u8]> for AlignedSlice<A> {
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl<A: Alignment> Borrow<AlignedSlice<A>> for AlignedBytes<A> {
    fn borrow(&self) -> &AlignedSlice<A> {
        self
    }
}

impl<A: Alignment> BorrowMut<AlignedSlice<A>> for AlignedBytes<A> {
    fn borrow_mut(&mut self) -> &mut AlignedSlice<A> {
        self
    }
}

impl<A: Alignment> Clone for AlignedBytes<A> {
    fn clone(&self) -> AlignedBytes<A> {
        let slice: &AlignedSlice<A> = self;
        slice.into()
    }

    fn clone_from(&mut self, other: &AlignedBytes<A>) {
        let source: &AlignedSlice<A> = other;
        let target: &mut AlignedSlice<A> = self;

        target.clone_from_slice(source);
    }
}

impl<A: Alignment> Deref for AlignedBytes<A> {
    type Target = AlignedSlice<A>;

    fn deref(&self) -> &AlignedSlice<A> {
        // SAFETY:
        // - the `data` pointer is a `NonNull` pointer to a single allocated object of size exactly `self.size`
        //   and is properly aligned since proper alignment for `u8` is 1;
        // -
        unsafe {
            let slice = std::slice::from_raw_parts(self.bytes_ptr.as_ptr(), self.size);
            std::mem::transmute(slice)
        }
    }
}

impl<A: Alignment> DerefMut for AlignedBytes<A> {
    fn deref_mut(&mut self) -> &mut AlignedSlice<A> {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(self.bytes_ptr.as_ptr(), self.size);
            std::mem::transmute(slice)
        }
    }
}

impl<A: Alignment> Deref for AlignedSlice<A> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<A: Alignment> DerefMut for AlignedSlice<A> {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { std::mem::transmute(self) }
    }
}

impl<A: Alignment> std::fmt::Debug for AlignedBytes<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deref = &**self;
        std::fmt::Debug::fmt(deref, f)
    }
}

impl<A: Alignment> std::fmt::Debug for AlignedSlice<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let deref: &[u8] = self;
        std::fmt::Debug::fmt(deref, f)
    }
}

impl<A: Alignment> Default for AlignedBytes<A> {
    fn default() -> Self {
        Self {
            bytes_ptr: NonNull::dangling(),
            size: 0,
            phantom: std::marker::PhantomData {},
        }
    }
}

impl<A: Alignment> Default for &AlignedSlice<A> {
    fn default() -> Self {
        let default_slice: &[u8] = Default::default();
        unsafe { std::mem::transmute(default_slice) }
    }
}

impl<A: Alignment> Default for &mut AlignedSlice<A> {
    fn default() -> Self {
        let default_slice: &mut [u8] = Default::default();
        unsafe { std::mem::transmute(default_slice) }
    }
}

impl<A: Alignment> std::hash::Hash for AlignedBytes<A> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.bytes_ptr, state)
    }
}

impl<A: Alignment> PartialEq for AlignedSlice<A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let other_slice: &[u8] = other;
        self.eq(other_slice)
    }
}

impl<A: Alignment> PartialEq<&AlignedSlice<A>> for Vec<u8> {
    #[inline]
    fn eq(&self, other: &&AlignedSlice<A>) -> bool {
        let slice: &[u8] = self;
        let other_slice: &[u8] = other;
        slice.eq(other_slice)
    }
}

impl<A: Alignment> PartialEq<Vec<u8>> for &AlignedSlice<A> {
    #[inline]
    fn eq(&self, other: &Vec<u8>) -> bool {
        other.eq(self)
    }
}

impl<A: Alignment> PartialEq<[u8]> for AlignedSlice<A> {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        let slice: &[u8] = self;
        slice.eq(other)
    }
}

impl<A: Alignment> PartialEq<AlignedSlice<A>> for [u8] {
    #[inline]
    fn eq(&self, other: &AlignedSlice<A>) -> bool {
        other.eq(self)
    }
}

impl<A: Alignment, const N: usize> PartialEq<[u8; N]> for AlignedSlice<A> {
    #[inline]
    fn eq(&self, other: &[u8; N]) -> bool {
        let slice: &[u8] = self;
        slice.eq(other)
    }
}

impl<A: Alignment, const N: usize> PartialEq<AlignedSlice<A>> for [u8; N] {
    #[inline]
    fn eq(&self, other: &AlignedSlice<A>) -> bool {
        other.eq(self)
    }
}

impl<A: Alignment> Eq for AlignedSlice<A> {}

impl<A: Alignment> PartialOrd for AlignedSlice<A> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let slice: &[u8] = self;
        let other_slice: &[u8] = other;

        slice.partial_cmp(other_slice)
    }
}

impl<A: Alignment> Ord for AlignedSlice<A> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let slice: &[u8] = self;
        let other_slice: &[u8] = other;

        slice.cmp(other_slice)
    }
}

impl<A: Alignment> PartialEq for AlignedBytes<A> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let slice: &AlignedSlice<A> = self;
        let other_slice: &AlignedSlice<A> = other;

        slice.eq(other_slice)
    }
}

impl<A: Alignment> PartialEq<Vec<u8>> for AlignedBytes<A> {
    #[inline]
    fn eq(&self, other: &Vec<u8>) -> bool {
        let slice: &AlignedSlice<A> = self;
        let other_slice: &[u8] = other;

        slice.eq(other_slice)
    }
}

impl<A: Alignment> PartialEq<AlignedBytes<A>> for Vec<u8> {
    #[inline]
    fn eq(&self, other: &AlignedBytes<A>) -> bool {
        other.eq(self)
    }
}

impl<A: Alignment> PartialEq<[u8]> for AlignedBytes<A> {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        let slice: &AlignedSlice<A> = self;

        slice.eq(other)
    }
}

impl<A: Alignment> PartialEq<AlignedBytes<A>> for [u8] {
    #[inline]
    fn eq(&self, other: &AlignedBytes<A>) -> bool {
        other.eq(self)
    }
}

impl<A: Alignment, const N: usize> PartialEq<[u8; N]> for AlignedBytes<A> {
    #[inline]
    fn eq(&self, other: &[u8; N]) -> bool {
        let slice: &AlignedSlice<A> = self;

        slice.eq(other)
    }
}

impl<A: Alignment, const N: usize> PartialEq<AlignedBytes<A>> for [u8; N] {
    #[inline]
    fn eq(&self, other: &AlignedBytes<A>) -> bool {
        other.eq(self)
    }
}

impl<A: Alignment> Eq for AlignedBytes<A> {}

impl<A: Alignment> PartialOrd for AlignedBytes<A> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let slice: &AlignedSlice<A> = self;
        let other_slice: &AlignedSlice<A> = other;

        slice.partial_cmp(other_slice)
    }
}

impl<A: Alignment> Ord for AlignedBytes<A> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let slice: &AlignedSlice<A> = self;
        let other_slice: &AlignedSlice<A> = other;

        slice.cmp(other_slice)
    }
}

// TODO: Implement indexing?
// TODO: Implement IntoIterator for AlignedBytes and an Iterator for AlignedSlice that iterates over aligned chunks.

/// Thin wrapper that represents an [`AlignedSlice`] of size at most the alignment size.
///
/// # Safety
/// Similarly to [`AlignedSlice`], the used `repr` is [`transparent`](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation),
/// and it is possible to directly [`std::mem::transmute`] an [`AlignedSlice<A>`] into an [`AlignedChunk<A>`] (and vice-versa).
/// This is only safe if the size of the the slice is at most [`A::size()`](`alignment::Alignment::size`).
#[repr(transparent)]
pub struct AlignedChunk<A: alignment::Alignment> {
    slice: AlignedSlice<A>,
}

/// An [`AlignedSlice`] of size _exactly equal to_ the alignment size.
pub enum AlignedBlock<'a, A: alignment::Alignment> {
    /// An aligned block of size `A::size()` located inline in the original
    /// aligned bytes span.
    Inline(&'a AlignedBlockSlice<A>),
    /// An owned block of size `A::size()`.
    Owned(AlignedBlockBytes<A>),
}

/// Thin wrapper for the underlying slice of [`AlignedBlock::Inline`].
///
/// # Safety
/// Similarly to [`AlignedSlice`], the used `repr` is [`transparent`](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation),
/// and it is possible to directly [`std::mem::transmute`] an [`AlignedSlice<A>`] into an [`AlignedBlockSlice<A>`] (and vice-versa).
/// This is only safe if the original slice is already aligned to [`A::size()`](`alignment::Alignment::size`)
/// _and_ the size of the the slice is exactly [`A::size()`](`alignment::Alignment::size`).
#[repr(transparent)]
pub struct AlignedBlockSlice<A: alignment::Alignment> {
    slice: AlignedSlice<A>,
}

/// Thin wrapper for the underlying bytes of [`AlignedBlock::Owned`].
///
/// # Safety
/// Similarly to [`AlignedSlice`], the used `repr` is [`transparent`](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation),
/// and it is possible to directly [`std::mem::transmute`] an [`AlignedBytes<A>`] into an [`AlignedBlockBytes<A>`] (and vice-versa).
/// This is only safe if the original bytes is already aligned to [`A::size()`](`alignment::Alignment::size`)
/// _and_ the size of the the slice is exactly [`A::size()`](`alignment::Alignment::size`).
#[repr(transparent)]
pub struct AlignedBlockBytes<A: alignment::Alignment> {
    buffer: AlignedBytes<A>,
}

/// Iterator over [`AlignedChunks`](`AlignedChunk`) of a given aligned bytes span.
pub struct AlignedChunkIterator<'a, A: alignment::Alignment> {
    bytes: &'a AlignedSlice<A>,
}

/// Iterator over [`AlignedBlocks`](`AlignedBlock`) of a given aligned bytes span.
pub struct AlignedBlockIterator<'a, A: alignment::Alignment> {
    inner_iter: AlignedChunkIterator<'a, A>,
}

impl<A: alignment::Alignment> Deref for AlignedChunk<A> {
    type Target = AlignedSlice<A>;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // repr(transparent) and the requirements for AlignedSlice are
        // a subset of those of AlignedChunk
        unsafe { mem::transmute(self) }
    }
}

impl<A: alignment::Alignment> Deref for AlignedBlockBytes<A> {
    type Target = AlignedSlice<A>;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<A: alignment::Alignment> Deref for AlignedBlockSlice<A> {
    type Target = AlignedSlice<A>;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}

impl<'a, A: alignment::Alignment> Deref for AlignedBlock<'a, A> {
    type Target = AlignedSlice<A>;

    fn deref(&self) -> &Self::Target {
        match self {
            AlignedBlock::Inline(slice) => slice,
            AlignedBlock::Owned(buffer) => buffer,
        }
    }
}

impl<A: alignment::Alignment> AlignedChunk<A> {
    /// Returns the length of the chunk. Guaranteed to be between 0 and `A::size()`, inclusive.
    pub fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns whether the chunk is empty.
    pub fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }
}

impl<'a> AlignedBlock<'a, alignment::TwoBlocks> {
    /// Split the block into two blocks aligned to [`alignment::Block`].
    pub fn blocks<'b>(
        &'b self,
    ) -> (
        AlignedBlock<'b, alignment::Block>,
        AlignedBlock<'b, alignment::Block>,
    ) {
        let slice: &'b AlignedSlice<alignment::TwoBlocks> = self;

        let block1 = unsafe { mem::transmute(&slice[..alignment::Block::size()]) };
        let block2 = unsafe { mem::transmute(&slice[alignment::Block::size()..]) };

        (AlignedBlock::Inline(block1), AlignedBlock::Inline(block2))
    }
}

impl<'a, A: alignment::Alignment> Iterator for AlignedChunkIterator<'a, A> {
    type Item = &'a AlignedChunk<A>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }

        if self.bytes.len() < A::size() {
            // SAFETY:
            // `self.bytes` is aligned to `A` and we checked its size does not exceed `A::size()`.
            let chunk = unsafe { mem::transmute(self.bytes) };
            self.bytes = Default::default();
            return Some(chunk);
        }

        // SAFETY:
        // `self.bytes` is aligned to `A` and we take exactly one block of size `A::size()`.
        let chunk = unsafe { mem::transmute(&self.bytes[..A::size()]) };
        self.bytes = self.bytes.offset(1);

        Some(chunk)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = (self.bytes.len() + A::size() - 1) / A::size();
        (size, Some(size))
    }
}

impl<'a, A: alignment::Alignment> ExactSizeIterator for AlignedChunkIterator<'a, A> {}

impl<'a, A: alignment::Alignment> Empty for AlignedChunkIterator<'a, A> {
    fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl<'a, A: alignment::Alignment> FusedIterator for AlignedChunkIterator<'a, A> {}

impl<'a, A: alignment::Alignment> Iterator for AlignedBlockIterator<'a, A> {
    type Item = AlignedBlock<'a, A>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iter.next() {
            Some(chunk) if chunk.len() < A::size() => {
                let mut buffer = AlignedBytes::<A>::new_zeroed(A::size());
                (&mut buffer[..chunk.len()]).copy_from_slice(chunk);
                let block = unsafe { mem::transmute(buffer) };

                Some(AlignedBlock::Owned(block))
            }
            Some(slice) => Some(AlignedBlock::Inline(unsafe { mem::transmute(slice) })),
            None => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner_iter.size_hint()
    }
}

impl<'a, A: alignment::Alignment> ExactSizeIterator for AlignedBlockIterator<'a, A> {}

impl<'a, A: alignment::Alignment> Empty for AlignedBlockIterator<'a, A> {
    fn is_empty(&self) -> bool {
        Empty::is_empty(&self.inner_iter)
    }
}

impl<'a, A: alignment::Alignment> FusedIterator for AlignedBlockIterator<'a, A> {}

impl<'a, A: alignment::Alignment> AlignedChunkIterator<'a, A> {
    /// Turn the iterator into an iterator of [`AlignedBlock`] by potentially padding the
    /// last chunk so that its size matches the alignment.
    pub fn padded(self) -> AlignedBlockIterator<'a, A> {
        AlignedBlockIterator { inner_iter: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_block_aligned_when_created_from_unaligned_slice() {
        let alignment_size = alignment::Block::size();
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
        let bytes = AlignedBytes::<alignment::Block>::new_zeroed(0);

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
        let bytes = AlignedBytes::<alignment::Page>::new_zeroed(0);

        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn block_alignment_from_page_alignment_is_identity() {
        let slice = (0..=47).collect::<Vec<u8>>();
        let page_aligned: &AlignedSlice<alignment::Page> =
            &AlignedBytes::<alignment::Page>::from(&slice);
        let block_aligned: &AlignedSlice<alignment::Block> = page_aligned.as_ref();

        assert_eq!(block_aligned, slice);
    }
}
