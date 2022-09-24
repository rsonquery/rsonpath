//! Stackless implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion or an explicit stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
//!
//! This implementation should be more performant than [`stack_based`](super::stack_based)
//! even on targets that don't support AVX2 SIMD operations.

use crate::classify::{classify_structural_characters, Structural};
use crate::debug;
use crate::engine::result::QueryResult;
use crate::engine::{Input, Runner};
use crate::query::automaton::{Automaton, State};
use crate::query::{JsonPathQuery, Label};
use crate::quotes::classify_quoted_sequences;
use aligners::{alignment, AlignedBytes};
use smallvec::{smallvec, SmallVec};

/// Stackless runner for a fixed JSONPath query.
///
/// The runner is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct StacklessRunner<'q> {
    automaton: Automaton<'q>,
}

impl StacklessRunner<'_> {
    /// Compile a query into a [`StacklessRunner`].
    ///
    /// Compilation time is proportional to the length of the query.
    pub fn compile_query(query: &JsonPathQuery) -> StacklessRunner<'_> {
        let automaton = Automaton::new(query);

        StacklessRunner { automaton }
    }
}

impl Runner for StacklessRunner<'_> {
    fn run<R: QueryResult>(&self, input: &Input) -> R {
        if self.automaton.is_empty_query() {
            return empty_query(input);
        }

        let mut result = R::default();
        let executor = query_executor(&self.automaton, input, &mut result);
        executor.run();

        result
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
    depth: u8,
    state: State,
    stack: SmallStack,
    automaton: &'b Automaton<'q>,
    bytes: &'b AlignedBytes<alignment::Page>,
    result: &'r mut R,
}

fn query_executor<'q, 'b, 'r, R: QueryResult>(
    automaton: &'b Automaton<'q>,
    bytes: &'b AlignedBytes<alignment::Page>,
    result: &'r mut R,
) -> Executor<'q, 'b, 'r, R> {
    Executor {
        depth: 1,
        state: automaton.initial_state(),
        stack: SmallStack::new(),
        automaton,
        bytes,
        result,
    }
}

impl<'q, 'b, 'r, R: QueryResult> Executor<'q, 'b, 'r, R> {
    fn run(mut self) {
        let quote_classifier = classify_quoted_sequences(self.bytes.relax_alignment());
        let mut block_event_source = classify_structural_characters(quote_classifier);
        let mut fallback_active = false;
        let mut next_event = block_event_source.next();

        while let Some(event) = next_event {
            debug!("====================");
            debug!("Event = {:?}", event);
            debug!("Depth = {:?}", self.depth);
            debug!("Stack = {:?}", self.stack);
            debug!("State = {:?}", self.state);
            debug!("====================");

            next_event = block_event_source.next();
            match event {
                Structural::Comma(_) => (),
                Structural::Closing(_) => {
                    debug!("Closing, decreasing depth and popping stack.");

                    self.depth -= 1;

                    if let Some(stack_frame) = self.stack.pop_if_at_or_below(self.depth) {
                        self.state = stack_frame.state
                    }
                }
                Structural::Opening(_) => {
                    debug!("Opening, increasing depth and pushing stack.");

                    if fallback_active {
                        let fallback = self.automaton[self.state].fallback_state();
                        self.transition_to(fallback);
                    }
                    fallback_active = true;

                    self.depth += 1;
                }
                Structural::Colon(idx) => {
                    debug!(
                        "Colon, label ending with {:?}",
                        std::str::from_utf8(&self.bytes[(if idx < 8 { 0 } else { idx - 8 })..idx])
                            .unwrap_or("[invalid utf8]")
                    );

                    let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));

                    for &(label, target) in self.automaton[self.state].transitions() {
                        if is_next_opening {
                            if self.is_match(idx, label) {
                                fallback_active = false;

                                if self.automaton.is_accepting(target) {
                                    self.result.report(idx);
                                }

                                self.transition_to(target);
                                break;
                            }
                        } else if self.automaton.is_accepting(target) && self.is_match(idx, label) {
                            self.result.report(idx);
                            break;
                        }
                    }
                }
            }
        }
    }

    fn transition_to(&mut self, target: State) {
        if target != self.state {
            self.stack.push(StackFrame {
                depth: self.depth,
                state: self.state,
            });
            self.state = target;
        }
    }

    fn is_match(&self, idx: usize, label: &Label) -> bool {
        let len = label.len() + 2;

        let mut closing_quote_idx = idx - 1;
        while self.bytes[closing_quote_idx] != b'"' {
            closing_quote_idx -= 1;
        }

        if closing_quote_idx + 1 < len {
            return false;
        }

        let start_idx = closing_quote_idx + 1 - len;
        let slice = &self.bytes[start_idx..closing_quote_idx + 1];
        label.bytes_with_quotes() == slice && (start_idx == 0 || self.bytes[start_idx - 1] != b'\\')
    }
}
