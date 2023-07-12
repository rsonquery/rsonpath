//! Engine decorator that performs **head skipping** &ndash; an extremely optimized search for
//! the first matching member name in a query starting with a self-looping state.
//! This happens in queries starting with a descendant selector.
use crate::{
    classification::{
        memmem::Memmem,
        quotes::{classify_quoted_sequences, resume_quote_classification, InnerIter, QuoteClassifiedIterator},
        structural::{resume_structural_classification, BracketType, Structural, StructuralIterator},
        ResumeClassifierBlockState, ResumeClassifierState,
    },
    debug,
    depth::Depth,
    engine::EngineError,
    input::{Input, InputBlockIterator},
    query::{
        automaton::{Automaton, State},
        JsonString,
    },
    result::Recorder,
    FallibleIterator, BLOCK_SIZE,
};

/// Trait that needs to be implemented by an [`Engine`](`super::Engine`) to use this submodule.
pub(super) trait CanHeadSkip<'b, 'r, I: Input, R: Recorder, const N: usize> {
    /// Function called when head-skipping finds a member name at which normal query execution
    /// should resume.
    ///
    /// The [`HeadSkip::run_head_skipping`] function will call this implementation
    /// whenever it finds a member name matching the first transition in the query.
    /// The structural `classifier` passed is guaranteed to have classified the
    /// `next_event` and nothing past that. It is guaranteed that
    /// `next_event` is [`Structural::Opening`].
    ///
    /// When called, the engine must start with in the automaton state as given in `state`
    /// and execute the query until a matching [`Structural::Closing`] character is encountered,
    /// using `classifier` for classification and `result` for reporting query results. The `classifier`
    /// must *not* be used to classify anything past the matching [`Structural::Closing`] character.
    fn run_on_subtree<Q, S>(
        &mut self,
        next_event: Structural,
        state: State,
        structural_classifier: S,
    ) -> Result<ResumeClassifierState<'b, I, Q, N>, EngineError>
    where
        I: Input,
        Q: QuoteClassifiedIterator<'b, I, N>,
        S: StructuralIterator<'b, I, Q, N>;

    fn recorder(&mut self) -> &'r R;
}

/// Configuration of the head-skipping decorator.
pub(super) struct HeadSkip<'b, 'q, I: Input, const N: usize> {
    bytes: &'b I,
    state: State,
    is_accepting: bool,
    member_name: &'q JsonString,
}

impl<'b, 'q, I: Input> HeadSkip<'b, 'q, I, BLOCK_SIZE> {
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
    /// with a state with a wildcard self-loop and a single member-labelled transition forward.
    /// Syntactically, if the [`fallback_state`](`crate::query::automaton::StateTable::fallback_state`)
    /// of the [`initial_state`](`crate::query::automaton::StateTable::initial_state`) is the same as the
    /// [`initial_state`](`crate::query::automaton::StateTable::initial_state`), and its
    /// [`transitions`](`crate::query::automaton::StateTable::transitions`) are a single-element list.
    ///
    /// This means that we can search for the label of the forward transition in the entire document,
    /// disregarding any additional structure &ndash; during execution we would always loop
    /// around in the initial state until encountering the desired member name. This search can be done
    /// extremely quickly with [`memchr::memmem`].
    ///
    /// In all other cases, head-skipping is not supported.
    pub(super) fn new(bytes: &'b I, automaton: &'b Automaton<'q>) -> Option<Self> {
        let initial_state = automaton.initial_state();
        let fallback_state = automaton[initial_state].fallback_state();
        let transitions = automaton[initial_state].transitions();

        if fallback_state == initial_state && transitions.len() == 1 {
            let (label, target_state) = transitions[0];

            if let Some(member_name) = label.get_member_name() {
                debug!("Automaton starts with a descendant search, using memmem heuristic.");

                return Some(Self {
                    bytes,
                    state: target_state,
                    is_accepting: automaton.is_accepting(target_state),
                    member_name,
                });
            }
        }

        None
    }

