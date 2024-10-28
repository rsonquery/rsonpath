//! Engine decorator that performs **head skipping** &ndash; an extremely optimized search for
//! the first matching member name in a query starting with a self-looping state.
//! This happens in queries starting with a descendant selector.

use std::marker::PhantomData;
use crate::{
    automaton::{Automaton, State},
    classification::{
        mask::Mask,
        memmem::Memmem,
        quotes::{InnerIter, QuoteClassifiedIterator, ResumedQuoteClassifier},
        simd::{dispatch_simd, Simd},
        structural::{BracketType, Structural, StructuralIterator},
        ResumeClassifierBlockState, ResumeClassifierState,
    },
    debug,
    depth::Depth,
    engine::EngineError,
    input::{
        error::{InputError, InputErrorConvertible},
        Input, InputBlockIterator,
    },
    result::Recorder,
    FallibleIterator, MaskType, BLOCK_SIZE,
};
use rsonpath_syntax::str::JsonString;
use crate::result::InputRecorder;

/// Trait that needs to be implemented by an [`Engine`](`super::Engine`) to use this submodule.
pub(super) trait CanHeadSkip<'i, 'r, I, R, V>
where
    I: Input<'i, 'r, R, BLOCK_SIZE> + 'i,
    R: Recorder<I::Block> + 'r,
    V: Simd,
{
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
    fn run_on_subtree(
        &mut self,
        next_event: Structural,
        state: State,
        structural_classifier: V::StructuralClassifier<'i, I::BlockIterator>,
    ) -> Result<ResumeState<'i, I::BlockIterator, V, MaskType>, EngineError>;

    fn recorder(&mut self) -> &'r R;
}

pub(super) struct ResumeState<'i, I, V, M>(
    pub(super) ResumeClassifierState<'i, I, V::QuotesClassifier<'i, I>, M, BLOCK_SIZE>,
)
where
    I: InputBlockIterator<'i, BLOCK_SIZE>,
    V: Simd;

/// Configuration of the head-skipping decorator.
pub(super) struct HeadSkip<'b, 'q, 'r, I, R, V, const N: usize>
where
    I: Input<'b, 'r, R, N> + 'b,
    R: InputRecorder<I::Block> + 'r,
{
    bytes: &'b I,
    state: State,
    is_accepting: bool,
    member_name: &'q JsonString,
    simd: V,
    _recorder: PhantomData<&'r R>,
}

