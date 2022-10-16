use super::StructuralIterator;
use crate::{
    classify::depth::seek::{
        resume_depth_seek_classification, DepthSeekBlock, DepthSeekIterator,
        DepthSeekIteratorResumeOutcome,
    },
    classify::depth::{
        resume_depth_classification, DepthBlock, DepthIterator, DepthIteratorResumeOutcome,
    },
    debug,
    query::Label,
    quotes::{QuoteClassifiedIterator, ResumeClassifierState},
};
use std::marker::PhantomData;

pub(crate) struct FastForwardingClassifier<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    classifier: Option<I>,
    phantom: PhantomData<&'b Q>,
}

#[derive(Debug)]
pub(crate) struct FastForwardToLabelResult {
    pub(crate) idx: usize,
    pub(crate) depth_increase: isize,
}

impl<'b, Q, I> FastForwardingClassifier<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    pub(crate) fn new(classifier: I) -> Self {
        Self {
            classifier: Some(classifier),
            phantom: PhantomData,
        }
    }

    pub(crate) fn fast_forward_to_end(&mut self, opening: u8) {
        debug!("Skipping");

        let classifier = unsafe { self.classifier.take().unwrap_unchecked() };
        let resume_state = classifier.stop();
        let DepthIteratorResumeOutcome(first_vector, mut depth_classifier) =
            resume_depth_classification(resume_state, opening);

        let mut current_vector = first_vector.or_else(|| depth_classifier.next());
        let mut current_depth = 1;

        'outer: while let Some(ref mut vector) = current_vector {
            vector.add_depth(current_depth);

            debug!("Fetched vector, current depth is {current_depth}");
            debug!("Estimate: {}", vector.estimate_lowest_possible_depth());

            while vector.estimate_lowest_possible_depth() <= 0
                && vector.advance_to_next_depth_decrease()
            {
                if vector.get_depth() == 0 {
                    debug!("Encountered depth 0, breaking.");
                    break 'outer;
                }
            }

            current_depth = vector.depth_at_end();
            current_vector = depth_classifier.next();
        }

        debug!("Skipping complete, resuming structural classification.");
        let resume_state = depth_classifier.stop(current_vector);
        self.classifier = Some(I::resume(resume_state));
    }

    pub(crate) fn fast_forward_to_label(
        &mut self,
        label: &Label,
        bytes: &[u8],
        opening: u8,
    ) -> Option<FastForwardToLabelResult> {
        debug!("Skipping to label {:?}", label);

        let classifier = self.classifier.take().unwrap();
        let resume_state = classifier.stop();
        let DepthSeekIteratorResumeOutcome(first_vector, mut depth_aware_label_classifier) =
            resume_depth_seek_classification(resume_state, label, opening);
        let mut result = None;

        let mut current_vector = first_vector.or_else(|| depth_aware_label_classifier.next());
        let mut current_depth = 1;

        'outer: while let Some(ref mut vector) = current_vector {
            vector.add_depth(current_depth);

            debug!("Fetched vector, current depth is {current_depth}");
            debug!("Estimate: {}", vector.estimate_lowest_possible_depth());

            while let Some(match_idx) = vector.advance_to_next_possible_match() {
                debug!("Possible match at {match_idx}");

                while let Some(depth_idx) = vector.advance_to_next_depth_decrease() {
                    if depth_idx > match_idx {
                        break;
                    }
                    if vector.get_depth() == 0 {
                        debug!("Encountered depth 0, breaking.");
                        break 'outer;
                    }
                }

                if let Some(colon_idx) = Self::is_actual_label_match(match_idx, label, bytes) {
                    debug!("True match!");
                    result = Some(FastForwardToLabelResult {
                        idx: colon_idx,
                        depth_increase: vector.get_depth(),
                    });
                    break 'outer;
                }
            }

            while vector.estimate_lowest_possible_depth() <= 0
                && vector.advance_to_next_depth_decrease().is_some()
            {
                if vector.get_depth() == 0 {
                    debug!("Encountered depth 0, breaking.");
                    break 'outer;
                }
            }

            vector.advance_to_end();
            current_depth = vector.depth_at_end();
            current_vector = depth_aware_label_classifier.next();
        }

        debug!("Resuming structural classification.");
        let mut resume_state =
            depth_aware_label_classifier.stop(current_vector, result.as_ref().map(|r| r.idx));

        if result.is_some() {
            // Check if the colon is marked as within quotes.
            // If yes, that is an error of state propagation through skipped blocks.
            // Flip the quote mask.
            if let Some(block) = resume_state.block.as_mut() {
                if (block.block.within_quotes_mask & (1u64 << block.idx)) != 0 {
                    debug!("Mask needs flipping!");
                    block.block.within_quotes_mask = !block.block.within_quotes_mask;
                    resume_state.iter.flip_quotes_bit();
                }
            }

            resume_state.offset_bytes(1);
        }

        self.classifier = Some(I::resume(resume_state));

        result
    }

    pub(crate) fn stop(mut self) -> ResumeClassifierState<'b, Q> {
        unsafe { self.classifier.take().unwrap_unchecked() }.stop()
    }

    fn is_actual_label_match(idx: usize, label: &Label, bytes: &[u8]) -> Option<usize> {
        if idx != 0 && bytes[idx - 1] != b'\\' {
            let len = label.bytes_with_quotes().len();
            let mut colon_idx = idx + len;

            while colon_idx < bytes.len() && bytes[colon_idx].is_ascii_whitespace() {
                colon_idx += 1;
            }

            if colon_idx < bytes.len()
                && bytes[colon_idx] == b':'
                && &bytes[idx..idx + len] == label.bytes_with_quotes()
            {
                return Some(colon_idx);
            }
        }

        None
    }
}

impl<'b, Q, I> std::ops::Deref for FastForwardingClassifier<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.classifier.as_ref().unwrap()
    }
}

impl<'b, Q, I> std::ops::DerefMut for FastForwardingClassifier<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.classifier.as_mut().unwrap()
    }
}
