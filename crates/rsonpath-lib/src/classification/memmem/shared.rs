use crate::input::error::InputError;
use crate::input::Input;
use crate::query::JsonString;

#[cfg(target_arch = "x86")]
pub(super) mod mask_32;
#[cfg(target_arch = "x86_64")]
pub(super) mod mask_64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_128;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_256;

pub(crate) fn find_label_in_first_block<'i, 'r, I, const N: usize>(
    input: &I,
    first_block: I::Block<'i, N>,
    start_idx: usize,
    label: &JsonString,
) -> Result<Option<(usize, I::Block<'i, N>)>, InputError>
where
    I: Input,
    'i: 'r,
{
    let block_idx = start_idx % N;
    let label_size = label.bytes_with_quotes().len();

    let res = first_block[block_idx..].iter().copied().enumerate().find(|&(i, c)| {
        let j = start_idx + i;
        c == b'"' && input.is_member_match(j, j + label_size - 1, label)
    });
    if let Some((res, _)) = res {
        return Ok(Some((res + start_idx, first_block)));
    }

    Ok(None)
}
