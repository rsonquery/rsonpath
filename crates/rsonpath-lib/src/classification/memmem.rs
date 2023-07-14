//! Classification ignoring the structure of the JSON and looking for the occurrence
//! of a specific member name as quickly as possible.
use crate::{
    input::{error::InputError, Input},
    query::JsonString,
    result::InputRecorder,
    BLOCK_SIZE,
};
use cfg_if::cfg_if;

/// Classifier that can quickly find a member name in a byte stream.
pub trait Memmem<'a, 'b, I: Input, const N: usize> {
    /// Find a member key identified by a given [`JsonString`].
    ///
    /// - `first_block` &ndash; optional first block to search; if not provided,
    /// the search will start at the next block returned by the underlying [`Input`] iterator.
    /// - `start_idx` &ndash; index of the start of search, either falling inside `first_block`,
    /// or at the start of the next block.
    ///
    /// # Errors
    /// Errors when reading the underlying [`Input`] are propagated.
    fn find_label(
        &mut self,
        first_block: Option<I::Block<'a, N>>,
        start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block<'a, N>)>, InputError>;
}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        type MemmemImpl<'a, 'b, 'r, I, R> = nosimd::SequentialMemmemClassifier<'a, 'b, 'r, I, R, BLOCK_SIZE>;
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        type MemmemImpl<'a, 'b, 'r, I, R> = avx2::Avx2MemmemClassifier<'a, 'b, 'r, I, R>;
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}

/// Walk through the JSON document represented by `bytes`
/// and classify quoted sequences.
#[must_use]
#[inline(always)]
pub fn memmem<'a, 'b, I: Input, R: InputRecorder>(
    input: &'a I,
    iter: &'b mut I::BlockIterator<'a, 'a, BLOCK_SIZE, R>,
) -> impl Memmem<'a, 'b, I, BLOCK_SIZE> {
    MemmemImpl::new(input, iter)
}
