use crate::{
    input::{
        error::{InputError, InputErrorConvertible},
        Input,
    },
    string_pattern::{matcher::StringPatternMatcher, StringPattern},
};

#[cfg(target_arch = "x86")]
pub(super) mod mask_32;
#[cfg(target_arch = "x86_64")]
pub(super) mod mask_64;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_128;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub(super) mod vector_256;

pub(crate) fn find_label_in_first_block<'i, 'r, I, SM, const N: usize>(
    input: &I,
    first_block: I::Block<'i, N>,
    start_idx: usize,
    label: &StringPattern,
) -> Result<Option<(usize, usize, I::Block<'i, N>)>, InputError>
where
    I: Input,
    SM: StringPatternMatcher,
    'i: 'r,
{
    let block_idx = start_idx % N;

    for (i, c) in first_block[block_idx..].iter().copied().enumerate() {
        let j = start_idx + i;

        if c == b'"' {
            if let Some(to) = input.pattern_match_from::<SM>(j, label).e()? {
                return Ok(Some((j, to, first_block)));
            }
        }
    }

    Ok(None)
}
