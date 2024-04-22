use super::{shared::AlternativeMatchResult, MatcherInput, StringPatternMatcher};
use crate::string_pattern::AlternativeRepresentation;

pub(crate) struct NosimdStringMatcher;

impl NosimdStringMatcher {
    pub(crate) fn pattern_match_forward<I: MatcherInput>(pattern: &crate::StringPattern, input: I) -> Option<usize> {
        let mut rem_pattern: &[u8] = &pattern.bytes;
        let mut input = input;
        let mut pat_idx = 0;
        let mut input_idx = 0;

        while !rem_pattern.is_empty() && rem_pattern.len() <= input.len() {
            if rem_pattern[0] == input.read_u8(0) {
                rem_pattern = &rem_pattern[1..];
                input.offset(1);
                input_idx += 1;
                pat_idx += 1;
            } else {
                match super::shared::attempt_alt_match_forward(pattern, &input, pat_idx) {
                    AlternativeMatchResult::Continue(input_offset, pat_offset) => {
                        rem_pattern = &rem_pattern[pat_offset..];
                        input.offset(input_offset);
                        input_idx += input_offset;
                        pat_idx += pat_offset;
                    }
                    AlternativeMatchResult::Mismatch => return None,
                }
            }
        }

        #[allow(clippy::if_then_some_else_none)] // The -1 can overflow if the condition is false.
        if rem_pattern.is_empty() {
            Some(input_idx - 1)
        } else {
            None
        }
    }

    pub(crate) fn pattern_match_backward<I: MatcherInput>(pattern: &crate::StringPattern, input: I) -> Option<usize> {
        let mut rem_pattern: &[u8] = &pattern.bytes;
        let mut input = input;
        let mut pat_len = rem_pattern.len();
        let mut input_len = input.len();

        while !rem_pattern.is_empty() && rem_pattern.len() <= input.len() {
            if rem_pattern[pat_len - 1] == input.read_u8(input_len - 1) {
                rem_pattern = &rem_pattern[..pat_len - 1];
                input.offset_back(1);
                input_len -= 1;
                pat_len -= 1;
                continue;
            } else if pat_len < pattern.alternatives.len() && input.read_u8(input_len - 1) == b'\\' {
                // When going backwards there's one nasty special case.
                // If the character ' or / is escaped it did match bytewise in the previous
                // iteration, but a backslash here should also be accepted.
                if let AlternativeRepresentation::SlashByteOrUSingle(_, _) = pattern.alternatives[pat_len] {
                    input.offset_back(1);
                    input_len -= 1;
                    continue;
                }
            }
            match super::shared::attempt_alt_match_backward(pattern, &input, input_len, pat_len - 1) {
                AlternativeMatchResult::Continue(input_offset, pat_offset) => {
                    rem_pattern = &rem_pattern[..pat_len - pat_offset];
                    input.offset_back(input_offset);
                    input_len -= input_offset;
                    pat_len -= pat_offset;
                }
                AlternativeMatchResult::Mismatch => return None,
            }
        }

        rem_pattern.is_empty().then_some(input_len)
    }
}

impl StringPatternMatcher for NosimdStringMatcher {
    fn pattern_match_forward(pattern: &crate::StringPattern, input: &[u8]) -> Option<usize> {
        Self::pattern_match_forward(pattern, input)
    }

    fn pattern_match_backward(pattern: &crate::StringPattern, input: &[u8]) -> Option<usize> {
        Self::pattern_match_backward(pattern, input)
    }
}
#[cfg(test)]
mod tests {
    use crate::{string_pattern::matcher::nosimd::NosimdStringMatcher, StringPattern};
    use rsonpath_syntax::str::JsonString;
    use test_case::test_case;

    #[test_case("abc\n\u{01F980}'abc", "\"abc\\n\u{01F980}'abc\""; "str1")]
    #[test_case("abc\n\u{01F980}'abc", "\"\\u0061bc\\n\u{01F980}'abc\""; "str2")]
    #[test_case("abc\n\u{01F980}'abc", "\"\\u0061bc\\u000a\u{01F980}'abc\""; "str3")]
    #[test_case("abc\n\u{01F980}'abc", "\"\\u0061bc\\u000A\u{01F980}'abc\""; "str4")]
    #[test_case("abc\n\u{01F980}'abc", "\"\\u0061bc\\u000A\u{01F980}\\'abc\""; "str5")]
    #[test_case("abc\n\u{01F980}'abc", "\"\\u0061bc\\u000A\\uD83E\\uDd80\\'abc\""; "str6")]
    fn test(pat: &str, str: &str) {
        let js = JsonString::new(pat);
        let pattern = StringPattern::new(&js);

        let str = str.as_bytes();
        let res_forward = NosimdStringMatcher::pattern_match_forward(&pattern, str);
        let res_backward = NosimdStringMatcher::pattern_match_backward(&pattern, str);

        assert_eq!(res_forward, Some(str.len() - 1));
        assert_eq!(res_backward, Some(0));

        for i in 0..str.len() {
            let (first, second) = str.split_at(i);
            let res_forward = NosimdStringMatcher::pattern_match_forward(&pattern, (first, second));
            let res_backward = NosimdStringMatcher::pattern_match_backward(&pattern, (first, second));

            assert_eq!(res_forward, Some(str.len() - 1));
            assert_eq!(res_backward, Some(0));
        }

        for i in 0..str.len() {
            let (first, second_and_third) = str.split_at(i);
            for j in 0..second_and_third.len() {
                let (second, third) = second_and_third.split_at(j);
                let res_forward = NosimdStringMatcher::pattern_match_forward(&pattern, (first, second, third));
                let res_backward = NosimdStringMatcher::pattern_match_backward(&pattern, (first, second, third));

                assert_eq!(res_forward, Some(str.len() - 1));
                assert_eq!(res_backward, Some(0));
            }
        }
    }
}
