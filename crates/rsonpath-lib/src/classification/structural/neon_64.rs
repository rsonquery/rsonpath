use super::{
    shared::{mask_64, vector_neon},
    *,
};
use crate::{
    bin_u64,
    classification::mask::m64,
    classification::{QuoteClassifiedBlock, ResumeClassifierBlockState},
    debug,
    input::InputBlock,
};

shared::structural_classifier!(NeonClassifier, BlockNeonClassifier, mask_64, 64, u64);

struct BlockNeonClassifier {
    internal_classifier: vector_neon::BlockClassifierNeon,
}

impl BlockNeonClassifier {
    fn new() -> Self {
        Self {
            // SAFETY: target feature invariant
            internal_classifier: unsafe { vector_neon::BlockClassifierNeon::new() },
        }
    }

    #[inline(always)]
    unsafe fn classify<'i, B: InputBlock<'i, 64>>(
        &mut self,
        quote_classified_block: QuoteClassifiedBlock<B, u64, 64>,
    ) -> mask_64::StructuralsBlock<B> {
        let (block1, block2, block3, block4) = quote_classified_block.block.quarters();
        let classification1 = self.internal_classifier.classify_block(block1);
        let classification2 = self.internal_classifier.classify_block(block2);
        let classification3 = self.internal_classifier.classify_block(block3);
        let classification4 = self.internal_classifier.classify_block(block4);

        let structural = m64::combine_16(
            classification1.structural,
            classification2.structural,
            classification3.structural,
            classification4.structural,
        );
        let nonquoted_structural = structural & !quote_classified_block.within_quotes_mask;

        bin_u64!("structural", structural);
        bin_u64!("nonquoted_structural", nonquoted_structural);

        mask_64::StructuralsBlock::new(quote_classified_block, nonquoted_structural)
    }
}
