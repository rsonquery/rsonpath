//! Input structures that can be fed into an [`Engine`](crate::engine::Engine).
//!
//! The engine itself is generic in the [`Input`] trait declared here.
//! There are a couple of different built-in implementations, each
//! suitable for a different scenario. Consult the module-level
//! documentation of each type to determine which to use. Here's a quick
//! cheat-sheet:
//!
//! | Input scenario | Type to use    |
//! |:---------------|:---------------|
//! | file based     | [`MmapInput`]  |
//! | memory based   | [`OwnedBytes`] |
//! | memory based, already aligned | [`BorrowedBytes`] |
//! | [`Read`](std::io::Read) based | [`BufferedInput`] |
//!
pub mod borrowed;
pub mod buffered;
pub mod error;
pub mod owned;
pub use borrowed::BorrowedBytes;
pub use buffered::BufferedInput;
pub use owned::OwnedBytes;
pub mod mmap;
pub use mmap::MmapInput;

use self::error::InputError;
use crate::{query::JsonString, recorder::InputRecorder, FallibleIterator};
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
    type BlockIterator<'a, 'r, const N: usize, R: InputRecorder>: InputBlockIterator<'a, N, Block = Self::Block<'a, N>>
    where
        Self: 'a, R: 'r;

    /// Type of the blocks returned by the `BlockIterator`.
    type Block<'a, const N: usize>: InputBlock<'a, N>
    where
        Self: 'a;

    /// Iterate over blocks of size `N` of the input.
    /// `N` has to be a power of two larger than 1.
    #[must_use]
    fn iter_blocks<'a, 'r, R: InputRecorder, const N: usize>(
        &'a self,
        recorder: &'r R,
    ) -> Self::BlockIterator<'a, 'r, N, R>;

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
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Result<Option<(usize, u8)>, InputError>;

    /// Search for the first byte in the input that is not ASCII whitespace
    /// starting from `from`. Returns a pair: the index of first such byte,
    /// and the byte itself; or `None` if no non-whitespace characters
    /// were found.
    ///
    /// # Errors
    /// This function can read more data from the input if no relevant characters are found
    /// in the current buffer, which can fail.
    fn seek_non_whitespace_forward(&self, from: usize) -> Result<Option<(usize, u8)>, InputError>;

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
    ///
    /// # Errors
    /// This function can read more data from the input if no relevant characters are found
    /// in the current buffer, which can fail.
    fn find_member(&self, from: usize, member: &JsonString) -> Result<Option<usize>, InputError>;

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
pub trait InputBlockIterator<'a, const N: usize>: FallibleIterator<Item = Self::Block, Error = InputError> {
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

struct LastBlock {
    bytes: [u8; MAX_BLOCK_SIZE],
    absolute_start: usize,
}

pub(super) mod in_slice {
    use super::{LastBlock, MAX_BLOCK_SIZE};
    use crate::query::JsonString;

    #[inline]
    pub(super) fn pad_last_block(bytes: &[u8]) -> LastBlock {
        let mut last_block_buf = [0; MAX_BLOCK_SIZE];
        let last_block_start = (bytes.len() / MAX_BLOCK_SIZE) * MAX_BLOCK_SIZE;
        let last_block_slice = &bytes[last_block_start..];

        last_block_buf[..last_block_slice.len()].copy_from_slice(last_block_slice);

        LastBlock {
            bytes: last_block_buf,
            absolute_start: last_block_start,
        }
    }

