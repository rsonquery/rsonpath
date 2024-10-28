use super::{
    shared::{mask_64, vector_256},
    *,
};
use crate::{block, classification::mask::m64, debug, input::error::InputErrorConvertible};
use std::marker::PhantomData;

super::shared::quote_classifier!(Avx2QuoteClassifier64, BlockAvx2Classifier, 64, u64);

struct BlockAvx2Classifier {
    internal_classifier: mask_64::BlockClassifier64Bit,
}

impl BlockAvx2Classifier {
    fn new() -> Self {
        Self {
            internal_classifier: mask_64::BlockClassifier64Bit::new(),
        }
    }

    #[inline(always)]
    unsafe fn classify<'a, B: InputBlock<'a, 64>>(&mut self, blocks: &B) -> u64 {
        block!(blocks[..64]);

        let (block1, block2) = blocks.halves();
        let classification1 = vector_256::classify_block(block1);
        let classification2 = vector_256::classify_block(block2);

        let slashes = m64::combine_32(classification1.slashes, classification2.slashes);
        let quotes = m64::combine_32(classification1.quotes, classification2.quotes);

        self.internal_classifier.classify(slashes, quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::{Constructor, QuotesImpl};
    use crate::{
        input::{Input, OwnedBytes},
        result::empty::EmptyRecorder,
        FallibleIterator,
    };
    use test_case::test_case;

    #[test_case("", 0)]
    #[test_case("abcd", 0)]
    #[test_case(r#""abcd""#, 0b01_1111)]
    #[test_case(r#""num": 42, "string": "something" "#, 0b0_0111_1111_1110_0011_1111_1000_0000_1111)]
    #[test_case(r#"abc\"abc\""#, 0b00_0000_0000)]
    #[test_case(r#"abc\\"abc\\""#, 0b0111_1110_0000)]
    #[test_case(r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"#, 0b0000_0000_0000_0110_0011_0000_0001_1110)]
    fn single_block(str: &str, expected: u64) {
        if !std::arch::is_x86_feature_detected!("avx2") {
            return;
        }

        let owned_str = str.to_owned();
        let input = OwnedBytes::from(owned_str);
        let mut leading_padding = input.leading_padding_len() as u64;
        let iter = input.iter_blocks(&EmptyRecorder);
        let mut classifier = Constructor::new(iter);

        // Drop padding-only blocks.
        while leading_padding >= 64 {
            let mask = classifier.next().unwrap().unwrap().within_quotes_mask;
            assert_eq!(mask, 0);
            leading_padding -= 64;
        }

        // The actual classification is now either contained in the next block,
        // or split between the two next blocks. Combine them.
        let first_mask = classifier.next().unwrap().unwrap().within_quotes_mask;
        let len_in_first_mask = if leading_padding == 0 { 0 } else { 64 - leading_padding };
        let second_mask = classifier.next().unwrap().unwrap().within_quotes_mask;
        let combined_mask = (first_mask >> leading_padding) | (second_mask << len_in_first_mask);

        assert_eq!(combined_mask, expected);
    }
}
