//! Reference implementation of a JSONPath query engine with recursive descent.

use crate::classify::ClassifierWithSkipping;
use crate::classify::{classify_structural_characters, Structural, StructuralIterator};
use crate::debug;
use crate::engine::error::EngineError;
use crate::engine::result::QueryResult;
use crate::engine::{Compiler, Engine, Input};
use crate::query::automaton::{Automaton, State};
use crate::query::error::CompilerError;
use crate::query::{JsonPathQuery, Label};
use crate::quotes::{classify_quoted_sequences, QuoteClassifiedIterator};
use aligners::{alignment, AlignedBytes, AlignedSlice};
use std::marker::PhantomData;

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
}

impl Engine for RecursiveEngine<'_> {
    #[inline]
    fn run<R: QueryResult>(&self, input: &Input) -> Result<R, EngineError> {
        if self.automaton.is_empty_query() {
            return empty_query(input);
        }

        let aligned_bytes: &AlignedSlice<alignment::Page> = input;
        let quote_classifier = classify_quoted_sequences(aligned_bytes.relax_alignment());
        let mut classifier = classify_structural_characters(quote_classifier);

        match classifier.next() {
            Some(Structural::Opening(idx)) => {
                let mut result = R::default();
                let mut execution_ctx =
                    ExecutionContext::new(classifier, &self.automaton, input, &mut result);
                execution_ctx.run(self.automaton.initial_state(), idx)?;
                Ok(result)
            }
            _ => Ok(R::default()),
        }
    }
}

fn empty_query<R: QueryResult>(bytes: &AlignedBytes<alignment::Page>) -> Result<R, EngineError> {
    let quote_classifier = classify_quoted_sequences(bytes.relax_alignment());
    let mut block_event_source = classify_structural_characters(quote_classifier);
    let mut result = R::default();

    if let Some(Structural::Opening(idx)) = block_event_source.next() {
        result.report(idx);
    }

    Ok(result)
}

macro_rules! decrease_depth {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        {
            $x.depth -= 1;
        }
    };
}

macro_rules! increase_depth {
    ($x:expr) => {
        #[cfg(debug_assertions)]
        {
            $x.depth += 1;
        }
    };
}

struct ExecutionContext<'q, 'b, 'r, Q, I, R>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
    R: QueryResult,
{
    classifier: ClassifierWithSkipping<'b, Q, I>,
    automaton: &'b Automaton<'q>,
    bytes: &'b [u8],
    #[cfg(debug_assertions)]
    depth: usize,
    result: &'r mut R,
    phantom: PhantomData<Q>,
}

