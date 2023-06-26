//! Reference implementation of a JSONPath query engine with recursive descent.
use crate::{
    classification::{
        quotes::{classify_quoted_sequences, QuoteClassifiedIterator},
        structural::{classify_structural_characters, BracketType, Structural, StructuralIterator},
        ResumeClassifierState,
    },
    debug,
    engine::{
        error::EngineError,
        head_skipping::{CanHeadSkip, HeadSkip},
        tail_skipping::TailSkip,
        Compiler, Engine,
    },
    error::InternalRsonpathError,
    input::Input,
    query::{
        automaton::{Automaton, State, TransitionLabel},
        error::{ArrayIndexError, CompilerError},
        JsonPathQuery, JsonString, NonNegativeArrayIndex,
    },
    result::{NodeTypeHint, QueryResult, QueryResultBuilder},
    FallibleIterator, BLOCK_SIZE,
};

/// Recursive implementation of the JSONPath query engine.
pub struct RecursiveEngine<'q> {
    automaton: Automaton<'q>,
}

impl Compiler for RecursiveEngine<'_> {
    type E<'q> = RecursiveEngine<'q>;

    #[must_use = "compiling the query only creates an engine instance that should be used"]
    #[inline(always)]
    fn compile_query(query: &JsonPathQuery) -> Result<RecursiveEngine, CompilerError> {
        let automaton = Automaton::new(query)?;
        debug!("DFA:\n {}", automaton);
        Ok(RecursiveEngine { automaton })
    }

    #[inline(always)]
    fn from_compiled_query(automaton: Automaton<'_>) -> Self::E<'_> {
        RecursiveEngine { automaton }
    }
}

impl Engine for RecursiveEngine<'_> {
    #[inline]
    fn run<I: Input, R: QueryResult>(&self, input: &I) -> Result<R, EngineError> {
        if self.automaton.is_empty_query() {
            return empty_query(input);
        }

        let quote_classifier = classify_quoted_sequences(input);
        let structural_classifier = classify_structural_characters(quote_classifier);
        let mut classifier = TailSkip::new(structural_classifier);

        match classifier.next()? {
            Some(Structural::Opening(b, idx)) => {
                let mut result = R::Builder::new(input);
                let mut execution_ctx = ExecutionContext::new(&self.automaton, input);
                execution_ctx.run(&mut classifier, self.automaton.initial_state(), idx, b, &mut result)?;
                Ok(result.finish())
            }
            _ => Ok(R::default()),
        }
    }
}

fn empty_query<R: QueryResult, I: Input>(input: &I) -> Result<R, EngineError> {
    let quote_classifier = classify_quoted_sequences(input);
    let mut block_event_source = classify_structural_characters(quote_classifier);
    let mut result = R::Builder::new(input);

    if let Some(Structural::Opening(_, idx)) = block_event_source.next()? {
        result.report(idx, crate::result::NodeTypeHint::AnyComplex)?;
    }

    Ok(result.finish())
}

struct ExecutionContext<'q, 'b, I: Input> {
    automaton: &'b Automaton<'q>,
    bytes: &'b I,
}

macro_rules! Classifier {
    () => {
        TailSkip<'b, I, Q, S, BLOCK_SIZE>
    };
}

impl<'q, 'b, I: Input> ExecutionContext<'q, 'b, I> {
    pub(crate) fn new(automaton: &'b Automaton<'q>, bytes: &'b I) -> Self {
        Self { automaton, bytes }
    }

