//! Main implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion, with an explicit minimal stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
//!

/* ## Overview
 *
 * This is the most complex part of the engine. It glues together all the moving parts from the other modules
 * and executes the main loop, iterating over the stream of [`Structural`] characters from the classifier.
 *
 * The main job is to do as little work as possible, and skip as much as possible. Head skipping is handled
 * independently in [`HeadSkip`], but the engine needs to drive tail-skipping. This is done by inspecting the
 * automaton at various points and skipping if we are:
 * - in a rejecting state, in which case nothing really matters until end of the subtree;
 * - in a unitary state after having already taken the only possible non-rejecting transition.
 *
 * The base work that has to be done at all times is reacting to opening and closing characters that dictate
 * the tree structure. Member name before an opening character can be easily found with a quick look-behind (skipping whitespace).
 * Most of the time atomic values don't matter - they're an empty subtree. They need to be handled in only two cases:
 *   - we are in a state that can accept (has a transition to an accepting state) and need to know the locations
 *     of atomics to possibly record them as matches;
 *   - we are in a list and the concrete value of the current index matters for the automaton, in which
 *     case we need to count atomics to advance the counter.
 *
 * In the first case, we require colons and commas to delimit atomics within objects and lists.
 * The "special" case of the first element in a list is handled separately and rather ungracefully.
 * In the second case, we only need commas to use as "milestones" to count our way through the list.
 *
 * ## Executor state
 *
 * The core driver of control is the current state of the query automaton.
 * We also need an auxiliary piece of information on whether we are in a list, and what
 * is the current list index. When entering a subtree and changing the state, it has to be preserved
 * on a stack. The stack used is supposed to be "small", in the sense that we only push when any part of the
 * state changes.
 */

#![allow(clippy::type_complexity)] // The private Classifier type is very complex, but we specifically macro it out.
use crate::input::SeekableBackwardsInput;
use crate::{
    automaton::{error::CompilerError, Automaton, State},
    classification::{
        simd::{self, config_simd, dispatch_simd, Simd, SimdConfiguration},
        structural::{BracketType, Structural, StructuralIterator},
    },
    debug,
    depth::Depth,
    engine::{
        error::EngineError,
        head_skipping::{CanHeadSkip, HeadSkip, ResumeState},
        select_root_query,
        tail_skipping::TailSkip,
        Compiler, Engine,
    },
    input::error::InputErrorConvertible,
    result::{
        approx_span::ApproxSpanRecorder, count::CountRecorder, index::IndexRecorder, nodes::NodesRecorder, Match,
        MatchCount, MatchIndex, MatchSpan, MatchedNodeType, Recorder, Sink,
    },
    FallibleIterator, MaskType, BLOCK_SIZE,
};
use rsonpath_syntax::{num::JsonUInt, str::JsonString, JsonPathQuery};
use smallvec::{smallvec, SmallVec};
use crate::result::InputRecorder;

/// Main engine for a fixed JSONPath query.
///
/// The engine is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct MainEngine<'q> {
    automaton: Automaton<'q>,
    simd: SimdConfiguration,
}

impl Compiler for MainEngine<'_> {
    type E<'q> = MainEngine<'q>;

    #[must_use = "compiling the query only creates an engine instance that should be used"]
    #[inline(always)]
    fn compile_query(query: &JsonPathQuery) -> Result<MainEngine, CompilerError> {
        let automaton = Automaton::new(query)?;
        debug!("DFA:\n {}", automaton);
        let simd = simd::configure();
        log::info!("SIMD configuration:\n {}", simd);
        Ok(MainEngine { automaton, simd })
    }

    #[inline(always)]
    fn from_compiled_query(automaton: Automaton<'_>) -> Self::E<'_> {
        let simd = simd::configure();
        log::info!("SIMD configuration:\n {}", simd);
        MainEngine { automaton, simd }
    }
}

