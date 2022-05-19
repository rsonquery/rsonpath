//! Stackless implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion or an explicit stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
//!
//! This implementation should be more performant than [`stack_based`](super::stack_based)
//! even on targets that don't support AVX2 SIMD operations.

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

        let count = descendant_only_automaton(&self.labels, input);

        CountResult { count }
    }
}

enum Seek {
    Direct,
    Recursive,
}

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
    use crate::bytes::classify::{classify_structural_characters, Structural};
    let mut block_event_source = classify_structural_characters(bytes.relax_alignment());

    match block_event_source.next() {
        Some(Structural::Opening(_)) => CountResult { count: 1 },
        _ => CountResult { count: 0 },
    }
}

struct SmallStack {
    contents: SmallVec<[u8; 64]>,
}

impl SmallStack {
    fn new() -> Self {
        Self {
            contents: smallvec![0; 64],
        }
    }

    fn peek(&mut self) -> u8 {
        debug_assert!(!self.contents.is_empty(), "SmallStack::peek on empty stack");
        *self.contents.last().unwrap()
    }

    fn pop(&mut self) -> u8 {
        debug_assert!(!self.contents.is_empty(), "SmallStack::pop on empty stack");
        self.contents.pop().unwrap()
    }

    fn push(&mut self, value: u8) {
        self.contents.push(value)
    }
}

fn descendant_only_automaton(labels: &[SeekLabel], bytes: &AlignedBytes<alignment::Page>) -> usize {
    use crate::bytes::classify::{classify_structural_characters, Structural};
    let mut depth: u8 = 0;
    let mut state: u8 = 1;
    let last_state = labels.len() as u8;
    let mut count: usize = 0;
    let mut stack = SmallStack::new();
    stack.push(0);
    let mut block_event_source = classify_structural_characters(bytes.relax_alignment()).peekable();
    while let Some(event) = block_event_source.next() {
        match event {
            Structural::Closing(_) => {
                let state_depth = stack.peek();
                depth -= 1;
                if depth <= state_depth {
                    stack.pop();
                    state -= 1;
                }
            }
            Structural::Opening(_) => {
                depth += 1;
            }
            Structural::Colon(idx) => {
                let event = block_event_source.peek();

                if matches!(event, Some(Structural::Opening(_))) || state == last_state {
                    let len = labels[(state - 1) as usize].1.len();
                    if idx >= len + 2 {
                        let mut closing_quote_idx = idx - 1;
                        while bytes[closing_quote_idx] != b'"' {
                            closing_quote_idx -= 1;
                        }
                        let opening_quote_idx = closing_quote_idx - len - 1;
                        let slice = &bytes[opening_quote_idx..closing_quote_idx + 1];
                        if slice == labels[(state - 1) as usize].1.bytes_with_quotes() {
                            if state == last_state {
                                count += 1;
                            } else {
                                state += 1;
                                stack.push(depth);
                            }
                        }
                    }
                }
            }
        }
    }
    count
}