    fn run<'r, Q, S, B, R>(
        &mut self,
        classifier: &mut Classifier!(),
        state: State,
        open_idx: usize,
        bracket_type: BracketType,
        result: &'r mut B,
    ) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
        B: QueryResultBuilder<'b, I, R>,
        R: QueryResult,
    {
        let mb_head_skip = HeadSkip::new(self.bytes, self.automaton);

        match mb_head_skip {
            Some(head_skip) => head_skip.run_head_skipping(self, result),
            None => self
                .run_on_subtree(classifier, state, open_idx, bracket_type, result)
                .map(|_| ()),
        }
    }

    fn run_on_subtree<'r, Q, S, B, R>(
        &mut self,
        classifier: &mut Classifier!(),
        state: State,
        open_idx: usize,
        bracket_type: BracketType,
        result: &'r mut B,
    ) -> Result<usize, EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
        B: QueryResultBuilder<'b, I, R>,
        R: QueryResult,
    {
        debug!("Run state {state}");
        let mut next_event = None;
        let mut latest_idx = open_idx;
        let fallback_state = self.automaton[state].fallback_state();
        let is_fallback_accepting = self.automaton.is_accepting(fallback_state);
        let is_list = bracket_type == BracketType::Square;

        let searching_list = self.automaton.has_any_array_item_transition(state);

        let is_accepting_list_item = is_list && self.automaton.has_any_array_item_transition_to_accepting(state);
        let needs_commas = is_list && (is_fallback_accepting || searching_list);
        let needs_colons = !is_list && self.automaton.has_transition_to_accepting(state);

        let mut array_count = NonNegativeArrayIndex::ZERO;

        let config_characters = |classifier: &mut Classifier!(), idx: usize| {
            if needs_commas {
                classifier.turn_commas_on(idx);
            } else {
                classifier.turn_commas_off();
            }

            if needs_colons {
                classifier.turn_colons_on(idx);
            } else {
                classifier.turn_colons_off();
            }
        };

        config_characters(classifier, open_idx);

        // When a list contains only one item, this block ensures that the list item is reported if appropriate without entering the loop below.
        let wants_first_item = self.automaton[state].transitions().iter().any(|t| match t {
            (TransitionLabel::ArrayIndex(i), s) if i.eq(&NonNegativeArrayIndex::ZERO) => {
                self.automaton.is_accepting(*s)
            }
            _ => false,
        }) || is_fallback_accepting;

        if is_list && wants_first_item {
            next_event = classifier.next()?;
            if let Some(Structural::Closing(_, close_idx)) = next_event {
                if let Some((next_idx, _)) = self.bytes.seek_non_whitespace_forward(open_idx + 1)? {
                    if next_idx < close_idx {
                        result.report(
                            next_idx,
                            NodeTypeHint::Atomic, /* since the next structural is the closing of the list */
                        )?;
                    }
                }
                return Ok(close_idx);
            }

            if matches!(next_event, Some(Structural::Comma(_))) {
                debug!("Accepting first item in the list.");
                result.report(
                    open_idx + 1,
                    NodeTypeHint::Atomic, /* since the next structural is a ','*/
                )?;
            }
        }

        loop {
            if next_event.is_none() {
                next_event = classifier.next()?;
            }
            debug!("Event: {next_event:?}");
            match next_event {
                Some(Structural::Comma(idx)) => {
                    latest_idx = idx;
                    next_event = classifier.next()?;

                    let is_next_opening = next_event.map_or(false, |s| s.is_opening());

                    if !is_next_opening && is_list && is_fallback_accepting {
                        debug!("Accepting on comma.");
                        result.report(idx, NodeTypeHint::Atomic /* since is_next_opening is false */)?;
                    }

                    // Once we are in comma search, we have already considered the option that the first item in the list is a match.  Iterate on the remaining items.

                    if let Err(ArrayIndexError::ExceedsUpperLimitError(_)) = array_count.try_increment() {
                        debug!("Exceeded possible array match in content.");
                        continue;
                    }

                    let match_index = self
                        .automaton
                        .has_array_index_transition_to_accepting(state, &array_count);

                    if is_accepting_list_item && !is_next_opening && match_index {
                        debug!("Accepting on list item.");
                        result.report(idx, NodeTypeHint::Atomic /* since is_next_opening is false */)?;
                    }
                }
                Some(Structural::Colon(idx)) => {
                    debug!("Colon");

                    latest_idx = idx;
                    next_event = classifier.next()?;
                    let is_next_opening = next_event.map_or(false, |s| s.is_opening());

                    if !is_next_opening {
                        let mut any_matched = false;

                        for &(label, target) in self.automaton[state].transitions() {
                            match label {
                                TransitionLabel::ObjectMember(member_name)
                                    if self.automaton.is_accepting(target) && self.is_match(idx, member_name)? =>
                                {
                                    debug!("Accept {idx}");
                                    result
                                        .report(idx, NodeTypeHint::Atomic /* since is_next_opening is false */)?;
                                    any_matched = true;
                                    break;
                                }
                                _ => {}
                            }
                        }
                        let fallback_state = self.automaton[state].fallback_state();
                        if !any_matched && self.automaton.is_accepting(fallback_state) {
                            debug!("Value accepted by fallback.");
                            result.report(idx, NodeTypeHint::Atomic /* since is_next_opening is false */)?;
                        }
                        #[cfg(feature = "unique-members")]
                        {
                            let is_next_closing = matches!(next_event, Some(Structural::Closing(_, _)));
                            if any_matched && !is_next_closing && self.automaton.is_unitary(state) {
                                let bracket_type = if is_list {
                                    BracketType::Square
                                } else {
                                    BracketType::Curly
                                };
                                debug!("Skipping unique state from {:?}", bracket_type);
                                let stop_at = classifier.skip(bracket_type)?;
                                next_event = Some(Structural::Opening(bracket_type, stop_at));
                            }
                        }
                    }
                }
                Some(Structural::Opening(b, idx)) => {
                    let mut matched = None;
                    let colon_idx = self
                        .bytes
                        .seek_non_whitespace_backward(idx - 1)
                        .and_then(|(char_idx, char)| (char == b':').then_some(char_idx));

                    for &(label, target) in self.automaton[state].transitions() {
                        match label {
                            TransitionLabel::ObjectMember(member_name) => {
                                if let Some(colon_idx) = colon_idx {
                                    debug!("Colon backtracked");
                                    if self.is_match(colon_idx, member_name)? {
                                        matched = Some(target);
                                        if self.automaton.is_accepting(target) {
                                            debug!("Accept Object Member {}", member_name.display());
                                            debug!("Accept {idx}");
                                            result.report(colon_idx, NodeTypeHint::Complex(b))?;
                                        }
                                        break;
                                    }
                                }
                            }
                            TransitionLabel::ArrayIndex(i) => {
                                if is_list && i.eq(&array_count) {
                                    matched = Some(target);
                                    if self.automaton.is_accepting(target) {
                                        debug!("Accept Array Index {i}");
                                        debug!("Accept {idx}");
                                        result.report(idx, NodeTypeHint::Complex(b))?;
                                    }
                                    break;
                                }
                            }
                        }
                    }

                    let end_idx = match matched {
                        Some(target) => self.run_on_subtree(classifier, target, idx, b, result)?,
                        None => {
                            let fallback = self.automaton[state].fallback_state();
                            debug!("Falling back to {fallback}");

                            if self.automaton.is_accepting(fallback) {
                                debug!("Accept {idx}");
                                result.report(idx, NodeTypeHint::Complex(b))?;
                            }

                            if self.automaton.is_rejecting(fallback_state) {
                                classifier.skip(b)?
                            } else {
                                self.run_on_subtree(classifier, fallback_state, idx, b, result)?
                            }
                        }
                    };

                    debug!("Return to {state}");
                    next_event = None;
                    latest_idx = end_idx;

                    #[cfg(feature = "unique-members")]
                    {
                        if matched.is_some() && self.automaton.is_unitary(state) {
                            let bracket_type = if is_list {
                                BracketType::Square
                            } else {
                                BracketType::Curly
                            };
                            debug!("Skipping unique state from {:?}", bracket_type);
                            let stop_at = classifier.skip(bracket_type)?;
                            latest_idx = stop_at;
                            break;
                        }
                    }

                    config_characters(classifier, end_idx);
                }
                Some(Structural::Closing(_, idx)) => {
                    latest_idx = idx;
                    break;
                }
                None => break,
            }
        }

        Ok(latest_idx)
    }

    fn is_match(&self, idx: usize, member_name: &JsonString) -> Result<bool, EngineError> {
        let len = member_name.bytes_with_quotes().len();

        let closing_quote_idx = match self.bytes.seek_backward(idx - 1, b'"') {
            Some(x) => x,
            None => return Err(EngineError::MalformedStringQuotes(idx - 1)),
        };

        if closing_quote_idx + 1 < len {
            return Ok(false);
        }

        let start_idx = closing_quote_idx + 1 - len;
        Ok(self.bytes.is_member_match(start_idx, closing_quote_idx, member_name))
    }
}

impl<'q, 'b, I: Input> CanHeadSkip<'b, I, BLOCK_SIZE> for ExecutionContext<'q, 'b, I> {
    fn run_on_subtree<'r, B, R, Q, S>(
        &mut self,
        next_event: Structural,
        state: State,
        structural_classifier: S,
        result: &'r mut B,
    ) -> Result<ResumeClassifierState<'b, I, Q, BLOCK_SIZE>, EngineError>
    where
        Q: QuoteClassifiedIterator<'b, I, BLOCK_SIZE>,
        B: QueryResultBuilder<'b, I, R>,
        R: QueryResult,
        S: StructuralIterator<'b, I, Q, BLOCK_SIZE>,
    {
        let mut classifier = TailSkip::new(structural_classifier);

        let bracket_type = match next_event {
            Structural::Closing(b, _) | Structural::Opening(b, _) => Ok(b),
            _ => Err(InternalRsonpathError::from_expectation("")),
        }?;

        self.run_on_subtree(&mut classifier, state, next_event.idx(), bracket_type, result)?;

        Ok(classifier.stop())
    }
}
