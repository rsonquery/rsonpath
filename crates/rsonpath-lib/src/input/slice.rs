use super::SliceSeekable;
use crate::string_pattern::{matcher::StringPatternMatcher, StringPattern};
use std::cmp;

impl<T: AsRef<[u8]>> SliceSeekable for T {
    fn pattern_match_from<M: StringPatternMatcher>(&self, from: usize, pattern: &StringPattern) -> Option<usize> {
        let bytes = self.as_ref();
        let to = from + pattern.len_limit();

        if from >= bytes.len() {
            return None;
        }

        let slice = &bytes[from..cmp::min(to, bytes.len())];
        let res = M::pattern_match_forward(pattern, slice)?;

        (from == 0 || bytes[from - 1] != b'\\').then_some(from + res)
    }

    fn pattern_match_to<M: StringPatternMatcher>(&self, to: usize, pattern: &StringPattern) -> Option<usize> {
        let bytes = self.as_ref();
        let from = to.saturating_sub(pattern.len_limit());

        let slice = &bytes[from..to];
        let idx = M::pattern_match_backward(pattern, slice)?;
        let in_bytes_idx = from + idx;

        (in_bytes_idx == 0 || bytes[in_bytes_idx - 1] != b'\\').then_some(in_bytes_idx)
    }

    fn seek_backward(&self, from: usize, needle: u8) -> Option<usize> {
        let bytes = self.as_ref();

        let mut idx = from;
        assert!(idx < bytes.len());

        loop {
            if bytes[idx] == needle {
                return Some(idx);
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }

    #[inline]
    fn seek_forward<const N: usize>(&self, from: usize, needles: [u8; N]) -> Option<(usize, u8)> {
        let bytes = self.as_ref();

        assert!(N > 0);
        let mut idx = from;

        if idx >= bytes.len() {
            return None;
        }

        loop {
            let b = bytes[idx];
            if needles.contains(&b) {
                return Some((idx, b));
            }
            idx += 1;
            if idx == bytes.len() {
                return None;
            }
        }
    }

    #[inline]
    fn seek_non_whitespace_forward(&self, from: usize) -> Option<(usize, u8)> {
        let bytes = self.as_ref();
        let mut idx = from;

        if idx >= bytes.len() {
            return None;
        }

        loop {
            let b = bytes[idx];
            if !b.is_ascii_whitespace() {
                return Some((idx, b));
            }
            idx += 1;
            if idx == bytes.len() {
                return None;
            }
        }
    }

    #[inline]
    fn seek_non_whitespace_backward(&self, from: usize) -> Option<(usize, u8)> {
        let bytes = self.as_ref();
        let mut idx = from;

        if idx >= bytes.len() {
            return None;
        }

        loop {
            let b = bytes[idx];
            if !b.is_ascii_whitespace() {
                return Some((idx, b));
            }
            if idx == 0 {
                return None;
            }
            idx -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    mod input_block_impl_for_slice {
        use crate::input::InputBlock;
        use pretty_assertions::assert_eq;

        #[test]
        fn halves_splits_in_half() {
            let bytes = r#"0123456789abcdef"#.as_bytes();

            let (half1, half2) = <&[u8] as InputBlock<16>>::halves(&bytes);

            assert_eq!(half1, "01234567".as_bytes());
            assert_eq!(half2, "89abcdef".as_bytes());
        }
    }

    mod seek_backward {
        use crate::input::SliceSeekable;
        use pretty_assertions::assert_eq;

        #[test]
        fn seeking_from_before_first_occurrence_returns_none() {
            let bytes = r#"{"seek":42}"#.as_bytes();

            let result = bytes.seek_backward(6, b':');

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_after_two_occurrences_returns_the_second_one() {
            let bytes = r#"{"seek":42,"find":37}"#.as_bytes();

            let result = bytes.seek_backward(bytes.len() - 1, b':');

            assert_eq!(result, Some(17));
        }
    }

    mod seek_forward_1 {
        use crate::input::SliceSeekable;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = bytes.seek_forward(0, [0]);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_needle_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = bytes.seek_forward(7, [b':']);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_not_needle_returns_next_needle() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = bytes.seek_forward(5, [b'2']);

            assert_eq!(result, Some((9, b'2')));
        }

        #[test]
        fn seeking_from_not_needle_when_there_is_no_needle_returns_none() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = bytes.seek_forward(5, [b'3']);

            assert_eq!(result, None);
        }
    }

    mod seek_forward_2 {
        use crate::input::SliceSeekable;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = bytes.seek_forward(0, [0, 1]);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_needle_1_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = bytes.seek_forward(7, [b':', b'4']);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_needle_2_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = bytes.seek_forward(7, [b'4', b':']);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_not_needle_when_next_is_needle_1_returns_that() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = bytes.seek_forward(5, [b'4', b'2']);

            assert_eq!(result, Some((8, b'4')));
        }

        #[test]
        fn seeking_from_not_needle_when_next_is_needle_2_returns_that() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = bytes.seek_forward(5, [b'2', b'4']);

            assert_eq!(result, Some((8, b'4')));
        }

        #[test]
        fn seeking_from_not_needle_when_there_is_no_needle_returns_none() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = bytes.seek_forward(5, [b'3', b'0']);

            assert_eq!(result, None);
        }
    }

