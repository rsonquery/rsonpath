use super::{
    shared::{mask_32, vector_128},
    *,
};
use crate::{
    block,
    classification::mask::m32,
    debug,
    input::{error::InputErrorConvertible, InputBlock, InputBlockIterator},
    FallibleIterator,
};
use std::marker::PhantomData;

super::shared::quote_classifier!(Sse2QuoteClassifier32, BlockSse2Classifier, 32, u32);

struct BlockSse2Classifier {
    internal_classifier: mask_32::BlockClassifier32Bit,
}

impl BlockSse2Classifier {
    fn new() -> Self {
        Self {
            internal_classifier: mask_32::BlockClassifier32Bit::new(),
        }
    }

    #[inline(always)]
    unsafe fn classify<'a, B: InputBlock<'a, 32>>(&mut self, blocks: &B) -> u32 {
        block!(blocks[..32]);

        let (block1, block2) = blocks.halves();

        let classification1 = vector_128::classify_block(block1);
        let classification2 = vector_128::classify_block(block2);

        let slashes = m32::combine_16(classification1.slashes, classification2.slashes);
        let quotes = m32::combine_16(classification1.quotes, classification2.quotes);

        self.internal_classifier.classify(slashes, quotes)
    }
}

#[cfg(all(test, cfg = "ssse3_32"))]
mod tests {
    use super::Sse2QuoteClassifier32;
    use crate::{
        input::{Input, OwnedBytes},
        result::empty::EmptyRecorder,
        FallibleIterator,
    };
    use test_case::test_case;

    #[test_case("" => None)]
    #[test_case("abcd" => Some(0))]
    #[test_case(r#""abcd""# => Some(0b01_1111))]
    #[test_case(r#""num": 42, "string": "something" "# => Some(0b0_0111_1111_1110_0011_1111_1000_0000_1111))]
    #[test_case(r#"abc\"abc\""# => Some(0b00_0000_0000))]
    #[test_case(r#"abc\\"abc\\""# => Some(0b0111_1110_0000))]
    #[test_case(r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"# => Some(0b0000_0000_0000_0110_0011_0000_0001_1110))]
    fn single_block(str: &str) -> Option<u32> {
        let owned_str = str.to_owned();
        let input = OwnedBytes::new(&owned_str).unwrap();
        let iter = input.iter_blocks::<_, 32>(&EmptyRecorder);
        let mut classifier = Sse2QuoteClassifier32::new(iter);
        classifier.next().unwrap().map(|x| x.within_quotes_mask)
    }
}
