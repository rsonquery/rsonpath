use std::fmt::Debug;

use rsonpath_syntax::str::JsonString;

#[derive(Clone)]
pub struct StringPattern {
    bytes: Vec<u8>,
    alternatives: Vec<AlternativeRepresentation>,
    len_limit: usize,
}

impl std::hash::Hash for StringPattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl PartialOrd for StringPattern {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.bytes.cmp(&other.bytes))
    }
}

impl Ord for StringPattern {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.bytes.cmp(&other.bytes)
    }
}

impl PartialEq for StringPattern {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for StringPattern {}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AlternativeRepresentation {
    SlashUSingle(u32, u8),
    SlashUPair(u32, u32, u8),
    USingle(u32),
    SlashByteOrUSingle(u8, u32),
    None,
}

struct StringPatternBuilder {
    bytes: Vec<u8>,
    alternatives: Vec<AlternativeRepresentation>,
    len_limit: usize,
}

impl StringPattern {
    pub fn unquoted(&self) -> &[u8] {
        &self.bytes[1..self.bytes.len() - 1]
    }

    pub fn quoted(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len_limit(&self) -> usize {
        self.len_limit
    }

    #[inline]
    #[must_use]
    pub fn new(string: &JsonString) -> Self {
        // A pattern to be matched consists of the bytes that should be matched in the "canonical"
        // representation of the string (the shortest possible valid representation), and possible
        // alternative escapes that should be considered if a mismatch occurs
        // at a given position relative to the canonical bytes.
        // We have the following cases:
        //   - The character is a control character or a special symbol that is canonically represented
        //     as backslash-itself. If it is mismatched at the backslash, there is no match alternative
        //     representation; on the second byte it can be replaced with uXXXX.
        //   - The character is a control character that can only be represented as a unicode escape;
        //     it has no alternative encodings.
        //   - The character is one of the two awfully designed JSON special cases:
        //     forward slash (/) or single quote ('). The canonical form of them is themselves, but they
        //     can also be present escaped (\/ or \'), or as a unicode escape.
        //   - The character is a "regular" character; it has only one alternative encoding - unicode
        //     escape, which is either a single sequence \uXXXX or a pair \uXXXX\uXXXX.
        let byte_length = string.quoted().len();
        let mut builder = StringPatternBuilder::new(byte_length);

        for char in string.unquoted().chars() {
            match char {
                '\u{0008}' => builder.short_escape(b'b', char),
                '\u{000C}' => builder.short_escape(b'f', char),
                '\n' => builder.short_escape(b'n', char),
                '\r' => builder.short_escape(b'r', char),
                '\t' => builder.short_escape(b't', char),
                '"' => builder.short_escape(b'"', char),
                '\\' => builder.short_escape(b'\\', char),
                '\u{0000}'..='\u{001F}' => builder.long_escape(char),
                '/' | '\'' => builder.special_escape(char),
                _ => builder.regular_escape(char),
            };
        }

        builder.into_pattern()
    }
}

impl StringPatternBuilder {
    fn new(byte_len: usize) -> Self {
        let mut this = Self {
            bytes: Vec::with_capacity(byte_len),
            alternatives: Vec::with_capacity(byte_len),
            len_limit: 0,
        };
        this.bytes.push(b'"');
        this.alternatives.push(AlternativeRepresentation::None);
        this.len_limit += 1;

        this
    }

    fn into_pattern(mut self) -> StringPattern {
        self.bytes.push(b'"');
        self.alternatives.push(AlternativeRepresentation::None);
        self.len_limit += 1;

        return StringPattern {
            bytes: self.bytes,
            alternatives: self.alternatives,
            len_limit: self.len_limit,
        };
    }

    fn short_escape(&mut self, code_letter: u8, c: char) {
        self.bytes.push(b'\\');
        self.bytes.push(code_letter);

        let mut utf16_buf = [0; 1];
        let utf16 = c.encode_utf16(&mut utf16_buf);
        let code = Self::encode(utf16[0]);

        self.alternatives.push(AlternativeRepresentation::None);
        self.alternatives.push(AlternativeRepresentation::USingle(code));

        self.len_limit += 6;
    }

