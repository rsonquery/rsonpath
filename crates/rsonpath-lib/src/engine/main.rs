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
    depth::Depth,
    engine::{
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
    result::{
        count::CountRecorder, index::IndexRecorder, nodes::NodesRecorder, Match, MatchCount, MatchIndex,
        MatchedNodeType, Recorder, Sink,
    },
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
    fn count<I>(&self, input: &I) -> Result<MatchCount, EngineError>
    where
        I: Input,
    {
        let recorder = CountRecorder::new();

        if self.automaton.is_empty_query() {
            empty_query(input, &recorder)?;
            return Ok(recorder.into());
        }

        let executor = query_executor(&self.automaton, input, &recorder);
        executor.run()?;

        Ok(recorder.into())
    }

    #[inline]
    fn indices<I, S>(&self, input: &I, sink: &mut S) -> Result<(), EngineError>
    where
        I: Input,
        S: Sink<MatchIndex>,
    {
        let recorder = IndexRecorder::new(sink);

        if self.automaton.is_empty_query() {
            empty_query(input, &recorder)?;
            return Ok(());
        }

        let executor = query_executor(&self.automaton, input, &recorder);
        executor.run()?;

        Ok(())
    }

    #[inline]
    fn run<I, S>(&self, input: &I, sink: &mut S) -> Result<(), EngineError>
    where
        I: Input,
        S: Sink<Match>,
    {
        let recorder = NodesRecorder::build_recorder(sink);

        if self.automaton.is_empty_query() {
            return empty_query(input, &recorder);
        }

        let executor = query_executor(&self.automaton, input, &recorder);
        executor.run()?;

        Ok(())
    }
}

fn empty_query<'i, I, R>(input: &'i I, recorder: &R) -> Result<(), EngineError>
where
    I: Input + 'i,
    R: Recorder<I::Block<'i, BLOCK_SIZE>>,
{
    {
        let iter = input.iter_blocks(recorder);
        let quote_classifier = classify_quoted_sequences(iter);
        let mut block_event_source = classify_structural_characters(quote_classifier);

        let last_event = block_event_source.next()?;
        if let Some(Structural::Opening(_, idx)) = last_event {
            let mut depth = Depth::ONE;
            recorder.record_match(idx, depth, MatchedNodeType::Complex)?;

            while let Some(ev) = block_event_source.next()? {
                match ev {
                    Structural::Closing(_, idx) => {
                        recorder.record_value_terminator(idx, depth)?;
                        depth.decrement().map_err(|err| EngineError::DepthBelowZero(idx, err))?;
                    }
                    Structural::Colon(_) => (),
                    Structural::Opening(_, idx) => {
                        depth
                            .increment()
                            .map_err(|err| EngineError::DepthAboveLimit(idx, err))?;
                    }
                    Structural::Comma(idx) => recorder.record_value_terminator(idx, depth)?,
                }
            }
        }
    }

    Ok(())
}

macro_rules! Classifier {
    () => {
        TailSkip<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, S, BLOCK_SIZE>
    };
}

struct Executor<'i, 'q, 'r, I, R> {
    depth: Depth,
    state: State,
    stack: SmallStack,
    automaton: &'i Automaton<'q>,
    input: &'i I,
    recorder: &'r R,
    next_event: Option<Structural>,
    is_list: bool,
    array_count: NonNegativeArrayIndex,
    has_any_array_item_transition: bool,
    has_any_array_item_transition_to_accepting: bool,
}

