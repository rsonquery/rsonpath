//! Main implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion or an explicit stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
//!
//! This implementation should be more performant than [`recursive`](super::recursive::RecursiveEngine)
//! even on targets that do not support AVX2 SIMD operations.

use crate::classify::{
    classify_structural_characters, resume_structural_classification, ClassifierWithSkipping,
    Structural,
};
use crate::debug;
use crate::engine::depth::Depth;
use crate::engine::error::EngineError;
use crate::engine::result::QueryResult;
use crate::engine::{Engine, Input};
use crate::query::automaton::{Automaton, State};
use crate::query::error::CompilerError;
use crate::query::{JsonPathQuery, Label};
use crate::quotes::{classify_quoted_sequences, QuoteClassifiedIterator, ResumeClassifierState};
use aligners::{alignment, AlignedBytes};
use smallvec::{smallvec, SmallVec};

use super::Compiler;

/// Main engine for a fixed JSONPath query.
///
/// The engine is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct MainEngine<'q> {
    automaton: Automaton<'q>,
}

impl Compiler for MainEngine<'_> {
    type E<'q> = MainEngine<'q>;

    /// Compile a query into a [`MainEngine`].
    ///
    /// Compilation time is proportional to the length of the query.
    ///
    /// # Errors
    /// [`CompilerError`] may be raised by the [`Automaton`] when compiling the query.
    #[must_use = "compiling the query only creates an engine instance that should be used"]
    #[inline(always)]
    fn compile_query(query: &JsonPathQuery) -> Result<MainEngine, CompilerError> {
        let automaton = Automaton::new(query)?;
        debug!("DFA:\n {}", automaton);
        Ok(MainEngine { automaton })
    }
}

impl Engine for MainEngine<'_> {
    #[inline]
    fn run<R: QueryResult>(&self, input: &Input) -> Result<R, EngineError> {
        if self.automaton.is_empty_query() {
            return Ok(empty_query(input));
        }

        let mut result = R::default();
        let executor = query_executor(&self.automaton, input, &mut result);
        executor.run()?;

        Ok(result)
    }
}

fn empty_query<R: QueryResult>(bytes: &AlignedBytes<alignment::Page>) -> R {
    let quote_classifier = classify_quoted_sequences(bytes.relax_alignment());
    let mut block_event_source = classify_structural_characters(quote_classifier);
    let mut result = R::default();

    if let Some(Structural::Opening(idx)) = block_event_source.next() {
        result.report(idx);
    }

    result
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StackFrame {
    depth: u8,
    state: State,
    is_list: bool,
}

#[derive(Debug)]
struct SmallStack {
    contents: SmallVec<[StackFrame; 32]>,
}

impl SmallStack {
    fn new() -> Self {
        Self {
            contents: smallvec![],
        }
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

struct Executor<'q, 'b, 'r, R: QueryResult> {
    depth: Depth,
    state: State,
    stack: SmallStack,
    automaton: &'b Automaton<'q>,
    bytes: &'b AlignedBytes<alignment::Page>,
    result: &'r mut R,
    next_event: Option<Structural>,
    is_list: bool,
}

fn query_executor<'q, 'b, 'r, R: QueryResult>(
    automaton: &'b Automaton<'q>,
    bytes: &'b AlignedBytes<alignment::Page>,
    result: &'r mut R,
) -> Executor<'q, 'b, 'r, R> {
    Executor {
        depth: Depth::default(),
        state: automaton.initial_state(),
        stack: SmallStack::new(),
        automaton,
        bytes,
        result,
        next_event: None,
        is_list: false,
    }
}

impl<'q, 'b, 'r, R: QueryResult> Executor<'q, 'b, 'r, R> {
    fn run(mut self) -> Result<(), EngineError> {
        #[cfg(feature = "head-skip")]
        {
            use memchr::memmem;

            let mut quote_classifier = ResumeClassifierState {
                iter: classify_quoted_sequences(self.bytes.relax_alignment()),
                block: None,
            };
            let initial_state = self.automaton.initial_state();

            if self.automaton[initial_state].fallback_state() == initial_state {
                if let Some(&(label, target_state)) =
                    self.automaton[initial_state].transitions().first()
                {
                    debug!("Automaton starts with a descendant search, using memmem heuristic.");
                    let needle = label.bytes_with_quotes();
                    let bytes: &[u8] = self.bytes;
                    let mut idx = 0;
                    let finder = memmem::Finder::new(needle);

                    while let Some(starting_quote_idx) = finder.find(&bytes[idx..]) {
                        idx += starting_quote_idx;
                        debug!("Needle found at {idx}");

                        if idx != 0 && bytes[idx - 1] != b'\\' {
                            let mut colon_idx = idx + needle.len();

                            while colon_idx < bytes.len() && bytes[colon_idx].is_ascii_whitespace()
                            {
                                colon_idx += 1;
                            }

                            if colon_idx < bytes.len() && bytes[colon_idx] == b':' {
                                debug!("Actual match with colon at {colon_idx}");
                                let distance = colon_idx - quote_classifier.get_idx();
                                debug!("Distance skipped: {distance}");
                                quote_classifier.offset_bytes(distance as isize);

                                // Check if the colon is marked as within quotes.
                                // If yes, that is an error of state propagation through skipped blocks.
                                // Flip the quote mask.
                                if let Some(block) = quote_classifier.block.as_mut() {
                                    if (block.block.within_quotes_mask & (1_u64 << block.idx)) != 0
                                    {
                                        debug!("Mask needs flipping!");
                                        block.block.within_quotes_mask =
                                            !block.block.within_quotes_mask;
                                        quote_classifier.iter.flip_quotes_bit();
                                    }
                                }

                                quote_classifier.offset_bytes(1);

                                self.state = target_state;

                                if self.automaton.is_accepting(self.state) {
                                    self.result.report(colon_idx);
                                }
                                quote_classifier = self.run_on_subtree(quote_classifier)?;
                                debug!("Quote classified up to {}", quote_classifier.get_idx());
                                idx = quote_classifier.get_idx();
                            } else {
                                idx += 1;
                            }
                        } else {
                            idx += 1;
                        }
                    }
                }
            } else {
                self.run_on_subtree(quote_classifier)?;
            }
        }
        #[cfg(not(feature = "head-skip"))]
        {
            let quote_classifier = ResumeClassifierState {
                iter: classify_quoted_sequences(self.bytes.relax_alignment()),
                block: None,
            };
            self.run_on_subtree(quote_classifier)?;
        }

        if self.depth != Depth::ZERO {
            Err(EngineError::MissingClosingCharacter())
        } else {
            Ok(())
        }
    }

