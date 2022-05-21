//! Stackless implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion or an explicit stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
//!
//! This implementation should be more performant than [`stack_based`](super::stack_based)
//! even on targets that don't support AVX2 SIMD operations.

use std::hint::unreachable_unchecked;

use crate::bytes::classify::{classify_structural_characters, Structural, StructuralIterator};
use crate::debug;
use crate::engine::result::CountResult;
use crate::engine::{Input, Runner};
use crate::query::{JsonPathQuery, JsonPathQueryNode, JsonPathQueryNodeType, Label};
use aligners::{alignment, AlignedBytes};
use smallvec::{smallvec, SmallVec};

/// Stackless runner for a fixed JSONPath query.
///
/// The runner is stateless, meaning that it can be executed
/// on any number of separate inputs, even on separate threads.
pub struct StacklessRunner<'a> {
    labels: Vec<SeekLabel<'a>>,
}

const MAX_AUTOMATON_SIZE: usize = 256;

impl StacklessRunner<'_> {
    /// Compile a query into a [`StacklessRunner`].
    ///
    /// Compilation time is proportional to the length of the query.
    pub fn compile_query(query: &JsonPathQuery) -> StacklessRunner<'_> {
        let labels = query_to_labels(query);

        assert!(labels.len() <= MAX_AUTOMATON_SIZE,
            "Max supported length of a query for StacklessRunner is currently {}. The supplied query has length {}.",
            MAX_AUTOMATON_SIZE,
            labels.len());

        StacklessRunner { labels }
    }
}

impl Runner for StacklessRunner<'_> {
    fn count(&self, input: &Input) -> CountResult {
        if self.labels.is_empty() {
            return empty_query(input);
        }

        let count = descendant_only_automaton(&self.labels, input).run();

        CountResult { count }
    }
}

#[derive(Clone, Copy, Debug)]
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
            JsonPathQueryNode::Descendant(label_node) => match label_node.as_ref() {
                JsonPathQueryNode::Label(label, next_node) => {
                    result.push(SeekLabel(Seek::Recursive, label));
                    node_opt = next_node.as_deref();
                }
                _ => panic! {"Unexpected type of node, expected Label."},
            },
            JsonPathQueryNode::Child(label_node) => match label_node.as_ref() {
                JsonPathQueryNode::Label(label, next_node) => {
                    result.push(SeekLabel(Seek::Direct, label));
                    node_opt = next_node.as_deref();
                }
                _ => panic! {"Unexpected type of node, expected Label."},
            },
            _ => panic! {"Unexpected type of node, expected Descendant or Child."},
        }
    }

    result
}

fn empty_query(bytes: &AlignedBytes<alignment::Page>) -> CountResult {
    let mut block_event_source = classify_structural_characters(bytes.relax_alignment());

    match block_event_source.next() {
        Some(Structural::Opening(_)) => CountResult { count: 1 },
        _ => CountResult { count: 0 },
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct StackFrame {
    depth: u8,
    label_idx: u8,
}

#[derive(Debug)]
struct SmallStack {
    contents: SmallVec<[StackFrame; 64]>,
}

impl SmallStack {
    fn new() -> Self {
        Self {
            contents: smallvec![StackFrame {
                depth: 0,
                label_idx: 0,
            }; 2],
        }
    }

    #[inline(always)]
    fn peek(&self) -> StackFrame {
        self.contents[self.contents.len() - 1]
    }

    #[inline(always)]
    fn pop(&mut self) -> StackFrame {
        debug_assert!(!self.contents.is_empty(), "SmallStack::pop on empty stack");
        self.contents.pop().unwrap()
    }

    #[inline(always)]
    fn pop_if_reached(&mut self, depth: u8) -> Option<StackFrame> {
        if depth <= self.peek().depth {
            return self.contents.pop();
        }
        None
    }

    #[inline(always)]
    fn push(&mut self, value: StackFrame) {
        self.contents.push(value)
    }
}

struct Automaton<'q, 'b> {
    depth: u8,
    recursive_state: u8,
    direct_states: SmallVec<[u8; 2]>,
    last_state: u8,
    count: usize,
    stack: SmallStack,
    labels: &'q [SeekLabel<'q>],
    bytes: &'b AlignedBytes<alignment::Page>,
}

fn descendant_only_automaton<'q, 'b>(
    labels: &'q [SeekLabel<'q>],
    bytes: &'b AlignedBytes<alignment::Page>,
) -> Automaton<'q, 'b> {
    Automaton {
        depth: 1,
        recursive_state: 0,
        direct_states: smallvec![],
        last_state: (labels.len() - 1) as u8,
        count: 0,
        stack: SmallStack::new(),
        labels,
        bytes,
    }
}

impl<'q, 'b> Automaton<'q, 'b> {
    fn run(mut self) -> usize {
        let mut block_event_source =
            classify_structural_characters(self.bytes.relax_alignment()).peekable();

        while let Some(event) = block_event_source.next() {
            match event {
                Structural::Closing(_) => {
                    self.depth -= 1;
                    self.direct_states.clear();
                    while let Some(stack_frame) = self.stack.pop_if_reached(self.depth) {
                        match self.labels[stack_frame.label_idx as usize].0 {
                            Seek::Recursive => self.recursive_state = stack_frame.label_idx,
                            Seek::Direct => self.direct_states.push(stack_frame.label_idx),
                        }
                    }
                }
                Structural::Opening(_) => {
                    self.depth += 1;
                    self.direct_states.clear();
                }
                Structural::Colon(idx) => {
                    let event = block_event_source.peek();
                    let label = self.labels[self.recursive_state as usize].1;

                    if (matches!(event, Some(Structural::Opening(_)))
                        || self.recursive_state == self.last_state)
                        && self.is_match(idx, label)
                    {
                        if self.recursive_state == self.last_state {
                            self.count += 1;
                        } else {
                            self.stack.push(StackFrame {
                                depth: self.depth,
                                label_idx: self.recursive_state,
                            });
                            self.recursive_state += 1;
                        }
                    }
                }
            }
        }
        self.count
    }

    fn is_match(&self, idx: usize, label: &Label) -> bool {
        let len = label.len();
        if idx < len + 2 {
            return false;
        }

        let mut closing_quote_idx = idx - 1;
        while self.bytes[closing_quote_idx] != b'"' {
            closing_quote_idx -= 1;
        }
        let opening_quote_idx = closing_quote_idx - len - 1;
        let slice = &self.bytes[opening_quote_idx..closing_quote_idx + 1];
        slice == label.bytes_with_quotes()
    }
}
