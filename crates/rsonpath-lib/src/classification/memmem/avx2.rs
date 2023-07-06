cfg_if::cfg_if! {
    if #[cfg(not(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        simd = "avx2")
    ))] {
        compile_error!{
            "internal error: AVX2 code included on unsupported target; \
            please report this issue at https://github.com/V0ldek/rsonpath/issues/new?template=bug_report.md"
        }
    }
}

use super::*;
use crate::input::error::InputError;
use crate::input::{Input, InputBlock, InputBlockIterator};
use crate::query::JsonString;
use crate::result::InputRecorder;
use crate::FallibleIterator;
use crate::{debug, bin};

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const SIZE: usize = 64;

pub(crate) struct Avx2MemmemClassifier<'a, 'b, 'r: 'a, I: Input, R: InputRecorder + 'r> {
    input: &'a I,
    iter: &'b mut I::BlockIterator<'a, 'r, SIZE, R>,
}

impl<'a, 'b, 'r, I: Input, R: InputRecorder> Avx2MemmemClassifier<'a, 'b, 'r, I, R>
where
    'a: 'r,
{
    #[inline]
    pub(crate) fn new(input: &'a I, iter: &'b mut I::BlockIterator<'a, 'r, SIZE, R>) -> Self {
        Self { input, iter }
    }
    // Here we want to detect the pattern `"c"`
    // For interblock communication we need to bit of information that requires extra work to get obtained.
    // one for the block cut being `"` and `c"` and one for `"c` and `"`. We only deal with one of them.
    unsafe fn find_letter(
        &mut self,
        label: &JsonString,
        mut offset: usize,
    ) -> Result<Option<(usize, I::Block<'a, SIZE>)>, InputError> {
        let first = _mm256_set1_epi8(label.bytes()[0] as i8);
        let second = _mm256_set1_epi8(b'"' as i8);
        let mut previous_block: u64 = 0;
        
        while let Some(block) = self.iter.next()? {
            let (block1, block2) = block.halves();
            let byte_vector1 = _mm256_loadu_si256(block1.as_ptr().cast::<__m256i>());
            let byte_vector2 = _mm256_loadu_si256(block2.as_ptr().cast::<__m256i>());

            let mut first_bitmask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector1, first)) as u64
                | ((_mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector2, first)) as u64) << 32);

            let second_bitmask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector1, second)) as u64
                | ((_mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector2, second)) as u64) << 32);
            first_bitmask &= (second_bitmask << 1 | 1); // we AND `"` bitmask with `c` bitmask to filter c's position in the stream following a `"`
                                                        // We should need the last bit of previous block. Instead of memoizing, we simply assume it is one.
                                                        // It could gives only add more potential match.

            let mut result = (previous_block | (first_bitmask << 1)) & second_bitmask;
            while result != 0 {
                let idx = result.trailing_zeros() as usize;
                if self.input.is_member_match(offset + idx - 2, offset + idx, label) {
                    return Ok(Some((offset + idx - 2, block)));
                }
                result &= !(1 << idx);
            }
            offset += SIZE;
            previous_block = first_bitmask << (SIZE - 1);
        }
        return Ok(None);
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn find_label_avx2(
        &mut self,
        label: &JsonString,
        mut offset: usize,
    ) -> Result<Option<(usize, I::Block<'a, SIZE>)>, InputError> {
        let label_size = label.bytes_with_quotes().len();
        if label.bytes().len() == 1 {
            return self.find_letter(label, offset);
        }
        let first = _mm256_set1_epi8(label.bytes()[0] as i8);
        let second = _mm256_set1_epi8(label.bytes()[1] as i8);
        let mut previous_block: u64 = 0;
        while let Some(block) = self.iter.next()? {
            let (block1, block2) = block.halves();
            let byte_vector1 = _mm256_loadu_si256(block1.as_ptr().cast::<__m256i>());
            let byte_vector2 = _mm256_loadu_si256(block2.as_ptr().cast::<__m256i>());

            let first_bitmask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector1, first)) as u64
                | ((_mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector2, first)) as u64) << 32);

            let second_bitmask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector1, second)) as u64
                | ((_mm256_movemask_epi8(_mm256_cmpeq_epi8(byte_vector2, second)) as u64) << 32);

            let mut result = (previous_block | (first_bitmask << 1)) & second_bitmask;
            debug!("printing result memmem");
            debug!(
                "{: >24}: {}",
                "block",
                std::str::from_utf8_unchecked(
                    &block
                        .iter()
                        .map(|x| if x.is_ascii_whitespace() { b' ' } else { *x })
                        .collect::<Vec<_>>()
                )
            );
            bin!("resul:", result);
            while result != 0 {
                let idx = result.trailing_zeros() as usize;
                debug!("offset:{}:{}", offset + idx - 2, offset + idx - 3 + label_size);
                if self
                    .input
                    .is_member_match(offset + idx - 2, offset + idx - 3 + label_size, label)
                {
                    return Ok(Some((offset + idx - 2, block)));
                }
                result &= !(1 << idx);
            }
            offset += SIZE;
            previous_block = first_bitmask << (SIZE - 1);
        }
        return Ok(None);
    }
}

impl<'a, 'b, 'r, I: Input, R: InputRecorder> Memmem<'a, 'b, I, SIZE> for Avx2MemmemClassifier<'a, 'b, 'r, I, R>
where
    'a: 'r,
{
    // Output the relative offsets
    fn find_label(
        &mut self,
        first_block: Option<I::Block<'a, SIZE>>,
        start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block<'a, SIZE>)>, InputError> {
        let next_block_offset = self.iter.get_offset();
        if let Some(b) = first_block {
            let block_idx = start_idx % SIZE;
            let n = label.bytes_with_quotes().len();
            let m = b
                .iter()
                .copied()
                .enumerate()
                .find(|&(i, c)| c == b'"' && self.input.is_member_match(start_idx + i, start_idx + i + n, label));
            if let Some((res, _)) = m {
                return Ok(Some((res, b)));
            }
        }
        unsafe { self.find_label_avx2(label, next_block_offset) }
    }
}
