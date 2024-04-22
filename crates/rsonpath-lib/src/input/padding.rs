use super::{SliceSeekable, MAX_BLOCK_SIZE};
use crate::{
    string_pattern::{self, StringPattern},
    JSON_SPACE_BYTE,
};

pub(super) struct PaddedBlock {
    bytes: [u8; MAX_BLOCK_SIZE],
    padding_len: usize,
}

pub struct EndPaddedInput<'a> {
    middle: &'a [u8],
    last_block: &'a PaddedBlock,
}

pub struct TwoSidesPaddedInput<'a> {
    first_block: &'a PaddedBlock,
    middle: &'a [u8],
    last_block: &'a PaddedBlock,
}

impl PaddedBlock {
    #[allow(clippy::unused_self)] // This is nicer than using the constant everywhere.
    pub(super) const fn len(&self) -> usize {
        MAX_BLOCK_SIZE
    }

    pub(super) fn padding_len(&self) -> usize {
        self.padding_len
    }

    pub(super) fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub(super) fn pad_first_block(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= MAX_BLOCK_SIZE);
        let mut block_buf = [JSON_SPACE_BYTE; MAX_BLOCK_SIZE];
        let block_start = MAX_BLOCK_SIZE - bytes.len();

        block_buf[block_start..].copy_from_slice(bytes);

        Self {
            bytes: block_buf,
            padding_len: block_start,
        }
    }

    pub(super) fn pad_last_block(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= MAX_BLOCK_SIZE);
        let mut last_block_buf = [JSON_SPACE_BYTE; MAX_BLOCK_SIZE];
        let block_end = bytes.len();

        last_block_buf[..block_end].copy_from_slice(bytes);

        Self {
            bytes: last_block_buf,
            padding_len: MAX_BLOCK_SIZE - block_end,
        }
    }
}

impl<'a> SliceSeekable for EndPaddedInput<'a> {
    #[cold]
    #[inline(never)]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        if from < self.middle.len() {
            self.seek_backward_from_middle(from, needle)
        } else {
            self.seek_backward_from_last(from, needle)
        }
    }

    #[cold]
    #[inline(never)]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        if from < self.middle.len() {
            self.seek_forward_from_middle(from, needles)
        } else {
            self.seek_forward_from_last(from, needles)
        }
    }

    #[cold]
    #[inline(never)]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)> {
        if from < self.middle.len() {
            self.seek_non_whitespace_forward_from_middle(from)
        } else {
            self.seek_non_whitespace_forward_from_last(from)
        }
    }

    #[cold]
    #[inline(never)]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        if from < self.middle.len() {
            self.seek_non_whitespace_backward_from_middle(from)
        } else {
            self.seek_non_whitespace_backward_from_last(from)
        }
    }

    #[cold]
    #[inline(never)]
    fn pattern_match_from<M>(&self, from: usize, pattern: &StringPattern) -> Option<usize> {
        self.cold_pattern_match_forward(pattern, from, from + pattern.len_limit())
    }

    #[cold]
    #[inline(never)]
    fn pattern_match_to<M>(&self, to: usize, pattern: &StringPattern) -> Option<usize> {
        self.cold_pattern_match_backward(pattern, to.saturating_sub(pattern.len_limit()), to)
    }
}