/* The engine has multiple entry methods depending on what type of result is required.
 * This allows more efficient implementations for simpler results. For example,
 * getting full Match objects is the most expensive, while a simple count is very fast in comparison.
 *
 * The logic for each entry point is analogous:
 * - we separately handle an empty query, which immediately returns an empty result,
 *   and a root-only query, which can be much faster in many cases.
 * - we set up an appropriate Recorder impl for the result type.
 * - we configure SIMD and run the Executor in its context.
 */
impl Engine for MainEngine<'_> {
    #[inline]
    fn count<'i, 'r, I, R, const N: usize>(&self, input: &I) -> Result<MatchCount, EngineError>
    where
        I: SeekableBackwardsInput<'i, 'r, R, N>,
        R: InputRecorder<I::Block> +'r,
    {
        if self.automaton.is_select_root_query() {
            return select_root_query::count(input);
        }
        if self.automaton.is_empty_query() {
            return Ok(0);
        }

        let recorder = CountRecorder::new();
        config_simd!(self.simd => |simd| {
            let executor = query_executor(&self.automaton, input, &recorder, simd);
            executor.run()
        })?;

        Ok(recorder.into())
    }

    #[inline]
    fn indices<'i, 'r, I, R, S, const N: usize>(&self, input: &I, sink: &mut S) -> Result<(), EngineError>
    where
        I: SeekableBackwardsInput<'i, 'r, R, N>,
        R: InputRecorder<I::Block> + 'r,
        S: Sink<MatchIndex>,
    {
        if self.automaton.is_select_root_query() {
            return select_root_query::index(input, sink);
        }
        if self.automaton.is_empty_query() {
            return Ok(());
        }

        let recorder = IndexRecorder::new(sink, input.leading_padding_len());
        config_simd!(self.simd => |simd| {
            let executor = query_executor(&self.automaton, input, &recorder, simd);
            executor.run()
        })?;

        Ok(())
    }

    #[inline]
    fn approximate_spans<'i, 'r, I, R, S, const N: usize>(&self, input: &I, sink: &mut S) -> Result<(), EngineError>
    where
        I: SeekableBackwardsInput<'i, 'r, R, N>,
        R: InputRecorder<I::Block> + 'r,
        S: Sink<MatchSpan>,
    {
        if self.automaton.is_select_root_query() {
            return select_root_query::approx_span(input, sink);
        }
        if self.automaton.is_empty_query() {
            return Ok(());
        }

        let recorder = ApproxSpanRecorder::new(sink, input.leading_padding_len());
        config_simd!(self.simd => |simd| {
            let executor = query_executor(&self.automaton, input, &recorder, simd);
            executor.run()
        })?;

        Ok(())
    }

    #[inline]
    fn matches<'i, 'r, I, R, S, const N: usize>(&self, input: &I, sink: &mut S) -> Result<(), EngineError>
    where
        I: SeekableBackwardsInput<'i, 'r, R, N>,
        R: InputRecorder<I::Block> + 'r,
        S: Sink<Match>,
    {
        if self.automaton.is_select_root_query() {
            return select_root_query::match_(input, sink);
        }
        if self.automaton.is_empty_query() {
            return Ok(());
        }

        let recorder = NodesRecorder::build_recorder(sink, input.leading_padding_len());
        config_simd!(self.simd => |simd| {
            let executor = query_executor(&self.automaton, input, &recorder, simd);
            executor.run()
        })?;

        Ok(())
    }
}

// This is a convenience macro to hide the enormous type of the classifier.
// It expects generic types `I` (the Input implementation), `R` (the Recorder),
// and `V` (the SIMD context).
macro_rules! Classifier {
    () => {
        TailSkip<
            'i,
            I::BlockIterator,
            V::QuotesClassifier<'i, I::BlockIterator>,
            V::StructuralClassifier<'i, I::BlockIterator>,
            V,
            BLOCK_SIZE>
    };
}

