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
use crate::input::{Input, InputBlock};
use crate::query::JsonString;
use crate::result::InputRecorder;
use crate::FallibleIterator;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const SIZE: usize = 64;

pub(crate) struct Avx2MemmemClassifier<'a, 'b, 'r: 'a, I: Input, R: InputRecorder + 'r> {
    input: &'a I,
    iter: &'b mut I::BlockIterator<'a, 'r, SIZE, R>,
    offset: usize,
}

impl<'a, 'b, 'r, I: Input, R: InputRecorder> Avx2MemmemClassifier<'a, 'b, 'r, I, R>
where
    'a: 'r,
{
    #[inline]
    pub(crate) fn new(input: &'a I, iter: &'b mut I::BlockIterator<'a, 'r, SIZE, R>, offset: usize) -> Self {
        Self { input, iter, offset }
    }

    fn find_letter(&mut self, c: u8) -> Result<Option<(usize, I::Block<'a, SIZE>)>, InputError> {
        // This should be memchr
        todo!()
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn find_label_avx2(&mut self, label: &JsonString) -> Result<Option<(usize, I::Block<'a, SIZE>)>, InputError> {
        let mut offset = self.offset;
        let label_size = label.bytes_with_quotes().len();
        if label.bytes().len() == 1 {
            return self.find_letter(label.bytes()[0]);
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
            while result != 0 {
                let idx = result.trailing_zeros() as usize;
                if self
                    .input
                    .is_member_match(offset + idx - 1, offset + idx - 1 + label_size, label)
                {
                    return Ok(Some((offset + idx - 1, block)));
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
        _first_block: Option<I::Block<'a, SIZE>>,
        _start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block<'a, SIZE>)>, InputError> {
        unsafe { self.find_label_avx2(label) }
    }
}