impl<'a> SliceSeekable for TwoSidesPaddedInput<'a> {
    #[cold]
    #[inline(never)]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        if from < MAX_BLOCK_SIZE {
            self.seek_backward_from_first(from, needle)
        } else if from < self.middle.len() + MAX_BLOCK_SIZE {
            self.seek_backward_from_middle(from, needle)
        } else {
            self.seek_backward_from_last(from, needle)
        }
    }

    #[cold]
    #[inline(never)]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        if from < MAX_BLOCK_SIZE {
            self.seek_forward_from_first(from, needles)
        } else if from < self.middle.len() + MAX_BLOCK_SIZE {
            self.seek_forward_from_middle(from, needles)
        } else {
            self.seek_forward_from_last(from, needles)
        }
    }

    #[cold]
    #[inline(never)]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)> {
        if from < MAX_BLOCK_SIZE {
            self.seek_non_whitespace_forward_from_first(from)
        } else if from < self.middle.len() + MAX_BLOCK_SIZE {
            self.seek_non_whitespace_forward_from_middle(from)
        } else {
            self.seek_non_whitespace_forward_from_last(from)
        }
    }

    #[cold]
    #[inline(never)]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        if from < MAX_BLOCK_SIZE {
            self.seek_non_whitespace_backward_from_first(from)
        } else if from < self.middle.len() + MAX_BLOCK_SIZE {
            self.seek_non_whitespace_backward_from_middle(from)
        } else {
            self.seek_non_whitespace_backward_from_last(from)
        }
    }

    #[cold]
    #[inline(never)]
    fn pattern_match_from<M>(&self, from: usize, pattern: &StringPattern) -> Option<usize> {
        self.cold_pattern_match_forward(pattern, from, from + pattern.len_limit())
    }

    #[cold]
    #[inline(never)]
    fn pattern_match_to<M>(&self, to: usize, pattern: &StringPattern) -> Option<usize> {
        self.cold_pattern_match_backward(pattern, to.saturating_sub(pattern.len_limit()), to)
    }
}

impl<'a> EndPaddedInput<'a> {
    pub(super) fn new(middle: &'a [u8], last: &'a PaddedBlock) -> Self {
        Self {
            middle,
            last_block: last,
        }
    }

    #[inline(always)]
    pub(super) fn middle(&self) -> &'a [u8] {
        self.middle
    }

    fn seek_backward_from_middle(&self, from: usize, needle: u8) -> Option<usize> {
        debug_assert!(from < self.middle.len());
        let bytes = self.middle;

        seek_backward_impl(bytes, from, needle)
    }

    fn seek_backward_from_last(&self, from: usize, needle: u8) -> Option<usize> {
        debug_assert!(from >= self.middle.len());
        let bytes = &self.last_block.bytes;

        seek_backward_impl(bytes, from - self.middle.len(), needle)
            .map(|x| x + self.middle.len())
            .or_else(|| self.seek_backward_from_middle(self.middle.len() - 1, needle))
    }

    fn seek_forward_from_middle<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        assert!(N > 0);
        debug_assert!(from < self.middle.len());
        let bytes = self.middle;

        seek_forward_impl(bytes, from, needles).or_else(|| self.seek_forward_from_last(bytes.len(), needles))
    }

    fn seek_forward_from_last<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        assert!(N > 0);
        debug_assert!(from >= self.middle.len());
        let bytes = &self.last_block.bytes;

        seek_forward_impl(bytes, from - self.middle.len(), needles).map(|(x, y)| (x + self.middle.len(), y))
    }

    fn seek_non_whitespace_forward_from_middle(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from < self.middle.len());
        let bytes = self.middle;

        seek_non_whitespace_forward_impl(bytes, from)
            .or_else(|| self.seek_non_whitespace_forward_from_last(bytes.len()))
    }

    fn seek_non_whitespace_forward_from_last(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from >= self.middle.len());
        let bytes = &self.last_block.bytes;

        seek_non_whitespace_forward_impl(bytes, from - self.middle.len()).map(|(x, y)| (x + self.middle.len(), y))
    }

    fn seek_non_whitespace_backward_from_middle(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from < self.middle.len());
        let bytes = self.middle;

        seek_non_whitespace_backward_impl(bytes, from)
    }

    fn seek_non_whitespace_backward_from_last(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from >= self.middle.len());
        let bytes = &self.last_block.bytes;

        seek_non_whitespace_backward_impl(bytes, from - self.middle.len())
            .map(|(x, y)| (x + self.middle.len(), y))
            .or_else(|| self.seek_non_whitespace_backward_from_middle(self.middle.len() - 1))
    }

    pub(super) fn try_slice(&self, start: usize, len: usize) -> Option<&'a [u8]> {
        debug_assert!(len < MAX_BLOCK_SIZE);

        if start < self.middle.len() {
            self.slice_middle(start, len)
        } else {
            self.slice_last(start, len)
        }
    }

    fn slice_middle(&self, start: usize, len: usize) -> Option<&'a [u8]> {
        Some(&self.middle[start..start + len])
    }

    fn slice_last(&self, start: usize, len: usize) -> Option<&'a [u8]> {
        let start = start - self.middle.len();
        (start < MAX_BLOCK_SIZE).then(|| &self.last_block.bytes[start..start + len])
    }

    /// Slice the entire input from `from` to `to` and return the part
    /// from the middle, and the part from the last block. Either or both
    /// may be empty when appropriate.
    fn slice_parts(&self, from: usize, to: usize) -> (&[u8], &[u8]) {
        use std::cmp::min;

        let middle_from = min(from, self.middle.len());
        let middle_to = min(to, self.middle.len());

        let from = from.saturating_sub(self.middle.len());
        let to = to.saturating_sub(self.middle.len());
        let last_from = min(from, self.last_block.len());
        let last_to = min(to, self.last_block.len());

        (
            &self.middle[middle_from..middle_to],
            &self.last_block.bytes[last_from..last_to],
        )
    }

    fn get_at(&self, idx: usize) -> Option<u8> {
        if idx < self.middle.len() {
            Some(self.middle[idx])
        } else if idx < self.middle.len() + MAX_BLOCK_SIZE {
            Some(self.last_block.bytes[idx - self.middle.len()])
        } else {
            None
        }
    }

    fn cold_pattern_match_forward(&self, pattern: &StringPattern, from: usize, to: usize) -> Option<usize> {
        let (middle_self, last_self) = self.slice_parts(from, to);
        let preceding_char = from.checked_sub(1).and_then(|x| self.get_at(x));

        let idx = string_pattern::matcher::nosimd::NosimdStringMatcher::pattern_match_forward(
            pattern,
            (middle_self, last_self),
        )?;
        preceding_char.map_or(true, |x| x != b'\\').then_some(from + idx)
    }

    fn cold_pattern_match_backward(&self, pattern: &StringPattern, from: usize, to: usize) -> Option<usize> {
        let (middle_self, last_self) = self.slice_parts(from, to);

        let idx = string_pattern::matcher::nosimd::NosimdStringMatcher::pattern_match_backward(
            pattern,
            (middle_self, last_self),
        )?;
        let preceding_char = (from + idx).checked_sub(1).and_then(|x| self.get_at(x));
        preceding_char.map_or(true, |x| x != b'\\').then_some(from + idx)
    }
}

