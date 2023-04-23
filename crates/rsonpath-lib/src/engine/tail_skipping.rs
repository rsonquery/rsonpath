use crate::classification::depth::{
    resume_depth_classification, DepthBlock, DepthIterator, DepthIteratorResumeOutcome,
};
#[cfg(feature = "head-skip")]
use crate::classification::ResumeClassifierState;
use crate::classification::{
    quotes::QuoteClassifiedIterator,
    structural::{BracketType, StructuralIterator},
    BLOCK_SIZE,
};
use crate::debug;
use crate::input::Input;
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

    pub(crate) fn skip(&mut self, opening: BracketType) -> usize {
        debug!("Skipping");
        let mut idx = 0;

        replace_with_or_abort(&mut self.classifier, |classifier| {
            let resume_state = classifier.stop();
            let DepthIteratorResumeOutcome(first_vector, mut depth_classifier) =
                resume_depth_classification(resume_state, opening);

            let mut current_vector = first_vector.or_else(|| depth_classifier.next());
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
                current_vector = depth_classifier.next();
            }

            debug!("Skipping complete, resuming structural classification.");
            let resume_state = depth_classifier.stop(current_vector);
            debug!("Finished at {}", resume_state.get_idx());
            idx = resume_state.get_idx();
            S::resume(resume_state)
        });

        idx
    }

    #[cfg(feature = "head-skip")]
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
