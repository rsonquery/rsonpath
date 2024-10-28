//! Classification ignoring the structure of the JSON and looking for the occurrence
//! of a specific member name as quickly as possible.
use crate::{
    input::{error::InputError, Input},
    result::InputRecorder,
    BLOCK_SIZE,
};
use rsonpath_syntax::str::JsonString;

/// Classifier that can quickly find a member name in a byte stream.
pub trait Memmem<'i, 'b, 'r, I, R, const N: usize>
where
    I: Input<'i, 'r, R, N>,
    R: InputRecorder<I::Block> + 'r,
{
    /// Find a member key identified by a given [`JsonString`].
    ///
    /// - `first_block` &ndash; optional first block to search; if not provided,
    ///    the search will start at the next block returned by the underlying [`Input`] iterator.
    /// - `start_idx` &ndash; index of the start of search, either falling inside `first_block`,
    ///    or at the start of the next block.
    ///
    /// # Errors
    /// Errors when reading the underlying [`Input`] are propagated.
    fn find_label(
        &mut self,
        first_block: Option<I::Block>,
        start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block)>, InputError>;
}

pub(crate) mod nosimd;
pub(crate) mod shared;

#[cfg(target_arch = "x86")]
pub(crate) mod avx2_32;
#[cfg(target_arch = "x86_64")]
pub(crate) mod avx2_64;
#[cfg(target_arch = "x86")]
pub(crate) mod sse2_32;
#[cfg(target_arch = "x86_64")]
pub(crate) mod sse2_64;

pub(crate) trait MemmemImpl {
    type Classifier<'i, 'b, 'r, I, R>: Memmem<'i, 'b, 'r, I, R, BLOCK_SIZE>
    where
        I: Input<'i, 'r, R, BLOCK_SIZE> + 'i,
        I::BlockIterator: 'b,
        R: InputRecorder<I::Block> + 'r,
        'i: 'r;

    fn memmem<'i, 'b, 'r, I, R>(
        input: &'i I,
        iter: &'b mut I::BlockIterator,
    ) -> Self::Classifier<'i, 'b, 'r, I, R>
    where
        I: Input<'i, 'r, R, BLOCK_SIZE>,
        R: InputRecorder<I::Block>,
        'i: 'r;
}
