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

    /// Compile a query into a [`RecursiveEngine`].
    ///
    /// # Errors
    /// [`CompilerError`] may be raised by the [`Automaton`] when compiling the query.
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
        let (fallback_state, is_fallback_accepting) = self.automaton[state].fallback_state();
        let is_list = self.bytes[open_idx] == b'[';

        if is_list && is_fallback_accepting {
            self.classifier.turn_commas_on(open_idx);
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
                Some(Structural::Opening(idx)) => {
                    debug!("Opening, falling back");
                    increase_depth!(self);

                    #[cfg(feature = "tail-skip")]
                    if self.automaton.is_rejecting(fallback_state) {
                        latest_idx = self.classifier.skip(self.bytes[idx]);
                    } else {
                        latest_idx = self.run(fallback_state, idx)?;
                    }
                    #[cfg(not(feature = "tail-skip"))]
                    {
                        latest_idx = self.run(fallback_state, idx)?;
                    }

                    if is_list && is_fallback_accepting {
                        self.classifier.turn_commas_on(latest_idx);
                    }

                    next_event = None;
                }
                Some(Structural::Closing(idx)) => {
                    debug!("Closing, popping stack");
                    latest_idx = idx;
                    decrease_depth!(self);
                    self.classifier.turn_commas_off();
                    break;
                }
                Some(Structural::Comma(idx)) => {
                    latest_idx = idx;
                    if is_list && is_fallback_accepting {
                        debug!("Accepting on comma.");
                        self.result.report(idx);
                    }
                    next_event = None;
                }
                Some(Structural::Colon(idx)) => {
                    latest_idx = idx;
                    next_event = self.classifier.next();
                    let next_opening = match next_event {
                        Some(Structural::Opening(idx)) => Some(idx),
                        _ => None,
                    };
                    let mut any_matched = false;
                    for &(label, target, is_accepting) in self.automaton[state].transitions() {
                        if let Some(next_idx) = next_opening {
                            if self.is_match(idx, label)? {
                                debug!("Matched transition to {target}");
                                if is_accepting {
                                    self.result.report(idx);
                                }
                                increase_depth!(self);
                                latest_idx = self.run(target, next_idx)?;
                                next_event = None;
                                any_matched = true;
                                break;
                            }
                        } else if is_accepting && self.is_match(idx, label)? {
                            debug!("Matched transition to acceptance in {target}");
                            self.result.report(idx);
                            any_matched = true;
                            break;
                        }
                    }

                    if !any_matched && is_fallback_accepting {
                        debug!("Value accepted by fallback.");
                        self.result.report(idx);
                    }
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
