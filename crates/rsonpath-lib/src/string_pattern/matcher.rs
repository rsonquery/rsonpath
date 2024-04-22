pub(crate) mod avx2_64;
pub(crate) mod nosimd;
mod shared;
use crate::StringPattern;

pub trait StringPatternMatcher {
    fn pattern_match_forward(pattern: &StringPattern, input: &[u8]) -> Option<usize>;
    fn pattern_match_backward(pattern: &StringPattern, input: &[u8]) -> Option<usize>;
}

pub(crate) trait MatcherInput {
    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn offset(&mut self, offset: usize);

    fn offset_back(&mut self, offset: usize);

    fn read_u8(&self, idx: usize) -> u8;

    fn read_u16(&self, idx: usize) -> u16;

    fn read_u32(&self, idx: usize) -> u32;
}

impl<'a> MatcherInput for &'a [u8] {
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

impl<'a> MatcherInput for (&'a [u8], &'a [u8]) {
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

impl<'a> MatcherInput for (&'a [u8], &'a [u8], &'a [u8]) {
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
