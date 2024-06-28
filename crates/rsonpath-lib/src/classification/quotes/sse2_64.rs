use super::{
    shared::{mask_64, vector_128},
    *,
};
use crate::{block, classification::mask::m64, debug, input::error::InputErrorConvertible};
use std::marker::PhantomData;

super::shared::quote_classifier!(Sse2QuoteClassifier64, BlockSse2Classifier, 64, u64);

struct BlockSse2Classifier {
    internal_classifier: mask_64::BlockClassifier64Bit,
}

impl BlockSse2Classifier {
    fn new() -> Self {
        Self {
            internal_classifier: mask_64::BlockClassifier64Bit::new(),
        }
    }

    #[inline(always)]
    unsafe fn classify<'a, B: InputBlock<'a, 64>>(&mut self, blocks: &B) -> u64 {
        block!(blocks[..64]);

        let (block1, block2, block3, block4) = blocks.quarters();

        let classification1 = vector_128::classify_block(block1);
        let classification2 = vector_128::classify_block(block2);
        let classification3 = vector_128::classify_block(block3);
        let classification4 = vector_128::classify_block(block4);

        let slashes = m64::combine_16(
            classification1.slashes,
            classification2.slashes,
            classification3.slashes,
            classification4.slashes,
        );
        let quotes = m64::combine_16(
            classification1.quotes,
            classification2.quotes,
            classification3.quotes,
            classification4.quotes,
        );

        self.internal_classifier.classify(slashes, quotes)
    }
}

#[cfg(test)]
mod tests {
    use super::Sse2QuoteClassifier64;
    use crate::{
        input::{Input, OwnedBytes},
        result::empty::EmptyRecorder,
        FallibleIterator,
    };
    use test_case::test_case;

    #[test_case("" => None)]
    #[test_case("abcd" => Some(0))]
    #[test_case(r#""abcd""# => Some(0b01_1111))]
    #[test_case(r#""number": 42, "string": "something" "# => Some(0b0011_1111_1111_0001_1111_1100_0000_0111_1111))]
    #[test_case(r#"abc\"abc\""# => Some(0b00_0000_0000))]
    #[test_case(r#"abc\\"abc\\""# => Some(0b0111_1110_0000))]
    #[test_case(r#"{"aaa":[{},{"b":{"c":[1,2,3]}}],"e":{"a":[[],[1,2,3],"# => Some(0b0_0000_0000_0000_0110_0011_0000_0000_0000_0110_0011_0000_0001_1110))]
    fn single_block(str: &str) -> Option<u64> {
        let owned_str = str.to_owned();
        let input = OwnedBytes::new(&owned_str).unwrap();
        let iter = input.iter_blocks::<_, 64>(&EmptyRecorder);
        let mut classifier = Sse2QuoteClassifier64::new(iter);
        classifier.next().unwrap().map(|x| x.within_quotes_mask)
    }
}
