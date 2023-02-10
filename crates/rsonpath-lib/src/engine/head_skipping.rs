use super::error::EngineError;
use super::result::QueryResult;
use crate::classify::{
    resume_structural_classification, ClassifierWithSkipping, Structural, StructuralIterator,
};
use crate::debug;
use crate::{
    query::{
        automaton::{Automaton, State},
        Label,
    },
    quotes::{classify_quoted_sequences, QuoteClassifiedIterator, ResumeClassifierState},
};
use aligners::{alignment, AlignedBytes};

pub(crate) trait CanHeadSkip<'b> {
    fn run_on_subtree<'r, R, Q, I>(
        &mut self,
        next_event: Structural,
        state: State,
        classifier: ClassifierWithSkipping<'b, Q, I>,
        result: &'r mut R,
    ) -> Result<ResumeClassifierState<'b, Q>, EngineError>
    where
        Q: QuoteClassifiedIterator<'b>,
        R: QueryResult,
        I: StructuralIterator<'b, Q>;
}

pub(crate) struct HeadSkip<'b, 'q> {
    bytes: &'b AlignedBytes<alignment::Page>,
    state: State,
    is_accepting: bool,
    label: &'q Label,
}

impl<'b, 'q> HeadSkip<'b, 'q> {
    pub(crate) fn new(
        bytes: &'b AlignedBytes<alignment::Page>,
        automaton: &'b Automaton<'q>,
    ) -> Option<Self> {
        let initial_state = automaton.initial_state();

        if automaton[initial_state].fallback_state() == initial_state {
            if let Some(&(label, target_state)) = automaton[initial_state].transitions().first() {
                debug!("Automaton starts with a descendant search, using memmem heuristic.");
                return Some(Self {
                    bytes,
                    state: target_state,
                    is_accepting: automaton.is_accepting(target_state),
                    label,
                });
            }
        }

        None
    }

    pub(crate) fn run_head_skipping<'r, E: CanHeadSkip<'b>, R: QueryResult>(
        &self,
        engine: &mut E,
        result: &'r mut R,
    ) -> Result<(), EngineError> {
        use memchr::memmem;

        let mut classifier_state = ResumeClassifierState {
            iter: classify_quoted_sequences(self.bytes.relax_alignment()),
            block: None,
            are_commas_on: false,
            are_colons_on: false,
        };
        let needle = self.label.bytes_with_quotes();
        let mut idx = 0;
        let finder = memmem::Finder::new(needle);

        while let Some(starting_quote_idx) = finder.find(&self.bytes[idx..]) {
            idx += starting_quote_idx;
            debug!("Needle found at {idx}");

            if idx != 0 && self.bytes[idx - 1] != b'\\' {
                let mut colon_idx = idx + needle.len();

                while colon_idx < self.bytes.len() && self.bytes[colon_idx].is_ascii_whitespace() {
                    colon_idx += 1;
                }

                if colon_idx < self.bytes.len() && self.bytes[colon_idx] == b':' {
                    let distance = colon_idx - classifier_state.get_idx();
                    debug!("Actual match with colon at {colon_idx}");
                    debug!("Distance skipped: {distance}");
                    classifier_state.offset_bytes(distance as isize);

                    if self.is_accepting {
                        result.report(colon_idx);
                    }

                    // Check if the colon is marked as within quotes.
                    // If yes, that is an error of state propagation through skipped blocks.
                    // Flip the quote mask.
                    if let Some(block) = classifier_state.block.as_mut() {
                        if (block.block.within_quotes_mask & (1_u64 << block.idx)) != 0 {
                            debug!("Mask needs flipping!");
                            block.block.within_quotes_mask = !block.block.within_quotes_mask;
                            classifier_state.iter.flip_quotes_bit();
                        }
                    }

                    classifier_state.offset_bytes(1);

                    let mut classifier = resume_structural_classification(classifier_state);
                    let next_event = classifier.next();

                    classifier_state = match next_event {
                        Some(opening @ Structural::Opening(opening_idx))
                            if self.bytes[colon_idx + 1..opening_idx]
                                .iter()
                                .all(u8::is_ascii_whitespace) =>
                        {
                            engine.run_on_subtree(
                                opening,
                                self.state,
                                ClassifierWithSkipping::new(classifier),
                                result,
                            )?
                        }
                        _ => classifier.stop(),
                    };

                    debug!("Quote classified up to {}", classifier_state.get_idx());
                    idx = classifier_state.get_idx();
                    continue;
                }
            }
            idx += 1;
        }

        Ok(())
    }
}
