use super::{
    shared::{mask_64, vector_256},
    *,
};
use crate::{
    bin_u64,
    classification::mask::m64,
    classification::{QuoteClassifiedBlock, ResumeClassifierBlockState, ResumeClassifierState},
    debug,
    input::{error::InputError, InputBlock, InputBlockIterator},
    FallibleIterator,
};

super::shared::structural_classifier!(Avx2Classifier64, BlockAvx2Classifier64, mask_64, 64, u64);

struct BlockAvx2Classifier64 {
    internal_classifier: vector_256::BlockClassifier256,
}

impl BlockAvx2Classifier64 {
    fn new() -> Self {
        Self {
            // SAFETY: target feature invariant
            internal_classifier: unsafe { vector_256::BlockClassifier256::new() },
        }
    }

    #[target_feature(enable = "avx2")]
    #[inline]
    unsafe fn classify<'i, B: InputBlock<'i, 64>>(
        &mut self,
        quote_classified_block: QuoteClassifiedBlock<B, u64, 64>,
    ) -> mask_64::StructuralsBlock<B> {
        let (block1, block2) = quote_classified_block.block.halves();
        let classification1 = self.internal_classifier.classify_block(block1);
        let classification2 = self.internal_classifier.classify_block(block2);

        let structural = m64::combine_32(classification1.structural, classification2.structural);
        let nonquoted_structural = structural & !quote_classified_block.within_quotes_mask;

        bin_u64!("structural", structural);
        bin_u64!("nonquoted_structural", nonquoted_structural);

        mask_64::StructuralsBlock::new(quote_classified_block, nonquoted_structural)
    }
}
