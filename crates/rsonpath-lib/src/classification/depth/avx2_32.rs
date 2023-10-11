use super::{shared::mask_32::DepthVector32, shared::vector_256::DelimiterClassifierImpl256, *};
use crate::{
    classification::{quotes::QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
    input::{error::InputError, InputBlock},
    FallibleIterator,
};
use std::marker::PhantomData;

const SIZE: usize = 32;

shared::depth_classifier!(Avx2VectorIterator32, DelimiterClassifierImpl256, DepthVector32, 32, u32);

#[inline(always)]
fn new_vector<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u32, SIZE>,
    classifier: &DelimiterClassifierImpl256,
) -> DepthVector32<'a, B> {
    new_vector_from(bytes, classifier, 0)
}

#[inline(always)]
fn new_vector_from<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u32, SIZE>,
    classifier: &DelimiterClassifierImpl256,
    idx: usize,
) -> DepthVector32<'a, B> {
    // SAFETY: target_feature invariant
    unsafe { new_avx2(bytes, classifier, idx) }
}

#[inline(always)]
unsafe fn new_avx2<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u32, SIZE>,
    classifier: &DelimiterClassifierImpl256,
    start_idx: usize,
) -> DepthVector32<'a, B> {
    let idx_mask = 0xFFFF_FFFF_u32 << start_idx;
    let block = &bytes.block;
    let (opening_mask, closing_mask) = classifier.get_opening_and_closing_masks(block);

    let opening_mask = opening_mask & (!bytes.within_quotes_mask) & idx_mask;
    let closing_mask = closing_mask & (!bytes.within_quotes_mask) & idx_mask;

    DepthVector32 {
        quote_classified: bytes,
        opening_mask,
        closing_mask,
        opening_count: opening_mask.count_ones(),
        depth: 0,
        idx: 0,
        phantom: PhantomData,
    }
}
