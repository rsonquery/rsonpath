use super::{shared::mask_64, shared::vector_128, *};
use crate::{
    classification::mask::m64,
    input::{error::InputErrorConvertible, InputBlock, InputBlockIterator},
};

const SIZE: usize = 64;

pub(crate) struct Constructor;

impl MemmemImpl for Constructor {
    type Classifier<'i, 'b, 'r, I, R> = Sse2MemmemClassifier64<'i, 'b, 'r, I, R>
    where
        I: Input<'i, 'r, R, BLOCK_SIZE> + 'i,
        I::BlockIterator: 'b,
        R: InputRecorder<I::Block> + 'r,
        'i: 'r;

    fn memmem<'i, 'b, 'r, I, R>(
        input: &'i I,
        iter: &'b mut I::BlockIterator,
    ) -> Self::Classifier<'i, 'b, 'r, I, R>
    where
        I: Input<'i, 'r, R, BLOCK_SIZE>,
        R: InputRecorder<I::Block>,
        'i: 'r,
    {
        Self::Classifier { input, iter }
    }
}

pub(crate) struct Sse2MemmemClassifier64<'i, 'b, 'r, I, R>
where
    I: Input<'i, 'r, R, SIZE>,
    R: InputRecorder<I::Block> + 'r,
{
    input: &'i I,
    iter: &'b mut I::BlockIterator,
}

impl<'i, 'b, 'r, I, R> Sse2MemmemClassifier64<'i, 'b, 'r, I, R>
where
    I: Input<'i, 'r, R, SIZE>,
    R: InputRecorder<I::Block>,
    'i: 'r,
{
    #[inline]
    #[allow(dead_code)]
    pub(crate) fn new(input: &'i I, iter: &'b mut I::BlockIterator) -> Self {
        Self { input, iter }
    }

    #[inline(always)]
    unsafe fn find_empty(
        &mut self,
        label: &JsonString,
        mut offset: usize,
    ) -> Result<Option<(usize, I::Block)>, InputError> {
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
                if self
                    .input
                    .is_member_match(offset + idx - 1, offset + idx + 1, label)
                    .e()?
                {
                    return Ok(Some((offset + idx - 1, block)));
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
        label: &JsonString,
        mut offset: usize,
    ) -> Result<Option<(usize, I::Block)>, InputError> {
        let classifier = vector_128::BlockClassifier128::new(label.unquoted().as_bytes()[0], b'"');
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

            if let Some(res) =
                mask_64::find_in_mask(self.input, label, previous_block, first_bitmask, second_bitmask, offset)?
            {
                return Ok(Some((res, block)));
            }

            offset += SIZE;
            previous_block = first_bitmask >> (SIZE - 1);
        }

        Ok(None)
    }

    #[inline(always)]
    unsafe fn find_label_sse2(
        &mut self,
        label: &JsonString,
        mut offset: usize,
    ) -> Result<Option<(usize, I::Block)>, InputError> {
        if label.unquoted().is_empty() {
            return self.find_empty(label, offset);
        } else if label.unquoted().len() == 1 {
            return self.find_letter(label, offset);
        }

        let classifier =
            vector_128::BlockClassifier128::new(label.unquoted().as_bytes()[0], label.unquoted().as_bytes()[1]);
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

            if let Some(res) =
                mask_64::find_in_mask(self.input, label, previous_block, first_bitmask, second_bitmask, offset)?
            {
                return Ok(Some((res, block)));
            }

            offset += SIZE;
            previous_block = first_bitmask >> (SIZE - 1);
        }

        Ok(None)
    }
}

impl<'i, 'b, 'r, I, R> Memmem<'i, 'b, 'r, I, R, SIZE> for Sse2MemmemClassifier64<'i, 'b, 'r, I, R>
where
    I: Input<'i, 'r, R, SIZE>,
    R: InputRecorder<I::Block>,
    'i: 'r,
{
    #[inline(always)]
    fn find_label(
        &mut self,
        first_block: Option<I::Block>,
        start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block)>, InputError> {
        if let Some(b) = first_block {
            if let Some(res) = shared::find_label_in_first_block(self.input, b, start_idx, label)? {
                return Ok(Some(res));
            }
        }
        let next_block_offset = self.iter.get_offset();
        // SAFETY: target feature invariant
        unsafe { self.find_label_sse2(label, next_block_offset) }
    }
}
