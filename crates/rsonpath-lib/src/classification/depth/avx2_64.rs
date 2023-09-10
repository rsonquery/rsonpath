use super::{shared::mask_64::DepthVector64, shared::vector_256::DelimiterClassifierImpl256, *};
use crate::{
    classification::mask::m64,
    classification::{quotes::QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
    input::{error::InputError, InputBlock},
    FallibleIterator,
};
use std::marker::PhantomData;

const SIZE: usize = 64;

shared::depth_classifier!(Avx2VectorIterator64, DelimiterClassifierImpl256, DepthVector64, 64, u64);

#[inline]
fn new_vector<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImpl256,
) -> DepthVector64<'a, B> {
    new_vector_from(bytes, classifier, 0)
}

#[inline]
fn new_vector_from<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImpl256,
    idx: usize,
) -> DepthVector64<'a, B> {
    // SAFETY: target_feature invariant
    unsafe { new_avx2(bytes, classifier, idx) }
}

#[target_feature(enable = "avx2")]
#[target_feature(enable = "popcnt")]
#[inline]
unsafe fn new_avx2<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImpl256,
    start_idx: usize,
) -> DepthVector64<'a, B> {
    let idx_mask = 0xFFFF_FFFF_FFFF_FFFF_u64 << start_idx;
    let (first_block, second_block) = bytes.block.halves();
    let (first_opening_mask, first_closing_mask) = classifier.get_opening_and_closing_masks(first_block);
    let (second_opening_mask, second_closing_mask) = classifier.get_opening_and_closing_masks(second_block);

    let combined_opening_mask = m64::combine_32(first_opening_mask, second_opening_mask);
    let combined_closing_mask = m64::combine_32(first_closing_mask, second_closing_mask);

    let opening_mask = combined_opening_mask & (!bytes.within_quotes_mask) & idx_mask;
    let closing_mask = combined_closing_mask & (!bytes.within_quotes_mask) & idx_mask;

    DepthVector64 {
        quote_classified: bytes,
        opening_mask,
        closing_mask,
        opening_count: opening_mask.count_ones(),
        depth: 0,
        idx: 0,
        phantom: PhantomData,
    }
}
