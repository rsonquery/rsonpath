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
use crate::query::automaton::{Automaton, TransitionTable};
use crate::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};
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
        if self.automaton.states().len() == 1 {
            return empty_query(input);
        }

        let mut result = R::default();
        query_automaton(self.automaton.states(), input, &mut result).run();

        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seek {
    Direct,
    Recursive,
}

#[derive(Clone, Copy, Debug)]
struct SeekLabel<'a>(Seek, &'a Label);

fn query_to_labels(query: &JsonPathQuery) -> Vec<SeekLabel> {
    debug_assert!(query.root().is_root());
    let mut node_opt = query.root().child();
    let mut result = vec![];

    while let Some(node) = node_opt {
        match node {
            JsonPathQueryNode::Descendant(label, next_node) => {
                result.push(SeekLabel(Seek::Recursive, label));
                node_opt = next_node.as_deref();
            }
            JsonPathQueryNode::Child(label, next_node) => {
                result.push(SeekLabel(Seek::Direct, label));
                node_opt = next_node.as_deref();
            }
            _ => panic! {"Unexpected type of node, expected Descendant or Child."},
        }
    }

    result
}

fn empty_query<R: QueryResult>(bytes: &AlignedBytes<alignment::Page>) -> R {
    let mut block_event_source = classify_structural_characters(bytes.relax_alignment());
    let mut result = R::default();

    if let Some(Structural::Opening(idx)) = block_event_source.next() {
        result.report(idx);
    }

    result
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StackFrame {
    depth: u8,
    state: u8,
}

#[derive(Debug)]
struct SmallStack {
    contents: SmallVec<[StackFrame; 64]>,
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
    state: Option<u8>,
    stack: SmallStack,
    states: &'q Vec<TransitionTable<'q>>,
    bytes: &'b AlignedBytes<alignment::Page>,
    result: &'r mut R,
}

fn query_automaton<'q, 'b, 'r, R: QueryResult>(
    states: &'q Vec<TransitionTable<'q>>,
    bytes: &'b AlignedBytes<alignment::Page>,
    result: &'r mut R,
) -> Executor<'q, 'b, 'r, R> {
    Executor {
        depth: 1,
        state: Some(0),
        stack: SmallStack::new(),
        states,
        bytes,
        result,
    }
}

impl<'q, 'b, 'r, R: QueryResult> Executor<'q, 'b, 'r, R> {
    fn run(mut self) {
        let mut block_event_source =
            classify_structural_characters(self.bytes.relax_alignment()).peekable();
        let mut fallback_active = false;
        let mut skip_first = true;
        let last_state = (self.states.len() - 1) as u8;

        while let Some(event) = block_event_source.next() {
            if skip_first {
                skip_first = false;
                continue;
            }
            debug!("====================");
            debug!("Event = {:?}", event);
            debug!("Depth = {:?}", self.depth);
            debug!("Stack = {:?}", self.stack);
            debug!("State = {:?}", self.state);
            debug!("====================");

            let next_event = block_event_source.peek();
            match event {
                Structural::Closing(_) => {
                    debug!("Closing, decreasing depth and popping stack.");

                    self.depth -= 1;

                    if let Some(stack_frame) = self.stack.pop_if_at_or_below(self.depth) {
                        self.state = Some(stack_frame.state)
                    }
                }
                Structural::Opening(_) => {
                    debug!("Opening, increasing depth and pushing stack.");

                    if let Some(state) = self.state {
                        if fallback_active {
                            let fallback = self.states[state as usize].fallback_state();
                            if fallback != Some(state) {
                                self.stack.push(StackFrame {
                                    depth: self.depth,
                                    state,
                                });
                                self.state = fallback;
                            }
                        } else {
                            fallback_active = true;
                        }
                    }
                    self.depth += 1;
                }
                Structural::Colon(idx) => {
                    if let Some(state) = self.state {
                        debug!(
                            "Colon, label ending with {:?}",
                            std::str::from_utf8(
                                &self.bytes[(if idx < 8 { 0 } else { idx - 8 })..idx]
                            )
                            .unwrap()
                        );

                        let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));

                        for &(label, target) in self.states[state as usize].transitions() {
                            if is_next_opening {
                                if self.is_match(idx, label) {
                                    fallback_active = false;

                                    if target == last_state {
                                        self.result.report(idx);
                                    }

                                    if target != state {
                                        self.stack.push(StackFrame {
                                            depth: self.depth,
                                            state,
                                        });
                                        self.state = Some(target);
                                    }
                                }
                            } else if target == last_state && self.is_match(idx, label) {
                                self.result.report(idx);
                            }
                        }
                    }
                }
            }
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
