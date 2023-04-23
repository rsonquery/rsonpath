use aligners::{alignment, AlignedBytes};
use cfg_if::cfg_if;
use std::ops::Deref;

use crate::query::Label;

/// Input into a query engine.
pub struct InMemoryInput {
    bytes: AlignedBytes<alignment::Page>,
}

impl Deref for InMemoryInput {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

pub type IBlock<'a, I, const N: usize> =
    <<I as Input>::BlockIterator<'a, N> as InputBlockIterator<'a, N>>::Block;

pub trait Input: Sized {
    type BlockIterator<'a, const N: usize>: InputBlockIterator<'a, N>
    where
        Self: 'a;

    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N>;

    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize>;

    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)>;

    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)>;

    #[cfg(feature = "head-skip")]
    fn find_label(&self, from: usize, label: &Label) -> Option<usize>;

    fn is_label_match(&self, from: usize, to: usize, label: &Label) -> bool;
}

pub trait InputBlockIterator<'a, const N: usize>: Iterator<Item = Self::Block> {
    type Block: InputBlock<'a, N>;

    fn offset(&mut self, count: isize);
}

pub trait InputBlock<'a, const N: usize>: Deref<Target = [u8]> {
    fn halves(&self) -> (&[u8], &[u8]);
}

impl Input for InMemoryInput {
    type BlockIterator<'a, const N: usize> = InMemoryBlockIterator<'a, N>;

    #[inline(always)]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N> {
        Self::BlockIterator {
            input: self,
            idx: 0,
        }
    }

    #[inline]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let mut idx = from;

        loop {
            if self.bytes[idx] == needle {
                return Some(idx);
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)> {
        let mut idx = from;

        loop {
            let b = self.bytes[idx];
            if b != b' ' && b != b'\t' && b != b'\n' && b != b'\r' {
                return Some((idx, b));
            }
            idx += 1;
            if idx == self.bytes.len() {
                return None;
            }
        }
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        let mut idx = from;

        loop {
            let b = self.bytes[idx];
            if b != b' ' && b != b'\t' && b != b'\n' && b != b'\r' {
                return Some((idx, b));
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    #[inline]
    #[cfg(feature = "head-skip")]
    fn find_label(&self, from: usize, label: &Label) -> Option<usize> {
        use memchr::memmem;

        let finder = memmem::Finder::new(label.bytes_with_quotes());

        let starting_quote_idx = finder.find(&self.bytes[from..])? + from;

        (starting_quote_idx != 0 && self.bytes[starting_quote_idx - 1] != b'\\')
            .then_some(starting_quote_idx)
    }

    #[inline]
    fn is_label_match(&self, from: usize, to: usize, label: &Label) -> bool {
        let slice = &self.bytes[from..to];
        label.bytes_with_quotes() == slice && (from == 0 || self.bytes[from - 1] != b'\\')
    }
}

pub struct InMemoryBlockIterator<'a, const N: usize> {
    input: &'a InMemoryInput,
    idx: usize,
}

pub struct InMemoryInputBlock<'a, const N: usize> {
    block: &'a [u8],
}

impl<'a, const N: usize> Deref for InMemoryInputBlock<'a, N> {
    type Target = [u8];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.block
    }
}

impl<'a, const N: usize> InputBlock<'a, N> for InMemoryInputBlock<'a, N> {
    #[inline(always)]
    fn halves(&self) -> (&[u8], &[u8]) {
        assert_eq!(N % 2, 0);
        (&self[..N / 2], &self[N / 2..])
    }
}

impl<'a, const N: usize> Iterator for InMemoryBlockIterator<'a, N> {
    type Item = InMemoryInputBlock<'a, N>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.input.len() {
            None
        } else {
            let block = &self.input[self.idx..self.idx + N];
            self.idx += N;

            Some(InMemoryInputBlock { block })
        }
    }
}

impl<'a, const N: usize> InputBlockIterator<'a, N> for InMemoryBlockIterator<'a, N> {
    type Block = Self::Item;

    #[inline(always)]
    fn offset(&mut self, count: isize) {
        assert!(count >= 0);
        self.idx += count as usize * N;
    }
}

impl InMemoryInput {
    /// Transmute a buffer into an input.
    ///
    /// The buffer must know its length, may be extended by auxiliary UTF8 characters
    /// and will be interpreted as a slice of bytes at the end.
    #[must_use]
    #[inline]
    pub fn new<T: Extend<char> + AsRef<[u8]>>(src: &mut T, alignment: usize) -> Self {
        assert_ne!(alignment, 0);
        let contents = src;
        let rem = contents.as_ref().len() % alignment;
        let pad = if rem == 0 { 0 } else { alignment - rem };

        let extension = std::iter::repeat('\0').take(pad);
        contents.extend(extension);

        debug_assert_eq!(contents.as_ref().len() % alignment, 0);

        Self {
            bytes: AlignedBytes::<alignment::Page>::from(contents.as_ref()),
        }
    }

    /// Transmute a buffer into an input.
    ///
    /// The buffer must know its length, may be extended by auxiliary bytes.
    #[inline]
    pub fn new_bytes<T: Extend<u8> + AsRef<[u8]>>(src: &mut T, alignment: usize) -> Self {
        assert_ne!(alignment, 0);
        cfg_if! {
            if #[cfg(feature = "simd")] {
                let contents = src;
                let rem = contents.as_ref().len() % alignment;
                let pad = if rem == 0 {
                    0
                } else {
                    alignment - rem
                };

                let extension = std::iter::repeat(0).take(pad + alignment);
                contents.extend(extension);

                debug_assert_eq!(contents.as_ref().len() % alignment, 0);

                Self {
                    bytes: AlignedBytes::<alignment::Page>::from(contents.as_ref()),
                }
            }
            else {
                Self {
                    bytes: AlignedBytes::<alignment::Page>::from(src.as_ref()),
                }
            }
        }
    }
}
