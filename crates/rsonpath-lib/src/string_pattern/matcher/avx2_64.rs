use std::{arch::x86_64::*, hint::unreachable_unchecked};

use super::{MatcherInput, StringPatternMatcher};
use crate::string_pattern::matcher::shared::AlternativeMatchResult;
use crate::{debug, StringPattern};

pub(crate) struct Avx2StringMatcher64;

impl StringPatternMatcher for Avx2StringMatcher64 {
    fn pattern_match_forward(pattern: &StringPattern, input: &[u8]) -> Option<usize> {
        // SAFETY: Avx2StringMatcher64 is only resolved in the simd module when the target features
        // are enabled.
        unsafe {
            return impl_(pattern, input);
        }

        #[target_feature(enable = "avx2")]
        #[target_feature(enable = "popcnt")]
        unsafe fn impl_(pattern: &StringPattern, input: &[u8]) -> Option<usize> {
            let mut rem_pattern: &[u8] = pattern.quoted();
            let mut input = input;
            let mut pat_idx = 0;
            let mut input_idx = 0;
            debug!(
                "{} cmpeq {}",
                std::str::from_utf8(rem_pattern).unwrap_or("[invalid UTF8]"),
                std::str::from_utf8(input).unwrap_or("[invalid UTF8]")
            );

            while !rem_pattern.is_empty() && rem_pattern.len() <= input.len() {
                let rem_ptr = rem_pattern.as_ptr();
                let in_ptr = input.as_ptr();
                match rem_pattern.len() {
                    0 => unreachable_unchecked(),
                    1 => {
                        if rem_ptr.read() == in_ptr.read() {
                            return Some(input_idx);
                        }
                        // Fallthrough to alt match.
                    }
                    2 => {
                        let rem_code = rem_ptr.cast::<u16>().read_unaligned();
                        let in_code = in_ptr.cast::<u16>().read_unaligned();
                        let xor = rem_code ^ in_code;
                        if xor == 0 {
                            return Some(input_idx + 1);
                        } else if xor > 0x00FF {
                            rem_pattern = &rem_pattern[1..];
                            input = &input[1..];
                            input_idx += 1;
                            pat_idx += 1;
                        }
                        // Fallthrough to alt match.
                    }
                    3 => {
                        let rem_code = rem_ptr.cast::<u16>().read_unaligned();
                        let in_code = in_ptr.cast::<u16>().read_unaligned();
                        let xor = rem_code ^ in_code;
                        if xor == 0 {
                            if rem_ptr.add(2).read() == in_ptr.add(2).read() {
                                return Some(input_idx + 2);
                            } else {
                                rem_pattern = &rem_pattern[2..];
                                input = &input[2..];
                                input_idx += 2;
                                pat_idx += 2;
                            }
                        } else if xor & 0x00FF == 0 {
                            rem_pattern = &rem_pattern[1..];
                            input = &input[1..];
                            input_idx += 1;
                            pat_idx += 1;
                        }
                        // Fallthrough to alt match.
                    }
                    4 => {
                        let rem_code = rem_ptr.cast::<u32>().read_unaligned();
                        let in_code = in_ptr.cast::<u32>().read_unaligned();
                        let xor = rem_code ^ in_code;
                        if xor == 0 {
                            return Some(input_idx + 3);
                        } else {
                            let mismatch = (xor.trailing_zeros() / 8) as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    5..=7 => {
                        let rem_code = rem_ptr.cast::<u32>().read_unaligned();
                        let in_code = in_ptr.cast::<u32>().read_unaligned();
                        let xor = rem_code ^ in_code;
                        if xor == 0 {
                            let offset = rem_pattern.len() ^ 4;
                            let rem_code = rem_ptr.add(offset).cast::<u32>().read_unaligned();
                            let in_code = in_ptr.add(offset).cast::<u32>().read_unaligned();
                            let xor = rem_code ^ in_code;
                            if xor == 0 {
                                return Some(input_idx + rem_pattern.len() - 1);
                            } else {
                                let mismatch = (xor.trailing_zeros() / 8) as usize + offset;
                                rem_pattern = &rem_pattern[mismatch..];
                                input = &input[mismatch..];
                                input_idx += mismatch;
                                pat_idx += mismatch;
                            }
                        } else {
                            let mismatch = (xor.trailing_zeros() / 8) as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    8 => {
                        let rem_code = rem_ptr.cast::<u64>().read_unaligned();
                        let in_code = in_ptr.cast::<u64>().read_unaligned();
                        let xor = rem_code ^ in_code;
                        if xor == 0 {
                            return Some(input_idx + 7);
                        } else {
                            let mismatch = (xor.trailing_zeros() / 8) as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    9..=15 => {
                        let rem_code = rem_ptr.cast::<u64>().read_unaligned();
                        let in_code = in_ptr.cast::<u64>().read_unaligned();
                        let xor = rem_code ^ in_code;
                        if xor == 0 {
                            let offset = rem_pattern.len() ^ 8;
                            let rem_code = rem_ptr.add(offset).cast::<u64>().read_unaligned();
                            let in_code = in_ptr.add(offset).cast::<u64>().read_unaligned();
                            let xor = rem_code ^ in_code;
                            if xor == 0 {
                                return Some(input_idx + rem_pattern.len() - 1);
                            } else {
                                let mismatch = (xor.trailing_zeros() / 8) as usize + offset;
                                rem_pattern = &rem_pattern[mismatch..];
                                input = &input[mismatch..];
                                input_idx += mismatch;
                                pat_idx += mismatch;
                            }
                        } else {
                            let mismatch = (xor.trailing_zeros() / 8) as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    16 => {
                        let rem_vec = _mm_loadu_si128(rem_ptr.cast());
                        let in_vec = _mm_loadu_si128(in_ptr.cast());
                        let cmpeq = _mm_cmpeq_epi8(rem_vec, in_vec);
                        let mask = _mm_movemask_epi8(cmpeq);
                        if mask == 0xFFFF {
                            return Some(input_idx + 15);
                        } else {
                            let mismatch = mask.trailing_ones() as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    17..=31 => {
                        let rem_vec = _mm_loadu_si128(rem_ptr.cast());
                        let in_vec = _mm_loadu_si128(in_ptr.cast());
                        let cmpeq = _mm_cmpeq_epi8(rem_vec, in_vec);
                        let mask = _mm_movemask_epi8(cmpeq);
                        if mask == 0xFFFF {
                            let offset = rem_pattern.len() ^ 16;
                            let rem_vec = _mm_loadu_si128(rem_ptr.add(offset).cast());
                            let in_vec = _mm_loadu_si128(in_ptr.add(offset).cast());
                            let cmpeq = _mm_cmpeq_epi8(rem_vec, in_vec);
                            let mask = _mm_movemask_epi8(cmpeq);
                            if mask == 0xFFFF {
                                return Some(input_idx + rem_pattern.len() - 1);
                            } else {
                                let mismatch = mask.trailing_ones() as usize + offset;
                                rem_pattern = &rem_pattern[mismatch..];
                                input = &input[mismatch..];
                                input_idx += mismatch;
                                pat_idx += mismatch;
                            }
                        } else {
                            let mismatch = mask.trailing_ones() as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    32 => {
                        let rem_vec = _mm256_loadu_si256(rem_ptr.cast());
                        let in_vec = _mm256_loadu_si256(in_ptr.cast());
                        let cmpeq = _mm256_cmpeq_epi8(rem_vec, in_vec);
                        let mask = _mm256_movemask_epi8(cmpeq) as u32;
                        if mask == 0xFFFF_FFFF {
                            return Some(input_idx + 31);
                        } else {
                            let mismatch = mask.trailing_ones() as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    33..=63 => {
                        let rem_vec = _mm256_loadu_si256(rem_ptr.cast());
                        let in_vec = _mm256_loadu_si256(in_ptr.cast());
                        let cmpeq = _mm256_cmpeq_epi8(rem_vec, in_vec);
                        let mask = _mm256_movemask_epi8(cmpeq) as u32;
                        if mask == 0xFFFF_FFFF {
                            let offset = rem_pattern.len() ^ 32;
                            let rem_vec = _mm256_loadu_si256(rem_ptr.add(offset).cast());
                            let in_vec = _mm256_loadu_si256(in_ptr.add(offset).cast());
                            let cmpeq = _mm256_cmpeq_epi8(rem_vec, in_vec);
                            let mask = _mm256_movemask_epi8(cmpeq) as u32;
                            if mask == 0xFFFF_FFFF {
                                return Some(input_idx + rem_pattern.len() - 1);
                            } else {
                                let mismatch = mask.trailing_ones() as usize + offset;
                                rem_pattern = &rem_pattern[mismatch..];
                                input = &input[mismatch..];
                                input_idx += mismatch;
                                pat_idx += mismatch;
                            }
                        } else {
                            let mismatch = mask.trailing_ones() as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                    _ => {
                        // >= 64
                        let rem_vec = _mm256_loadu_si256(rem_ptr.cast());
                        let in_vec = _mm256_loadu_si256(in_ptr.cast());
                        let cmpeq = _mm256_cmpeq_epi8(rem_vec, in_vec);
                        let mask = _mm256_movemask_epi8(cmpeq) as u32;
                        if mask == 0xFFFF_FFFF {
                            let rem_vec = _mm256_loadu_si256(rem_ptr.add(32).cast());
                            let in_vec = _mm256_loadu_si256(in_ptr.add(32).cast());
                            let cmpeq = _mm256_cmpeq_epi8(rem_vec, in_vec);
                            let mask = _mm256_movemask_epi8(cmpeq) as u32;
                            if mask == 0xFFFF_FFFF {
                                rem_pattern = &rem_pattern[64..];
                                input = &input[64..];
                                input_idx += 64;
                                pat_idx += 64;
                                continue;
                            } else {
                                let mismatch = mask.trailing_ones() as usize + 32;
                                rem_pattern = &rem_pattern[mismatch..];
                                input = &input[mismatch..];
                                input_idx += mismatch;
                                pat_idx += mismatch;
                            }
                        } else {
                            let mismatch = mask.trailing_ones() as usize;
                            rem_pattern = &rem_pattern[mismatch..];
                            input = &input[mismatch..];
                            input_idx += mismatch;
                            pat_idx += mismatch;
                        }
                        // Fallthrough to alt match.
                    }
                }
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

            #[allow(clippy::if_then_some_else_none)] // The -1 can overflow if the condition is false.
            if rem_pattern.is_empty() {
                Some(input_idx - 1)
            } else {
                None
            }
        }
    }

    fn pattern_match_backward(pattern: &crate::StringPattern, input: &[u8]) -> Option<usize> {
        super::nosimd::NosimdStringMatcher::pattern_match_backward(pattern, input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        string_pattern::matcher::{avx2_64::Avx2StringMatcher64, StringPatternMatcher},
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
        let res_forward = Avx2StringMatcher64::pattern_match_forward(&pattern, str);
        let res_backward = Avx2StringMatcher64::pattern_match_backward(&pattern, str);

        assert_eq!(res_forward, Some(str.len() - 1));
        assert_eq!(res_backward, Some(0));
    }
}
