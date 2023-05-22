//! Input structures that can be fed into an [`Engine`](crate::engine::Engine).
//!
//! The engine itself is generic in the [`Input`] trait declared here.
//! There are a couple of different built-in implementations, each
//! suitable for a different scenario.
//!
//! ## [`BorrowedBytes`]
//!
//! Borrows a slice of bytes of the input document. Choose this implementation if:
//!
//! 1. You already have the data loaded in-memory and it is properly padded.
//! The length of the buffer **MUST** be divisible by [`MAX_BLOCK_SIZE`].
//!
//! ### Performance characteristics
//!
//! This type of input is the fastest to process for the engine,
//! since there is no additional overhead from loading anything to memory.
//!
//! ## [`OwnedBytes`]
//!
//! Takes ownership of the input data. Choose this implementation
//! if:
//! 1. You already have the data loaded in-memory, but it is not properly
//! padded, and:
//! a) it is in a [`Vec`] or [`String`] and you can transfer its ownership
//! using one of the [`From`] implementations on [`OwnedBytes`]; OR
//! b) its size is relatively small, so copying it is acceptable &ndash; use the
//! [`new`](`OwnedBytes::new`) function for this.
//!
//! ### Performance characteristics
//!
//! Runtime performance is the same as for [`BorrowedBytes`]. The overhead comes from
//! the input construction.
//!
//! The specialized [`From`] implementations are fast, since they take ownership
//! of the data without copying &ndash; they do pad the data to [`MAX_BLOCK_SIZE`],
//! which might cause a reallocation for a [`Vec`] or [`String`].
//!
//! For data of small length (around a megabyte) full copy is going to be faster still
//! than using a buffered input stream.

pub mod borrowed;
pub mod error;
pub mod owned;
pub use borrowed::BorrowedBytes;
pub use owned::OwnedBytes;

use crate::query::JsonString;
use std::ops::Deref;

/// Shorthand for the associated [`InputBlock`] type for given
/// [`Input`]'s iterator.
///
/// Typing `IBlock<'a, I, N>` is a bit more ergonomic than
/// `<<I as Input>::BlockIterator<'a, N> as InputBlockIterator<'a, N>>::Block`.
pub type IBlock<'a, I, const N: usize> = <<I as Input>::BlockIterator<'a, N> as InputBlockIterator<'a, N>>::Block;

/// Global padding guarantee for all [`Input`] implementations.
/// Iterating over blocks of at most this size is guaranteed
/// to produce only full blocks.
///
/// # Remarks
/// This is set to `128` and unlikely to change.
/// Widest available SIMD is AVX512, which has 64-byte blocks.
/// The engine processes blocks in pairs, thus 128 is the highest possible request made to a block iterator.
/// For this value to change a new, wider SIMD implementation would have to appear.
pub const MAX_BLOCK_SIZE: usize = 128;

/// UTF-8 encoded bytes representing a JSON document that support
/// block-by-block iteration and basic seeking procedures.
pub trait Input: Sized {
    /// Type of the iterator used by [`iter_blocks`](Input::iter_blocks), parameterized
    /// by the lifetime of source input and the size of the block.
    type BlockIterator<'a, const N: usize>: InputBlockIterator<'a, N>
    where
        Self: 'a;

    /// Iterate over blocks of size `N` of the input.
    /// `N` has to be a power of two larger than 1.
    #[must_use]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N>;

    /// Search for an occurrence of `needle` in the input,
    /// starting from `from` and looking back. Returns the index
    /// of the first occurrence or `None` if the `needle` was not found.
    #[must_use]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize>;

    /// Search for the first byte in the input that is not ASCII whitespace
    /// starting from `from`. Returns a pair: the index of first such byte,
    /// and the byte itself; or `None` if no non-whitespace characters
    /// were found.
    #[must_use]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)>;

    /// Search for the first byte in the input that is not ASCII whitespace
    /// starting from `from` and looking back. Returns a pair:
    /// the index of first such byte, and the byte itself;
    /// or `None` if no non-whitespace characters were found.
    #[must_use]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)>;

    /// Search the input for the first occurrence of member name `member`
    /// (comparing bitwise, including double quotes delimiters)
    /// starting from `from`. Returns the index of the first occurrence,
    /// or `None` if no occurrence was found.
    ///
    /// This will also check if the leading double quote is not
    /// escaped by a backslash character, but will ignore any other
    /// structural properties of the input. In particular, the member
    /// might be found at an arbitrary depth.
    #[cfg(feature = "head-skip")]
    #[must_use]
    fn find_member(&self, from: usize, member: &JsonString) -> Option<usize>;

    /// Decide whether the slice of input between `from` (inclusive)
    /// and `to` (exclusive) matches the `member` (comparing bitwise,
    /// including double quotes delimiters).
    ///
    /// This will also check if the leading double quote is not
    /// escaped by a backslash character.
    #[must_use]
    fn is_member_match(&self, from: usize, to: usize, member: &JsonString) -> bool;
}

/// An iterator over blocks of input of size `N`.
/// Implementations MUST guarantee that the blocks returned from `next`
/// are *exactly* of size `N`.
pub trait InputBlockIterator<'a, const N: usize>: Iterator<Item = Self::Block> {
    /// The type of blocks returned.
    type Block: InputBlock<'a, N>;

    /// Offset the iterator by `count` full blocks forward.
    ///
    /// The `count` parameter must be greater than 0.
    fn offset(&mut self, count: isize);
}

/// A block of bytes of size `N` returned from [`InputBlockIterator`].
pub trait InputBlock<'a, const N: usize>: Deref<Target = [u8]> {
    /// Split the block in half, giving two slices of size `N`/2.
    fn halves(&self) -> (&[u8], &[u8]);
}

impl<'a, const N: usize> InputBlock<'a, N> for &'a [u8] {
    #[inline(always)]
    fn halves(&self) -> (&[u8], &[u8]) {
        assert_eq!(N % 2, 0);
        (&self[..N / 2], &self[N / 2..])
    }
}