    #[inline]
    pub(super) fn seek_backward(bytes: &[u8], from: usize, needle: u8) -> Option<usize> {
        let mut idx = from;
        assert!(idx < bytes.len());

        loop {
            if bytes[idx] == needle {
                return Some(idx);
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    #[inline]
    pub(super) fn seek_forward<const N: usize>(bytes: &[u8], from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        assert!(N > 0);
        let mut idx = from;

        if idx >= bytes.len() {
            return None;
        }

        loop {
            let b = bytes[idx];
            if needles.contains(&b) {
                return Some((idx, b));
            }
            idx += 1;
            if idx == bytes.len() {
                return None;
            }
        }
    }

    #[inline]
    pub(super) fn seek_non_whitespace_forward(bytes: &[u8], from: usize) -> Option<(usize, u8)> {
        let mut idx = from;

        if idx >= bytes.len() {
            return None;
        }

        loop {
            let b = bytes[idx];
            if !b.is_ascii_whitespace() {
                return Some((idx, b));
            }
            idx += 1;
            if idx == bytes.len() {
                return None;
            }
        }
    }

    #[inline]
    pub(super) fn seek_non_whitespace_backward(bytes: &[u8], from: usize) -> Option<(usize, u8)> {
        let mut idx = from;

        if idx >= bytes.len() {
            return None;
        }

        loop {
            let b = bytes[idx];
            if !b.is_ascii_whitespace() {
                return Some((idx, b));
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    #[inline]
    pub(super) fn find_member(bytes: &[u8], from: usize, member: &JsonString) -> Option<usize> {
        use memchr::memmem;

        let finder = memmem::Finder::new(member.bytes_with_quotes());
        let mut idx = from;

        if bytes.len() <= idx {
            return None;
        }

        loop {
            match finder.find(&bytes[idx..bytes.len()]) {
                Some(offset) => {
                    let starting_quote_idx = offset + idx;
                    if bytes[starting_quote_idx - 1] != b'\\' {
                        return Some(starting_quote_idx);
                    } else {
                        idx = starting_quote_idx + member.bytes_with_quotes().len() + 1;
                    }
                }
                None => return None,
            }
        }
    }

    #[inline]
    pub(super) fn is_member_match(bytes: &[u8], from: usize, to: usize, member: &JsonString) -> bool {
        let slice = &bytes[from..to + 1];
        member.bytes_with_quotes() == slice && (from == 0 || bytes[from - 1] != b'\\')
    }
}

#[cfg(test)]
mod tests {
    use super::{in_slice, MAX_BLOCK_SIZE};

    mod input_block_impl_for_slice {
        use pretty_assertions::assert_eq;

        #[test]
        fn halves_splits_in_half() {
            use super::super::InputBlock;

            let bytes = r#"0123456789abcdef"#.as_bytes();

            let (half1, half2) = <&[u8] as InputBlock<16>>::halves(&bytes);

            assert_eq!(half1, "01234567".as_bytes());
            assert_eq!(half2, "89abcdef".as_bytes());
        }
    }

    mod pad_last_block {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn on_empty_bytes_is_all_zero() {
            let result = in_slice::pad_last_block(&[]);

            assert_eq!(result.absolute_start, 0);
            assert_eq!(result.bytes, [0; MAX_BLOCK_SIZE]);
        }

        #[test]
        fn on_bytes_smaller_than_full_block_gives_entire_block() {
            let bytes = r#"{"test":42}"#.as_bytes();

            let result = in_slice::pad_last_block(bytes);

            assert_eq!(result.absolute_start, 0);
            assert_eq!(&result.bytes[0..11], bytes);
            assert_eq!(&result.bytes[11..], [0; MAX_BLOCK_SIZE - 11]);
        }

        #[test]
        fn on_bytes_equal_to_full_block_gives_all_zero() {
            let bytes = [42; MAX_BLOCK_SIZE];

            let result = in_slice::pad_last_block(&bytes);

            assert_eq!(result.absolute_start, MAX_BLOCK_SIZE);
            assert_eq!(result.bytes, [0; MAX_BLOCK_SIZE]);
        }

        #[test]
        fn on_bytes_longer_than_full_block_gives_last_fragment_padded() {
            let mut bytes = [42; 2 * MAX_BLOCK_SIZE + 77];
            bytes[2 * MAX_BLOCK_SIZE..].fill(69);

            let result = in_slice::pad_last_block(&bytes);

            assert_eq!(result.absolute_start, 2 * MAX_BLOCK_SIZE);
            assert_eq!(result.bytes[0..77], [69; 77]);
            assert_eq!(result.bytes[77..], [0; MAX_BLOCK_SIZE - 77]);
        }
    }

    mod seek_backward {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn seeking_from_before_first_occurrence_returns_none() {
            let bytes = r#"{"seek":42}"#.as_bytes();

            let result = in_slice::seek_backward(bytes, 6, b':');

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_after_two_occurrences_returns_the_second_one() {
            let bytes = r#"{"seek":42,"find":37}"#.as_bytes();

            let result = in_slice::seek_backward(bytes, bytes.len() - 1, b':');

            assert_eq!(result, Some(17));
        }
    }

    mod seek_forward_1 {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = in_slice::seek_forward(&bytes, 0, [0]);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_needle_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = in_slice::seek_forward(bytes, 7, [b':']);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_not_needle_returns_next_needle() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = in_slice::seek_forward(bytes, 5, [b'2']);

            assert_eq!(result, Some((9, b'2')));
        }

        #[test]
        fn seeking_from_not_needle_when_there_is_no_needle_returns_none() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = in_slice::seek_forward(bytes, 5, [b'3']);

            assert_eq!(result, None);
        }
    }

    mod seek_forward_2 {

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = in_slice::seek_forward(&bytes, 0, [0, 1]);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_needle_1_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = in_slice::seek_forward(bytes, 7, [b':', b'4']);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_needle_2_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = in_slice::seek_forward(bytes, 7, [b'4', b':']);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_not_needle_when_next_is_needle_1_returns_that() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = in_slice::seek_forward(bytes, 5, [b'4', b'2']);

            assert_eq!(result, Some((8, b'4')));
        }

        #[test]
        fn seeking_from_not_needle_when_next_is_needle_2_returns_that() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = in_slice::seek_forward(bytes, 5, [b'2', b'4']);

            assert_eq!(result, Some((8, b'4')));
        }

        #[test]
        fn seeking_from_not_needle_when_there_is_no_needle_returns_none() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = in_slice::seek_forward(bytes, 5, [b'3', b'0']);

            assert_eq!(result, None);
        }
    }

    mod seek_non_whitespace_forward {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = in_slice::seek_non_whitespace_forward(&bytes, 0);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_non_whitespace_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = in_slice::seek_non_whitespace_forward(bytes, 7);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_whitespace_returns_next_non_whitespace() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = in_slice::seek_non_whitespace_forward(bytes, 5);

            assert_eq!(result, Some((8, b'4')));
        }

        #[test]
        fn seeking_from_whitespace_when_there_is_no_more_non_whitespace_returns_none() {
            let bytes = "seek: \t\n ".as_bytes();

            let result = in_slice::seek_non_whitespace_forward(bytes, 5);

            assert_eq!(result, None);
        }
    }

    mod seek_non_whitespace_backward {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = in_slice::seek_non_whitespace_backward(&bytes, 0);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_non_whitespace_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = in_slice::seek_non_whitespace_backward(bytes, 7);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_whitespace_returns_previous_non_whitespace() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = in_slice::seek_non_whitespace_backward(bytes, 7);

            assert_eq!(result, Some((4, b':')));
        }
    }

