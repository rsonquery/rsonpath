use crate::input::{
    error::{InputError, InputErrorConvertible},
    Input,
};
use rsonpath_syntax::str::JsonString;
use crate::result::InputRecorder;

#[cfg(target_arch = "x86")]
pub(super) mod mask_32;
#[cfg(target_arch = "x86_64")]
pub(super) mod mask_64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_128;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_256;

pub(crate) fn find_label_in_first_block<'i, 'r, I, R, const N: usize>(
    input: &I,
    first_block: I::Block,
    start_idx: usize,
    label: &JsonString,
) -> Result<Option<(usize, I::Block)>, InputError>
where
    I: Input<'i, 'r, R, N>,
    R: InputRecorder<I::Block> + 'r,
    'i: 'r,
{
    let block_idx = start_idx % N;
    let label_size = label.quoted().len();

    for (i, c) in first_block[block_idx..].iter().copied().enumerate() {
        let j = start_idx + i;

        if c == b'"' && input.is_member_match(j, j + label_size, label).e()? {
            return Ok(Some((j, first_block)));
        }
    }

    Ok(None)
}