    fn long_escape(&mut self, c: char) {
        self.bytes.push(b'\\');
        self.bytes.push(b'u');
        self.bytes.push(b'0');
        self.bytes.push(b'0');
        self.bytes.push(Self::encode_nibble((c as u8 & 0xF0) >> 4));
        self.bytes.push(Self::encode_nibble(c as u8 & 0x0F));

        for _ in 0..6 {
            self.alternatives.push(AlternativeRepresentation::None);
        }

        self.len_limit += 6;
    }

    fn special_escape(&mut self, c: char) {
        self.bytes.push(c as u8);

        let mut utf16_buf = [0; 1];
        let utf16 = c.encode_utf16(&mut utf16_buf);
        let code = Self::encode(utf16[0]);

        self.alternatives
            .push(AlternativeRepresentation::SlashByteOrUSingle(c as u8, code));

        self.len_limit += 6;
    }

    fn regular_escape(&mut self, c: char) {
        let mut utf8_buf = [0; 4];
        let mut utf16_buf = [0; 2];
        let utf8 = c.encode_utf8(&mut utf8_buf);
        let utf16 = c.encode_utf16(&mut utf16_buf);

        self.bytes.extend_from_slice(utf8.as_bytes());
        let len = utf8.len();
        let repr;

        if utf16.len() == 1 {
            let code = Self::encode(utf16[0]);
            repr = AlternativeRepresentation::SlashUSingle(code, len as u8);
            self.alternatives.push(repr);
            self.len_limit += 6;
        } else {
            let code1 = Self::encode(utf16[0]);
            let code2 = Self::encode(utf16[1]);
            repr = AlternativeRepresentation::SlashUPair(code1, code2, len as u8);
            self.alternatives.push(repr);
            self.len_limit += 12;
        }

        for _ in 1..utf8.len() {
            self.alternatives.push(AlternativeRepresentation::None);
        }
        let last_idx = self.alternatives.len() - 1;
        self.alternatives[last_idx] = repr;
    }

    fn encode(utf16: u16) -> u32 {
        let bytes = utf16.to_be_bytes();
        let mut result = [0; 4];
        result[0] = Self::encode_nibble((bytes[0] & 0xF0) >> 4);
        result[1] = Self::encode_nibble(bytes[0] & 0x0F);
        result[2] = Self::encode_nibble((bytes[1] & 0xF0) >> 4);
        result[3] = Self::encode_nibble(bytes[1] & 0x0F);

        u32::from_ne_bytes(result)
    }

