//! Input structures that can be fed into an [`Engine`](crate::engine::Engine).
//!
//! The engine itself is generic in the [`Input`] trait declared here.
//! There are a couple of different built-in implementations, each
//! suitable for a different scenario. Consult the module-level
//! documentation of each type to determine which to use. Here's a quick
//! cheat-sheet:
//!
//! | Input scenario                | Type to use       |
//! |:------------------------------|:------------------|
//! | memory based                  | [`BorrowedBytes`] |
//! | memory based, take ownership  | [`OwnedBytes`]    |
//! | file based                    | [`MmapInput`]     |
//! | [`Read`](std::io::Read) based | [`BufferedInput`] |

pub mod borrowed;
pub mod buffered;
pub mod error;
pub mod mmap;
pub mod owned;
mod padding;
mod slice;
pub use borrowed::BorrowedBytes;
pub use buffered::BufferedInput;
pub use mmap::MmapInput;
pub use owned::OwnedBytes;

use self::error::InputError;
use crate::{
    classification::simd::Simd,
    result::InputRecorder,
    string_pattern::{matcher::StringPatternMatcher, StringPattern},
};
use std::ops::Deref;

/// Make the struct repr(C) with alignment equal to [`MAX_BLOCK_SIZE`].
macro_rules! repr_align_block_size {
    ($it:item) => {
        #[repr(C, align(128))]
        $it
    };
}
pub(crate) use repr_align_block_size;

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
    type BlockIterator<'i, 'r, R, const N: usize>: InputBlockIterator<
        'i,
        N,
        Block = Self::Block<'i, N>,
        Error = Self::Error,
    >
    where
        Self: 'i,
        R: InputRecorder<Self::Block<'i, N>> + 'r;

    /// Type of errors that can occur when operating on this [`Input`].
    type Error: Into<InputError>;

    /// Type of the blocks returned by the `BlockIterator`.
    type Block<'i, const N: usize>: InputBlock<'i, N>
    where
        Self: 'i;

    /// Return the length of the entire input, if known.
    ///
    /// This is meant to be used merely as a hint.
    /// There are [`Input`] implementations that may not be able to know the entire
    /// length a priori, and they should return [`None`].
    #[inline(always)]
    #[must_use]
    fn len_hint(&self) -> Option<usize> {
        None
    }

    /// Return the length of the padding added at the start of the input.
    ///
    /// This depends on the particular [`Input`] implementation, and may be zero.
    /// In any case the length of the entire input should be equivalent to the length
    /// of the source plus [`leading_padding_len`](`Input::leading_padding_len`) plus
    /// [`trailing_padding_len`](`Input::trailing_padding_len`).
    #[must_use]
    fn leading_padding_len(&self) -> usize;

    /// Return the length of the padding added at the end of the input.
    ///
    /// This depends on the particular [`Input`] implementation, and may be zero.
    /// In any case the length of the entire input should be equivalent to the length
    /// of the source plus [`leading_padding_len`](`Input::leading_padding_len`) plus
    /// [`trailing_padding_len`](`Input::trailing_padding_len`).
    #[must_use]
    fn trailing_padding_len(&self) -> usize;

    /// Iterate over blocks of size `N` of the input.
    /// `N` has to be a power of two larger than 1.
    #[must_use]
    fn iter_blocks<'i, 'r, R, const N: usize>(&'i self, recorder: &'r R) -> Self::BlockIterator<'i, 'r, R, N>
    where
        R: InputRecorder<Self::Block<'i, N>>;

    /// Search for an occurrence of `needle` in the input,
    /// starting from `from` and looking back. Returns the index
    /// of the first occurrence or `None` if the `needle` was not found.
    #[must_use]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize>;

    /// Search for an occurrence of any of the `needles` in the input,
    /// starting from `from` and looking forward. Returns the index
    /// of the first occurrence and the needle found, or `None` if none of the `needles` were not found.
    ///
    /// # Errors
    /// This function can read more data from the input if no relevant characters are found
    /// in the current buffer, which can fail.
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, Self::Error>;

    /// Search for the first byte in the input that is not ASCII whitespace
    /// starting from `from`. Returns a pair: the index of first such byte,
    /// and the byte itself; or `None` if no non-whitespace characters
    /// were found.
    ///
    /// # Errors
    /// This function can read more data from the input if no relevant characters are found
    /// in the current buffer, which can fail.
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, Self::Error>;

    /// Search for the first byte in the input that is not ASCII whitespace
    /// starting from `from` and looking back. Returns a pair:
    /// the index of first such byte, and the byte itself;
    /// or `None` if no non-whitespace characters were found.
    #[must_use]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)>;

    /// Decide whether the slice of input between `from` (inclusive)
    /// and `to` (exclusive) matches the string `pattern`.
    ///
    /// This will also check if the leading double quote is not
    /// escaped by a backslash character.
    ///
    /// Comparison is done based on the `pattern`'s data and MUST be a proper JSON string
    /// comparison as defined in the [JSONPath spec](https://www.rfc-editor.org/rfc/rfc9535#name-semantics-3).
    ///
    /// # Errors
    /// This function can read more data from the input if `to` falls beyond
    /// the range that was already read, and the read operation can fail.

    fn pattern_match_from<M: StringPatternMatcher>(
        &self,
        from: usize,
        pattern: &StringPattern,
    ) -> Result<Option<usize>, Self::Error>;

    fn pattern_match_to<M: StringPatternMatcher>(
        &self,
        to: usize,
        pattern: &StringPattern,
    ) -> Result<Option<usize>, Self::Error>;
}

