//The errors about mask_32 don't need to be fixed because they go away when the website is compiled
use super::{
    shared::{mask_32::DepthVector32, vector_128::DelimiterClassifierImpl128},
    *,
};
use crate::{
    classification::{mask::m32, QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
    input::InputBlock,
};
use core::marker::PhantomData;

const SIZE: usize = 32;

shared::depth_classifier!(WasmVectorIterator32, DelimiterClassifierImpl128, DepthVector32, 32, u32);

#[inline(always)]
fn new_vector<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u32, SIZE>,
    classifier: &DelimiterClassifierImpl128,
) -> DepthVector32<'a, B> {
    new_vector_from(bytes, classifier, 0)
}

#[inline(always)]
fn new_vector_from<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u32, SIZE>,
    classifier: &DelimiterClassifierImpl128,
    idx: usize,
) -> DepthVector32<'a, B> {
    // SAFETY: target_feature invariant
    unsafe { new_wasm(bytes, classifier, idx) }
}

#[inline(always)]
unsafe fn new_wasm<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u32, SIZE>,
    classifier: &DelimiterClassifierImpl128,
    start_idx: usize,
) -> DepthVector32<'a, B> {
    let idx_mask = 0xFFFF_FFFF_u32 << start_idx;

    let (block1, block2) = bytes.block.halves();

    let (opening_mask1, closing_mask1) = classifier.get_opening_and_closing_masks(block1);
    let (opening_mask2, closing_mask2) = classifier.get_opening_and_closing_masks(block2);

    let combined_opening_mask = m32::combine_16(opening_mask1, opening_mask2);
    let combined_closing_mask = m32::combine_16(closing_mask1, closing_mask2);

    let opening_mask = combined_opening_mask & (!bytes.within_quotes_mask) & idx_mask;
    let closing_mask = combined_closing_mask & (!bytes.within_quotes_mask) & idx_mask;

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