    fn encode_nibble(nibble: u8) -> u8 {
        match nibble {
            0x00..=0x09 => b'0' + nibble,
            0x0A..=0x0F => b'a' + nibble - 0x0A,
            _ => unreachable!(),
        }
    }
}

pub fn cmpeq_forward<I: CmpeqInput>(pattern: &StringPattern, input: I) -> Option<usize> {
    const SLASH_U_CODE: u16 = u16::from_ne_bytes([b'\\', b'u']);

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
            match pattern.alternatives[pat_idx] {
                AlternativeRepresentation::None => return None,
                AlternativeRepresentation::SlashUSingle(code, pat_offset) => {
                    if input.len() >= 6 && input.read_u16(0) == SLASH_U_CODE && input.read_u32(2) | 0x20202020 == code {
                        input.offset(6);
                        input_idx += 6;
                        rem_pattern = &rem_pattern[pat_offset as usize..];
                        pat_idx += pat_offset as usize;
                        continue;
                    } else {
                        return None;
                    }
                }
                AlternativeRepresentation::SlashUPair(code_1, code_2, pat_offset) => {
                    if input.len() >= 12
                        && input.read_u16(0) == SLASH_U_CODE
                        && input.read_u32(2) | 0x20202020 == code_1
                        && input.read_u16(6) == SLASH_U_CODE
                        && input.read_u32(8) | 0x20202020 == code_2
                    {
                        input.offset(12);
                        input_idx += 12;
                        rem_pattern = &rem_pattern[pat_offset as usize..];
                        pat_idx += pat_offset as usize;
                        continue;
                    } else {
                        return None;
                    }
                }
                AlternativeRepresentation::USingle(code) => {
                    if input.len() >= 5 && input.read_u8(0) == b'u' && input.read_u32(1) | 0x20202020 == code {
                        input.offset(5);
                        input_idx += 5;
                        rem_pattern = &rem_pattern[1..];
                        pat_idx += 1;
                        continue;
                    } else {
                        return None;
                    }
                }
                AlternativeRepresentation::SlashByteOrUSingle(byte, code) => {
                    if input.len() >= 2 && input.read_u8(0) == b'\\' && input.read_u8(1) == byte {
                        input.offset(2);
                        input_idx += 2;
                        rem_pattern = &rem_pattern[1..];
                        pat_idx += 1;
                        continue;
                    } else if input.len() >= 6
                        && input.read_u16(0) == SLASH_U_CODE
                        && input.read_u32(2) | 0x20202020 == code
                    {
                        input.offset(6);
                        input_idx += 6;
                        rem_pattern = &rem_pattern[1..];
                        pat_idx += 1;
                        continue;
                    } else {
                        return None;
                    }
                }
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

pub fn cmpeq_backward<I: CmpeqInput>(pattern: &StringPattern, input: I) -> Option<usize> {
    const SLASH_U_CODE: u16 = u16::from_ne_bytes([b'\\', b'u']);

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
        match pattern.alternatives[pat_len - 1] {
            AlternativeRepresentation::None => return None,
            AlternativeRepresentation::SlashUSingle(code, pat_offset) => {
                if input.len() >= 6
                    && input.read_u16(input_len - 6) == SLASH_U_CODE
                    && input.read_u32(input_len - 4) | 0x20202020 == code
                {
                    input.offset_back(6);
                    input_len -= 6;
                    rem_pattern = &rem_pattern[..pat_len - pat_offset as usize];
                    pat_len -= pat_offset as usize;
                    continue;
                } else {
                    return None;
                }
            }
            AlternativeRepresentation::SlashUPair(code_1, code_2, pat_offset) => {
                if input.len() >= 12
                    && input.read_u16(input_len - 12) == SLASH_U_CODE
                    && input.read_u32(input_len - 10) | 0x20202020 == code_1
                    && input.read_u16(input_len - 6) == SLASH_U_CODE
                    && input.read_u32(input_len - 4) | 0x20202020 == code_2
                {
                    input.offset_back(12);
                    input_len -= 12;
                    rem_pattern = &rem_pattern[..pat_len - pat_offset as usize];
                    pat_len -= pat_offset as usize;
                    continue;
                } else {
                    return None;
                }
            }
            AlternativeRepresentation::USingle(code) => {
                if input.len() >= 5
                    && input.read_u8(input_len - 5) == b'u'
                    && input.read_u32(input_len - 4) | 0x20202020 == code
                {
                    input.offset_back(5);
                    input_len -= 5;
                    rem_pattern = &rem_pattern[..pat_len - 1];
                    pat_len -= 1;
                    continue;
                } else {
                    return None;
                }
            }
            AlternativeRepresentation::SlashByteOrUSingle(byte, code) => {
                if input.len() >= 2 && input.read_u8(input_len - 2) == b'\\' && input.read_u8(input_len - 1) == byte {
                    input.offset_back(2);
                    input_len -= 2;
                    rem_pattern = &rem_pattern[..pat_len - 1];
                    pat_len -= 1;
                    continue;
                } else if input.len() >= 6
                    && input.read_u16(input_len - 6) == SLASH_U_CODE
                    && input.read_u32(input_len - 4) | 0x20202020 == code
                {
                    input.offset_back(6);
                    input_len -= 6;
                    rem_pattern = &rem_pattern[..pat_len - 1];
                    pat_len -= 1;
                    continue;
                } else {
                    return None;
                }
            }
        }
    }

    rem_pattern.is_empty().then_some(input_len)
}

pub trait CmpeqInput {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn offset(&mut self, offset: usize);

    fn offset_back(&mut self, offset: usize);

    fn read_u8(&self, idx: usize) -> u8;

    fn read_u16(&self, idx: usize) -> u16;

    fn read_u32(&self, idx: usize) -> u32;
}

impl<'a> CmpeqInput for &'a [u8] {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        <[u8]>::is_empty(self)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    #[inline(always)]
    fn offset(&mut self, offset: usize) {
        *self = &self[offset..];
    }

    #[inline(always)]
    fn offset_back(&mut self, offset: usize) {
        *self = &self[..self.len() - offset];
    }

    #[inline(always)]
    fn read_u8(&self, idx: usize) -> u8 {
        self[idx]
    }

    #[inline(always)]
    fn read_u16(&self, idx: usize) -> u16 {
        u16::from_ne_bytes(self[idx..idx + 2].try_into().expect("length 2"))
    }

    #[inline(always)]
    fn read_u32(&self, idx: usize) -> u32 {
        u32::from_ne_bytes(self[idx..idx + 4].try_into().expect("length 4"))
    }
}

impl<'a> CmpeqInput for (&'a [u8], &'a [u8]) {
    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    fn offset(&mut self, offset: usize) {
        let first_offset = std::cmp::min(self.0.len(), offset);
        let second_offset = offset - first_offset;
        *self = (&self.0[first_offset..], &self.1[second_offset..])
    }

    // (10, 3), offset by 4
    // second_offset = 3 - 4 = 0
    // first_offset = 10 - (4 - (3 - 0)) = 9
    // (9, 0)
    fn offset_back(&mut self, offset: usize) {
        let second_offset = self.1.len().saturating_sub(offset);
        let first_offset = self.0.len() - (offset - (self.1.len() - second_offset));
        *self = (&self.0[..first_offset], &self.1[..second_offset])
    }

    fn read_u8(&self, idx: usize) -> u8 {
        if idx < self.0.len() {
            self.0[idx]
        } else {
            self.1[idx - self.0.len()]
        }
    }

    fn read_u16(&self, idx: usize) -> u16 {
        let b1 = self.read_u8(idx);
        let b2 = self.read_u8(idx + 1);
        u16::from_ne_bytes([b1, b2])
    }

    fn read_u32(&self, idx: usize) -> u32 {
        let b1 = self.read_u8(idx);
        let b2 = self.read_u8(idx + 1);
        let b3 = self.read_u8(idx + 2);
        let b4 = self.read_u8(idx + 3);
        u32::from_ne_bytes([b1, b2, b3, b4])
    }
}

impl<'a> CmpeqInput for (&'a [u8], &'a [u8], &'a [u8]) {
    fn is_empty(&self) -> bool {
        self.0.is_empty() && self.1.is_empty() && self.2.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len() + self.1.len() + self.2.len()
    }