/// An iterator over blocks of input of size `N`.
/// Implementations MUST guarantee that the blocks returned from `next`
/// are *exactly* of size `N`.
pub trait InputBlockIterator<'i, const N: usize> {
    /// The type of blocks returned.
    type Block: InputBlock<'i, N>;

    /// Type of errors that can occur when reading from this iterator.
    type Error: Into<InputError>;

    /// Advances the iterator and returns the next value.
    ///
    /// # Errors
    /// May fail depending on the implementation.
    fn next(&mut self) -> Result<Option<Self::Block>, Self::Error>;

    /// Get the offset of the iterator in the input.
    ///
    /// The offset is the starting point of the block that will be returned next
    /// from this iterator, if any. It starts as 0 and increases by `N` on every
    /// block retrieved.
    fn get_offset(&self) -> usize;

    /// Offset the iterator by `count` full blocks forward.
    ///
    /// The `count` parameter must be greater than 0.
    fn offset(&mut self, count: isize);
}

/// A block of bytes of size `N` returned from [`InputBlockIterator`].
pub trait InputBlock<'i, const N: usize>: Deref<Target = [u8]> {
    /// Split the block in half, giving two slices of size `N`/2.
    #[must_use]
    fn halves(&self) -> (&[u8], &[u8]);

    /// Split the block in four, giving four slices of size `N`/4.
    #[inline]
    #[must_use]
    fn quarters(&self) -> (&[u8], &[u8], &[u8], &[u8]) {
        assert_eq!(N % 4, 0);
        let (half1, half2) = self.halves();
        let (q1, q2) = (&half1[..N / 4], &half1[N / 4..]);
        let (q3, q4) = (&half2[..N / 4], &half2[N / 4..]);

        (q1, q2, q3, q4)
    }
}

impl<'i, const N: usize> InputBlock<'i, N> for &'i [u8] {
    #[inline(always)]
    fn halves(&self) -> (&[u8], &[u8]) {
        assert_eq!(N % 2, 0);
        (&self[..N / 2], &self[N / 2..])
    }
}

pub(super) trait SliceSeekable {
    fn pattern_match_from<M: StringPatternMatcher>(&self, from: usize, pattern: &StringPattern) -> Option<usize>;

    fn pattern_match_to<M: StringPatternMatcher>(&self, to: usize, pattern: &StringPattern) -> Option<usize>;

    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize>;

    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)>;

    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)>;

    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)>;
}

// This is mostly adapted from [slice::align_to](https://doc.rust-lang.org/std/primitive.slice.html#method.align_to).
fn align_to<const N: usize>(bytes: &[u8]) -> (&[u8], &[u8], &[u8]) {
    let ptr = bytes.as_ptr();
    let offset = ptr.align_offset(N);
    if offset > bytes.len() {
        (bytes, &[], &[])
    } else {
        let (left, rest) = bytes.split_at(offset);
        let middle_len = (rest.len() / N) * N;
        let (middle, right) = rest.split_at(middle_len);

        (left, middle, right)
    }
}

#[cfg(test)]
mod tests {
    use super::align_to;
    use crate::input::MAX_BLOCK_SIZE;

    // Run all tests for the actual alignment we use.
    const N: usize = MAX_BLOCK_SIZE;
    const SIZE: usize = 1024;

    #[repr(align(128))]
    struct Aligned {
        bytes: [u8; SIZE],
    }

    #[test]
    fn test_all_alignments() {
        // We construct a byte array that is already aligned,
        // and then take all suffixes for all possible misalignments
        // and small sizes.
        let aligned = Aligned { bytes: get_bytes() };
        let slice = &aligned.bytes;

        for i in 0..slice.len() {
            let misalignment = i % N;
            test_with_misalignment(misalignment, &slice[i..]);
        }
    }

    fn get_bytes() -> [u8; SIZE] {
        let mut bytes = [0; SIZE];

        for (i, b) in bytes.iter_mut().enumerate() {
            let x = i % (u8::MAX as usize);
            *b = x as u8;
        }

        bytes
    }

    fn test_with_misalignment(misalignment: usize, slice: &[u8]) {
        let expected_left_len = (N - misalignment) % N;
        let expected_rem_len = slice.len() - expected_left_len;
        let expected_middle_len = (expected_rem_len / N) * N;
        let expected_right_len = expected_rem_len - expected_middle_len;

        let (left, middle, right) = align_to::<N>(slice);
        let glued_back: Vec<_> = [left, middle, right].into_iter().flatten().copied().collect();

        assert_eq!(left.len(), expected_left_len, "misalignment = {misalignment}");
        assert_eq!(middle.len(), expected_middle_len, "misalignment = {misalignment}");
        assert_eq!(right.len(), expected_right_len, "misalignment = {misalignment}");
        assert_eq!(glued_back, slice, "misalignment = {misalignment}");
    }
}
