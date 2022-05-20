//! Stackless implementation of a JSONPath query engine.
//!
//! Core engine for processing of JSONPath queries, based on the
//! [Stackless Processing of Streamed Trees](https://hal.archives-ouvertes.fr/hal-03021960) paper.
//! Entire query execution is done without recursion or an explicit stack, linearly through
//! the JSON structure, which allows efficient SIMD operations and optimized register usage.
//!
//! This implementation should be more performant than [`stack_based`](super::stack_based)
//! even on targets that don't support AVX2 SIMD operations.

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

        let count = descendant_only_automaton(&self.labels, input);

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
    use crate::bytes::classify::{classify_structural_characters, Structural};
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
            contents: smallvec![],
        }
    }

    #[inline(always)]
    fn peek(&mut self) -> Option<StackFrame> {
        self.contents.last().copied()
    }

    #[inline(always)]
    fn pop(&mut self) -> StackFrame {
        debug_assert!(!self.contents.is_empty(), "SmallStack::pop on empty stack");
        self.contents.pop().unwrap()
    }

    #[inline(always)]
    fn pop_if_reached(&mut self, depth: u8) -> Option<StackFrame> {
        if let Some(stack_frame) = self.peek() {
            if depth <= stack_frame.depth {
                return self.contents.pop();
            }
        }
        None
    }

    #[inline(always)]
    fn push(&mut self, value: StackFrame) {
        self.contents.push(value)
    }
}

fn descendant_only_automaton(labels: &[SeekLabel], bytes: &AlignedBytes<alignment::Page>) -> usize {
    use crate::bytes::classify::{classify_structural_characters, Structural};
    let mut depth: u8 = 0;
    let mut recursive_state: u8 = 0;
    let mut direct_states: SmallVec<[u8; 2]> = smallvec![];
    let last_state = (labels.len() - 1) as u8;
    let mut count: usize = 0;
    let mut stack = SmallStack::new();
    stack.push(StackFrame {
        depth: 0,
        label_idx: 0,
    });
    let mut block_event_source = classify_structural_characters(bytes.relax_alignment()).peekable();
    while let Some(event) = block_event_source.next() {
        /*debug!("====================");
        debug!("Event = {:?}", event);
        debug!("Depth = {:?}", depth);
        debug!("Stack = {:?}", stack);
        debug!("Direct = {:?}", direct_states);
        debug!("Recursive = {:?}", recursive_state);
        debug!("Count = {:?}", count);
        debug!("====================");*/

        match event {
            Structural::Closing(_) => {
                //debug!("Closing, decreasing depth and popping stack.");
                depth -= 1;
                direct_states.clear();
                while let Some(stack_frame) = stack.pop_if_reached(depth) {
                    match labels[stack_frame.label_idx as usize].0 {
                        Seek::Recursive => recursive_state = stack_frame.label_idx,
                        Seek::Direct => direct_states.push(stack_frame.label_idx),
                    }
                }
            }
            Structural::Opening(_) => {
                //debug!("Opening, increasing depth and pushing stack.");
                for direct_states_idx in 0..direct_states.len() {
                    let direct_state = direct_states[direct_states_idx];
                    stack.push(StackFrame {
                        depth,
                        label_idx: direct_state,
                    });
                }

                depth += 1;
                direct_states.clear();
            }
            Structural::Colon(idx) => {
                /*debug!(
                    "Colon, label ending with {:?}",
                    std::str::from_utf8(&bytes[idx - 5..idx]).unwrap()
                );*/

                let event = block_event_source.peek();
                let is_next_opening = matches!(event, Some(Structural::Opening(_)));
                let mut expanded_count = 0;
                let mut flushed_states = false;

                /*if is_next_opening {
                    for direct_states_idx in 0..direct_states.len() {
                        let direct_state = direct_states[direct_states_idx];
                        stack.push(StackFrame {
                            depth,
                            label_idx: direct_state,
                        });
                    }
                }

                for direct_states_idx in 0..direct_states.len() {
                    let direct_state = direct_states[direct_states_idx];
                    if (is_next_opening || direct_state == last_state)
                        && is_match(bytes, idx, labels[direct_state as usize].1)
                    {
                        if direct_state == last_state {
                            debug!("Hit!");
                            count += 1;
                        } else {
                            let next_state = labels[(direct_state + 1) as usize];

                            match next_state.0 {
                                Seek::Recursive => {
                                    recursive_state = direct_state + 1;
                                    direct_states.clear();
                                    flushed_states = true;
                                    break;
                                }
                                Seek::Direct => {
                                    direct_states[expanded_count] = direct_state + 1;
                                    expanded_count += 1;
                                }
                            }
                        }
                    }
                }*/

                if !flushed_states {
                    if is_next_opening {
                        unsafe { direct_states.set_len(expanded_count) };
                    }

                    if (is_next_opening || recursive_state == last_state)
                        && is_match(bytes, idx, labels[recursive_state as usize].1)
                    {
                        if recursive_state == last_state {
                            debug!("Hit!");
                            count += 1;
                        } else {
                            let next_state = labels[(recursive_state + 1) as usize];

                            match next_state.0 {
                                Seek::Recursive => {
                                    stack.push(StackFrame {
                                        depth,
                                        label_idx: recursive_state,
                                    });
                                    recursive_state += 1;
                                    direct_states.clear();
                                }
                                Seek::Direct => {
                                    direct_states.push(recursive_state + 1);
                                }
                            }
                        }
                    }
                } else {
                    stack.push(StackFrame {
                        depth,
                        label_idx: recursive_state,
                    });
                }

                if is_next_opening {
                    block_event_source.next();
                    depth += 1;
                }
            }
        }
    }
    count
}

fn is_match(bytes: &[u8], idx: usize, label: &Label) -> bool {
    let len = label.len();
    if idx < len + 2 {
        return false;
    }

    let mut closing_quote_idx = idx - 1;
    while bytes[closing_quote_idx] != b'"' {
        closing_quote_idx -= 1;
    }
    let opening_quote_idx = closing_quote_idx - len - 1;
    let slice = &bytes[opening_quote_idx..closing_quote_idx + 1];
    slice == label.bytes_with_quotes()
}