    fn offset(&mut self, offset: usize) {
        let first_offset = std::cmp::min(self.0.len(), offset);
        let second_offset_base = offset - first_offset;
        let second_offset = std::cmp::min(self.1.len(), second_offset_base);
        let third_offset = second_offset_base - second_offset;
        *self = (
            &self.0[first_offset..],
            &self.1[second_offset..],
            &self.2[third_offset..],
        )
    }

    // (10, 3, 4), offset by 10
    // third_offset = 4 - 10 = 0
    // second_offset_base = 10 - (4 - 0) = 6
    // second_offset = 3 - 6 = 0
    // first_offset = 10 - (6 - (3 - 0)) = 7
    // (7, 0, 0)
    fn offset_back(&mut self, offset: usize) {
        let third_offset = self.2.len().saturating_sub(offset);
        let second_offset_base = offset - (self.2.len() - third_offset);
        let second_offset = self.1.len().saturating_sub(second_offset_base);
        let first_offset = self.0.len() - (second_offset_base - (self.1.len() - second_offset));
        *self = (
            &self.0[..first_offset],
            &self.1[..second_offset],
            &self.2[..third_offset],
        )
    }

    fn read_u8(&self, idx: usize) -> u8 {
        if idx < self.0.len() {
            self.0[idx]
        } else {
            let idx_2 = idx - self.0.len();
            if idx_2 < self.1.len() {
                self.1[idx_2]
            } else {
                self.2[idx_2 - self.1.len()]
            }
        }
    }

