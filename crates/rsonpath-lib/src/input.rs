pub mod borrowed;
pub mod owned;
pub use owned::OwnedBytes;

use crate::query::Label;
use std::ops::Deref;

pub type IBlock<'a, I, const N: usize> =
    <<I as Input>::BlockIterator<'a, N> as InputBlockIterator<'a, N>>::Block;

pub trait Input: Sized {
    type BlockIterator<'a, const N: usize>: InputBlockIterator<'a, N>
    where
        Self: 'a;

    #[must_use]
    fn iter_blocks<const N: usize>(&self) -> Self::BlockIterator<'_, N>;

    #[must_use]
    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize>;

    #[must_use]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)>;

    #[must_use]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)>;

    #[cfg(feature = "head-skip")]
    #[must_use]
    fn find_label(&self, from: usize, label: &Label) -> Option<usize>;

    #[must_use]
    fn is_label_match(&self, from: usize, to: usize, label: &Label) -> bool;
}

pub trait InputBlockIterator<'a, const N: usize>: Iterator<Item = Self::Block> {
    type Block: InputBlock<'a, N>;

    fn offset(&mut self, count: isize);
}

pub trait InputBlock<'a, const N: usize>: Deref<Target = [u8]> {
    fn halves(&self) -> (&[u8], &[u8]);
}

impl<'a, const N: usize> InputBlock<'a, N> for &'a [u8] {
    #[inline(always)]
    fn halves(&self) -> (&[u8], &[u8]) {
        assert_eq!(N % 2, 0);
        (&self[..N / 2], &self[N / 2..])
    }
}