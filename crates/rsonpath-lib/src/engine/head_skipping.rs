//! Engine decorator that performs **head skipping** &ndash; an extremely optimized search for
//! the first matching label in a query starting with a self-looping state.
//! This happens in queries starting with a descendant selector.
use super::error::EngineError;
use crate::classification::{
    quotes::{classify_quoted_sequences, QuoteClassifiedIterator},
    structural::{resume_structural_classification, Structural, StructuralIterator},
    ResumeClassifierState,
};
use crate::debug;
use crate::query::{
    automaton::{Automaton, State},
    Label,
};
use crate::result::QueryResult;
use aligners::{alignment, AlignedBytes};

/// Trait that needs to be implemented by an [`Engine`](`super::Engine`) to use this submodule.
pub(super) trait CanHeadSkip<'b> {
    /// Function called when head-skipping finds a label at which normal query execution
    /// should resume.
    ///
    /// The [`HeadSkip::run_head_skipping`] function will call this implementation
    /// whenever it finds a label matching the first transition in the query.
    /// The structural `classifier` passed is guaranteed to have classified the
    /// `next_event` and nothing past that. It is guaranteed that
    /// `next_event` is [`Structural::Opening`].
    ///
    /// When called, the engine must start with in the automaton state as given in `state`
    /// and execute the query until a matching [`Structural::Closing`] character is encountered,
    /// using `classifier` for classification and `result` for reporting query results. The `classifier`
    /// must *not* be used to classify anything past the matching [`Structural::Closing`] character.
    fn run_on_subtree<'r, R, Q, I>(
        &mut self,
        next_event: Structural,
        state: State,
        structural_classifier: I,
        result: &'r mut R,
    ) -> Result<ResumeClassifierState<'b, Q>, EngineError>
    where
        Q: QuoteClassifiedIterator<'b>,
        R: QueryResult,
        I: StructuralIterator<'b, Q>;
}

/// Configuration of the head-skipping decorator.
pub(super) struct HeadSkip<'b, 'q> {
    bytes: &'b AlignedBytes<alignment::Page>,
    state: State,
    is_accepting: bool,
    label: &'q Label,
}

impl<'b, 'q> HeadSkip<'b, 'q> {
    /// Create a new instance of the head-skipping decorator over a given input
    /// and for a compiled query [`Automaton`].
    ///
    /// # Returns
    /// If head-skipping is possible for the query represented by `automaton`,
    /// returns [`Some`] with a configured instance of [`HeadSkip`].
    /// If head-skipping is not possible, returns [`None`].
    ///
    /// ## Details
    /// Head-skipping is possible if the query automaton starts
    /// with a state with a wildcard self-loop and a single labelled transition forward.
    /// Syntacticly, if the [`fallback_state`](`crate::query::automaton::StateTable::fallback_state`)
    /// of the [`initial_state`](`crate::query::automaton::StateTable::initial_state`) is the same as the
    /// [`initial_state`](`crate::query::automaton::StateTable::initial_state`), and its
    /// [`transitions`](`crate::query::automaton::StateTable::transitions`) are a single-element list.
    ///
    /// This means that we can search for the label of the forward transition in the entire document,
    /// disregarding any additional structure &ndash; during execution we would always loop
    /// around in the initial state until encountering the desired label. This search can be done
    /// extremely quickly with [`memchr::memmem`].
    ///
    /// In all other cases, head-skipping is not supported.
    pub(super) fn new(
        bytes: &'b AlignedBytes<alignment::Page>,
        automaton: &'b Automaton<'q>,
    ) -> Option<Self> {
        let initial_state = automaton.initial_state();
        let fallback_state = automaton[initial_state].fallback_state();
        let transitions = automaton[initial_state].transitions();

        if fallback_state == initial_state && transitions.len() == 1 {
            let (label, target_state) = transitions[0];
            debug!("Automaton starts with a descendant search, using memmem heuristic.");
            return Some(Self {
                bytes,
                state: target_state,
                is_accepting: automaton.is_accepting(target_state),
                label,
            });
        }

        None
    }

    /// Run a preconfigured [`HeadSkip`] using the given `engine` and reporting
    /// to the `result`.
    pub(super) fn run_head_skipping<'r, E: CanHeadSkip<'b>, R: QueryResult>(
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
            classifier_state.are_colons_on = false;
            classifier_state.are_commas_on = false;
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
                            engine.run_on_subtree(opening, self.state, classifier, result)?
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
