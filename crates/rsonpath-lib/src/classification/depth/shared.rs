pub(super) mod mask_32;
pub(super) mod mask_64;
pub(super) mod vector_128;
pub(super) mod vector_256;

macro_rules! depth_classifier {
    ($name:ident, $core:ident, $vector:ident, $size:literal, $mask_ty:ty) => {
        pub(crate) struct $name<'i, I, Q> {
            iter: Q,
            classifier: $core,
            were_commas_on: bool,
            were_colons_on: bool,
            phantom: PhantomData<(&'i (), I)>,
        }

        impl<'a, I, Q> $name<'a, I, Q>
        where
            I: InputBlockIterator<'a, $size>,
            Q: QuoteClassifiedIterator<'a, I, $mask_ty, $size>,
        {
            #[allow(dead_code)]
            pub(crate) fn new(iter: Q, opening: BracketType) -> Self {
                Self {
                    iter,
                    classifier: $core::new(opening),
                    were_commas_on: false,
                    were_colons_on: false,
                    phantom: PhantomData,
                }
            }
        }

        impl<'a, I, Q> FallibleIterator for $name<'a, I, Q>
        where
            I: InputBlockIterator<'a, $size>,
            Q: QuoteClassifiedIterator<'a, I, $mask_ty, $size>,
        {
            type Item = $vector<'a, I::Block>;
            type Error = InputError;

            #[inline(always)]
            fn next(&mut self) -> Result<Option<Self::Item>, Self::Error> {
                let quote_classified = self.iter.next()?;
                Ok(quote_classified.map(|q| new_vector(q, &self.classifier)))
            }
        }

        impl<'a, I, Q> DepthIterator<'a, I, Q, $mask_ty, $size> for $name<'a, I, Q>
        where
            I: InputBlockIterator<'a, $size>,
            Q: QuoteClassifiedIterator<'a, I, $mask_ty, $size>,
        {
            type Block = $vector<'a, I::Block>;

            fn stop(self, block: Option<Self::Block>) -> ResumeClassifierState<'a, I, Q, $mask_ty, $size> {
                let block_state = block.and_then(|b| {
                    let idx = b.idx;
                    debug!("Depth iterator stopping at index {idx}");
                    if idx >= b.quote_classified.len() {
                        None
                    } else {
                        Some(ResumeClassifierBlockState {
                            block: b.quote_classified,
                            idx,
                        })
                    }
                });

                ResumeClassifierState {
                    iter: self.iter,
                    block: block_state,
                    are_commas_on: self.were_commas_on,
                    are_colons_on: self.were_colons_on,
                }
            }

            fn resume(
                state: ResumeClassifierState<'a, I, Q, $mask_ty, $size>,
                opening: BracketType,
            ) -> (Option<Self::Block>, Self) {
                let classifier = $core::new(opening);
                let first_block = state.block.and_then(|b| {
                    if b.idx == $size {
                        None
                    } else {
                        Some(new_vector_from(b.block, &classifier, b.idx))
                    }
                });

                (
                    first_block,
                    $name {
                        iter: state.iter,
                        classifier,
                        phantom: PhantomData,
                        were_commas_on: state.are_commas_on,
                        were_colons_on: state.are_colons_on,
                    },
                )
            }
        }
    };
}

pub(crate) use depth_classifier;
