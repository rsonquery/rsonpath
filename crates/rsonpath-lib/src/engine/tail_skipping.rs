use crate::classification::depth::{
    resume_depth_classification, DepthBlock, DepthIterator, DepthIteratorResumeOutcome,
};
use crate::classification::quotes::QuoteClassifiedIterator;
use crate::classification::structural::StructuralIterator;
#[cfg(feature = "head-skip")]
use crate::classification::ResumeClassifierState;
use crate::debug;
use replace_with::replace_with_or_abort;
use std::marker::PhantomData;

pub(crate) struct TailSkip<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    classifier: I,
    phantom: PhantomData<&'b Q>,
}

impl<'b, Q, I> TailSkip<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    pub(crate) fn new(classifier: I) -> Self {
        Self {
            classifier,
            phantom: PhantomData,
        }
    }

    pub(crate) fn skip(&mut self, opening: u8) -> usize {
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
            I::resume(resume_state)
        });

        idx
    }

    #[cfg(feature = "head-skip")]
    pub(crate) fn stop(self) -> ResumeClassifierState<'b, Q> {
        self.classifier.stop()
    }
}

impl<'b, Q, I> std::ops::Deref for TailSkip<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.classifier
    }
}

impl<'b, Q, I> std::ops::DerefMut for TailSkip<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.classifier
    }
}
