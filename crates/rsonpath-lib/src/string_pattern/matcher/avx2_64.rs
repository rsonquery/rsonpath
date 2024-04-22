use super::{shared::AlternativeMatchResult, StringPatternMatcher};
use crate::string_pattern::glibc_impls;

pub(crate) struct Avx2StringMatcher64;

impl StringPatternMatcher for Avx2StringMatcher64 {
    fn pattern_match_forward(pattern: &crate::StringPattern, input: &[u8]) -> Option<usize> {
        let mut rem_pattern: &[u8] = &pattern.bytes;
        let mut input = input;
        let mut pat_idx = 0;
        let mut input_idx = 0;

        while !rem_pattern.is_empty() && rem_pattern.len() <= input.len() {
            if let Some(mismatch) =
                unsafe { glibc_impls::memcmpidx_avx2_64::cmpeq_forward(input, rem_pattern, rem_pattern.len()) }
            {
                rem_pattern = &rem_pattern[mismatch..];
                input = &input[mismatch..];
                input_idx += mismatch;
                pat_idx += mismatch;

                match super::shared::attempt_alt_match_forward(pattern, &input, pat_idx) {
                    AlternativeMatchResult::Continue(input_offset, pat_offset) => {
                        rem_pattern = &rem_pattern[pat_offset..];
                        input = &input[input_offset..];
                        input_idx += input_offset;
                        pat_idx += pat_offset;
                    }
                    AlternativeMatchResult::Mismatch => return None,
                }
            } else {
                return Some(input_idx + rem_pattern.len() - 1);
            }
        }

        #[allow(clippy::if_then_some_else_none)] // The -1 can overflow if the condition is false.
        if rem_pattern.is_empty() {
            Some(input_idx - 1)
        } else {
            None
        }
    }

    fn pattern_match_backward(pattern: &crate::StringPattern, input: &[u8]) -> Option<usize> {
        super::nosimd::NosimdStringMatcher::pattern_match_backward(pattern, input)
    }
}
