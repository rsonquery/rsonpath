//! Reference implementation of a JSONPath query engine with recursive descent.

use crate::classify::ClassifierWithSkipping;
use crate::classify::{classify_structural_characters, Structural, StructuralIterator};
use crate::debug;
use crate::engine::error::EngineError;
use crate::engine::result::QueryResult;
use crate::engine::{Input, Runner};
use crate::query::automaton::{Automaton, State};
use crate::query::error::CompilerError;
use crate::query::{JsonPathQuery, Label};
use crate::quotes::{classify_quoted_sequences, QuoteClassifiedIterator};
use aligners::{alignment, AlignedBytes, AlignedSlice};
use std::marker::PhantomData;

/// Recursive implementation of the JSONPath query engine.
pub struct StackBasedRunner<'q> {
    automaton: Automaton<'q>,
}

impl<'q> StackBasedRunner<'q> {
    /// Compile a query into a [`StackBasedRunner`].
    /// 
    /// # Errors
    /// [`CompilerError`] may be raised by the [`Automaton`] when compiling the query.
    #[must_use = "compiling the query only creates an engine instance that should be used"]
    #[inline(always)]
    pub fn compile_query(query: &'q JsonPathQuery) -> Result<Self, CompilerError> {
        let automaton = Automaton::new(query)?;
        debug!("DFA:\n {}", automaton);
        Ok(StackBasedRunner { automaton })
    }
}

impl Runner for StackBasedRunner<'_> {
    #[inline]
    fn run<R: QueryResult>(&self, input: &Input) -> Result<R, EngineError> {
        if self.automaton.is_empty_query() {
            return empty_query(input);
        }

        let aligned_bytes: &AlignedSlice<alignment::Page> = input;
        let quote_classifier = classify_quoted_sequences(aligned_bytes.relax_alignment());
        let mut classifier = classify_structural_characters(quote_classifier);
        classifier.next();
        let mut result = R::default();
        let mut execution_ctx =
            ExecutionContext::new(classifier, &self.automaton, input, &mut result);
        execution_ctx.run(self.automaton.initial_state())?;
        Ok(result)
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

    pub(crate) fn run(&mut self, state: State) -> Result<(), EngineError> {
        debug!("Run state {state}, depth {}", self.depth);
        let mut next_event = None;
        loop {
            if next_event.is_none() {
                next_event = self.classifier.next();
            }
            debug!("Event: {next_event:?}");
            match next_event {
                Some(Structural::Opening(idx)) => {
                    debug!("Opening, falling back");
                    increase_depth!(self);
                    let next_state = self.automaton[state].fallback_state();

                    if self.automaton.is_rejecting(next_state) {
                        self.classifier.skip(self.bytes[idx]);
                    } else {
                        self.run(next_state)?;
                    }
                    next_event = None;
                }
                Some(Structural::Closing(_)) => {
                    debug!("Closing, popping stack");
                    decrease_depth!(self);
                    break;
                }
                Some(Structural::Colon(idx)) => {
                    next_event = self.classifier.next();
                    let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));
                    for &(label, target) in self.automaton[state].transitions() {
                        if is_next_opening {
                            if self.is_match(idx, label)? {
                                debug!("Matched transition to {target}");
                                if self.automaton.is_accepting(target) {
                                    self.result.report(idx);
                                }
                                increase_depth!(self);
                                self.run(target)?;
                                next_event = None;
                                break;
                            }
                        } else if self.automaton.is_accepting(target)
                            && self.is_match(idx, label)?
                        {
                            debug!("Matched transition to acceptance in {target}");
                            self.result.report(idx);
                            break;
                        }
                    }
                }
                #[cfg(feature = "commas")]
                Some(Structural::Comma(_)) => next_event = None,
                None => break,
            }
        }

        Ok(())
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
