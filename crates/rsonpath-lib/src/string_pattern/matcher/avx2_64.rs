use std::{arch::x86_64::*, hint::unreachable_unchecked};

use super::{shared::AlternativeMatchResult, StringPatternMatcher};
use crate::{
    classification::simd::dispatch_simd,
    debug,
    string_pattern::{glibc_impls, AlternativeRepresentation},
    StringPattern,
};

pub(crate) struct Avx2StringMatcher64;

impl StringPatternMatcher for Avx2StringMatcher64 {
    fn pattern_match_forward(pattern: &StringPattern, input: &[u8]) -> Option<usize> {
        unsafe {
            return impl_(pattern, input);
        }
        /*let mut rem_pattern: &[u8] = &pattern.bytes();
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
        }*/

        #[target_feature(enable = "avx2")]
        #[target_feature(enable = "popcnt")]
        unsafe fn impl_(pattern: &StringPattern, input: &[u8]) -> Option<usize> {
            const SLASH_U_CODE: u16 = u16::from_ne_bytes([b'\\', b'u']);

            let mut rem_pattern: &[u8] = &pattern.quoted();
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
                        if mask == 0xFFFFFFFF {
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
                        if mask == 0xFFFFFFFF {
                            let offset = rem_pattern.len() ^ 32;
                            let rem_vec = _mm256_loadu_si256(rem_ptr.add(offset).cast());
                            let in_vec = _mm256_loadu_si256(in_ptr.add(offset).cast());
                            let cmpeq = _mm256_cmpeq_epi8(rem_vec, in_vec);
                            let mask = _mm256_movemask_epi8(cmpeq) as u32;
                            if mask == 0xFFFFFFFF {
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
                    _ => todo!(),
                }
                match pattern.alternatives[pat_idx] {
                    AlternativeRepresentation::None => return None,
                    AlternativeRepresentation::SlashUSingle(code, pat_offset) => {
                        if input.len() >= 6
                            && input.read_u16(0) == SLASH_U_CODE
                            && input.read_u32(2) | 0x20202020 == code
                        {
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
pub fn cmpeq_forward<I: CmpeqInput>(pattern: &StringPattern, input: I) -> Option<usize> {
    const SLASH_U_CODE: u16 = u16::from_ne_bytes([b'\\', b'u']);

    let mut rem_pattern: &[u8] = &pattern.quoted();
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

    let mut rem_pattern: &[u8] = &pattern.quoted();
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