fn query_executor<'i, 'q, 'r, I, R>(
    automaton: &'i Automaton<'q>,
    input: &'i I,
    recorder: &'r R,
) -> Executor<'i, 'q, 'r, I, R>
where
    I: Input,
    R: Recorder<I::Block<'i, BLOCK_SIZE>>,
{
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

impl<'i, 'q, 'r, I, R> Executor<'i, 'q, 'r, I, R>
where
    'i: 'r,
    I: Input,
    R: Recorder<I::Block<'i, BLOCK_SIZE>>,
{
    fn run(mut self) -> Result<(), EngineError> {
        let mb_head_skip = HeadSkip::new(self.input, self.automaton);

        match mb_head_skip {
            Some(head_skip) => head_skip.run_head_skipping(&mut self),
            None => self.run_and_exit(),
        }
    }

    fn run_and_exit(mut self) -> Result<(), EngineError> {
        let iter = self.input.iter_blocks(self.recorder);
        let quote_classifier = classify_quoted_sequences(iter);
        let structural_classifier = classify_structural_characters(quote_classifier);
        let mut classifier = TailSkip::new(structural_classifier);

        self.run_on_subtree(&mut classifier)?;

        self.verify_subtree_closed()
    }

    fn run_on_subtree<Q, S>(&mut self, classifier: &mut Classifier!()) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, BLOCK_SIZE>,
        S: StructuralIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, BLOCK_SIZE>,
    {
        loop {
            if self.next_event.is_none() {
                self.next_event = match classifier.next() {
                    Ok(e) => e,
                    Err(err) => return Err(EngineError::InputError(err)),
                };
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
            NodeTypeHint::Atomic => self.input.seek_non_whitespace_forward(start_idx)?,
        }
        .map(|x| x.0);

        match index {
            Some(idx) => self.recorder.record_match(idx, self.depth, hint.into()),
            None => Err(EngineError::MissingItem()),
        }
    }

    fn handle_colon<Q, S>(
        &mut self,
        #[allow(unused_variables)] classifier: &mut Classifier!(),
        idx: usize,
    ) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, BLOCK_SIZE>,
        S: StructuralIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, BLOCK_SIZE>,
    {
        debug!("Colon");

        let is_next_opening = if let Some((_, c)) = self.input.seek_non_whitespace_forward(idx + 1)? {
            c == b'{' || c == b'['
        } else {
            false
        };

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
                self.next_event = classifier.next()?;
                let is_next_closing = self.next_event.map_or(false, |s| s.is_closing());
                if any_matched && !is_next_closing && self.automaton.is_unitary(self.state) {
                    if let Some(s) = self.next_event {
                        match s {
                            Structural::Closing(_, idx) => {
                                self.recorder.record_value_terminator(idx, self.depth)?;
                            }
                            Structural::Comma(idx) => self.recorder.record_value_terminator(idx, self.depth)?,
                            Structural::Colon(_) | Structural::Opening(_, _) => (),
                        }
                    }
                    let bracket_type = self.current_node_bracket_type();
                    debug!("Skipping unique state from {bracket_type:?}");
                    let stop_at = classifier.skip(bracket_type)?;
                    self.next_event = Some(Structural::Closing(bracket_type, stop_at));
                }
            }
        }

        Ok(())
    }

    fn handle_comma<Q, S>(&mut self, _classifier: &mut Classifier!(), idx: usize) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, BLOCK_SIZE>,
        S: StructuralIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, BLOCK_SIZE>,
    {
        self.recorder.record_value_terminator(idx, self.depth)?;
        let is_next_opening = if let Some((_, c)) = self.input.seek_non_whitespace_forward(idx + 1)? {
            c == b'{' || c == b'['
        } else {
            false
        };

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

        if self.is_list && !is_next_opening && match_index {
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
        Q: QuoteClassifiedIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, BLOCK_SIZE>,
        S: StructuralIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, BLOCK_SIZE>,
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
                let closing_idx = classifier.skip(bracket_type)?;
                return self.recorder.record_value_terminator(closing_idx, self.depth);
            } else {
                self.transition_to(fallback, bracket_type);
            }

            if self.automaton.is_accepting(fallback) {
                self.record_match_detected_at(idx, NodeTypeHint::Complex(bracket_type))?;
            }
        }

        self.depth
            .increment()
            .map_err(|err| EngineError::DepthAboveLimit(idx, err))?;

        self.is_list = bracket_type == BracketType::Square;
        let mut needs_commas = false;

        if self.is_list {
            self.has_any_array_item_transition = self.automaton.has_any_array_item_transition(self.state);
            self.has_any_array_item_transition_to_accepting =
                self.automaton.has_any_array_item_transition_to_accepting(self.state);

            let fallback = self.automaton[self.state].fallback_state();
            let is_fallback_accepting = self.automaton.is_accepting(fallback);

            let searching_list = is_fallback_accepting || self.has_any_array_item_transition;

            if searching_list {
                needs_commas = true;
                self.array_count = NonNegativeArrayIndex::ZERO;
                debug!("Initialized array count to {}", self.array_count);

                let wants_first_item =
                    is_fallback_accepting || self.automaton.has_first_array_index_transition_to_accepting(self.state);

                if wants_first_item {
                    let next = self.input.seek_non_whitespace_forward(idx + 1)?;

                    match next {
                        Some((_, b'[' | b'{' | b']')) => (), // Complex value or empty list.
                        Some((value_idx, _)) => {
                            self.record_match_detected_at(
                                value_idx,
                                NodeTypeHint::Atomic, /* since the next structural is a ','*/
                            )?;
                        }
                        _ => (),
                    }
                }
            }
        }

        if !self.is_list && self.automaton.has_transition_to_accepting(self.state) {
            classifier.turn_colons_and_commas_on(idx);
        } else if needs_commas {
            classifier.turn_colons_off();
            classifier.turn_commas_on(idx);
        } else {
            classifier.turn_colons_and_commas_off();
        }

        Ok(())
    }

    fn handle_closing<Q, S>(&mut self, classifier: &mut Classifier!(), idx: usize) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, BLOCK_SIZE>,
        S: StructuralIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, BLOCK_SIZE>,
    {
        debug!("Closing, decreasing depth and popping stack.");

        self.depth
            .decrement()
            .map_err(|err| EngineError::DepthBelowZero(idx, err))?;
        self.recorder.record_value_terminator(idx, self.depth)?;

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

impl<'i, 'q, 'r, I, R> CanHeadSkip<'i, 'r, I, R, BLOCK_SIZE> for Executor<'i, 'q, 'r, I, R>
where
    I: Input,
    R: Recorder<I::Block<'i, BLOCK_SIZE>>,
    'i: 'r,
{
    fn run_on_subtree<Q, S>(
        &mut self,
        next_event: Structural,
        state: State,
        structural_classifier: S,
    ) -> Result<ResumeClassifierState<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, BLOCK_SIZE>, EngineError>
    where
        Q: QuoteClassifiedIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, BLOCK_SIZE>,
        S: StructuralIterator<'i, I::BlockIterator<'i, 'r, BLOCK_SIZE, R>, Q, BLOCK_SIZE>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeTypeHint {
    Atomic,
    Complex(BracketType),
}

impl From<NodeTypeHint> for MatchedNodeType {
    #[inline(always)]
    fn from(value: NodeTypeHint) -> Self {
        match value {
            NodeTypeHint::Atomic => Self::Atomic,
            NodeTypeHint::Complex(_) => Self::Complex,
        }
    }
}
