use super::{shared::mask_64, shared::vector_128, *};
use crate::{
    classification::mask::m64,
    input::{error::InputErrorConvertible, InputBlock, InputBlockIterator},
};
use std::marker::PhantomData;

const SIZE: usize = 64;

pub(crate) struct Constructor;

impl MemmemImpl for Constructor {
    type Classifier<'i, 'b, 'r, I, SM, R>
        = Sse2MemmemClassifier64<'i, 'b, 'r, I, SM, R>
    where
        I: Input + 'i,
        SM: StringPatternMatcher,
        <I as Input>::BlockIterator<'i, 'r, R, BLOCK_SIZE>: 'b,
        R: InputRecorder<<I as Input>::Block<'i, BLOCK_SIZE>> + 'r,
        'i: 'r;

    fn memmem<'i, 'b, 'r, I, SM, R>(
        input: &'i I,
        iter: &'b mut <I as Input>::BlockIterator<'i, 'r, R, BLOCK_SIZE>,
    ) -> Self::Classifier<'i, 'b, 'r, I, SM, R>
    where
        I: Input,
        SM: StringPatternMatcher,
        R: InputRecorder<<I as Input>::Block<'i, BLOCK_SIZE>>,
        'i: 'r,
    {
        Self::Classifier {
            input,
            iter,
            phantom_data: PhantomData,
        }
    }
}

pub(crate) struct Sse2MemmemClassifier64<'i, 'b, 'r, I, SM, R>
where
    I: Input,
    SM: StringPatternMatcher,
    R: InputRecorder<I::Block<'i, SIZE>> + 'r,
{
    input: &'i I,
    iter: &'b mut I::BlockIterator<'i, 'r, R, SIZE>,
    phantom_data: PhantomData<SM>,
}