    fn read_u16(&self, idx: usize) -> u16 {
        let b1 = self.read_u8(idx);
        let b2 = self.read_u8(idx + 1);
        u16::from_ne_bytes([b1, b2])
    }

    fn read_u32(&self, idx: usize) -> u32 {
        let b1 = self.read_u8(idx);
        let b2 = self.read_u8(idx + 1);
        let b3 = self.read_u8(idx + 2);
        let b4 = self.read_u8(idx + 3);
        u32::from_ne_bytes([b1, b2, b3, b4])
    }
}

impl Debug for StringPattern {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f
            .debug_struct("StringPattern")
            .field("bytes", &self.bytes.iter().copied().map(DebugByte).collect::<Vec<_>>())
            .field("as_string", &std::str::from_utf8(&self.bytes).unwrap())
            .field("alternatives", &self.alternatives)
            .finish();
    }
}

impl Debug for AlternativeRepresentation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SlashUSingle(arg0, arg1) => f
                .debug_tuple("SlashUSingle")
                .field(&DebugCode(*arg0))
                .field(arg1)
                .finish(),
            Self::SlashUPair(arg0, arg1, arg2) => f
                .debug_tuple("SlashUPair")
                .field(&DebugCode(*arg0))
                .field(&DebugCode(*arg1))
                .field(arg2)
                .finish(),
            Self::USingle(arg0) => f.debug_tuple("USingle").field(&DebugCode(*arg0)).finish(),
            Self::SlashByteOrUSingle(arg0, arg1) => f
                .debug_tuple("SlashByteOrUSingle")
                .field(&DebugByte(*arg0))
                .field(&DebugCode(*arg1))
                .finish(),
            Self::None => write!(f, "None"),
        }
    }
}

struct DebugByte(u8);
struct DebugCode(u32);

impl Debug for DebugByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0x20..=0x7F => write!(f, "b'{}'", self.0 as char),
            _ => write!(f, "0x{:0>2x}", self.0),
        }
    }
}

impl Debug for DebugCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:0>8x}", self.0)
    }
}

#[cfg(test)]
mod tests {
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
        let pattern = super::StringPattern::new(&js);

        let str = str.as_bytes();
        let res_forward = super::cmpeq_forward(&pattern, str);
        let res_backward = super::cmpeq_backward(&pattern, str);

        assert_eq!(res_forward, Some(str.len() - 1));
        assert_eq!(res_backward, Some(0));

        for i in 0..str.len() {
            let (first, second) = str.split_at(i);
            let res_forward = super::cmpeq_forward(&pattern, (first, second));
            let res_backward = super::cmpeq_backward(&pattern, (first, second));

            assert_eq!(res_forward, Some(str.len() - 1));
            assert_eq!(res_backward, Some(0));
        }

        for i in 0..str.len() {
            let (first, second_and_third) = str.split_at(i);
            for j in 0..second_and_third.len() {
                let (second, third) = second_and_third.split_at(j);
                let res_forward = super::cmpeq_forward(&pattern, (first, second, third));
                let res_backward = super::cmpeq_backward(&pattern, (first, second, third));

                assert_eq!(res_forward, Some(str.len() - 1));
                assert_eq!(res_backward, Some(0));
            }
        }
    }
}
