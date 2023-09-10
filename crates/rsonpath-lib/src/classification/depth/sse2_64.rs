use super::{shared::mask_64::DepthVector64, shared::vector_128::DelimiterClassifierImpl128, *};
use crate::{
    classification::mask::m64,
    classification::{quotes::QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
    input::{error::InputError, InputBlock},
    FallibleIterator,
};
use std::marker::PhantomData;

const SIZE: usize = 64;

shared::depth_classifier!(
    Ssse3VectorIterator64,
    DelimiterClassifierImpl128,
    DepthVector64,
    64,
    u64
);

#[inline]
fn new_vector<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImpl128,
) -> DepthVector64<'a, B> {
    new_vector_from(bytes, classifier, 0)
}

#[inline]
fn new_vector_from<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImpl128,
    idx: usize,
) -> DepthVector64<'a, B> {
    // SAFETY: target_feature invariant
    unsafe { new_sse2(bytes, classifier, idx) }
}

#[target_feature(enable = "sse2")]
#[target_feature(enable = "popcnt")]
#[inline]
unsafe fn new_sse2<'a, B: InputBlock<'a, SIZE>>(
    bytes: QuoteClassifiedBlock<B, u64, SIZE>,
    classifier: &DelimiterClassifierImpl128,
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