impl<'b, 'q, 'r, I, R, V> HeadSkip<'b, 'q, 'r, I, R, V, BLOCK_SIZE>
where
    I: Input<'b, 'r, R, BLOCK_SIZE>,
    R: Recorder<I::Block>,
    V: Simd
{
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
    /// extremely quickly with [`classification::memmem`](crate::classification::memmem).
    ///
    /// In all other cases, head-skipping is not supported.
    pub(super) fn new(bytes: &'b I, automaton: &'b Automaton<'q>, simd: V) -> Option<Self> {
        let initial_state = automaton.initial_state();
        let fallback_state = automaton[initial_state].fallback_state();
        let transitions = automaton[initial_state].member_transitions();

        if fallback_state == initial_state
            && transitions.len() == 1
            && automaton[initial_state].array_transitions().is_empty()
        {
            let (member_name, target_state) = transitions[0];
            debug!("Automaton starts with a descendant search, using memmem heuristic.");
            return Some(Self {
                bytes,
                state: target_state,
                is_accepting: automaton.is_accepting(target_state),
                member_name,
                simd,
                _recorder: PhantomData,
            });
        }

        None
    }

    /// Run a preconfigured [`HeadSkip`] using the given `engine` and reporting
    /// to the `result`.
    pub(super) fn run_head_skipping<E>(&self, engine: &mut E) -> Result<(), EngineError>
    where
        'b: 'r,
        E: CanHeadSkip<'b, 'r, I, R, V>,
        R: Recorder<I::Block> + 'r,
    {
        dispatch_simd!(self.simd; self, engine =>
        fn<'b, 'q, 'r, I, V, E, R>(head_skip: &HeadSkip<'b, 'q, 'r, I, R, V, BLOCK_SIZE>, engine: &mut E) -> Result<(), EngineError>
        where
            'b: 'r,
            E: CanHeadSkip<'b, 'r, I, R, V>,
            R: Recorder<I::Block> + 'r,
            I: Input<'b, 'r, R, BLOCK_SIZE>,
            V: Simd
        {
            let mut input_iter = head_skip.bytes.iter_blocks(engine.recorder());
            let mut idx = 0;
            let mut first_block = None;

            loop {
                let mut memmem = head_skip.simd.memmem(head_skip.bytes, &mut input_iter);
                debug!("Starting memmem search from {idx}");

                if let Some((starting_quote_idx, last_block)) = memmem.find_label(first_block, idx, head_skip.member_name)? {
                    drop(memmem);

                    first_block = Some(last_block);
                    idx = starting_quote_idx;
                    debug!("Needle found at {idx}");
                    let seek_start_idx = idx + head_skip.member_name.quoted().len();

                match head_skip.bytes.seek_non_whitespace_forward(seek_start_idx).e()? {
                    Some((colon_idx, b':')) => {
                        let (next_idx, next_c) = head_skip
                            .bytes
                            .seek_non_whitespace_forward(colon_idx + 1).e()?
                            .ok_or(EngineError::MissingItem())?;

                            let ResumedQuoteClassifier {
                                classifier: quote_classifier,
                                first_block: quote_classified_first_block,
                            } = head_skip.simd.resume_quote_classification(input_iter, first_block);

                            // Temporarily set the index within the current block to zero.
                            // This makes sense for the move below.
                            let mut classifier_state = ResumeClassifierState {
                                iter: quote_classifier,
                                block: quote_classified_first_block
                                    .map(|b| ResumeClassifierBlockState { block: b, idx: 0 }),
                                are_colons_on: false,
                                are_commas_on: head_skip.is_accepting,
                            };

                            debug!("Actual match with colon at {colon_idx}");
                            debug!("Next significant character at {next_idx}");
                            debug!("Classifier is at {}", classifier_state.get_idx());
                            debug!("We will forward to {colon_idx} first, then to {next_idx}",);

                            // Now we want to move the entire iterator state so that the current block is quote-classified,
                            // and correctly points to the place the engine would expect had it found the matching key
                            // in the regular loop. If the value is atomic, we handle it ourselves. If the value is complex,
                            // the engine wants to start one byte *after* the opening character. However, the match report
                            // has to happen before we advance one more byte, or else the opening character might be lost
                            // in the output (if it happens at a block boundary).
                            if next_c == b'{' || next_c == b'[' {
                                forward_to(&mut classifier_state, next_idx)?;
                                if head_skip.is_accepting {
                                    engine.recorder().record_match(
                                        next_idx,
                                        Depth::ZERO,
                                        crate::result::MatchedNodeType::Complex,
                                    )?;
                                }
                                forward_to(&mut classifier_state, next_idx + 1)?;
                            } else {
                                forward_to(&mut classifier_state, next_idx)?;
                            };

                            // We now have the block where we want and we ran quote classification, but during the `forward_to`
                            // call we lose all the flow-through quote information that usually is passed from one block to the next.
                            // We need to manually verify the soundness of the classification. Fortunately:
                            // 1. we know that resume_idx is either the start of a value, or one byte after an opening -
                            //    in a valid JSON this character can be within quotes if and only if it is itself a quote;
                            // 2. the only way the mask can be wrong is if it is flipped - marks chars within quotes
                            //    as outside and vice versa - so it suffices to flip it if it is wrong.
                            if let Some(block) = classifier_state.block.as_mut() {
                                let should_be_quoted = block.block.block[block.idx] == b'"';
                                if block.block.within_quotes_mask.is_lit(block.idx) != should_be_quoted {
                                    debug!("Mask needs flipping!");
                                    block.block.within_quotes_mask = !block.block.within_quotes_mask;
                                    classifier_state.iter.flip_quotes_bit();
                                }
                            }

                            classifier_state = match next_c {
                                b'{' | b'[' => {
                                    debug!("resuming");
                                    let classifier = head_skip.simd.resume_structural_classification(classifier_state);
                                    engine
                                        .run_on_subtree(
                                            Structural::Opening(
                                                if next_c == b'{' {
                                                    BracketType::Curly
                                                } else {
                                                    BracketType::Square
                                                },
                                                next_idx,
                                            ),
                                            head_skip.state,
                                            classifier,
                                        )?
                                        .0
                                }
                                _ if head_skip.is_accepting => {
                                    engine.recorder().record_match(
                                        next_idx,
                                        Depth::ZERO,
                                        crate::result::MatchedNodeType::Atomic,
                                    )?;
                                    let mut classifier = head_skip.simd.resume_structural_classification(classifier_state);
                                    let next_structural = classifier.next()?;

                                    match next_structural {
                                        Some(s) => engine.recorder().record_value_terminator(s.idx(), Depth::ZERO)?,
                                        None => return Err(EngineError::MissingClosingCharacter()),
                                    }
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

            return Ok(());

            /// Move the state forward to `index`.
            ///
            /// # Errors
            /// If the offset crosses block boundaries, then a new block is read from the underlying
            /// [`Input`](crate::input::Input) implementation, which can fail.
            ///
            /// # Panics
            /// If the `index` is not ahead of the current position of the state ([`get_idx`](ResumeClassifierState::get_idx)).
            #[inline(always)]
            #[allow(clippy::panic_in_result_fn)]
            fn forward_to<'i, I, Q, M, const N: usize>(state: &mut ResumeClassifierState<'i, I, Q, M, N>, index: usize) -> Result<(), InputError>
            where
                I: InputBlockIterator<'i, N>,
                Q: QuoteClassifiedIterator<'i, I, M, N>,
            {
                let current_block_start = state.iter.get_offset();
                let current_block_idx = state.block.as_ref().map_or(0, |b| b.idx);
                let current_idx = current_block_start + current_block_idx;

                debug!(
                    "Calling forward_to({index}) when the inner iter offset is {current_block_start} and block idx is {current_block_idx:?}"
                );

                // We want to move by this much forward, and delta > 0.
                assert!(index > current_idx);
                let delta = index - current_idx;

                // First we virtually pretend to move *backward*, setting the index of the current block to zero,
                // and adjust the delta to cover that distance. This makes calculations simpler.
                // Then we need to skip zero or more blocks and set our self.block to the last one we visit.
                let remaining = delta + current_block_idx;
                let blocks_to_skip = remaining / N;
                let remainder = remaining % N;

                match state.block.as_mut() {
                    Some(b) if blocks_to_skip == 0 => {
                        b.idx = remaining;
                    }
                    Some(_) => {
                        state.block = state
                            .iter
                            .offset(blocks_to_skip as isize)?
                            .map(|b| ResumeClassifierBlockState {
                                block: b,
                                idx: remainder,
                            });
                    }
                    None => {
                        state.block = state
                            .iter
                            .offset((blocks_to_skip + 1) as isize)?
                            .map(|b| ResumeClassifierBlockState {
                                block: b,
                                idx: remainder,
                            });
                    }
                }

                debug!("forward_to({index}) results in idx moved to {}", state.get_idx());

                Ok(())
            }
        })
    }
}
