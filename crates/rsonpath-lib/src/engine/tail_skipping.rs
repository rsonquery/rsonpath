#![allow(clippy::expect_used)] // Enforcing the classifier invariant is clunky without this.
use crate::{
    classification::{
        depth::{DepthBlock, DepthIterator, DepthIteratorResumeOutcome},
        quotes::QuoteClassifiedIterator,
        simd::{dispatch_simd, Simd},
        structural::{BracketType, StructuralIterator},
        ResumeClassifierState,
    },
    debug,
    engine::error::EngineError,
    input::InputBlockIterator,
    lookup_table::LookUpTable,
    lookup_table::LookUpTableImpl,
    FallibleIterator, MaskType, BLOCK_SIZE,
};
use std::marker::PhantomData;

pub(crate) struct TailSkip<'i, I, Q, S, V, const N: usize> {
    classifier: Option<S>,
    simd: V,
    _phantom: (PhantomData<&'i ()>, PhantomData<(I, Q)>),
}

impl<'i, I, V> TailSkip<'i, I, V::QuotesClassifier<'i, I>, V::StructuralClassifier<'i, I>, V, BLOCK_SIZE>
where
    I: InputBlockIterator<'i, BLOCK_SIZE>,
    V: Simd,
{
    pub(crate) fn new(classifier: V::StructuralClassifier<'i, I>, simd: V) -> Self {
        Self {
            classifier: Some(classifier),
            simd,
            _phantom: (PhantomData, PhantomData),
        }
    }

    /// Returns the index position where the parser skips to. Given the a opening bracket this returns the position of
    /// the closing bracket.
    ///
    /// The skip is based either on opening_idx + lut to find goal position via a data structure OR given just the
    /// BracketType the parser iteratively reads blocks until the closing bracket is found.
    pub(crate) fn skip(
        &mut self,
        opening_idx_padded: usize,
        bracket_type: BracketType,
        lut: Option<&LookUpTableImpl>,
        padding: usize,
    ) -> Result<usize, EngineError> {
        // debug!("Skipping BracketType: {:?} from {}", bracket_type, opening_idx_padded);
        if let Some(lut) = lut {
            self.skip_with_lut(opening_idx_padded, bracket_type, &lut, padding)
        } else {
            debug!("Skipping without LUT");
            self.skip_without_lut(bracket_type)
        }

        // let opening_idx = opening_idx_padded - padding;
        // let closing_idx_pad = self.skip_without_lut(bracket_type)?;
        // let closing_idx = padding + closing_idx_pad as usize;
        // debug!(
        //     "ITE:({},{}) No-PAD:({},{})",
        //     opening_idx_padded, closing_idx_pad, opening_idx, closing_idx
        // );
        // Ok(closing_idx)
    }

    // RICARDO TODO
    // 0. Use LUT to get opening -> closing index
    // 1. Tell the Structural Classifier (self.classifier) to jump
    // 2. S tells its quote classifier to jump
    // 3. Q tells the InputIterator to jump
    // 4. Implement jump in InputBlockIterators
    // 5. Q needs to reclassify the new current block.
    // 6. S needs to reclassify the new current block.
    // 7. This function returns the skipped-to index.
    fn skip_with_lut(
        &mut self,
        opening_idx_padded: usize,
        bracket_type: BracketType,
        lut: &LookUpTableImpl,
        padding: usize,
    ) -> Result<usize, EngineError> {
        let opening_idx = opening_idx_padded - padding;

        // 0. Use LUT to get opening -> closing index
        // Can fail if key is not in lut
        // TODO: think about random hits here
        if let Some(idx_close) = lut.get(&(opening_idx_padded - padding)) {
            // Shift index by 1 or its off aligned TODO: fix lut
            let idx_close = idx_close + 1;
            let idx_close_pad = padding + idx_close as usize;

            // 1. Tell the Structural Classifier (self.classifier) to jump
            self.classifier
                .as_mut()
                .expect("tail skip must always hold a classifier")
                .jump_to_idx(idx_close_pad)?;

            debug!(
                "LUT:({},{}) No-PAD:({},{})",
                opening_idx_padded, idx_close_pad, opening_idx, idx_close
            );
            // 7. This function returns the skipped-to index.
            Ok(idx_close_pad)
        } else {
            // Do this when you were not able to find any hits in the lut
            let closing_idx_padded = self.skip_without_lut(bracket_type)?;
            let closing_idx = closing_idx_padded - padding;
            debug!(
                "ITE:({},{}) No-PAD:({},{})",
                opening_idx_padded, closing_idx_padded, opening_idx, closing_idx
            );

            Ok(closing_idx_padded)
        }
    }

    fn skip_without_lut(&mut self, bracket_type: BracketType) -> Result<usize, EngineError> {
        dispatch_simd!(self.simd; self, bracket_type =>
        fn <'i, I, V>(
            tail_skip: &mut TailSkip<'i, I, V::QuotesClassifier<'i, I>, V::StructuralClassifier<'i, I>, V, BLOCK_SIZE>,
            opening: BracketType) -> Result<usize, EngineError>
        where
            I: InputBlockIterator<'i, BLOCK_SIZE>,
            V: Simd
        {
            let mut idx = 0;
            let mut err = None;

            let classifier = tail_skip.classifier.take().expect("tail skip must always hold a classifier");

            tail_skip.classifier = Some('a: {
                let resume_state = classifier.stop();
                let DepthIteratorResumeOutcome(first_vector, mut depth_classifier) =
                    tail_skip.simd.resume_depth_classification(resume_state, opening);

                let mut current_vector = match first_vector {
                    Some(v) => Some(v),
                    None => match depth_classifier.next() {
                        Ok(v) => v,
                        Err(e) => {
                            err = Some(e);
                            let resume_state = depth_classifier.stop(None);
                            break 'a tail_skip.simd.resume_structural_classification(resume_state);
                        }
                    },
                };
                let mut current_depth = 1;

                'outer: while let Some(ref mut vector) = current_vector {
                    vector.add_depth(current_depth);

                    // TODO uncomment later
                    // debug!("Fetched vector, current depth is {current_depth}");
                    // debug!("Estimate: {}", vector.estimate_lowest_possible_depth());

                    if vector.estimate_lowest_possible_depth() <= 0 {
                        while vector.advance_to_next_depth_decrease() {
                            if vector.get_depth() == 0 {
                                // TODO uncomment later
                                // debug!("Encountered depth 0, breaking.");
                                break 'outer;
                            }
                        }
                    }

                    current_depth = vector.depth_at_end();
                    current_vector = match depth_classifier.next() {
                        Ok(v) => v,
                        Err(e) => {
                            err = Some(e);
                            let resume_state = depth_classifier.stop(None);
                            break 'a tail_skip.simd.resume_structural_classification(resume_state);
                        }
                    };
                }

                // TODO uncomment later
                // debug!("Skipping complete, resuming structural classification.");
                let resume_state = depth_classifier.stop(current_vector);
                // TODO uncomment later
                // debug!("Finished at {}", resume_state.get_idx());
                idx = resume_state.get_idx();
                tail_skip.simd.resume_structural_classification(resume_state)
            });

            if let Some(err) = err {
                Err(err.into())
            } else {
                Ok(idx)
            }
        })
    }

    pub(crate) fn stop(self) -> ResumeClassifierState<'i, I, V::QuotesClassifier<'i, I>, MaskType, BLOCK_SIZE> {
        self.classifier.expect("tail skip must always hold a classifier").stop()
    }
}

impl<'i, I, Q, S, V, const N: usize> std::ops::Deref for TailSkip<'i, I, Q, S, V, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
    S: StructuralIterator<'i, I, Q, MaskType, N>,
    V: Simd,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.classifier
            .as_ref()
            .expect("tail skip must always hold a classifier")
    }
}

impl<'i, I, Q, S, V, const N: usize> std::ops::DerefMut for TailSkip<'i, I, Q, S, V, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, MaskType, N>,
    S: StructuralIterator<'i, I, Q, MaskType, N>,
    V: Simd,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.classifier
            .as_mut()
            .expect("tail skip must always hold a classifier")
    }
}