impl<'a> TwoSidesPaddedInput<'a> {
    pub(super) fn new(first: &'a PaddedBlock, middle: &'a [u8], last: &'a PaddedBlock) -> Self {
        Self {
            first_block: first,
            middle,
            last_block: last,
        }
    }

    #[inline(always)]
    pub(super) fn middle(&self) -> &'a [u8] {
        self.middle
    }

    fn seek_backward_from_first(&self, from: usize, needle: u8) -> Option<usize> {
        debug_assert!(from < MAX_BLOCK_SIZE);
        let bytes = &self.first_block.bytes;

        seek_backward_impl(bytes, from, needle)
    }

    fn seek_backward_from_middle(&self, from: usize, needle: u8) -> Option<usize> {
        debug_assert!(from >= MAX_BLOCK_SIZE);
        let bytes = self.middle;

        seek_backward_impl(bytes, from - MAX_BLOCK_SIZE, needle)
            .map(|x| x + MAX_BLOCK_SIZE)
            .or_else(|| self.seek_backward_from_first(MAX_BLOCK_SIZE - 1, needle))
    }

    fn seek_backward_from_last(&self, from: usize, needle: u8) -> Option<usize> {
        debug_assert!(from >= self.middle.len() + MAX_BLOCK_SIZE);
        let bytes = &self.last_block.bytes;

        seek_backward_impl(bytes, from - self.middle.len() - MAX_BLOCK_SIZE, needle)
            .map(|x| x + self.middle.len() + MAX_BLOCK_SIZE)
            .or_else(|| {
                if self.middle.is_empty() {
                    self.seek_backward_from_first(MAX_BLOCK_SIZE - 1, needle)
                } else {
                    self.seek_backward_from_middle(self.middle.len() + MAX_BLOCK_SIZE - 1, needle)
                }
            })
    }

    fn seek_forward_from_first<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        assert!(N > 0);
        debug_assert!(from < MAX_BLOCK_SIZE);
        let bytes = &self.first_block.bytes;

        seek_forward_impl(bytes, from, needles).or_else(|| {
            if self.middle.is_empty() {
                self.seek_forward_from_last(bytes.len(), needles)
            } else {
                self.seek_forward_from_middle(bytes.len(), needles)
            }
        })
    }

    fn seek_forward_from_middle<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        assert!(N > 0);
        debug_assert!(from >= MAX_BLOCK_SIZE);
        let bytes = self.middle;

        seek_forward_impl(bytes, from - MAX_BLOCK_SIZE, needles)
            .map(|(x, y)| (x + MAX_BLOCK_SIZE, y))
            .or_else(|| self.seek_forward_from_last(bytes.len() + MAX_BLOCK_SIZE, needles))
    }

    fn seek_forward_from_last<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        assert!(N > 0);
        debug_assert!(from >= self.middle.len() + MAX_BLOCK_SIZE);
        let bytes = &self.last_block.bytes;

        seek_forward_impl(bytes, from - self.middle.len() - MAX_BLOCK_SIZE, needles)
            .map(|(x, y)| (x + self.middle.len() + MAX_BLOCK_SIZE, y))
    }

    fn seek_non_whitespace_forward_from_first(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from < MAX_BLOCK_SIZE);
        let bytes = &self.first_block.bytes;

        seek_non_whitespace_forward_impl(bytes, from).or_else(|| {
            if self.middle.is_empty() {
                self.seek_non_whitespace_forward_from_last(bytes.len())
            } else {
                self.seek_non_whitespace_forward_from_middle(bytes.len())
            }
        })
    }

    fn seek_non_whitespace_forward_from_middle(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from >= MAX_BLOCK_SIZE);
        let bytes = self.middle;

        seek_non_whitespace_forward_impl(bytes, from - MAX_BLOCK_SIZE)
            .map(|(x, y)| (x + MAX_BLOCK_SIZE, y))
            .or_else(|| self.seek_non_whitespace_forward_from_last(bytes.len() + MAX_BLOCK_SIZE))
    }

    fn seek_non_whitespace_forward_from_last(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from >= self.middle.len() + MAX_BLOCK_SIZE);
        let bytes = &self.last_block.bytes;

        seek_non_whitespace_forward_impl(bytes, from - self.middle.len() - MAX_BLOCK_SIZE)
            .map(|(x, y)| (x + self.middle.len() + MAX_BLOCK_SIZE, y))
    }

    fn seek_non_whitespace_backward_from_first(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from < MAX_BLOCK_SIZE);
        let bytes = &self.first_block.bytes;

        seek_non_whitespace_backward_impl(bytes, from)
    }

    fn seek_non_whitespace_backward_from_middle(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from >= MAX_BLOCK_SIZE);
        let bytes = self.middle;

        seek_non_whitespace_backward_impl(bytes, from - MAX_BLOCK_SIZE)
            .map(|(x, y)| (x + MAX_BLOCK_SIZE, y))
            .or_else(|| self.seek_non_whitespace_backward_from_first(MAX_BLOCK_SIZE - 1))
    }

    fn seek_non_whitespace_backward_from_last(&self, from: usize) -> Option<(usize, u8)> {
        debug_assert!(from >= self.middle.len() + MAX_BLOCK_SIZE);
        let bytes = &self.last_block.bytes;

        seek_non_whitespace_backward_impl(bytes, from - self.middle.len() - MAX_BLOCK_SIZE)
            .map(|(x, y)| (x + self.middle.len() + MAX_BLOCK_SIZE, y))
            .or_else(|| {
                if self.middle.is_empty() {
                    self.seek_non_whitespace_backward_from_first(MAX_BLOCK_SIZE - 1)
                } else {
                    self.seek_non_whitespace_backward_from_middle(self.middle.len() + MAX_BLOCK_SIZE - 1)
                }
            })
    }

    pub(super) fn try_slice(&self, start: usize, len: usize) -> Option<&'a [u8]> {
        debug_assert!(len < MAX_BLOCK_SIZE);

        if start < MAX_BLOCK_SIZE {
            Some(self.slice_first(start, len))
        } else if start < self.middle.len() + MAX_BLOCK_SIZE {
            Some(self.slice_middle(start, len))
        } else {
            self.slice_last(start, len)
        }
    }

    fn slice_first(&self, start: usize, len: usize) -> &'a [u8] {
        &self.first_block.bytes[start..start + len]
    }

    fn slice_middle(&self, start: usize, len: usize) -> &'a [u8] {
        let start = start - MAX_BLOCK_SIZE;
        &self.middle[start..start + len]
    }

    fn slice_last(&self, start: usize, len: usize) -> Option<&'a [u8]> {
        let start = start - self.middle.len() - MAX_BLOCK_SIZE;
        (start < MAX_BLOCK_SIZE).then(|| &self.last_block.bytes[start..start + len])
    }

    /// Slice the entire input from `from` to `to` and return the part
    /// from the first block, from the middle, and the part from the last block.
    /// Any and all of them may be empty when appropriate.
    fn slice_parts(&self, from: usize, to: usize) -> (&[u8], &[u8], &[u8]) {
        use std::cmp::min;

        let first_from = min(from, MAX_BLOCK_SIZE);
        let first_to = min(to, MAX_BLOCK_SIZE);

        let from = from.saturating_sub(MAX_BLOCK_SIZE);
        let to = to.saturating_sub(MAX_BLOCK_SIZE);
        let middle_from = min(from, self.middle.len());
        let middle_to = min(to, self.middle.len());

        let from = from.saturating_sub(self.middle.len());
        let to = to.saturating_sub(self.middle.len());
        let last_from = min(from, self.last_block.len());
        let last_to = min(to, self.last_block.len());

        (
            &self.first_block.bytes[first_from..first_to],
            &self.middle[middle_from..middle_to],
            &self.last_block.bytes[last_from..last_to],
        )
    }

    fn get_at(&self, idx: usize) -> Option<u8> {
        if idx < MAX_BLOCK_SIZE {
            Some(self.first_block.bytes[idx])
        } else if idx < self.middle.len() + MAX_BLOCK_SIZE {
            Some(self.middle[idx - MAX_BLOCK_SIZE])
        } else if idx < self.middle.len() + 2 * MAX_BLOCK_SIZE {
            Some(self.last_block.bytes[idx - MAX_BLOCK_SIZE - self.middle.len()])
        } else {
            None
        }
    }

    fn cold_pattern_match_forward(&self, pattern: &StringPattern, from: usize, to: usize) -> Option<usize> {
        let (first_self, middle_self, last_self) = self.slice_parts(from, to);
        let preceding_char = from.checked_sub(1).and_then(|x| self.get_at(x));

        let idx = string_pattern::matcher::nosimd::NosimdStringMatcher::pattern_match_forward(
            pattern,
            (first_self, middle_self, last_self),
        )?;
        preceding_char.map_or(Some(from + idx), |x| (x != b'\\').then_some(from + idx))
    }

    fn cold_pattern_match_backward(&self, pattern: &StringPattern, from: usize, to: usize) -> Option<usize> {
        let (first_self, middle_self, last_self) = self.slice_parts(from, to);

        let idx = string_pattern::matcher::nosimd::NosimdStringMatcher::pattern_match_backward(
            pattern,
            (first_self, middle_self, last_self),
        )?;
        let preceding_char = (from + idx).checked_sub(1).and_then(|x| self.get_at(x));
        preceding_char.map_or(Some(from + idx), |x| (x != b'\\').then_some(from + idx))
    }
}

