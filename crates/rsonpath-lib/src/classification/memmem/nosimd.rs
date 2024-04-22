use std::marker::PhantomData;

use super::*;
use crate::{
    input::{error::InputErrorConvertible, InputBlockIterator},
    string_pattern::StringPattern,
};

pub(crate) struct Constructor;

impl MemmemImpl for Constructor {
    type Classifier<'i, 'b, 'r, I, SM, R> = SequentialMemmemClassifier<'i, 'b, 'r, I, SM, R, BLOCK_SIZE>
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
            phantom: PhantomData,
        }
    }
}

pub(crate) struct SequentialMemmemClassifier<'i, 'b, 'r, I, SM, R, const N: usize>
where
    I: Input,
    R: InputRecorder<I::Block<'i, N>> + 'r,
{
    input: &'i I,
    iter: &'b mut I::BlockIterator<'i, 'r, R, N>,
    phantom: PhantomData<SM>,
}

impl<'i, 'b, 'r, I, SM, R, const N: usize> SequentialMemmemClassifier<'i, 'b, 'r, I, SM, R, N>
where
    I: Input,
    SM: StringPatternMatcher,
    R: InputRecorder<I::Block<'i, N>> + 'r,
{
    #[inline]
    fn find_label_sequential(
        &mut self,
        label: &StringPattern,
        mut offset: usize,
    ) -> Result<Option<(usize, usize, I::Block<'i, N>)>, InputError> {
        let first_c = if label.unquoted().is_empty() {
            b'"'
        } else {
            label.unquoted()[0]
        };

        while let Some(block) = self.iter.next().e()? {
            for (i, c) in block.iter().copied().enumerate() {
                let j = offset + i;

                if (c == first_c || c == b'\\') && j > 0 {
                    if let Some(to) = self.input.pattern_match_from::<SM>(j - 1, label).e()? {
                        return Ok(Some((j - 1, to, block)));
                    }
                }
            }

            offset += block.len();
        }

        Ok(None)
    }
}

impl<'i, 'b, 'r, I, SM, R, const N: usize> Memmem<'i, 'b, 'r, I, N>
    for SequentialMemmemClassifier<'i, 'b, 'r, I, SM, R, N>
where
    I: Input,
    SM: StringPatternMatcher,
    R: InputRecorder<I::Block<'i, N>> + 'r,
{
    // Output the relative offsets
    fn find_label(
        &mut self,
        first_block: Option<I::Block<'i, N>>,
        start_idx: usize,
        label: &StringPattern,
    ) -> Result<Option<(usize, usize, I::Block<'i, N>)>, InputError> {
        if let Some(b) = first_block {
            if let Some(res) = shared::find_label_in_first_block::<_, SM, N>(self.input, b, start_idx, label)? {
                return Ok(Some(res));
            }
        }
        let next_block_offset = self.iter.get_offset();

        self.find_label_sequential(label, next_block_offset)
    }
}