    mod find_member {
        use super::*;
        use crate::query::JsonString;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = in_slice::find_member(&bytes, 0, &JsonString::new("abc"));

            assert_eq!(result, None);
        }

        #[test]
        fn starting_from_before_first_occurrence_returns_that() {
            let bytes = r#"{"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::find_member(bytes, 0, &JsonString::new("needle"));

            assert_eq!(result, Some(1));
        }

        #[test]
        fn starting_from_exactly_first_occurrence_returns_that() {
            let bytes = r#"{"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::find_member(bytes, 1, &JsonString::new("needle"));

            assert_eq!(result, Some(1));
        }

        #[test]
        fn starting_from_after_last_occurrence_returns_none() {
            let bytes = r#"{"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::find_member(bytes, 2, &JsonString::new("needle"));

            assert_eq!(result, None);
        }

        #[test]
        fn when_match_is_partial_due_to_escaped_double_quote_returns_none() {
            let bytes = r#"{"fake\"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::find_member(bytes, 0, &JsonString::new("needle"));

            assert_eq!(result, None);
        }

        #[test]
        fn when_looking_for_string_with_escaped_double_quote_returns_that() {
            let bytes = r#"{"fake\"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::find_member(bytes, 0, &JsonString::new(r#"fake\"needle"#));

            assert_eq!(result, Some(1));
        }
    }

    mod is_member_match {
        use super::*;
        use crate::query::JsonString;
        use pretty_assertions::assert_eq;

        #[test]
        fn on_exact_match_returns_true() {
            let bytes = r#"{"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::is_member_match(bytes, 1, 8, &JsonString::new("needle"));

            assert_eq!(result, true);
        }

        #[test]
        fn matching_without_double_quotes_returns_false() {
            let bytes = r#"{"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::is_member_match(bytes, 2, 7, &JsonString::new("needle"));

            assert_eq!(result, false);
        }

        #[test]
        fn when_match_is_partial_due_to_escaped_double_quote_returns_false() {
            let bytes = r#"{"fake\"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::is_member_match(bytes, 7, 14, &JsonString::new("needle"));

            assert_eq!(result, false);
        }

        #[test]
        fn when_looking_for_string_with_escaped_double_quote_returns_true() {
            let bytes = r#"{"fake\"needle":42,"other":37}"#.as_bytes();

            let result = in_slice::is_member_match(bytes, 1, 14, &JsonString::new(r#"fake\"needle"#));

            assert_eq!(result, true);
        }
    }
}