    /// Run a preconfigured [`HeadSkip`] using the given `engine` and reporting
    /// to the `result`.
    pub(super) fn run_head_skipping<'r, E: CanHeadSkip<'b, 'r, I, R, BLOCK_SIZE>, R: Recorder>(
        &self,
        engine: &mut E,
    ) -> Result<(), EngineError>
    where
        R: 'r,
        'r: 'b,
    {
        let mut input_iter = self.bytes.iter_blocks(engine.recorder());
        let mut idx = 0;
        let mut first_block = None;

        loop {
            let mut memmem = crate::classification::memmem::memmem(self.bytes, &mut input_iter);
            debug!("Starting memmem search from {idx}");

            if let Some((starting_quote_idx, last_block)) = memmem.find_label(first_block, idx, self.member_name)? {
                drop(memmem);

                first_block = Some(last_block);
                idx = starting_quote_idx;
                debug!("Needle found at {idx}");
                let seek_start_idx = idx + self.member_name.bytes_with_quotes().len();

                match self.bytes.seek_non_whitespace_forward(seek_start_idx)? {
                    Some((colon_idx, char)) if char == b':' => {
                        let (next_idx, next_c) = self
                            .bytes
                            .seek_non_whitespace_forward(colon_idx + 1)?
                            .ok_or(EngineError::MissingItem())?;

                        // The goal is initializing the quote classifier correctly.
                        // We can do it as follows:
                        // - Initialize it to point to the start of the first block.
                        // - Now we need to move it to the next_idx. Calculate the offset from that point.
                        // - Offset by that much plus one.
                        let start_of_second_block = input_iter.get_offset();
                        debug_assert!(start_of_second_block >= BLOCK_SIZE);
                        let start_of_first_block = start_of_second_block - BLOCK_SIZE;
                        let distance_to_colon = colon_idx - start_of_first_block;
                        let distance_to_value = next_idx - start_of_first_block - distance_to_colon + 1;

                        let (quote_classifier, quote_classified_first_block) =
                            resume_quote_classification(input_iter, first_block);
                        let mut classifier_state = ResumeClassifierState {
                            iter: quote_classifier,
                            block: quote_classified_first_block
                                .map(|b| ResumeClassifierBlockState { block: b, idx: 0 }),
                            are_colons_on: false,
                            are_commas_on: self.is_accepting,
                        };

                        debug!("Actual match with colon at {colon_idx}");
                        debug!("Next significant character at {next_idx}");
                        debug!("Classifier claims it's at {}", classifier_state.get_idx());
                        debug!(
                            "It also has its first block at {}",
                            classifier_state.block.as_ref().unwrap().idx
                        );
                        debug!("We want to offset by {distance_to_colon} first, then by {distance_to_value}",);

                        classifier_state.offset_bytes(distance_to_colon as isize)?;

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

                        classifier_state.offset_bytes(distance_to_value as isize)?;

                        classifier_state = match next_c {
                            b'{' | b'[' => {
                                debug!("resuming");
                                if self.is_accepting {
                                    engine.recorder().record_match(
                                        next_idx,
                                        Depth::ZERO,
                                        crate::result::MatchedNodeType::Complex,
                                    );
                                }
                                let classifier = resume_structural_classification(classifier_state);
                                engine.run_on_subtree(
                                    Structural::Opening(
                                        if next_c == b'{' {
                                            BracketType::Curly
                                        } else {
                                            BracketType::Square
                                        },
                                        next_idx,
                                    ),
                                    self.state,
                                    classifier,
                                )?
                            }
                            _ if self.is_accepting => {
                                engine.recorder().record_match(
                                    next_idx,
                                    Depth::ZERO,
                                    crate::result::MatchedNodeType::Atomic,
                                );
                                let mut classifier = resume_structural_classification(classifier_state);
                                let next_structural = classifier.next()?.unwrap();
                                // The value we found must be atomic, since the next structural is not an Opening.
                                // To ensure correct processing by the recorder, we report the match, and then
                                // a terminating structural. We deliberately lie that it's a comma to not influence
                                // the depth. This is a HACK, we should probably have a more clear way of
                                // communicating this to the recorder.

                                engine
                                    .recorder()
                                    .record_structural(Structural::Comma(next_structural.idx()));
                                classifier.stop()
                            }
                            _ => classifier_state,
                        };

                        debug!("Quote classified up to {}", classifier_state.get_idx());
                        idx = classifier_state.get_idx();

                        first_block = classifier_state.block.map(|b| b.block.block);
                        input_iter = classifier_state.iter.into_inner();
                    }
                    _ => idx += 1,
                }
            } else {
                debug!("No memmem matches, exiting");
                break;
            }
        }

        Ok(())
    }
}
