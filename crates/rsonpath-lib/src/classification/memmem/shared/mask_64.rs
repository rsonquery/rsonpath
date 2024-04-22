use crate::{
    input::{
        error::{InputError, InputErrorConvertible},
        Input,
    },
    string_pattern::{matcher::StringPatternMatcher, StringPattern},
};

#[inline(always)]
pub(crate) fn find_in_mask<I: Input, SM: StringPatternMatcher>(
    input: &I,
    pattern: &StringPattern,
    previous_block: u64,
    first: u64,
    second: u64,
    slash: u64,
    offset: usize,
) -> Result<Option<(usize, usize)>, InputError> {
    let slash_override = (previous_block | (slash << 1)) | slash;
    let character_mask = (previous_block | (first << 1)) & second;
    let mut result = slash_override | character_mask;
    while result != 0 {
        let idx = result.trailing_zeros() as usize;
        if offset + idx > 1 {
            if let Some(to) = input.pattern_match_from::<SM>(offset + idx - 2, pattern).e()? {
                return Ok(Some((offset + idx - 2, to)));
            }
        }
        result &= !(1 << idx);
    }
    Ok(None)
}
