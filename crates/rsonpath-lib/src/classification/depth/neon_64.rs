use super::{
    shared::{mask_64::DepthVector64, vector_neon::DelimiterClassifierImplNeon},
    *,
};
use crate::{
    classification::{mask::m64, QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
    input::InputBlock,
};
use std::marker::PhantomData;

const SIZE: usize = 64;

shared::depth_classifier!(NeonVectorIterator, DelimiterClassifierImplNeon, DepthVector64, 64, u64);

#[inline(always)]
fn new_vector<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImplNeon,
) -> DepthVector64<'a, B> {
    new_vector_from(bytes, classifier, 0)
}

#[inline(always)]
fn new_vector_from<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImplNeon,
    idx: usize,
) -> DepthVector64<'a, B> {
    // SAFETY: target_feature invariant
    unsafe { new_neon(bytes, classifier, idx) }
}

#[inline(always)]
unsafe fn new_neon<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImplNeon,
    start_idx: usize,
) -> DepthVector64<'a, B> {
    let idx_mask = 0xFFFF_FFFF_FFFF_FFFF_u64 << start_idx;
    let (block1, block2, block3, block4) = bytes.block.quarters();
    let (opening_mask1, closing_mask1) = classifier.get_opening_and_closing_masks(block1);
    let (opening_mask2, closing_mask2) = classifier.get_opening_and_closing_masks(block2);
    let (opening_mask3, closing_mask3) = classifier.get_opening_and_closing_masks(block3);
    let (opening_mask4, closing_mask4) = classifier.get_opening_and_closing_masks(block4);

    let combined_opening_mask = m64::combine_16(opening_mask1, opening_mask2, opening_mask3, opening_mask4);
    let combined_closing_mask = m64::combine_16(closing_mask1, closing_mask2, closing_mask3, closing_mask4);

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
