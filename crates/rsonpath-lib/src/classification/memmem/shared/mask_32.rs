use crate::{
    input::{
        error::{InputError, InputErrorConvertible},
        Input,
    },
    string_pattern::{matcher::StringPatternMatcher, StringPattern},
};

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub(crate) fn find_in_mask<I: Input, SM: StringPatternMatcher>(
    input: &I,
    pattern: &StringPattern,
    previous_slash: u32,
    previous_quote: u32,
    previous_first: u32,
    first: u32,
    second: u32,
    slash: u32,
    quotes: u32,
    offset: usize,
) -> Result<Option<(usize, usize)>, InputError> {
    let slash_override = previous_slash | (slash << 1) | slash;
    let first_mask = (first << 1) | previous_first;
    let quote_mask = (quotes << 2) | previous_quote;
    let character_mask = first_mask & second & quote_mask;
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
