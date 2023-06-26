use crate::{
    classification::{
        depth::{resume_depth_classification, DepthBlock, DepthIterator, DepthIteratorResumeOutcome},
        quotes::QuoteClassifiedIterator,
        structural::{BracketType, StructuralIterator},
        ResumeClassifierState,
    },
    debug,
    engine::error::EngineError,
    input::Input,
    FallibleIterator, BLOCK_SIZE,
};
use replace_with::replace_with_or_abort;
use std::marker::PhantomData;

pub(crate) struct TailSkip<'b, I, Q, S, const N: usize>
where
    I: Input,
    Q: QuoteClassifiedIterator<'b, I, N>,
    S: StructuralIterator<'b, I, Q, N>,
{
    classifier: S,
    phantom: PhantomData<&'b (I, Q)>,
}

impl<'b, I, Q, S> TailSkip<'b, I, Q, S, BLOCK_SIZE>
where
    I: Input,
    Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
    S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
{
    pub(crate) fn new(classifier: S) -> Self {
        Self {
            classifier,
            phantom: PhantomData,
        }
    }

    pub(crate) fn skip(&mut self, opening: BracketType) -> Result<usize, EngineError> {
        debug!("Skipping");
        let mut idx = 0;
        let mut err = None;

        replace_with_or_abort(&mut self.classifier, |classifier| {
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
                        return S::resume(resume_state);
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
                        return S::resume(resume_state);
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

    pub(crate) fn stop(self) -> ResumeClassifierState<'b, I, Q, BLOCK_SIZE> {
        self.classifier.stop()
    }
}

impl<'b, I, Q, S, const N: usize> std::ops::Deref for TailSkip<'b, I, Q, S, N>
where
    I: Input,
    Q: QuoteClassifiedIterator<'b, I, N>,
    S: StructuralIterator<'b, I, Q, N>,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.classifier
    }
}

impl<'b, I, Q, S, const N: usize> std::ops::DerefMut for TailSkip<'b, I, Q, S, N>
where
    I: Input,
    Q: QuoteClassifiedIterator<'b, I, N>,
    S: StructuralIterator<'b, I, Q, N>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.classifier
    }
}
