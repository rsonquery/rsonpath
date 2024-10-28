use crate::{
    debug,
    input::{
        error::{InputError, InputErrorConvertible},
        Input,
    },
};
use rsonpath_syntax::str::JsonString;
use crate::result::InputRecorder;

#[inline(always)]
pub(crate) fn find_in_mask<'i, 'r, I, R, const N: usize>(
    input: &I,
    label: &JsonString,
    previous_block: u64,
    first: u64,
    second: u64,
    offset: usize,
) -> Result<Option<usize>, InputError>
where
    R: InputRecorder<I::Block> + 'r,
    I: Input<'i, 'r, R, N>
{
    let label_size = label.quoted().len();
    let mut result = (previous_block | (first << 1)) & second;
    while result != 0 {
        let idx = result.trailing_zeros() as usize;
        debug!("{offset} + {idx} - 2 to {offset} + {idx} + {label_size} - 3");
        if offset + idx > 1
            && input
                .is_member_match(offset + idx - 2, offset + idx + label_size - 2, label)
                .e()?
        {
            return Ok(Some(offset + idx - 2));
        }
        result &= !(1 << idx);
    }
    Ok(None)
}
