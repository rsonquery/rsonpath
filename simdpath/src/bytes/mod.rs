#[cfg(feature = "nosimd")]
pub use self::nosimd::*;
#[cfg(not(feature = "nosimd"))]
pub use self::simd::*;

pub fn find_unescaped_byte(byte: u8, slice: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < slice.len() {
        let j = find_byte(byte, &slice[i..]);
        match j {
            None => return None,
            Some(j) if j == 0 => return Some(j + i),
            Some(j) => {
                if slice[j + i - 1] != b'\\' {
                    return Some(j + i);
                } else {
                    i = j + i + 1;
                }
            }
        }
    }
    None
}

pub fn find_unescaped_byte2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
    let mut i = 0;
    while i < slice.len() {
        let j = find_byte2(byte1, byte2, &slice[i..]);
        match j {
            None => return None,
            Some(j) if j == 0 => return Some(j + i),
            Some(j) => {
                if slice[j + i - 1] != b'\\' {
                    return Some(j + i);
                } else {
                    i = j + i + 1;
                }
            }
        }
    }
    None
}

#[inline(always)]
pub fn find_non_whitespace(slice: &[u8]) -> Option<usize> {
    if slice.is_empty() {
        None
    } else {
        Some(0)
    }
}

#[cfg(not(feature = "nosimd"))]
mod simd {
    use memchr::*;

    #[inline(always)]
    pub fn find_byte(byte: u8, slice: &[u8]) -> Option<usize> {
        memchr(byte, slice)
    }

    #[inline(always)]
    pub fn find_byte2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
        memchr2(byte1, byte2, slice)
    }
}

#[cfg(feature = "nosimd")]
mod nosimd {

    #[inline(always)]
    pub fn find_byte(byte: u8, slice: &[u8]) -> Option<usize> {
        let mut i = 0;
        while i < slice.len() {
            if slice[i] == byte {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    #[inline(always)]
    pub fn find_byte2(byte1: u8, byte2: u8, slice: &[u8]) -> Option<usize> {
        let mut i = 0;
        while i < slice.len() {
            if slice[i] == byte1 || slice[i] == byte2 {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline(always)]
    fn find_non_whitespace(slice: &[u8]) -> Option<usize> {
        // Insignificant whitespace in JSON:
        // https://datatracker.ietf.org/doc/html/rfc4627#section-2
        const WHITESPACES: [u8; 4] = [b' ', b'\n', b'\t', b'\r'];
        let mut i = 0;
        while i < slice.len() {
            if !WHITESPACES.contains(&slice[i]) {
                return Some(i);
            }
            i += 1;
        }
        None
    }

    #[test]
    fn find_byte_when_there_are_no_bytes_returns_none() {
        let test_string = "";
        let test_bytes = test_string.as_bytes();

        let result = find_byte(b'{', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte_when_there_is_only_that_byte_returns_0() {
        let test_string = "{";
        let test_bytes = test_string.as_bytes();

        let result = find_byte(b'{', test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte_when_byte_exists_in_input_returns_first_occurence() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let test_bytes = test_string.as_bytes();

        let result = find_byte(b'"', test_bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_byte_when_byte_does_not_exist_in_input_returns_none() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let test_bytes = test_string.as_bytes();

        let result = find_byte(b'{', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte2_when_there_are_no_bytes_returns_none() {
        let test_string = "";
        let test_bytes = test_string.as_bytes();

        let result = find_byte2(b'{', b'}', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_byte2_when_there_is_only_first_byte_returns_0() {
        let test_string = "{";
        let test_bytes = test_string.as_bytes();

        let result = find_byte2(b'{', b'}', test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte2_when_there_is_only_second_byte_returns_0() {
        let test_string = "}";
        let test_bytes = test_string.as_bytes();

        let result = find_byte2(b'{', b'}', test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_byte2_when_byte_exists_in_input_returns_first_occurence_1() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let test_bytes = test_string.as_bytes();

        let result = find_byte2(b'"', b'L', test_bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_byte2_when_byte_exists_in_input_returns_first_occurence_2() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let test_bytes = test_string.as_bytes();

        let result = find_byte2(b'P', b'G', test_bytes);

        assert_eq!(Some(29), result);
    }

    #[test]
    fn find_byte2_when_neither_byte_does_not_exist_in_input_returns_none() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let test_bytes = test_string.as_bytes();

        let result = find_byte2(b'M', b'X', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_there_is_no_bytes_returns_none() {
        let test_string = "";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'{', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_there_is_only_that_byte_returns_0() {
        let test_string = "{";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'{', test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_unescaped_byte_when_all_matching_bytes_are_escaped_returns_none_1() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches"#;
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_all_matching_bytes_are_escaped_returns_none_2() {
        let test_string = r#"personne qui \"fait la plonge\" dans la restauration"#;
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_some_matching_bytes_are_escaped_returns_first_unescaped_1() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches",
        {
            "label": 123
        }"#;
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(Some(55), result);
    }

    #[test]
    fn find_unescaped_byte_when_some_matching_bytes_are_escaped_returns_first_unescaped_2() {
        let test_string = r#"personne qui \"fait la plonge\" dans la restauration
        },
        "en""#;
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(Some(72), result);
    }

    #[test]
    fn find_unescaped_byte2_when_there_is_no_bytes_returns_none() {
        let test_string = "";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'{', b'}', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte2_when_there_is_only_first_byte_returns_0() {
        let test_string = "{";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'{', b'}', test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_unescaped_byte2_when_there_is_only_second_byte_returns_0() {
        let test_string = "}";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'}', test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_unescaped_byte2_when_some_matching_bytes_are_escaped_returns_first_unescaped_1() {
        let test_string = r#"administrateur de \"La Libre Parole\", maire de Garches",
        {
            "label": 123
        }"#;
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'}', b'{', test_bytes);

        assert_eq!(Some(66), result);
    }

    #[test]
    fn find_unescaped_byte2_when_some_matching_bytes_are_escaped_returns_first_unescaped_2() {
        let test_string = r#"personne qui \"fait la plonge\" dans la restauration
        },
        "en""#;
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'}', test_bytes);

        assert_eq!(Some(61), result);
    }

    #[test]
    fn find_non_whitespace_byte_when_there_are_no_bytes_returns_none() {
        let test_string = "";
        let test_bytes = test_string.as_bytes();

        let result = find_non_whitespace(test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_non_whitespace_byte_when_there_is_only_one_non_whitespace_byte_returns_0() {
        let test_string = "x";
        let test_bytes = test_string.as_bytes();

        let result = find_non_whitespace(test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_when_there_is_leading_whitespace_returns_first_non_whitespace() {
        let test_string = " \t\n\r  \t  \n\t  \r \n\r  x";
        let test_bytes = test_string.as_bytes();

        let result = find_non_whitespace(test_bytes);

        assert_eq!(Some(19), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_vertical_tab_as_whitespace() {
        let test_bytes = [11]; // U+000B - VERTICAL TAB

        let result = find_non_whitespace(&test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_form_feed_as_whitespace() {
        let test_bytes = [12]; // U+000C - FORM FEED

        let result = find_non_whitespace(&test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_next_line_as_whitespace() {
        let test_bytes = [133]; // U+0085 - NEXT LINE

        let result = find_non_whitespace(&test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_non_whitespace_byte_does_not_treat_no_break_space_as_whitespace() {
        let test_bytes = [160]; // U+00A0 - NO-BREAK SPACE

        let result = find_non_whitespace(&test_bytes);

        assert_eq!(Some(0), result);
    }
}