impl<'q, 'b, 'r, Q, I, R> ExecutionContext<'q, 'b, 'r, Q, I, R>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
    R: QueryResult,
{
    #[cfg(debug_assertions)]
    pub(crate) fn new(
        classifier: I,
        automaton: &'b Automaton<'q>,
        bytes: &'b [u8],
        result: &'r mut R,
    ) -> Self {
        Self {
            classifier: ClassifierWithSkipping::new(classifier),
            automaton,
            bytes,
            depth: 1,
            result,
            phantom: PhantomData,
        }
    }

    #[cfg(not(debug_assertions))]
    pub(crate) fn new(
        classifier: I,
        automaton: &'b Automaton<'q>,
        bytes: &'b [u8],
        result: &'r mut R,
    ) -> Self {
        Self {
            classifier: ClassifierWithSkipping::new(classifier),
            automaton,
            bytes,
            result,
            phantom: PhantomData,
        }
    }

    pub(crate) fn run(&mut self, state: State, open_idx: usize) -> Result<usize, EngineError> {
        debug!("Run state {state}, depth {}", self.depth);
        let mut next_event = None;
        let mut latest_idx = open_idx;
        let fallback_state = self.automaton[state].fallback_state();
        let is_fallback_accepting = self.automaton.is_accepting(fallback_state);
        let is_list = self.bytes[open_idx] == b'[';
        let needs_commas = is_list && is_fallback_accepting;
        let needs_colons = !is_list && self.automaton.has_transition_to_accepting(state);

        if needs_commas {
            self.classifier.turn_commas_on(open_idx);
        } else {
            self.classifier.turn_commas_off();
        }

        if needs_colons {
            self.classifier.turn_colons_on(open_idx);
        } else {
            self.classifier.turn_colons_off();
        }

        if needs_commas {
            next_event = self.classifier.next();
            if let Some(Structural::Closing(close_idx)) = next_event {
                for idx in (open_idx + 1)..close_idx {
                    if !self.bytes[idx].is_ascii_whitespace() {
                        debug!("Accepting only item in the list.");
                        self.result.report(idx);
                        break;
                    }
                }
                return Ok(close_idx);
            }

            debug!("Accepting first item in the list.");
            self.result.report(open_idx + 1);
        }

        loop {
            if next_event.is_none() {
                next_event = self.classifier.next();
            }
            debug!("Event: {next_event:?}");
            match next_event {
                Some(Structural::Comma(idx)) => {
                    latest_idx = idx;
                    next_event = self.classifier.next();
                    let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));

                    if !is_next_opening && is_list && is_fallback_accepting {
                        debug!("Accepting on comma.");
                        self.result.report(idx);
                    }
                }
                Some(Structural::Colon(idx)) => {
                    debug!(
                        "Colon, label ending with {:?}",
                        std::str::from_utf8(&self.bytes[(if idx < 8 { 0 } else { idx - 8 })..idx])
                            .unwrap_or("[invalid utf8]")
                    );

                    latest_idx = idx;
                    next_event = self.classifier.next();
                    let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));

                    if !is_next_opening {
                        let mut any_matched = false;

                        for &(label, target) in self.automaton[state].transitions() {
                            if self.automaton.is_accepting(target) && self.is_match(idx, label)? {
                                debug!("Accept {idx}");
                                self.result.report(idx);
                                any_matched = true;
                                break;
                            }
                        }
                        let fallback_state = self.automaton[state].fallback_state();
                        if !any_matched && self.automaton.is_accepting(fallback_state) {
                            debug!("Value accepted by fallback.");
                            self.result.report(idx);
                        }
                        #[cfg(feature = "unique-labels")]
                        {
                            let is_next_closing =
                                matches!(next_event, Some(Structural::Closing(_)));
                            if any_matched && !is_next_closing && self.automaton.is_unitary(state) {
                                let opening = if is_list { b'[' } else { b'{' };
                                debug!("Skipping unique state from {}", opening as char);
                                let stop_at = self.classifier.skip(opening);
                                next_event = Some(Structural::Closing(stop_at));
                            }
                        }
                    }
                }
                Some(Structural::Opening(idx)) => {
                    increase_depth!(self);

                    let mut matched = None;
                    let colon_idx = {
                        let mut colon_idx = idx - 1;
                        while colon_idx > 0 && self.bytes[colon_idx].is_ascii_whitespace() {
                            colon_idx -= 1;
                        }
                        (self.bytes[colon_idx] == b':').then_some(colon_idx)
                    };

                    if let Some(colon_idx) = colon_idx {
                        debug!(
                            "Colon backtracked, label ending with {:?}",
                            std::str::from_utf8(
                                &self.bytes
                                    [(if colon_idx < 8 { 0 } else { colon_idx - 8 })..colon_idx]
                            )
                            .unwrap_or("[invalid utf8]")
                        );
                        for &(label, target) in self.automaton[state].transitions() {
                            if self.is_match(colon_idx, label)? {
                                matched = Some(target);
                                if self.automaton.is_accepting(target) {
                                    debug!("Accept {idx}");
                                    self.result.report(colon_idx);
                                }
                                break;
                            }
                        }
                    }

                    let end_idx = match matched {
                        Some(target) => self.run(target, idx)?,
                        None => {
                            let fallback = self.automaton[state].fallback_state();
                            debug!("Falling back to {fallback}");

                            if self.automaton.is_accepting(fallback) {
                                debug!("Accept {idx}");
                                self.result.report(idx);
                            }

                            #[cfg(feature = "tail-skip")]
                            if self.automaton.is_rejecting(fallback_state) {
                                self.classifier.skip(self.bytes[idx])
                            } else {
                                self.run(fallback_state, idx)?
                            }
                            #[cfg(not(feature = "tail-skip"))]
                            {
                                self.run(fallback_state, idx)?
                            }
                        }
                    };

                    latest_idx = end_idx;

                    if needs_commas {
                        self.classifier.turn_commas_on(end_idx);
                    } else {
                        self.classifier.turn_commas_off();
                    }

                    if needs_colons {
                        self.classifier.turn_colons_on(end_idx);
                    } else {
                        self.classifier.turn_colons_off();
                    }

                    next_event = None;
                }
                Some(Structural::Closing(idx)) => {
                    latest_idx = idx;
                    decrease_depth!(self);
                    break;
                }
                None => break,
            }
        }

        Ok(latest_idx)
    }

    fn is_match(&self, idx: usize, label: &Label) -> Result<bool, EngineError> {
        let len = label.len() + 2;

        let mut closing_quote_idx = idx - 1;
        while self.bytes[closing_quote_idx] != b'"' {
            if closing_quote_idx == 0 {
                return Err(EngineError::MalformedLabelQuotes(idx));
            }

            closing_quote_idx -= 1;
        }

        if closing_quote_idx + 1 < len {
            return Ok(false);
        }

        let start_idx = closing_quote_idx + 1 - len;
        let slice = &self.bytes[start_idx..closing_quote_idx + 1];

        Ok(label.bytes_with_quotes() == slice
            && (start_idx == 0 || self.bytes[start_idx - 1] != b'\\'))
    }
}