impl<'i, 'b, 'r, I, SM, R> Sse2MemmemClassifier64<'i, 'b, 'r, I, SM, R>
where
    I: Input,
    SM: StringPatternMatcher,
    R: InputRecorder<I::Block<'i, SIZE>>,
    'i: 'r,
{
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new(input: &'i I, iter: &'b mut I::BlockIterator<'i, 'r, R, SIZE>) -> Self {
        Self {
            input,
            iter,
            phantom_data: PhantomData,
        }
    }

    #[inline(always)]
    unsafe fn find_empty(
        &mut self,
        label: &StringPattern,
        mut offset: usize,
    ) -> Result<Option<(usize, usize, I::Block<'i, SIZE>)>, InputError> {
        let classifier = vector_128::BlockClassifier128::new(b'"', b'"');
        let mut previous_block: u64 = 0;

        while let Some(block) = self.iter.next().e()? {
            let (block1, block2, block3, block4) = block.quarters();
            let classified1 = classifier.classify_block(block1);
            let classified2 = classifier.classify_block(block2);
            let classified3 = classifier.classify_block(block3);
            let classified4 = classifier.classify_block(block4);

            let first_bitmask = m64::combine_16(
                classified1.first,
                classified2.first,
                classified3.first,
                classified4.first,
            );
            let second_bitmask = m64::combine_16(
                classified1.second,
                classified2.second,
                classified3.second,
                classified4.second,
            );

            let mut result = (previous_block | (first_bitmask << 1)) & second_bitmask;
            while result != 0 {
                let idx = result.trailing_zeros() as usize;
                if let Some(to) = self.input.pattern_match_from::<SM>(offset + idx - 1, label).e()? {
                    return Ok(Some((offset + idx - 1, to, block)));
                }
                result &= !(1 << idx);
            }

            offset += SIZE;
            previous_block = first_bitmask >> (SIZE - 1);
        }

        Ok(None)
    }

    // Here we want to detect the pattern `"c"`
    // For interblock communication we need to bit of information that requires extra work to get obtained.
    // one for the block cut being `"` and `c"` and one for `"c` and `"`. We only deal with one of them.
    #[inline(always)]
    unsafe fn find_letter(
        &mut self,
        pattern: &StringPattern,
        mut offset: usize,
    ) -> Result<Option<(usize, usize, I::Block<'i, SIZE>)>, InputError> {
        let classifier = vector_128::BlockClassifier128::new(pattern.unquoted()[0], b'"');
        let mut previous_slash: u64 = 0;
        let mut previous_first: u64 = 0;
        let mut previous_quote: u64 = 0;

        while let Some(block) = self.iter.next().e()? {
            let (block1, block2, block3, block4) = block.quarters();
            let classified1 = classifier.classify_block(block1);
            let classified2 = classifier.classify_block(block2);
            let classified3 = classifier.classify_block(block3);
            let classified4 = classifier.classify_block(block4);

            let first_bitmask = m64::combine_16(
                classified1.first,
                classified2.first,
                classified3.first,
                classified4.first,
            );
            let second_bitmask = m64::combine_16(
                classified1.second,
                classified2.second,
                classified3.second,
                classified4.second,
            );
            let slash_bitmask = m64::combine_16(
                classified1.slashes,
                classified2.slashes,
                classified3.slashes,
                classified4.slashes,
            );
            let quote_bitmask = m64::combine_16(
                classified1.quotes,
                classified2.quotes,
                classified3.quotes,
                classified4.quotes,
            );

            if let Some((from, to)) = mask_64::find_in_mask::<_, SM>(
                self.input,
                pattern,
                previous_slash,
                previous_quote,
                previous_first,
                first_bitmask,
                second_bitmask,
                slash_bitmask,
                quote_bitmask,
                offset,
            )? {
                return Ok(Some((from, to, block)));
            }

            offset += SIZE;
            previous_slash = (slash_bitmask & (quote_bitmask << 1)) >> (SIZE - 1);
            previous_first = (first_bitmask & (quote_bitmask << 1)) >> (SIZE - 1);
            previous_quote = quote_bitmask >> (SIZE - 2);
        }

        Ok(None)
    }

    #[inline(always)]
    unsafe fn find_label_sse2(
        &mut self,
        pattern: &StringPattern,
        mut offset: usize,
    ) -> Result<Option<(usize, usize, I::Block<'i, SIZE>)>, InputError> {
        if pattern.unquoted().is_empty() {
            return self.find_empty(pattern, offset);
        } else if pattern.unquoted().len() == 1 {
            return self.find_letter(pattern, offset);
        }

        let classifier = vector_128::BlockClassifier128::new(pattern.unquoted()[0], pattern.unquoted()[1]);
        let mut previous_slash: u64 = 0;
        let mut previous_first: u64 = 0;
        let mut previous_quote: u64 = 0;

        while let Some(block) = self.iter.next().e()? {
            let (block1, block2, block3, block4) = block.quarters();
            let classified1 = classifier.classify_block(block1);
            let classified2 = classifier.classify_block(block2);
            let classified3 = classifier.classify_block(block3);
            let classified4 = classifier.classify_block(block4);

            let first_bitmask = m64::combine_16(
                classified1.first,
                classified2.first,
                classified3.first,
                classified4.first,
            );
            let second_bitmask = m64::combine_16(
                classified1.second,
                classified2.second,
                classified3.second,
                classified4.second,
            );
            let slash_bitmask = m64::combine_16(
                classified1.slashes,
                classified2.slashes,
                classified3.slashes,
                classified4.slashes,
            );
            let quote_bitmask = m64::combine_16(
                classified1.quotes,
                classified2.quotes,
                classified3.quotes,
                classified4.quotes,
            );

            if let Some((from, to)) = mask_64::find_in_mask::<_, SM>(
                self.input,
                pattern,
                previous_slash,
                previous_quote,
                previous_first,
                first_bitmask,
                second_bitmask,
                slash_bitmask,
                quote_bitmask,
                offset,
            )? {
                return Ok(Some((from, to, block)));
            }

            offset += SIZE;
            previous_slash = (slash_bitmask & (quote_bitmask << 1)) >> (SIZE - 1);
            previous_first = (first_bitmask & (quote_bitmask << 1)) >> (SIZE - 1);
            previous_quote = quote_bitmask >> (SIZE - 2);
        }

        Ok(None)
    }
}

impl<'i, 'b, 'r, I, SM, R> Memmem<'i, 'b, 'r, I, SIZE> for Sse2MemmemClassifier64<'i, 'b, 'r, I, SM, R>
where
    I: Input,
    SM: StringPatternMatcher,
    R: InputRecorder<I::Block<'i, SIZE>>,
    'i: 'r,
{
    #[inline(always)]
    fn find_label(
        &mut self,
        first_block: Option<I::Block<'i, SIZE>>,
        start_idx: usize,
        pattern: &StringPattern,
    ) -> Result<Option<(usize, usize, I::Block<'i, SIZE>)>, InputError> {
        if let Some(b) = first_block {
            if let Some(res) = shared::find_pattern_in_first_block::<_, SM, SIZE>(self.input, b, start_idx, pattern)? {
                return Ok(Some(res));
            }
        }
        let next_block_offset = self.iter.get_offset();
        // SAFETY: target feature invariant
        unsafe { self.find_label_sse2(pattern, next_block_offset) }
    }
}
