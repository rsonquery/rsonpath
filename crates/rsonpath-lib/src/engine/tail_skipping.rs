use crate::{
    classification::{
        depth::{DepthBlock as _, DepthIterator as _, DepthIteratorResumeOutcome},
        quotes::QuoteClassifiedIterator,
        simd::{dispatch_simd, Simd},
        structural::{BracketType, StructuralIterator},
        ResumeClassifierState,
    },
    debug,
    engine::error::EngineError,
    input::InputBlockIterator,
    FallibleIterator as _, MaskType, BLOCK_SIZE,
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

    #[allow(
        clippy::expect_used,
        reason = "Enforcing the classifier invariant is clunky without this."
    )]
    pub(crate) fn skip(&mut self, opening: BracketType) -> Result<usize, EngineError> {
        dispatch_simd!(self.simd; self, opening =>
        fn <'i, I, V>(
            tail_skip: &mut TailSkip<'i, I, V::QuotesClassifier<'i, I>, V::StructuralClassifier<'i, I>, V, BLOCK_SIZE>,
            opening: BracketType) -> Result<usize, EngineError>
        where
            I: InputBlockIterator<'i, BLOCK_SIZE>,
            V: Simd
        {
            debug!("Skipping");
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

                    debug!("Fetched vector, current depth is {current_depth}");
                    debug!("Estimate: {}", vector.estimate_lowest_possible_depth());

                    if vector.estimate_lowest_possible_depth() <= 0 {
                        while vector.advance_to_next_depth_decrease() {
                            if vector.get_depth() == 0 {
                                debug!("Encountered depth 0, breaking.");
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

                debug!("Skipping complete, resuming structural classification.");
                let resume_state = depth_classifier.stop(current_vector);
                debug!("Finished at {}", resume_state.get_idx());
                idx = resume_state.get_idx() - 1;
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
