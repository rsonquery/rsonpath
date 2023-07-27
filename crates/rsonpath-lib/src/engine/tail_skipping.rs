#![allow(clippy::expect_used)] // Enforcing the classifier invariant is clunky without this.
use crate::{
    classification::{
        depth::{resume_depth_classification, DepthBlock, DepthIterator, DepthIteratorResumeOutcome},
        quotes::QuoteClassifiedIterator,
        structural::{BracketType, StructuralIterator},
        ResumeClassifierState,
    },
    debug,
    engine::error::EngineError,
    input::InputBlockIterator,
    FallibleIterator, BLOCK_SIZE,
};
use std::marker::PhantomData;

pub(crate) struct TailSkip<'i, I, Q, S, const N: usize> {
    classifier: Option<S>,
    _phantom: (PhantomData<&'i ()>, PhantomData<(I, Q)>),
}

impl<'i, I, Q, S> TailSkip<'i, I, Q, S, BLOCK_SIZE>
where
    I: InputBlockIterator<'i, BLOCK_SIZE>,
    Q: QuoteClassifiedIterator<'i, I, BLOCK_SIZE>,
    S: StructuralIterator<'i, I, Q, BLOCK_SIZE>,
{
    pub(crate) fn new(classifier: S) -> Self {
        Self {
            classifier: Some(classifier),
            _phantom: (PhantomData, PhantomData),
        }
    }

    pub(crate) fn skip(&mut self, opening: BracketType) -> Result<usize, EngineError> {
        debug!("Skipping");
        let mut idx = 0;
        let mut err = None;

        let classifier = self.classifier.take().expect("tail skip must always hold a classifier");

        self.classifier = Some('a: {
            let resume_state = classifier.stop();
            let DepthIteratorResumeOutcome(first_vector, mut depth_classifier) =
                resume_depth_classification(resume_state, opening);

            let mut current_vector = match first_vector {
                Some(v) => Some(v),
                None => match depth_classifier.next() {
                    Ok(v) => v,
                    Err(e) => {
                        err = Some(e);
                        let resume_state = depth_classifier.stop(None);
                        break 'a S::resume(resume_state);
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
                        break 'a S::resume(resume_state);
                    }
                };
            }

            debug!("Skipping complete, resuming structural classification.");
            let resume_state = depth_classifier.stop(current_vector);
            debug!("Finished at {}", resume_state.get_idx());
            idx = resume_state.get_idx();
            S::resume(resume_state)
        });

        if let Some(err) = err {
            Err(err.into())
        } else {
            Ok(idx)
        }
    }

    pub(crate) fn stop(self) -> ResumeClassifierState<'i, I, Q, BLOCK_SIZE> {
        self.classifier.expect("tail skip must always hold a classifier").stop()
    }
}

impl<'i, I, Q, S, const N: usize> std::ops::Deref for TailSkip<'i, I, Q, S, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, N>,
    S: StructuralIterator<'i, I, Q, N>,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        self.classifier
            .as_ref()
            .expect("tail skip must always hold a classifier")
    }
}

impl<'i, I, Q, S, const N: usize> std::ops::DerefMut for TailSkip<'i, I, Q, S, N>
where
    I: InputBlockIterator<'i, N>,
    Q: QuoteClassifiedIterator<'i, I, N>,
    S: StructuralIterator<'i, I, Q, N>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.classifier
            .as_mut()
            .expect("tail skip must always hold a classifier")
    }
}
