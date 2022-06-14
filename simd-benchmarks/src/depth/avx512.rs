use aligners::{alignment::TwoTo, AlignedSlice};

use super::*;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const BYTES_IN_AVX512_REGISTER: usize = 512 / 8;

/// Works on a 64-byte slice, but uses a heuristic to quickly
/// respond to queries and not count the depth exactly unless
/// needed.
///
/// The heuristic checks if it is possible to achieve the queried
/// depth within the block by counting the number of opening
/// and closing structural characters. This can be done much
/// more quickly than precise depth calculation.
#[cfg_attr(
    docsrs,
    doc(cfg(all(
        feature = "avx512",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        any(target_arch = "x86", target_arch = "x86_64")
    )))
)]
pub struct LazyVector {
    opening_mask: u64,
    closing_mask: u64,
    len: usize,
    rev_idx: usize,
}

impl LazyVector {
    fn new_sequential(bytes: &AlignedSlice<TwoTo<6>>) -> (Self, &AlignedSlice<TwoTo<6>>) {
        let mut vector = Self {
            opening_mask: 0,
            closing_mask: 0,
            len: bytes.len(),
            rev_idx: bytes.len() - 1,
        };
        for byte in bytes.iter() {
            vector.opening_mask >>= 1;
            vector.closing_mask >>= 1;
            match byte {
                b'{' => {
                    vector.opening_mask |= 1 << 63;
                }
                b'[' => {
                    vector.opening_mask |= 1 << 63;
                }
                b'}' => {
                    vector.closing_mask |= 1 << 63;
                }
                b']' => {
                    vector.closing_mask |= 1 << 63;
                }
                _ => (),
            };
        }

        (vector, Default::default())
    }

    #[target_feature(enable = "avx512f")]
    #[target_feature(enable = "avx512bw")]
    #[inline]
    unsafe fn new_avx512(bytes: &AlignedSlice<TwoTo<6>>) -> (Self, &AlignedSlice<TwoTo<6>>) {
        let (opening_mask, closing_mask) = get_opening_and_closing_masks(bytes);

        let vector = Self {
            opening_mask,
            closing_mask,
            len: BYTES_IN_AVX512_REGISTER,
            rev_idx: BYTES_IN_AVX512_REGISTER - 1,
        };

        (vector, bytes.offset(1))
    }
}

impl LazyVector {
    #[inline]
    pub fn new(bytes: &AlignedSlice<TwoTo<6>>) -> (Self, &AlignedSlice<TwoTo<6>>) {
        if bytes.len() >= BYTES_IN_AVX512_REGISTER {
            unsafe { Self::new_avx512(bytes) }
        } else {
            Self::new_sequential(bytes)
        }
    }
}

impl<'a> DepthBlock<'a> for LazyVector {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn advance(&mut self) -> bool {
        if self.rev_idx == 0 {
            return false;
        }
        self.rev_idx -= 1;
        true
    }

    #[inline]
    fn advance_by(&mut self, i: usize) -> usize {
        let j = std::cmp::min(i, self.rev_idx);
        self.rev_idx -= j;
        j
    }

    #[inline]
    fn is_depth_greater_or_equal_to(&self, depth: isize) -> bool {
        let closing_count = self.closing_mask.count_ones() as isize;
        if depth <= -closing_count {
            return true;
        }

        let actual_depth = ((self.opening_mask << self.rev_idx).count_ones() as i32)
            - ((self.closing_mask << self.rev_idx).count_ones() as i32);
        actual_depth as isize >= depth
    }

    #[inline(always)]
    fn depth_at_end(self) -> isize {
        ((self.opening_mask.count_ones() as i32) - (self.closing_mask.count_ones() as i32)) as isize
    }
}

#[inline]
#[target_feature(enable = "avx512f")]
#[target_feature(enable = "avx512bw")]
unsafe fn get_opening_and_closing_masks(bytes: &AlignedSlice<TwoTo<6>>) -> (u64, u64) {
    debug_assert!(bytes.len() >= BYTES_IN_AVX512_REGISTER);

    let byte_vector = _mm512_load_si512(bytes.as_ptr() as *const i32);
    let opening_brace_mask = _mm512_set1_epi8(b'{' as i8);
    let opening_bracket_mask = _mm512_set1_epi8(b'[' as i8);
    let closing_brace_mask = _mm512_set1_epi8(b'}' as i8);
    let closing_bracket_mask = _mm512_set1_epi8(b']' as i8);
    let opening_brace_cmp = _mm512_cmpeq_epi8_mask(byte_vector, opening_brace_mask);
    let opening_bracket_cmp = _mm512_cmpeq_epi8_mask(byte_vector, opening_bracket_mask);
    let closing_brace_cmp = _mm512_cmpeq_epi8_mask(byte_vector, closing_brace_mask);
    let closing_bracket_cmp = _mm512_cmpeq_epi8_mask(byte_vector, closing_bracket_mask);
    let opening_cmp = opening_brace_cmp | opening_bracket_cmp;
    let closing_cmp = closing_brace_cmp | closing_bracket_cmp;
    (opening_cmp, closing_cmp)
}