    mod seek_non_whitespace_forward {
        use crate::input::SliceSeekable;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = bytes.seek_non_whitespace_forward(0);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_non_whitespace_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = bytes.seek_non_whitespace_forward(7);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_whitespace_returns_next_non_whitespace() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = bytes.seek_non_whitespace_forward(5);

            assert_eq!(result, Some((8, b'4')));
        }

        #[test]
        fn seeking_from_whitespace_when_there_is_no_more_non_whitespace_returns_none() {
            let bytes = "seek: \t\n ".as_bytes();

            let result = bytes.seek_non_whitespace_forward(5);

            assert_eq!(result, None);
        }
    }

    mod seek_non_whitespace_backward {
        use crate::input::SliceSeekable;
        use pretty_assertions::assert_eq;

        #[test]
        fn in_empty_slice_returns_none() {
            let bytes = [];

            let result = bytes.seek_non_whitespace_backward(0);

            assert_eq!(result, None);
        }

        #[test]
        fn seeking_from_non_whitespace_returns_that() {
            let bytes = r#"{"seek": 42}"#.as_bytes();

            let result = bytes.seek_non_whitespace_backward(7);

            assert_eq!(result, Some((7, b':')));
        }

        #[test]
        fn seeking_from_whitespace_returns_previous_non_whitespace() {
            let bytes = "seek: \t\n42}".as_bytes();

            let result = bytes.seek_non_whitespace_backward(7);

            assert_eq!(result, Some((4, b':')));
        }
    }

    mod pattern_match_from {
        use crate::{
            input::SliceSeekable,
            string_pattern::{matcher::nosimd::NosimdStringMatcher, StringPattern},
        };
        use pretty_assertions::assert_eq;
        use rsonpath_syntax::str::JsonString;

        #[test]
        fn on_exact_match_returns_true() {
            let bytes = r#"{"needle":42,"other":37}"#.as_bytes();

            let result =
                bytes.pattern_match_from::<NosimdStringMatcher>(1, &StringPattern::new(&JsonString::new("needle")));

            assert_eq!(result, Some(8));
        }

        #[test]
        fn matching_without_double_quotes_returns_false() {
            let bytes = r#"{"needle":42,"other":37}"#.as_bytes();

            let result =
                bytes.pattern_match_from::<NosimdStringMatcher>(2, &StringPattern::new(&JsonString::new("needle")));

            assert_eq!(result, None);
        }

        #[test]
        fn when_match_is_partial_due_to_escaped_double_quote_returns_false() {
            let bytes = r#"{"fake\"needle":42,"other":37}"#.as_bytes();

            let result =
                bytes.pattern_match_from::<NosimdStringMatcher>(7, &StringPattern::new(&JsonString::new("needle")));

            assert_eq!(result, None);
        }

        #[test]
        #[ignore = "proper unicode and escape support is not implemented yet"]
        fn when_looking_for_string_with_escaped_double_quote_returns_true() {
            let bytes = r#"{"fake\"needle":42,"other":37}"#.as_bytes();

            let result = bytes
                .pattern_match_from::<NosimdStringMatcher>(1, &StringPattern::new(&JsonString::new(r#"fake"needle"#)));

            assert_eq!(result, Some(15));
        }
    }
}