/// This is the heart of an Engine run that holds the entire execution state.
struct Executor<'i, 'q, 'r, I, R, V> {
    /// Current depth in the JSON tree.
    depth: Depth,
    /// Current automaton state.
    state: State,
    /// Lookahead of at most one Structural character.
    next_event: Option<Structural>,
    /// Whether the current JSON node is a list.
    is_list: bool,
    /// Index of the next element in the list, if is_list is true.
    // FIXME: This and is_list can probably be merged into Option<JsonUInt> carrying all the information.
    array_count: JsonUInt,
    /// Execution stack.
    stack: SmallStack,
    /// Read-only access to the query automaton.
    automaton: &'i Automaton<'q>,
    /// Handle to the input.
    input: &'i I,
    /// Handle to the recorder.
    recorder: &'r R,
    /// Resolved SIMD context.
    simd: V,
}

/// Initialize the [`Executor`] for the initial state of a query.
fn query_executor<'i, 'q, 'r, I, R, V: Simd>(
    automaton: &'i Automaton<'q>,
    input: &'i I,
    recorder: &'r R,
    simd: V,
) -> Executor<'i, 'q, 'r, I, R, V>
where
    I: SeekableBackwardsInput<'i, 'r, R, BLOCK_SIZE>,
    R: Recorder<I::Block>,
{
    Executor {
        depth: Depth::ZERO,
        state: automaton.initial_state(),
        stack: SmallStack::new(),
        automaton,
        input,
        recorder,
        simd,
        next_event: None,
        is_list: false,
        array_count: JsonUInt::ZERO,
    }
}

