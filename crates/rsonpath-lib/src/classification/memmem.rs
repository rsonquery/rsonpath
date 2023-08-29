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
pub trait Memmem<'i, 'b, 'r, I: Input, const N: usize> {
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
        first_block: Option<I::Block<'i, N>>,
        start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block<'i, N>)>, InputError>;
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod avx2_32;
#[cfg(target_arch = "x86_64")]
mod avx2_64;
mod nosimd;
mod shared;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod ssse3_32;
#[cfg(target_arch = "x86_64")]
mod ssse3_64;

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        type MemmemImpl<'a, 'b, 'r, I, R> = nosimd::SequentialMemmemClassifier<'a, 'b, 'r, I, R, BLOCK_SIZE>;
    }
    else if #[cfg(simd = "avx2_64")] {
        type MemmemImpl<'a, 'b, 'r, I, R> = avx2_64::Avx2MemmemClassifier64<'a, 'b, 'r, I, R>;
    }
    else if #[cfg(simd = "avx2_32")] {
        type MemmemImpl<'a, 'b, 'r, I, R> = avx2_32::Avx2MemmemClassifier32<'a, 'b, 'r, I, R>;
    }
    else if #[cfg(simd = "ssse3_64")] {
        type MemmemImpl<'a, 'b, 'r, I, R> = ssse3_64::Ssse3MemmemClassifier64<'a, 'b, 'r, I, R>;
    }
    else if #[cfg(simd = "ssse3_32")] {
        type MemmemImpl<'a, 'b, 'r, I, R> = ssse3_32::Ssse3MemmemClassifier32<'a, 'b, 'r, I, R>;
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}

/// Walk through the JSON document represented by `bytes`
/// and classify quoted sequences.
#[must_use]
#[inline(always)]
pub fn memmem<'i, 'b, 'r, I, R>(
    input: &'i I,
    iter: &'b mut I::BlockIterator<'i, 'r, BLOCK_SIZE, R>,
) -> impl Memmem<'i, 'b, 'r, I, BLOCK_SIZE>
where
    I: Input,
    R: InputRecorder<I::Block<'i, BLOCK_SIZE>>,
    'i: 'r,
{
    MemmemImpl::new(input, iter)
}