    fn run_on_subtree<I: QuoteClassifiedIterator<'b>>(
        &mut self,
        quote_classifier: ResumeClassifierState<'b, I>,
    ) -> Result<ResumeClassifierState<'b, I>, EngineError> {
        let mut classifier =
            ClassifierWithSkipping::new(resume_structural_classification(quote_classifier));
        let mut fallback_active = false;
        let mut start = true;

        while let Some(event) = self.next_event.or_else(|| classifier.next()) {
            debug!("====================");
            debug!("Event = {:?}", event);
            debug!("Depth = {:?}", self.depth);
            debug!("Stack = {:?}", self.stack);
            debug!("State = {:?}", self.state);
            debug!("====================");

            if start && !matches!(event, Structural::Opening(_)) {
                self.next_event = Some(event);
                break;
            } else {
                start = false;
            }

            self.next_event = None;
            match event {
                Structural::Comma(idx) => {
                    let fallback = self.automaton[self.state].fallback_state();
                    if self.is_list && self.automaton.is_accepting(fallback) {
                        debug!("Accepting on comma.");
                        self.result.report(idx);
                    }
                }
                Structural::Closing(idx) => {
                    debug!("Closing, decreasing depth and popping stack.");

                    self.depth
                        .decrement()
                        .map_err(|err| EngineError::DepthBelowZero(idx, err))?;

                    if let Some(stack_frame) = self.stack.pop_if_at_or_below(*self.depth) {
                        self.state = stack_frame.state;
                        self.is_list = stack_frame.is_list;
                    }

                    if self.depth == Depth::ZERO {
                        break;
                    }
                }
                Structural::Opening(idx) => {
                    debug!(
                        "Opening {}, increasing depth and pushing stack.",
                        self.bytes[idx]
                    );
                    let fallback = self.automaton[self.state].fallback_state();

                    if fallback_active {
                        debug!("Falling back to {fallback}");

                        if self.automaton.is_rejecting(fallback) {
                            classifier.skip(self.bytes[idx]);
                            continue;
                        } else {
                            self.transition_to(fallback);
                        }
                    } else {
                        fallback_active = true;
                    }

                    let fallback = self.automaton[self.state].fallback_state();
                    if self.bytes[idx] == b'[' {
                        self.is_list = true;

                        if self.automaton.is_accepting(fallback) {
                            self.next_event = classifier.next();
                            if let Some(Structural::Closing(close_idx)) = self.next_event {
                                for next_idx in (idx + 1)..close_idx {
                                    if !self.bytes[next_idx].is_ascii_whitespace() {
                                        debug!("Accepting only item in the list.");
                                        self.result.report(next_idx);
                                        break;
                                    }
                                }
                            } else {
                                debug!("Accepting first item in the list.");
                                self.result.report(idx + 1);
                            }
                        }
                    }
                    self.depth
                        .increment()
                        .map_err(|err| EngineError::DepthAboveLimit(idx, err))?;
                }
                Structural::Colon(idx) => {
                    debug!(
                        "Colon, label ending with {:?}",
                        std::str::from_utf8(&self.bytes[(if idx < 8 { 0 } else { idx - 8 })..idx])
                            .unwrap_or("[invalid utf8]")
                    );

                    self.next_event = classifier.next();
                    let is_next_opening = matches!(self.next_event, Some(Structural::Opening(_)));
                    let mut any_matched = false;

                    for &(label, target) in self.automaton[self.state].transitions() {
                        if is_next_opening {
                            if self.is_match(idx, label)? {
                                fallback_active = false;

                                if self.automaton.is_accepting(target) {
                                    self.result.report(idx);
                                }

                                self.transition_to(target);
                                any_matched = true;
                                break;
                            }
                        } else if self.automaton.is_accepting(target)
                            && self.is_match(idx, label)?
                        {
                            self.result.report(idx);
                            any_matched = true;
                            break;
                        }
                    }

                    let fallback = self.automaton[self.state].fallback_state();
                    if !any_matched && self.automaton.is_accepting(fallback) {
                        debug!("Value accepted by fallback.");
                        self.result.report(idx);
                    }
                }
            }
        }

        Ok(classifier.stop())
    }

    fn transition_to(&mut self, target: State) {
        if target != self.state {
            self.stack.push(StackFrame {
                depth: *self.depth,
                state: self.state,
                is_list: self.is_list,
            });
            self.state = target;
        }
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
