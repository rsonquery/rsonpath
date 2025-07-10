use super::{shared::AlternativeMatchResult, MatcherInput, StringPatternMatcher};
use crate::string_pattern::matcher::nosimd::NosimdStringMatcher;

/// Experimental matcher based on the [`regex`] crate.
pub struct RegexStringMatcher;

impl StringPatternMatcher for RegexStringMatcher {
    fn pattern_match_forward(pattern: &crate::StringPattern, input: &[u8]) -> Option<usize> {
        pattern.regex_forward.find(input).map(|c| c.end() - 1)
    }

    fn pattern_match_backward(pattern: &crate::StringPattern, input: &[u8]) -> Option<usize> {
        pattern.regex_backward.find(input).map(|c| c.start())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        string_pattern::matcher::{regex::RegexStringMatcher, StringPatternMatcher},
        StringPattern,
    };
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
        let res_forward = RegexStringMatcher::pattern_match_forward(&pattern, str);
        let res_backward = RegexStringMatcher::pattern_match_backward(&pattern, str);

        assert_eq!(res_forward, Some(str.len() - 1));
        assert_eq!(res_backward, Some(0));
    }
}
