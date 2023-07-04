//! Main implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion or an explicit stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
use crate::{
    classification::{
        quotes::{classify_quoted_sequences, QuoteClassifiedIterator},
        structural::{classify_structural_characters, BracketType, Structural, StructuralIterator},
        ResumeClassifierState,
    },
    debug,
    engine::{
        depth::Depth,
        error::EngineError,
        head_skipping::{CanHeadSkip, HeadSkip},
        tail_skipping::TailSkip,
        Compiler, Engine, Input,
    },
    query::{
        automaton::{Automaton, State, TransitionLabel},
        error::CompilerError,
        JsonPathQuery, JsonString, NonNegativeArrayIndex,
    },
    recorder::Recorder,
    FallibleIterator, BLOCK_SIZE,
};
use smallvec::{smallvec, SmallVec};

/// Main engine for a fixed JSONPath query.
///
/// The engine is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct MainEngine<'q> {
    automaton: Automaton<'q>,
}

impl Compiler for MainEngine<'_> {
    type E<'q> = MainEngine<'q>;

    #[must_use = "compiling the query only creates an engine instance that should be used"]
    #[inline(always)]
    fn compile_query(query: &JsonPathQuery) -> Result<MainEngine, CompilerError> {
        let automaton = Automaton::new(query)?;
        debug!("DFA:\n {}", automaton);
        Ok(MainEngine { automaton })
    }

    #[inline(always)]
    fn from_compiled_query(automaton: Automaton<'_>) -> Self::E<'_> {
        MainEngine { automaton }
    }
}

impl Engine for MainEngine<'_> {
    #[inline]
    fn run<I: Input, R: Recorder>(&self, input: &I) -> Result<R::Result, EngineError> {
        if self.automaton.is_empty_query() {
            return empty_query::<I, R>(input);
        }

        let recorder = R::new();
        let executor = query_executor(&self.automaton, input, &recorder);
        executor.run()?;

        Ok(recorder.finish())
    }
}

fn empty_query<I: Input, R: Recorder>(bytes: &I) -> Result<R::Result, EngineError> {
    let recorder = R::new();
    {
        let quote_classifier = classify_quoted_sequences(bytes, &recorder);
        let mut block_event_source = classify_structural_characters(quote_classifier);

        if let Some(Structural::Opening(_, idx)) = block_event_source.next()? {
            recorder.record_match(idx);
        }
    }

    Ok(recorder.finish())
}

macro_rules! Classifier {
    () => {
        TailSkip<'b, I, Q, S, BLOCK_SIZE>
    };
}

struct Executor<'b, 'q, 'r, I: Input, R: Recorder> {
    depth: Depth,
    state: State,
    stack: SmallStack,
    automaton: &'b Automaton<'q>,
    input: &'b I,
    recorder: &'r R,
    next_event: Option<Structural>,
    is_list: bool,
    array_count: NonNegativeArrayIndex,
    has_any_array_item_transition: bool,
    has_any_array_item_transition_to_accepting: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeTypeHint {
    Atomic,
    AnyComplex,
    Complex(BracketType),
}

fn query_executor<'b, 'q, 'r, I: Input, R: Recorder>(
    automaton: &'b Automaton<'q>,
    input: &'b I,
    recorder: &'r R,
) -> Executor<'b, 'q, 'r, I, R> {
    Executor {
        depth: Depth::ZERO,
        state: automaton.initial_state(),
        stack: SmallStack::new(),
        automaton,
        input,
        recorder,
        next_event: None,
        is_list: false,
        array_count: NonNegativeArrayIndex::ZERO,
        has_any_array_item_transition: false,
        has_any_array_item_transition_to_accepting: false,
    }
}

