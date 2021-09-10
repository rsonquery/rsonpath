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

    #[test]
    fn find_unescaped_byte_when_all_matching_bytes_are_escaped_returns_none_1() {
        let test_string = "administrateur de \\\"La Libre Parole\\\", maire de Garches";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_all_matching_bytes_are_escaped_returns_none_2() {
        let test_string = "personne qui \\\"fait la plonge\\\" dans la restauration";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte_when_some_matching_bytes_are_escaped_returns_first_unescaped_1() {
        let test_string =
            "administrateur de \\\"La Libre Parole\\\", maire de Garches\", { \"label\": 123 }";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(Some(55), result);
    }

    #[test]
    fn find_unescaped_byte_when_some_matching_bytes_are_escaped_returns_first_unescaped_2() {
        let test_string =
            "personne qui \\\"fait la plonge\\\" dans la restauration\n            },\n            \"en\"";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(Some(80), result);
    }

    #[test]
    fn find_unescaped_byte_when_there_is_only_that_byte_returns_0() {
        let test_string = "\"";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte(b'"', test_bytes);

        assert_eq!(Some(0), result);
    }

    #[test]
    fn find_unescaped_byte2_when_all_matching_bytes_are_escaped_returns_none_1() {
        let test_string = "administrateur de \\\"La Libre Parole\\\", maire de Garches";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'{', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte2_when_all_matching_bytes_are_escaped_returns_none_2() {
        let test_string = "personne qui \\\"fait la plonge\\\" dans la restauration";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'{', test_bytes);

        assert_eq!(None, result);
    }

    #[test]
    fn find_unescaped_byte2_when_some_matching_bytes_are_escaped_returns_first_unescaped_1() {
        let test_string =
            "administrateur de \\\"La Libre Parole\\\", maire de Garches, { \"label\": 123 }";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'{', test_bytes);

        assert_eq!(Some(57), result);
    }

    #[test]
    fn find_unescaped_byte2_when_some_matching_bytes_are_escaped_returns_first_unescaped_2() {
        let test_string =
            "personne qui \\\"fait la plonge\\\" dans la restauration\n            },\n            \"en\"";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'}', test_bytes);

        assert_eq!(Some(65), result);
    }

    #[test]
    fn find_unescaped_byte2_when_there_is_only_that_byte_returns_0() {
        let test_string = "\"";
        let test_bytes = test_string.as_bytes();

        let result = find_unescaped_byte2(b'"', b'}', test_bytes);

        assert_eq!(Some(0), result);
    }
}
