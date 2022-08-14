use aligners::{alignment::Twice, AlignedBlock, AlignedSlice};
use cfg_if::cfg_if;

use crate::BlockAlignment;

pub struct QuoteClassifiedBlock<'a> {
    pub block: &'a AlignedBlock<Twice<BlockAlignment>>,
    pub offset: usize,
    pub within_quotes_mask: u64,
}

pub trait QuoteClassifiedIterator<'a>:
    Iterator<Item = QuoteClassifiedBlock<'a>> + len_trait::Empty + 'a
{
}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        use nosimd::SequentialQuoteClassifier;
        use crate::BlockAlignment;

        /// Walk through the JSON document represented by `bytes`
        /// and classify quoted sequences.
        #[inline(always)]
        pub fn classify_quoted_sequences(
            bytes: &AlignedSlice<BlockAlignment>,
        ) -> impl QuoteClassifiedIterator {
            SequentialQuoteClassifier::new(bytes)
        }
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        use avx2::Avx2QuoteClassifier;
        use aligners::alignment;

        /// Walk through the JSON document represented by `bytes`
        /// and classify quoted sequences.
        #[inline(always)]
        pub fn classify_quoted_sequences(
            bytes: &AlignedSlice<alignment::Twice<BlockAlignment>>,
        ) -> impl QuoteClassifiedIterator {
            Avx2QuoteClassifier::new(bytes)
        }
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}