impl<'b, 'q, 'r, I: Input, R: Recorder> Executor<'b, 'q, 'r, I, R>
where
    'r: 'b,
{
    fn run(mut self) -> Result<(), EngineError> {
        let mb_head_skip = HeadSkip::new(self.input, self.automaton);

        match mb_head_skip {
            Some(head_skip) => head_skip.run_head_skipping(&mut self),
            None => self.run_and_exit(),
        }
    }

    fn run_and_exit(mut self) -> Result<(), EngineError> {
        let quote_classifier = classify_quoted_sequences(self.input, self.recorder);
        let structural_classifier = classify_structural_characters(quote_classifier);
        let mut classifier = TailSkip::new(structural_classifier);

        self.run_on_subtree(&mut classifier)?;

        self.verify_subtree_closed()
    }

    fn run_on_subtree<Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>, S: StructuralIterator<'b, I, Q, BLOCK_SIZE>>(
        &mut self,
        classifier: &mut Classifier!(),
    ) -> Result<(), EngineError> {
        loop {
            if self.next_event.is_none() {
                self.next_event = classifier.next()?;
            }
            if let Some(event) = self.next_event {
                debug!("====================");
                debug!("Event = {:?}", event);
                debug!("Depth = {:?}", self.depth);
                debug!("Stack = {:?}", self.stack);
                debug!("State = {:?}", self.state);
                debug!("====================");

                self.next_event = None;
                match event {
                    Structural::Colon(idx) => self.handle_colon(classifier, idx)?,
                    Structural::Comma(idx) => self.handle_comma(classifier, idx)?,
                    Structural::Opening(b, idx) => self.handle_opening(classifier, b, idx)?,
                    Structural::Closing(_, idx) => {
                        self.handle_closing(classifier, idx)?;

                        if self.depth == Depth::ZERO {
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn record_match_detected_at(&mut self, start_idx: usize, hint: NodeTypeHint) -> Result<(), EngineError> {
        debug!("Reporting result somewhere after {start_idx} with hint {hint:?}");

        let index = match hint {
            NodeTypeHint::Complex(BracketType::Curly) => self.input.seek_forward(start_idx, [b'{'])?,
            NodeTypeHint::Complex(BracketType::Square) => self.input.seek_forward(start_idx, [b'['])?,
            NodeTypeHint::AnyComplex | NodeTypeHint::Atomic => self.input.seek_non_whitespace_forward(start_idx)?,
        }
        .map(|x| x.0);

        match index {
            Some(idx) => {
                self.recorder.record_match(idx);
                Ok(())
            }
            None => Err(EngineError::MissingItem()),
        }
    }

    fn handle_colon<Q, S>(&mut self, classifier: &mut Classifier!(), idx: usize) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
    {
        debug!("Colon");

        self.next_event = classifier.next()?;
        let is_next_opening = self.next_event.map_or(false, |s| s.is_opening());

        if !is_next_opening {
            let mut any_matched = false;

            for &(label, target) in self.automaton[self.state].transitions() {
                match label {
                    TransitionLabel::ArrayIndex(_) => {}
                    TransitionLabel::ObjectMember(member_name) => {
                        if self.automaton.is_accepting(target) && self.is_match(idx, member_name)? {
                            self.record_match_detected_at(
                                idx + 1,
                                NodeTypeHint::Atomic, /* since is_next_opening is false */
                            )?;
                            any_matched = true;
                            break;
                        }
                    }
                }
            }
            let fallback_state = self.automaton[self.state].fallback_state();
            if !any_matched && self.automaton.is_accepting(fallback_state) {
                self.record_match_detected_at(idx + 1, NodeTypeHint::Atomic /* since is_next_opening is false */)?;
            }
            #[cfg(feature = "unique-members")]
            {
                let is_next_closing = self.next_event.map_or(false, |s| s.is_closing());
                if any_matched && !is_next_closing && self.automaton.is_unitary(self.state) {
                    let bracket_type = self.current_node_bracket_type();
                    debug!("Skipping unique state from {bracket_type:?}");
                    let stop_at = classifier.skip(bracket_type)?;
                    self.next_event = Some(Structural::Closing(bracket_type, stop_at));
                }
            }
        }

        Ok(())
    }

    fn handle_comma<Q, S>(&mut self, classifier: &mut Classifier!(), idx: usize) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
    {
        self.next_event = classifier.next()?;

        let is_next_opening = self.next_event.map_or(false, |s| s.is_opening());

        let is_fallback_accepting = self.automaton.is_accepting(self.automaton[self.state].fallback_state());

        if !is_next_opening && self.is_list && is_fallback_accepting {
            debug!("Accepting on comma.");
            self.record_match_detected_at(idx + 1, NodeTypeHint::Atomic /* since is_next_opening is false */)?;
        }

        // After wildcard, check for a matching array index.
        // If the index increment exceeds the field's limit, give up.
        if self.is_list && self.array_count.try_increment().is_err() {
            return Ok(());
        }
        debug!("Incremented array count to {}", self.array_count);

        let match_index = self
            .automaton
            .has_array_index_transition_to_accepting(self.state, &self.array_count);

        if !is_next_opening && match_index {
            debug!("Accepting on list item.");
            self.record_match_detected_at(idx + 1, NodeTypeHint::Atomic /* since is_next_opening is false */)?;
        }

        Ok(())
    }

    fn handle_opening<Q, S>(
        &mut self,
        classifier: &mut Classifier!(),
        bracket_type: BracketType,
        idx: usize,
    ) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
    {
        debug!("Opening {bracket_type:?}, increasing depth and pushing stack.",);
        let mut any_matched = false;

        let colon_idx = self.find_preceding_colon(idx);

        for &(label, target) in self.automaton[self.state].transitions() {
            match label {
                TransitionLabel::ArrayIndex(i) => {
                    if self.is_list && i.eq(&self.array_count) {
                        any_matched = true;
                        self.transition_to(target, bracket_type);
                        if self.automaton.is_accepting(target) {
                            debug!("Accept {idx}");
                            self.record_match_detected_at(idx, NodeTypeHint::Complex(bracket_type))?;
                        }
                        break;
                    }
                }
                TransitionLabel::ObjectMember(member_name) => {
                    if let Some(colon_idx) = colon_idx {
                        if self.is_match(colon_idx, member_name)? {
                            any_matched = true;
                            self.transition_to(target, bracket_type);
                            if self.automaton.is_accepting(target) {
                                self.record_match_detected_at(colon_idx + 1, NodeTypeHint::Complex(bracket_type))?;
                            }
                            break;
                        }
                    }
                }
            }
        }

        if !any_matched && self.depth != Depth::ZERO {
            let fallback = self.automaton[self.state].fallback_state();
            debug!("Falling back to {fallback}");

            if self.automaton.is_rejecting(fallback) {
                classifier.skip(bracket_type)?;
                return Ok(());
            } else {
                self.transition_to(fallback, bracket_type);
            }

            if self.automaton.is_accepting(fallback) {
                self.record_match_detected_at(idx, NodeTypeHint::Complex(bracket_type))?;
            }
        }

        if bracket_type == BracketType::Square {
            self.is_list = true;
            self.has_any_array_item_transition = self.automaton.has_any_array_item_transition(self.state);
            self.has_any_array_item_transition_to_accepting =
                self.automaton.has_any_array_item_transition_to_accepting(self.state);

            let fallback = self.automaton[self.state].fallback_state();
            let is_fallback_accepting = self.automaton.is_accepting(fallback);

            let searching_list = is_fallback_accepting || self.has_any_array_item_transition;

            if searching_list {
                classifier.turn_commas_on(idx);
                self.array_count = NonNegativeArrayIndex::ZERO;
                debug!("Initialized array count to {}", self.array_count);

                let wants_first_item =
                    is_fallback_accepting || self.automaton.has_first_array_index_transition_to_accepting(self.state);

                if wants_first_item {
                    self.next_event = classifier.next()?;

                    match self.next_event {
                        Some(Structural::Closing(_, close_idx)) => {
                            if let Some((next_idx, _)) = self.input.seek_non_whitespace_forward(idx + 1)? {
                                if next_idx < close_idx {
                                    self.record_match_detected_at(
                                        next_idx,
                                        NodeTypeHint::Atomic, /* since the next structural is the closing of the list */
                                    )?;
                                }
                            }
                        }
                        Some(Structural::Comma(_)) => {
                            self.record_match_detected_at(
                                idx + 1,
                                NodeTypeHint::Atomic, /* since the next structural is a ','*/
                            )?;
                        }
                        _ => (),
                    }
                }
            } else {
                classifier.turn_commas_off();
            }
        } else {
            classifier.turn_commas_off();
            self.is_list = false;
        }

        if !self.is_list && self.automaton.has_transition_to_accepting(self.state) {
            classifier.turn_colons_on(idx);
        } else {
            classifier.turn_colons_off();
        }
        self.depth
            .increment()
            .map_err(|err| EngineError::DepthAboveLimit(idx, err))?;

        Ok(())
    }

    fn handle_closing<Q, S>(&mut self, classifier: &mut Classifier!(), idx: usize) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
    {
        debug!("Closing, decreasing depth and popping stack.");

        self.depth
            .decrement()
            .map_err(|err| EngineError::DepthBelowZero(idx, err))?;

        if let Some(stack_frame) = self.stack.pop_if_at_or_below(*self.depth) {
            self.state = stack_frame.state;
            self.is_list = stack_frame.is_list;
            self.array_count = stack_frame.array_count;
            self.has_any_array_item_transition = stack_frame.has_any_array_item_transition;
            self.has_any_array_item_transition_to_accepting = stack_frame.has_any_array_item_transition_to_accepting;

            debug!("Restored array count to {}", self.array_count);

            #[cfg(feature = "unique-members")]
            if self.automaton.is_unitary(self.state) {
                let bracket_type = self.current_node_bracket_type();
                debug!("Skipping unique state from {bracket_type:?}");
                let close_idx = classifier.skip(bracket_type)?;
                self.next_event = Some(Structural::Closing(bracket_type, close_idx));
                return Ok(());
            }
        }

        if self.is_list
            && (self.automaton.is_accepting(self.automaton[self.state].fallback_state())
                || self.has_any_array_item_transition)
        {
            classifier.turn_commas_on(idx);
        } else {
            classifier.turn_commas_off();
        }

        if !self.is_list && self.automaton.has_transition_to_accepting(self.state) {
            classifier.turn_colons_on(idx);
        } else {
            classifier.turn_colons_off();
        }

        Ok(())
    }

    fn transition_to(&mut self, target: State, opening: BracketType) {
        let target_is_list = opening == BracketType::Square;

        let fallback = self.automaton[self.state].fallback_state();
        let is_fallback_accepting = self.automaton.is_accepting(fallback);
        let searching_list = is_fallback_accepting || self.has_any_array_item_transition;

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
                has_any_array_item_transition: self.has_any_array_item_transition,
                has_any_array_item_transition_to_accepting: self.has_any_array_item_transition_to_accepting,
            });
            self.state = target;
        }
    }

    fn find_preceding_colon(&self, idx: usize) -> Option<usize> {
        if self.depth == Depth::ZERO {
            None
        } else {
            let (char_idx, char) = self.input.seek_non_whitespace_backward(idx - 1)?;

            (char == b':').then_some(char_idx)
        }
    }

    fn is_match(&self, idx: usize, member_name: &JsonString) -> Result<bool, EngineError> {
        let len = member_name.bytes_with_quotes().len();

        let closing_quote_idx = match self.input.seek_backward(idx - 1, b'"') {
            Some(x) => x,
            None => return Err(EngineError::MalformedStringQuotes(idx - 1)),
        };

        if closing_quote_idx + 1 < len {
            return Ok(false);
        }

        let start_idx = closing_quote_idx + 1 - len;
        Ok(self.input.is_member_match(start_idx, closing_quote_idx, member_name))
    }

    fn verify_subtree_closed(&self) -> Result<(), EngineError> {
        if self.depth != Depth::ZERO {
            Err(EngineError::MissingClosingCharacter())
        } else {
            Ok(())
        }
    }

    #[cfg(feature = "unique-members")]
    fn current_node_bracket_type(&self) -> BracketType {
        if self.is_list {
            BracketType::Square
        } else {
            BracketType::Curly
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StackFrame {
    depth: u8,
    state: State,
    is_list: bool,
    array_count: NonNegativeArrayIndex,
    has_any_array_item_transition: bool,
    has_any_array_item_transition_to_accepting: bool,
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

impl<'b, 'q, 'r, I: Input, R: Recorder> CanHeadSkip<'b, 'r, I, R, BLOCK_SIZE> for Executor<'b, 'q, 'r, I, R>
where
    'r: 'b,
{
    fn run_on_subtree<Q, S>(
        &mut self,
        next_event: Structural,
        state: State,
        structural_classifier: S,
    ) -> Result<ResumeClassifierState<'b, I, Q, BLOCK_SIZE>, EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
    {
        let mut classifier = TailSkip::new(structural_classifier);

        self.state = state;
        self.next_event = Some(next_event);

        self.run_on_subtree(&mut classifier)?;
        self.verify_subtree_closed()?;

        Ok(classifier.stop())
    }

    fn recorder(&mut self) -> &'r R {
        self.recorder
    }
}
