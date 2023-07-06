use crate::{
    input::{error::InputError, Input},
    query::JsonString,
    result::InputRecorder,
    BLOCK_SIZE,
};
use cfg_if::cfg_if;

pub trait Memmem<'a, 'b, I: Input, const N: usize> {
    fn find_label(
        &mut self,
        first_block: Option<I::Block<'a, N>>,
        start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block<'a, N>)>, InputError>;
}

cfg_if! {
    if #[cfg(simd = "avx2")] {
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
