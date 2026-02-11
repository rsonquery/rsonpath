use crate::{
    debug,
    input::{
        error::{InputError, InputErrorConvertible as _},
        Input,
    },
    string_pattern::StringPattern,
};

#[inline(always)]
pub(crate) fn find_in_mask<I: Input>(
    input: &I,
    label: &StringPattern,
    previous_block: u64,
    first: u64,
    second: u64,
    offset: usize,
) -> Result<Option<usize>, InputError> {
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
