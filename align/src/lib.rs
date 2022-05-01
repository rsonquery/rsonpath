#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(
    explicit_outlives_requirements,
    unreachable_pub,
    semicolon_in_expressions_from_macros,
    unused_import_braces,
    single_use_lifetimes,
    unused_lifetimes
)]
#![warn(
    clippy::undocumented_unsafe_blocks,
    clippy::cargo_common_metadata,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::ptr_as_ptr,
    clippy::cloned_instead_of_copied,
    clippy::unreadable_literal,
    clippy::must_use_candidate
)]
// feature(doc_cfg) is nightly (https://doc.rust-lang.org/unstable-book/language-features/doc-cfg.html)
// Since we don't want the entire crate to be nightly, this is enabled only when building documentation.
#![cfg_attr(docsrs, feature(doc_cfg))]

//! Structures providing guarantees on byte sequence alignment.
//!
//! For some crucial data it might be beneficial to align them to page boundaries
//! for better cache performance. This crate uses the [`page_size`](https://crates.io/crates/page_size)
//! crate to get the page size.
//!
//! # Examples
//!
//! ```
//! # use align::{alignment::{self, Alignment}};
//! assert_eq!(page_size::get(), alignment::Page::size());
//! ```
//! ```
//! # use align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let possibly_unaligned = [1, 2, 3];
//! let aligned = AlignedBytes::<alignment::Page>::from(possibly_unaligned);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr.align_offset(page_size::get()), 0);
//! assert_eq!(aligned, possibly_unaligned);
//! ```
//!
//! To create a new aligned block of bytes it's easiest to use [`new_zeroed`](`AlignedBytes::new_zeroed`).
//!
//! ```
//! # use align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let aligned = AlignedBytes::<alignment::Page>::new_zeroed(1024);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr.align_offset(page_size::get()), 0);
//! assert!(aligned.iter().all(|&x| x == 0));
//! ```
//!
//! You can also use [`new`](`AlignedBytes::new`) to possibly skip initialization.
//! This is `unsafe`, since the underlying memory might be uninitialized, but may be useful
//! if you immediately want to initialize the memory afterwards.
//!
//! ```
//! # use align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let mut aligned = unsafe { AlignedBytes::<alignment::Page>::new(1024) };
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr.align_offset(page_size::get()), 0);
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
//! # use align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let aligned = AlignedBytes::<alignment::Page>::new_initialize(8, |i| { i as u8 });
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr.align_offset(page_size::get()), 0);
//! assert_eq!(aligned, [0, 1, 2, 3, 4, 5, 6, 7]);
//! ```
//!
//! ## SIMD
//!
//! Loading block-aligned bytes into SIMD is generally faster than unaligned.
//! The SIMD alignment constructs are enabled with the `simd` default feature.
//!
#![cfg_attr(not(feature = "simd"), doc = "```ignore")]
#![cfg_attr(feature = "simd", doc = "```")]
//!
//! # use align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
//! let possibly_unaligned = [1, 2, 3];
//! let aligned = AlignedBytes::<alignment::SimdBlock>::from(possibly_unaligned);
//! let ptr = aligned.as_ptr();
//!
//! assert_eq!(ptr.align_offset(alignment::SimdBlock::size()), 0);
//! assert_eq!(aligned, possibly_unaligned);
//! ```

pub mod alignment;

use cfg_if::cfg_if;
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
    #[must_use]
    pub unsafe fn new(size: usize) -> Self {
        Self::new_impl(size)
    }

    // Extracted so that this fn isn't all in an `unsafe` context by default.
    fn new_impl(size: usize) -> Self {
        if size == 0 {
            return Self::default();
        }

        if size > (isize::MAX as usize) {
            panic!("cannot allocate more than `isize::MAX` bytes, attempted to allocate {size}");
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
    /// # use align::{Aligned, AlignedBytes, alignment::{self, Alignment}};
    /// let aligned = AlignedBytes::<alignment::Page>::new_initialize(8, |i| { (i % 2) as u8 });
    /// let ptr = aligned.as_ptr();
    ///
    /// assert_eq!(ptr as usize % alignment::Page::size(), 0);
    /// assert_eq!(aligned, [0, 1, 0, 1, 0, 1, 0, 1]);
    /// ```
    pub fn new_initialize<F>(size: usize, f: F) -> Self
    where
        F: Fn(usize) -> u8,
    {
        // SAFETY:
        // All bytes are initialized right after.
        let mut block = unsafe { Self::new(size) };

        for i in 0..block.size {
            block[i] = f(i);
        }

        block
    }

    /// Create new block of bytes of given length and initialize
    /// to all-zeroes.
    /// # Panics
    /// If allocating memory fails, i.e. internal call to [`std::alloc::alloc_zeroed`] panics.
    #[must_use]
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

    /// Create a new block of bytes by copying the given bytes
    /// and padding them with zeroes, so that the total size is
    /// divisible by the alignment size.
    ///
    /// This is primarily useful to guarantee that [`AlignedBlockIterator`]
    /// returns full blocks of size exactly equal to the alignment,
    /// as otherwise the final block can be potentially smaller.
    #[must_use]
    pub fn new_padded(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Self::default();
        }

        let size = bytes.len();
        let padding = if size % A::size() == 0 {
            0
        } else {
            A::size() - size % A::size()
        };
        let padded_size = size + padding;

        let mut aligned = Self::new_zeroed(padded_size);
        aligned[..size].copy_from_slice(bytes);

        aligned
    }

    /// Return the size of the alignment in bytes.
    #[must_use]
    pub fn alignment_size() -> usize {
        A::size()
    }
}

