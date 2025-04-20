use super::MatcherInput;
use crate::{string_pattern::AlternativeRepresentation, StringPattern};

const SLASH_U_CODE: u16 = u16::from_ne_bytes([b'\\', b'u']);
const ASCII_LOWERCASE: u32 = 0x2020_2020;

pub(super) enum AlternativeMatchResult {
    Continue(usize, usize),
    Mismatch,
}

#[inline(always)]
pub(super) fn attempt_alt_match_forward<I: MatcherInput>(
    pattern: &StringPattern,
    input: &I,
    pat_idx: usize,
) -> AlternativeMatchResult {
    match pattern.alternatives[pat_idx] {
        AlternativeRepresentation::None => AlternativeMatchResult::Mismatch,
        AlternativeRepresentation::SlashUSingle(code, pat_offset) => {
            if input.len() >= 6 && input.read_u16(0) == SLASH_U_CODE && input.read_u32(2) | ASCII_LOWERCASE == code {
                AlternativeMatchResult::Continue(6, pat_offset as usize)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
        AlternativeRepresentation::SlashUPair(code_1, code_2, pat_offset) => {
            if input.len() >= 12
                && input.read_u16(0) == SLASH_U_CODE
                && input.read_u32(2) | ASCII_LOWERCASE == code_1
                && input.read_u16(6) == SLASH_U_CODE
                && input.read_u32(8) | ASCII_LOWERCASE == code_2
            {
                AlternativeMatchResult::Continue(12, pat_offset as usize)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
        AlternativeRepresentation::USingle(code) => {
            if input.len() >= 5 && input.read_u8(0) == b'u' && input.read_u32(1) | ASCII_LOWERCASE == code {
                AlternativeMatchResult::Continue(5, 1)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
        AlternativeRepresentation::SlashByteOrUSingle(byte, code) => {
            if input.len() >= 2 && input.read_u8(0) == b'\\' && input.read_u8(1) == byte {
                AlternativeMatchResult::Continue(2, 1)
            } else if input.len() >= 6
                && input.read_u16(0) == SLASH_U_CODE
                && input.read_u32(2) | ASCII_LOWERCASE == code
            {
                AlternativeMatchResult::Continue(6, 1)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
    }
}

#[inline(always)]
pub(super) fn attempt_alt_match_backward<I: MatcherInput>(
    pattern: &StringPattern,
    input: &I,
    input_len: usize,
    pat_idx: usize,
) -> AlternativeMatchResult {
    match pattern.alternatives[pat_idx] {
        AlternativeRepresentation::None => AlternativeMatchResult::Mismatch,
        AlternativeRepresentation::SlashUSingle(code, pat_offset) => {
            if input.len() >= 6
                && input.read_u16(input_len - 6) == SLASH_U_CODE
                && input.read_u32(input_len - 4) | ASCII_LOWERCASE == code
            {
                AlternativeMatchResult::Continue(6, pat_offset as usize)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
        AlternativeRepresentation::SlashUPair(code_1, code_2, pat_offset) => {
            if input.len() >= 12
                && input.read_u16(input_len - 12) == SLASH_U_CODE
                && input.read_u32(input_len - 10) | ASCII_LOWERCASE == code_1
                && input.read_u16(input_len - 6) == SLASH_U_CODE
                && input.read_u32(input_len - 4) | ASCII_LOWERCASE == code_2
            {
                AlternativeMatchResult::Continue(12, pat_offset as usize)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
        AlternativeRepresentation::USingle(code) => {
            if input.len() >= 5
                && input.read_u8(input_len - 5) == b'u'
                && input.read_u32(input_len - 4) | ASCII_LOWERCASE == code
            {
                AlternativeMatchResult::Continue(5, 1)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
        AlternativeRepresentation::SlashByteOrUSingle(byte, code) => {
            if input.len() >= 2 && input.read_u8(input_len - 2) == b'\\' && input.read_u8(input_len - 1) == byte {
                AlternativeMatchResult::Continue(2, 1)
            } else if input.len() >= 6
                && input.read_u16(input_len - 6) == SLASH_U_CODE
                && input.read_u32(input_len - 4) | ASCII_LOWERCASE == code
            {
                AlternativeMatchResult::Continue(6, 1)
            } else {
                AlternativeMatchResult::Mismatch
            }
        }
    }
}
