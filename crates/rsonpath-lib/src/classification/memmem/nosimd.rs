use super::*;
use crate::input::error::InputError;
use crate::input::{Input, InputBlockIterator};
use crate::query::JsonString;
use crate::result::InputRecorder;
use crate::FallibleIterator;
use crate::{block, debug};

pub(crate) struct SequentialMemmemClassifier<'a, 'b, 'r: 'a, I: Input, R: InputRecorder + 'r, const N: usize> {
    input: &'a I,
    iter: &'b mut I::BlockIterator<'a, 'r, N, R>,
}

impl<'a, 'b, 'r, I: Input, R: InputRecorder, const N: usize> SequentialMemmemClassifier<'a, 'b, 'r, I, R, N> {
    #[inline]
    pub(crate) fn new(input: &'a I, iter: &'b mut I::BlockIterator<'a, 'r, N, R>) -> Self {
        Self { input, iter }
    }

    #[inline]
    fn find_label_sequential(
        &mut self,
        label: &JsonString,
        mut offset: usize,
    ) -> Result<Option<(usize, I::Block<'a, N>)>, InputError> {
        let label_size = label.bytes_with_quotes().len();
        let first_c = label.bytes()[0];

        while let Some(block) = self.iter.next()? {
            let res = block.iter().copied().enumerate().find(|&(i, c)| {
                let j = offset + i;
                c == first_c && self.input.is_member_match(j - 1, j + label_size - 2, label)
            });

            if let Some((res, _)) = res {
                return Ok(Some((res + offset - 1, block)));
            }

            offset += block.len();
        }

        Ok(None)
    }
}

impl<'a, 'b, 'r, I: Input, R: InputRecorder, const N: usize> Memmem<'a, 'b, I, N>
    for SequentialMemmemClassifier<'a, 'b, 'r, I, R, N>
{
    // Output the relative offsets
    fn find_label(
        &mut self,
        first_block: Option<I::Block<'a, N>>,
        start_idx: usize,
        label: &JsonString,
    ) -> Result<Option<(usize, I::Block<'a, N>)>, InputError> {
        let next_block_offset = self.iter.get_offset();
        if let Some(b) = first_block {
            let block_idx = start_idx % N;
            let label_size = label.bytes_with_quotes().len();
            debug!("half block fetches for {:?} starting at {:?}", label, block_idx);
            block!(b[block_idx..]);
            let m = b[block_idx..].iter().copied().enumerate().find(|&(i, c)| {
                let j = start_idx + i;
                c == b'"' && self.input.is_member_match(j, j + label_size - 1, label)
            });
            if let Some((res, _)) = m {
                return Ok(Some((res + start_idx, b)));
            }
        }

        self.find_label_sequential(label, next_block_offset)
    }
}
