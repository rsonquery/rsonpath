use super::{
    shared::{mask_32, vector_256},
    *,
};
use crate::{
    bin_u32,
    classification::{QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
    input::InputBlock,
};

super::shared::structural_classifier!(Avx2Classifier32, BlockAvx2Classifier32, mask_32, 32, u32);

struct BlockAvx2Classifier32 {
    internal_classifier: vector_256::BlockClassifier256,
}

impl BlockAvx2Classifier32 {
    fn new() -> Self {
        Self {
            // SAFETY: target feature invariant
            internal_classifier: unsafe { vector_256::BlockClassifier256::new() },
        }
    }

    #[inline(always)]
    unsafe fn classify<'i, B: InputBlock<'i, 32>>(
        &mut self,
        quote_classified_block: QuoteClassifiedBlock<B, u32, 32>,
    ) -> mask_32::StructuralsBlock<B> {
        let block = &quote_classified_block.block;
        let classification = self.internal_classifier.classify_block(block);

        let structural = classification.structural;
        let nonquoted_structural = structural & !quote_classified_block.within_quotes_mask;

        bin_u32!("structural", structural);
        bin_u32!("nonquoted_structural", nonquoted_structural);

        mask_32::StructuralsBlock::new(quote_classified_block, nonquoted_structural)
    }
}
