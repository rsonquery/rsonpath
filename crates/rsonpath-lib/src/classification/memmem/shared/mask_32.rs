use crate::{input::Input, query::JsonString};

#[inline(always)]
pub(crate) fn find_in_mask<I: Input>(
    input: &I,
    label: &JsonString,
    previous_block: u32,
    first: u32,
    second: u32,
    offset: usize,
) -> Result<Option<usize>, InputError> {
    let label_size = label.bytes_with_quotes().len();
    let mut result = (previous_block | (first << 1)) & second;
    while result != 0 {
        let idx = result.trailing_zeros() as usize;
        if offset + idx > 1
            && input
                .is_member_match(offset + idx - 2, offset + idx + label_size - 3, label)
                .map_err(|x| x.into())?
        {
            return Some(offset + idx - 2);
        }
        result &= !(1 << idx);
    }
    None
}
