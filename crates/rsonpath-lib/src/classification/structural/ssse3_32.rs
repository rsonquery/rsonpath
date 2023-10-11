use super::{
    shared::{mask_32, vector_128},
    *,
};
use crate::{
    bin_u32,
    classification::mask::m32,
    classification::{QuoteClassifiedBlock, ResumeClassifierBlockState, ResumeClassifierState},
    debug,
    input::{error::InputError, InputBlock, InputBlockIterator},
    FallibleIterator,
};

super::shared::structural_classifier!(Ssse3Classifier32, BlockSse2Classifier32, mask_32, 32, u32);

struct BlockSse2Classifier32 {
    internal_classifier: vector_128::BlockClassifier128,
}

impl BlockSse2Classifier32 {
    #[inline]
    fn new() -> Self {
        Self {
            // SAFETY: target feature invariant
            internal_classifier: unsafe { vector_128::BlockClassifier128::new() },
        }
    }

    #[target_feature(enable = "ssse3")]
    #[inline]
    unsafe fn classify<'i, B: InputBlock<'i, 32>>(
        &mut self,
        quote_classified_block: QuoteClassifiedBlock<B, u32, 32>,
    ) -> mask_32::StructuralsBlock<B> {
        let (block1, block2) = quote_classified_block.block.halves();
        let classification1 = self.internal_classifier.classify_block(block1);
        let classification2 = self.internal_classifier.classify_block(block2);

        let structural = m32::combine_16(classification1.structural, classification2.structural);
        let nonquoted_structural = structural & !quote_classified_block.within_quotes_mask;

        bin_u32!("structural", structural);
        bin_u32!("nonquoted_structural", nonquoted_structural);

        mask_32::StructuralsBlock::new(quote_classified_block, nonquoted_structural)
    }
}