impl<'i, 'r, I, R, V> Executor<'i, '_, 'r, I, R, V>
where
    'i: 'r,
    I: SeekableBackwardsInput<'i, 'r, R, BLOCK_SIZE>,
    R: Recorder<I::Block>,
    V: Simd,
{
    fn run(mut self) -> Result<(), EngineError> {
        // First we check if head-skipping is possible for a given query automaton.
        // If yes, delegate the control to HeadSkip and give it full access to this Executor.
        // Otherwise, we run our normal one-shot engine.
        let mb_head_skip = HeadSkip::new(self.input, self.automaton, self.simd);

        match mb_head_skip {
            Some(head_skip) => head_skip.run_head_skipping(&mut self),
            None => self.run_and_exit(),
        }
    }

    /// One-shot run of the engine on whatever JSON tree starts at the current input.
    /// As soon as the tree is closed, the engine exits.
    fn run_and_exit(mut self) -> Result<(), EngineError> {
        let iter = self.input.iter_blocks(self.recorder);
        let quote_classifier = self.simd.classify_quoted_sequences(iter);
        let structural_classifier = self.simd.classify_structural_characters(quote_classifier);
        let mut classifier = TailSkip::new(structural_classifier, self.simd);

        self.run_on_subtree(&mut classifier)?;

        self.verify_subtree_closed()
    }

    /// This is _the_ main loop, the heart and the soul of the engine.
    /// We loop through the document based on the `classifier`'s outputs and handle each event.
    /// Once the perceived depth of the document goes to zero, this method terminates.
    fn run_on_subtree(&mut self, classifier: &mut Classifier!()) -> Result<(), EngineError> {
        dispatch_simd!(self.simd; self, classifier =>
        fn<'i, 'q, 'r, I, R, V>(eng: &mut Executor<'i, 'q, 'r, I, R, V>, classifier: &mut Classifier!()) -> Result<(), EngineError>
        where
            'i: 'r,
            I: SeekableBackwardsInput<'i, 'r, R, BLOCK_SIZE>,
            R: Recorder<I::Block>,
            V: Simd
        {
            loop {
                // Fetch the next element only if the lookahead is empty.
                if eng.next_event.is_none() {
                    eng.next_event = match classifier.next() {
                        Ok(e) => e,
                        Err(err) => return Err(EngineError::InputError(err)),
                    };
                }
                if let Some(event) = eng.next_event.take() {
                    debug!("====================");
                    debug!("Event = {:?}", event);
                    debug!("Depth = {:?}", eng.depth);
                    debug!("Stack = {:?}", eng.stack);
                    debug!("State = {:?}", eng.state);
                    debug!("====================");

                    match event {
                        Structural::Colon(idx) => eng.handle_colon(classifier, idx)?,
                        Structural::Comma(idx) => eng.handle_comma(classifier, idx)?,
                        Structural::Opening(b, idx) => eng.handle_opening(classifier, b, idx)?,
                        Structural::Closing(_, idx) => {
                            eng.handle_closing(classifier, idx)?;

                            if eng.depth == Depth::ZERO {
                                break;
                            }
                        }
                    }
                } else {
                    break;
                }
            }

            Ok(())
        })
    }

    /// Handle a colon at index `idx`.
    /// This method only handles atomic values after the colon.
    /// Objects and arrays are processed at their respective opening character.
    #[inline(always)]
    fn handle_colon(
        &mut self,
        #[allow(unused_variables)] classifier: &mut Classifier!(),
        idx: usize,
    ) -> Result<(), EngineError> {
        debug!("Colon");

        // Lookahead to see if the next character is an opening.
        // If yes, the logic will be handled in handle_opening and we bail.
        if let Some((_, c)) = self.input.seek_non_whitespace_forward(idx + 1).e()? {
            if c == b'{' || c == b'[' {
                return Ok(());
            }
        }

        // Atomic values are only relevant if the automaton accepts.
        // Look at accepting transitions and try to match them with the label.
        let mut any_matched = false;

        for &(member_name, target) in self.automaton[self.state].member_transitions() {
            if self.automaton.is_accepting(target) && self.is_match(idx, member_name)? {
                self.record_match_detected_at(idx + 1, NodeType::Atomic)?;
                any_matched = true;
                break;
            }
        }
        // Alternatively, match consider the fallback transition if it accepts.
        let fallback_state = self.automaton[self.state].fallback_state();
        if !any_matched && self.automaton.is_accepting(fallback_state) {
            self.record_match_detected_at(idx + 1, NodeType::Atomic)?;
        }

        // Tail skipping.
        // If we are in a unitary state and have matched a transition, we can skip the rest of the subtree,
        // since member names are unique.
        if any_matched && self.automaton.is_unitary(self.state) {
            // We need to look ahead for some bookkeeping.
            // 1. If the next event is closing then there's no reason to spin up the skipping machinery,
            //    since it would exit immediately anyway.
            // 2. If the next character is a comma then we need to notify the recorder.
            // 3. Realistically, a colon should never happen. An opening is not interesting and will be skipped.
            self.next_event = classifier.next()?;
            match self.next_event {
                None | Some(Structural::Closing(_, _)) => {
                    return Ok(());
                }
                Some(Structural::Comma(idx)) => self.recorder.record_value_terminator(idx, self.depth)?,
                Some(Structural::Colon(_) | Structural::Opening(_, _)) => (),
            }
            let bracket_type = self.current_node_bracket_type();
            debug!("Skipping unique state from {bracket_type:?}");
            let stop_at = classifier.skip(bracket_type)?;
            // Skipping stops at the closing character *and consumes it*. We still need the main loop to properly
            // handle a closing, so we set the lookahead to the correct character.
            self.next_event = Some(Structural::Closing(bracket_type, stop_at));
        }

        Ok(())
    }

    /// Handle a comma at index `idx`.
    /// This method only handles atomic values after the comma.
    /// Objects and arrays are processed at their respective opening character.
    #[inline(always)]
    fn handle_comma(&mut self, _classifier: &mut Classifier!(), idx: usize) -> Result<(), EngineError> {
        debug!("Comma");

        self.recorder.record_value_terminator(idx, self.depth)?;

        if self.is_list {
            // If the index increment exceeds the field's limit, give up.
            if self.array_count.try_increment().is_err() {
                return Ok(());
            }

            // Lookahead to see if the next character is an opening.
            // If yes, the logic will be handled in handle_opening and we bail.
            if let Some((_, c)) = self.input.seek_non_whitespace_forward(idx + 1).e()? {
                if c == b'{' || c == b'[' {
                    return Ok(());
                }
            }

            // Check the fallback transition first since it's cheap, then check for the specific index.
            let is_fallback_accepting = self.automaton.is_accepting(self.automaton[self.state].fallback_state());

            if is_fallback_accepting
                || self
                    .automaton
                    .has_array_index_transition_to_accepting(self.state, &self.array_count)
            {
                debug!("Accepting list item on comma.");
                self.record_match_detected_at(idx + 1, NodeType::Atomic)?;
            }
        }

        Ok(())
    }

    /// Handle the opening of a subtree with given `bracket_type` at index `idx`.
    #[inline(always)]
    fn handle_opening(
        &mut self,
        classifier: &mut Classifier!(),
        bracket_type: BracketType,
        idx: usize,
    ) -> Result<(), EngineError> {
        debug!("Opening {bracket_type:?}, increasing depth and pushing stack.",);

        // Check all transitions relevant to the current subtree - array if in list, member if not.
        let mut any_matched = false;
        if self.is_list {
            for trans in self.automaton[self.state].array_transitions() {
                if trans.matches(self.array_count) {
                    let target = trans.target_state();
                    any_matched = true;
                    self.transition_to(target, bracket_type);
                    if self.automaton.is_accepting(target) {
                        debug!("Accept {idx}");
                        self.record_match_detected_at(idx, NodeType::Complex(bracket_type))?;
                    }
                    break;
                }
            }
        } else {
            let colon_idx = self.find_preceding_colon(idx);

            for &(member_name, target) in self.automaton[self.state].member_transitions() {
                if let Some(colon_idx) = colon_idx {
                    if self.is_match(colon_idx, member_name)? {
                        any_matched = true;
                        self.transition_to(target, bracket_type);
                        if self.automaton.is_accepting(target) {
                            debug!("Accept {idx}");
                            self.record_match_detected_at(colon_idx + 1, NodeType::Complex(bracket_type))?;
                        }
                        break;
                    }
                }
            }
        }

        // If nothing matched trigger the fallback transition.
        if !any_matched && self.depth != Depth::ZERO {
            let fallback = self.automaton[self.state].fallback_state();
            debug!("Falling back to {fallback}");

            if self.automaton.is_rejecting(fallback) {
                // Tail skipping. Skip the entire subtree. The skipping consumes the closing character.
                // We still need to notify the recorder - in case the value being skipped was actually accepted.
                let closing_idx = classifier.skip(bracket_type)?;
                return self.recorder.record_value_terminator(closing_idx, self.depth);
            } else {
                self.transition_to(fallback, bracket_type);
            }

            if self.automaton.is_accepting(fallback) {
                self.record_match_detected_at(idx, NodeType::Complex(bracket_type))?;
            }
        }

        // At this point we will be actually digging into the subtree.
        self.depth
            .increment()
            .map_err(|err| EngineError::DepthAboveLimit(idx, err))?;

        self.is_list = bracket_type == BracketType::Square;
        let mut needs_commas = false;

        // If we're starting a list, there's a very hairy problem of accepting the first element in the list,
        // if it is atomic. We process objects and arrays on their opening character, and atomics on their preceding comma.
        // The first element doesn't have a preceding comma, so if it needs to be accepted we need to handle it now.
        //
        // Additionally, whether to enable commas or not depends on whether an item of the list can ever be accepted.
        if self.is_list {
            let fallback = self.automaton[self.state].fallback_state();
            let is_fallback_accepting = self.automaton.is_accepting(fallback);

            if is_fallback_accepting || self.automaton.has_any_array_item_transition(self.state) {
                needs_commas = true;
                self.array_count = JsonUInt::ZERO;
                debug!("Initialized array count to {}", self.array_count);

                let wants_first_item =
                    is_fallback_accepting || self.automaton.has_first_array_index_transition_to_accepting(self.state);

                if wants_first_item {
                    let next = self.input.seek_non_whitespace_forward(idx + 1).e()?;

                    // We only handle the match if it exists and is atomic. The possible cases
                    // in a well-formed JSON for the next character are:
                    // - '[', for an array value
                    // - '{' for an object value
                    // - ']' if the list was empty and has no values
                    // - otherwise it's the first character of an atomic value.
                    match next {
                        Some((_, b'[' | b'{' | b']')) => (), // Complex value or empty list.
                        Some((value_idx, _)) => {
                            self.record_match_detected_at(value_idx, NodeType::Atomic)?;
                        }
                        _ => (),
                    }
                }
            }
        }

        // Decide which structural characters need to be handled in this subtree.
        if !self.is_list && self.automaton.has_transition_to_accepting(self.state) {
            // When accepting values in an object we need colons for the member names,
            // and commas to report where atomic values end (for the Recorder).
            // This is the only case that needs colons.
            classifier.turn_colons_and_commas_on(idx);
        } else if needs_commas {
            classifier.turn_colons_off();
            classifier.turn_commas_on(idx);
        } else {
            classifier.turn_colons_and_commas_off();
        }

        Ok(())
    }

    /// Handle the closing of a subtree at index `idx`.
    #[inline(always)]
    fn handle_closing(&mut self, classifier: &mut Classifier!(), idx: usize) -> Result<(), EngineError> {
        debug!("Closing, decreasing depth and popping stack.");

        self.depth
            .decrement()
            .map_err(|err| EngineError::DepthBelowZero(idx, err))?;
        self.recorder.record_value_terminator(idx, self.depth)?;

        // Restore the state from the stack if the transition was not a loop.
        if let Some(stack_frame) = self.stack.pop_if_at_or_below(*self.depth) {
            self.state = stack_frame.state;
            self.is_list = stack_frame.is_list;
            self.array_count = stack_frame.array_count;

            debug!("Restored array count to {}", self.array_count);

            // We have taken a transition when entering the just-closed subtree. If the state is unitary
            // we can just skip the rest of the current subtree.
            if self.automaton.is_unitary(self.state) {
                let bracket_type = self.current_node_bracket_type();
                debug!("Skipping unique state from {bracket_type:?}");
                let close_idx = classifier.skip(bracket_type)?;
                // Skipping stops at the closing character *and consumes it*. We still need the main loop to properly
                // handle a closing, so we set the lookahead to the correct character.
                self.next_event = Some(Structural::Closing(bracket_type, close_idx));
                return Ok(());
            }
        }

        if self.is_list {
            if self.automaton.is_accepting(self.automaton[self.state].fallback_state())
                || self.automaton.has_any_array_item_transition(self.state)
            {
                classifier.turn_commas_on(idx);
            } else {
                classifier.turn_commas_off();
            }
        } else if self.automaton.has_transition_to_accepting(self.state) {
            classifier.turn_colons_and_commas_on(idx);
        } else {
            classifier.turn_colons_off();
        }

        Ok(())
    }

    /// Trigger the transition to the `target` state into a new subtree
    /// that opened with `opening`.
    #[inline(always)]
    fn transition_to(&mut self, target: State, opening: BracketType) {
        let target_is_list = opening == BracketType::Square;

        let fallback = self.automaton[self.state].fallback_state();
        let is_fallback_accepting = self.automaton.is_accepting(fallback);
        let searching_list = is_fallback_accepting || self.automaton.has_any_array_item_transition(self.state);

        // To keep the stack small, we only push if the state only changes in any meaningful way.
        if target != self.state || target_is_list != self.is_list || searching_list {
            debug!(
                "push {}, goto {target}, is_list = {target_is_list}, array_count: {}",
                self.state, self.array_count
            );

            self.stack.push(StackFrame {
                depth: *self.depth,
                state: self.state,
                is_list: self.is_list,
                array_count: self.array_count,
            });
            self.state = target;
        }
    }

    /// Find the preceding non-whitespace character and return its index if it's a colon.
    fn find_preceding_colon(&self, idx: usize) -> Option<usize> {
        if self.depth == Depth::ZERO {
            None
        } else {
            let (char_idx, char) = self.input.seek_non_whitespace_backward(idx - 1)?;

            (char == b':').then_some(char_idx)
        }
    }

    /// Check if the label ended with a colon at index `idx` matches the `member_name`.
    #[inline(always)]
    fn is_match(&self, idx: usize, member_name: &JsonString) -> Result<bool, EngineError> {
        let len = member_name.quoted().len();

        // The colon can be preceded by whitespace before the actual label.
        let closing_quote_idx = match self.input.seek_backward(idx - 1, b'"') {
            Some(x) => x,
            None => return Err(EngineError::MalformedStringQuotes(idx - 1)),
        };

        // First check if the length matches.
        if closing_quote_idx + 1 < len {
            return Ok(false);
        }

        // Do the expensive memcmp.
        let start_idx = closing_quote_idx + 1 - len;
        self.input
            .is_member_match(start_idx, closing_quote_idx + 1, member_name)
            .map_err(|x| x.into().into())
    }

    /// Pass information to the Recorder that we found a match of type `ty` at `start_idx`.
    fn record_match_detected_at(&mut self, start_idx: usize, ty: NodeType) -> Result<(), EngineError> {
        debug!("Reporting result somewhere after {start_idx} with node type {ty:?}");

        let index = match ty {
            NodeType::Complex(BracketType::Curly) => self.input.seek_forward(start_idx, [b'{']).e()?,
            NodeType::Complex(BracketType::Square) => self.input.seek_forward(start_idx, [b'[']).e()?,
            NodeType::Atomic => self.input.seek_non_whitespace_forward(start_idx).e()?,
        }
        .map(|x| x.0);

        match index {
            Some(idx) => self.recorder.record_match(idx, self.depth, ty.into()),
            None => Err(EngineError::MissingItem()),
        }
    }

    /// Verify that we have reached zero depth, raise an error if not.
    fn verify_subtree_closed(&self) -> Result<(), EngineError> {
        if self.depth != Depth::ZERO {
            Err(EngineError::MissingClosingCharacter())
        } else {
            Ok(())
        }
    }

    /// Get the [`BracketType`] of current subtree.
    fn current_node_bracket_type(&self) -> BracketType {
        if self.is_list {
            BracketType::Square
        } else {
            BracketType::Curly
        }
    }
}