impl<A: Alignment> AlignedSlice<A> {
    /// Returns the slice offset by `count` aligned blocks.
    /// This is equivalent to skipping `count * A::size()` bytes.
    ///
    /// # Panics
    /// If there are less than `count` blocks until end of the slice.
    #[must_use]
    pub fn offset(&self, count: isize) -> &Self {
        let offset_in_bytes = A::size() * (count as usize);

        if self.bytes.len() < offset_in_bytes {
            panic!(
                "offset {count} out of range for AlignedSlice of {} aligned blocks",
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

    /// Return an iterator over consecutive aligned blocks of the slice.
    #[must_use]
    pub fn iter_blocks(&self) -> AlignedBlockIterator<A> {
        AlignedBlockIterator { bytes: self }
    }

    /// Relax the alignment to a smaller one.
    ///
    /// # Panics
    /// If `B::size()` > `A::size()`.
    #[must_use]
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
    fn deref_mut<'a>(&'a mut self) -> &'a mut AlignedSlice<A> {
        // SAFETY:
        // 1. All the conditions for from_raw_parts_mut:
        //   > `data` must be valid for reads for `len * mem::size_of::<T>()` many bytes, and it must be properly aligned.
        //   - `T` is `u8` and we allocated `len` bytes in AlignedBytes' ctors. Proper alignment for `u8` is 1, trivially satisfied.
        //   > `data` must point to `len` consecutive properly initialized values of type `T`.
        //   - This is upheld by AlignedBytes' constructors.
        //   > The memory referenced by the returned slice must not be accessed through any other pointer
        //   > (not derived from the return value) for the duration of lifetime `'a`. Both read and write accesses are forbidden.
        //   - This follows from the explicit lifetimes given. To call deref_mut we mutably borrow the AlignedBytes for 'a,
        //     and return a mutable borrow of a slice valid for 'a. Because of borrow rules, this can be the only valid mutable
        //     reference to the underlying bytes.
        //   > The total size len * mem::size_of::<T>() of the slice must be no larger than isize::MAX. See the safety documentation of pointer::offset.
        //   - This is asserted in AlignedBytes' ctor.
        // 2. transmute is safe because of AlignedSlice's repr(transparent).
        unsafe {
            let slice: &'a mut [u8] =
                std::slice::from_raw_parts_mut(self.bytes_ptr.as_ptr(), self.size);
            std::mem::transmute(slice)
        }
    }
}

impl<A: Alignment> Deref for AlignedSlice<A> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        // SAFETY:
        // Using AlignedSlice's repr(transparent).
        unsafe { std::mem::transmute(self) }
    }
}

impl<A: Alignment> DerefMut for AlignedSlice<A> {
    fn deref_mut(&mut self) -> &mut [u8] {
        // SAFETY:
        // Using AlignedSlice's repr(transparent).
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
        // SAFETY:
        // Using AlignedSlice's repr(transparent).
        unsafe { std::mem::transmute(default_slice) }
    }
}

impl<A: Alignment> Default for &mut AlignedSlice<A> {
    fn default() -> Self {
        let default_slice: &mut [u8] = Default::default();
        // SAFETY:
        // Using AlignedSlice's repr(transparent).
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
// TODO: Implement IntoIterator for AlignedBytes and an Iterator for AlignedSlice that iterates over aligned blocks.

/// Thin wrapper that represents an [`AlignedSlice`] of size at most the alignment size.
///
/// # Safety
/// Similarly to [`AlignedSlice`], the used `repr` is [`transparent`](https://doc.rust-lang.org/reference/type-layout.html#the-transparent-representation),
/// and it is possible to directly [`std::mem::transmute`] an [`AlignedSlice<A>`] into an [`AlignedBlock<A>`] (and vice-versa).
/// This is only safe if the size of the the slice is at most [`A::size()`](`alignment::Alignment::size`).
#[repr(transparent)]
pub struct AlignedBlock<A: alignment::Alignment> {
    slice: AlignedSlice<A>,
}

/// Iterator over [`AlignedBlocks`](`AlignedBlock`) of a given aligned bytes span.
pub struct AlignedBlockIterator<'a, A: alignment::Alignment> {
    bytes: &'a AlignedSlice<A>,
}

impl<A: alignment::Alignment> Deref for AlignedBlock<A> {
    type Target = AlignedSlice<A>;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // repr(transparent) and the requirements for AlignedSlice are
        // a subset of those of AlignedBlock
        unsafe { mem::transmute(self) }
    }
}

impl<A: alignment::Alignment> AlignedBlock<A> {
    /// Returns the length of the block. Guaranteed to be `A::size()`.
    #[must_use]
    pub fn len(&self) -> usize {
        self.slice.len()
    }

    /// Returns whether the block is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slice.is_empty()
    }
}

impl<'a, A: alignment::Alignment> Iterator for AlignedBlockIterator<'a, A> {
    type Item = &'a AlignedBlock<A>;

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

impl<A: alignment::Alignment> ExactSizeIterator for AlignedBlockIterator<'_, A> {}

impl<A: alignment::Alignment> Empty for AlignedBlockIterator<'_, A> {
    fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

impl<A: alignment::Alignment> FusedIterator for AlignedBlockIterator<'_, A> {}

cfg_if! {
    if #[cfg(feature = "simd")] {
        mod simd;
        pub use simd::*;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