#[inline(always)]
fn seek_backward_impl(bytes: &[u8], from: usize, needle: u8) -> Option<usize> {
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

#[inline(always)]
fn seek_forward_impl<const N: usize>(bytes: &[u8], from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
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

#[inline(always)]
fn seek_non_whitespace_forward_impl(bytes: &[u8], from: usize) -> Option<(usize, u8)> {
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

#[inline(always)]
fn seek_non_whitespace_backward_impl(bytes: &[u8], from: usize) -> Option<(usize, u8)> {
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

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn on_empty_bytes_is_all_whitespace() {
        let result = PaddedBlock::pad_last_block(&[]);

        assert_eq!(result.bytes, [JSON_SPACE_BYTE; MAX_BLOCK_SIZE]);
    }

    #[test]
    fn on_bytes_smaller_than_full_block_gives_entire_block() {
        let bytes = r#"{"test":42}"#.as_bytes();

        let result = PaddedBlock::pad_last_block(bytes);

        assert_eq!(&result.bytes[0..11], bytes);
        assert_eq!(&result.bytes[11..], [JSON_SPACE_BYTE; MAX_BLOCK_SIZE - 11]);
    }

    #[test]
    fn on_bytes_equal_to_full_block_does_not_change_block() {
        let bytes = [42; MAX_BLOCK_SIZE];

        let result = PaddedBlock::pad_last_block(&bytes);

        assert_eq!(result.bytes, bytes);
    }

    mod two_sided_padded_input {
        mod seek_forward_1 {
            use crate::input::{
                padding::{PaddedBlock, TwoSidesPaddedInput},
                SliceSeekable,
            };
            use pretty_assertions::assert_eq;
            use std::iter;

            #[test]
            fn in_empty_slice_returns_none() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: &[],
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_forward(0, [0]);

                assert_eq!(result, None);
            }

            #[test]
            fn seeking_from_first_block_from_needle_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(r#"{"seek": 42}"#.as_bytes()),
                    middle: &[],
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_forward(123, [b':']);

                assert_eq!(result, Some((123, b':')));
            }

            #[test]
            fn seeking_from_middle_block_from_needle_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: r#"{"seek": 42}"#.as_bytes(),
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_forward(128 + 7, [b':']);

                assert_eq!(result, Some((128 + 7, b':')));
            }

            #[test]
            fn seeking_from_last_block_from_needle_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: &iter::repeat(b' ').take(256).collect::<Vec<_>>(),
                    last_block: &PaddedBlock::pad_last_block(r#"{"seek": 42}"#.as_bytes()),
                };

                let result = input.seek_forward(128 + 256 + 7, [b':']);

                assert_eq!(result, Some((128 + 256 + 7, b':')));
            }

            #[test]
            fn seeking_from_first_block_from_not_needle_returns_next_needle() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(r"seek: \t\n42}".as_bytes()),
                    middle: &[],
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_forward(119, [b'2']);

                assert_eq!(result, Some((126, b'2')));
            }

            #[test]
            fn seeking_from_middle_block_from_not_needle_returns_next_needle() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: r"seek: \t\n42}".as_bytes(),
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_forward(128 + 5, [b'2']);

                assert_eq!(result, Some((128 + 11, b'2')));
            }

            #[test]
            fn seeking_from_last_block_from_not_needle_returns_next_needle() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: &iter::repeat(b' ').take(256).collect::<Vec<_>>(),
                    last_block: &PaddedBlock::pad_last_block(r"seek: \t\n42}".as_bytes()),
                };

                let result = input.seek_forward(128 + 256 + 5, [b'2']);

                assert_eq!(result, Some((128 + 256 + 11, b'2')));
            }

            #[test]
            fn seeking_from_first_block_from_not_needle_when_there_is_no_needle_returns_none() {
                let bytes = "seek: \t\n42}".as_bytes();

                let result = bytes.seek_forward(5, [b'3']);

                assert_eq!(result, None);
            }
        }

        mod seek_forward_2 {
            use crate::input::{
                padding::{PaddedBlock, TwoSidesPaddedInput},
                SliceSeekable,
            };
            use pretty_assertions::assert_eq;

            #[test]
            fn in_empty_input_returns_none() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: &[],
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_forward(0, [0, 1]);

                assert_eq!(result, None);
            }

            #[test]
            fn seeking_from_needle_1_returns_that() {
                let bytes = r#"{"seek": 42}"#.as_bytes();

                let result = bytes.seek_forward(7, [b':', b'4']);

                assert_eq!(result, Some((7, b':')));
            }

            #[test]
            fn seeking_from_needle_2_returns_that() {
                let bytes = r#"{"seek": 42}"#.as_bytes();

                let result = bytes.seek_forward(7, [b'4', b':']);

                assert_eq!(result, Some((7, b':')));
            }

            #[test]
            fn seeking_from_not_needle_when_next_is_needle_1_returns_that() {
                let bytes = "seek: \t\n42}".as_bytes();

                let result = bytes.seek_forward(5, [b'4', b'2']);

                assert_eq!(result, Some((8, b'4')));
            }

            #[test]
            fn seeking_from_not_needle_when_next_is_needle_2_returns_that() {
                let bytes = "seek: \t\n42}".as_bytes();

                let result = bytes.seek_forward(5, [b'2', b'4']);

                assert_eq!(result, Some((8, b'4')));
            }

            #[test]
            fn seeking_from_not_needle_when_there_is_no_needle_returns_none() {
                let bytes = "seek: \t\n42}".as_bytes();

                let result = bytes.seek_forward(5, [b'3', b'0']);

                assert_eq!(result, None);
            }
        }

        mod seek_backward {
            use crate::input::{
                padding::{PaddedBlock, TwoSidesPaddedInput},
                SliceSeekable,
            };
            use pretty_assertions::assert_eq;

            #[test]
            fn in_empty_slice_returns_none() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: &[],
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_non_whitespace_forward(0);

                assert_eq!(result, None);
            }

            #[test]
            fn seeking_from_needle_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: r#"{"seek": 42}"#.as_bytes(),
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_backward(136, b':');

                assert_eq!(result, Some(135));
            }

            #[test]
            fn seeking_from_not_needle_when_previous_is_needle_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: "seek: \t\n42}".as_bytes(),
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_backward(137, b'4');

                assert_eq!(result, Some(136));
            }

            #[test]
            fn seeking_from_not_needle_when_there_is_no_needle_returns_none() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: "seek: \t\n42}".as_bytes(),
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_backward(138, b'3');

                assert_eq!(result, None);
            }
        }

        mod seek_non_whitespace_forward {
            use crate::input::{
                padding::{PaddedBlock, TwoSidesPaddedInput},
                SliceSeekable,
            };
            use pretty_assertions::assert_eq;
            use std::iter;

            #[test]
            fn in_empty_slice_returns_none() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: &[],
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_non_whitespace_forward(0);

                assert_eq!(result, None);
            }

            #[test]
            fn seeking_from_first_block_from_non_whitespace_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(r#"{"seek": 42}"#.as_bytes()),
                    middle: &[],
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_non_whitespace_forward(123);

                assert_eq!(result, Some((123, b':')));
            }

            #[test]
            fn seeking_from_middle_block_from_non_whitespace_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: r#"{"seek": 42}"#.as_bytes(),
                    last_block: &PaddedBlock::pad_last_block(&[]),
                };

                let result = input.seek_non_whitespace_forward(128 + 7);

                assert_eq!(result, Some((128 + 7, b':')));
            }

            #[test]
            fn seeking_from_last_block_from_non_whitespace_returns_that() {
                let input = TwoSidesPaddedInput {
                    first_block: &PaddedBlock::pad_first_block(&[]),
                    middle: &iter::repeat(b' ').take(256).collect::<Vec<_>>(),
                    last_block: &PaddedBlock::pad_last_block(r#"{"seek": 42}"#.as_bytes()),
                };

                let result = input.seek_non_whitespace_forward(128 + 256 + 7);

                assert_eq!(result, Some((128 + 256 + 7, b':')));
            }

            #[test]
            fn seeking_from_whitespace_returns_next_non_whitespace() {
                let bytes = "seek: \t\n42}".as_bytes();

                let result = bytes.seek_non_whitespace_forward(5);

                assert_eq!(result, Some((8, b'4')));
            }

            #[test]
            fn seeking_from_whitespace_when_there_is_no_more_non_whitespace_returns_none() {
                let bytes = "seek: \t\n ".as_bytes();

                let result = bytes.seek_non_whitespace_forward(5);

                assert_eq!(result, None);
            }
        }
    }
}