/// A single frame on the [`Executor`]'s stack enabling restoration of the entire
/// execution state to before a subtree opening.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StackFrame {
    depth: u8,
    state: State,
    is_list: bool,
    array_count: JsonUInt,
}

#[derive(Debug)]
struct SmallStack {
    contents: SmallVec<[StackFrame; 128]>,
}

impl SmallStack {
    fn new() -> Self {
        Self { contents: smallvec![] }
    }

    #[inline]
    fn peek(&mut self) -> Option<StackFrame> {
        self.contents.last().copied()
    }

    #[inline]
    fn pop_if_at_or_below(&mut self, depth: u8) -> Option<StackFrame> {
        if let Some(stack_frame) = self.peek() {
            if depth <= stack_frame.depth {
                return self.contents.pop();
            }
        }
        None
    }

    #[inline]
    fn push(&mut self, value: StackFrame) {
        self.contents.push(value)
    }
}

impl<'i, 'r, I, R, V> CanHeadSkip<'i, 'r, I, R, V> for Executor<'i, '_, 'r, I, R, V>
where
    I: SeekableBackwardsInput<'i, 'r, R, BLOCK_SIZE>,
    R: Recorder<I::Block>,
    V: Simd,
    'i: 'r,
{
    fn run_on_subtree(
        &mut self,
        next_event: Structural,
        state: State,
        structural_classifier: V::StructuralClassifier<'i, I::BlockIterator>,
    ) -> Result<ResumeState<'i, I::BlockIterator, V, MaskType>, EngineError> {
        let mut classifier = TailSkip::new(structural_classifier, self.simd);

        self.state = state;
        self.next_event = Some(next_event);

        self.run_on_subtree(&mut classifier)?;
        self.verify_subtree_closed()?;

        Ok(ResumeState(classifier.stop()))
    }

    fn recorder(&mut self) -> &'r R {
        self.recorder
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeType {
    Atomic,
    Complex(BracketType),
}

impl From<NodeType> for MatchedNodeType {
    #[inline(always)]
    fn from(value: NodeType) -> Self {
        match value {
            NodeType::Atomic => Self::Atomic,
            NodeType::Complex(_) => Self::Complex,
        }
    }
}
