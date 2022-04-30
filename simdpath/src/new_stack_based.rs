//! WIP version of [`stack_based`](`crate::stack_based`).
use std::iter::Peekable;

use crate::bytes::classify::{classify_structural_characters, Structural, StructuralIterator};
use crate::engine::{result, Input, Runner};
use crate::query::{JsonPathQuery, JsonPathQueryNode, Label};
use align::{alignment, AlignedSlice};
use log::*;

/// New version of [`StackBasedRunner`](`crate::stack_based::StackBasedRunner`).
pub struct NewStackBasedRunner<'a> {
    query: &'a JsonPathQuery,
}

impl<'a> NewStackBasedRunner<'a> {
    /// Compile a query into a [`NewStackBasedRunner`].
    pub fn compile_query(query: &'a JsonPathQuery) -> Self {
        NewStackBasedRunner { query }
    }
}

impl<'a> Runner for NewStackBasedRunner<'a> {
    fn count(&self, input: &Input) -> result::CountResult {
        let aligned_bytes: &AlignedSlice<alignment::Page> = input;
        let classifier = classify_structural_characters(aligned_bytes.relax_alignment());
        let execution_ctx = ExecutionContext::new(self.query, classifier, input);
        result::CountResult {
            count: execution_ctx.run(),
        }
    }
}

struct ExecutionContext<'q, 'b, I>
where
    I: StructuralIterator<'b>,
{
    query: &'q JsonPathQuery,
    classifier: Peekable<I>,
    count: usize,
    bytes: &'b [u8],
    recursive_state: State<'q>,
}

impl<'q, 'b, I> ExecutionContext<'q, 'b, I>
where
    I: StructuralIterator<'b>,
{
    pub fn new(query: &'q JsonPathQuery, classifier: I, bytes: &'b [u8]) -> Self {
        Self {
            query,
            classifier: classifier.peekable(),
            count: 0,
            bytes,
            recursive_state: State(Mode::Skip, None),
        }
    }

    pub fn run(mut self) -> usize {
        self.transition_based_on_node(Some(self.query.root()));
        self.count
    }

    fn run_state(&mut self, state: State<'q>) {
        let mode = state.0;
        let next_node = state.1;
        debug!(
            "Running state: ({:?}, {:?})",
            mode,
            next_node.map(|x| x.debug_description())
        );
        match mode {
            Mode::Initial => self.transition_based_on_node(next_node),
            Mode::MatchLabel(label) => {
                if let Some(&Structural::Colon(idx)) = self.classifier.peek() {
                    self.classifier.next();
                    let len = label.len();
                    if idx >= len + 2 {
                        let mut closing_quote_idx = idx - 1;
                        while self.bytes[closing_quote_idx] != b'"' {
                            closing_quote_idx -= 1;
                        }

                        let opening_quote_idx = closing_quote_idx - len - 1;
                        let slice = &self.bytes[opening_quote_idx..closing_quote_idx + 1];

                        if slice == label.bytes_with_quotes() {
                            debug!(
                                "Label matched: {}",
                                std::str::from_utf8(label.bytes_with_quotes()).unwrap()
                            );
                            self.transition_based_on_node(next_node)
                        }
                    }
                    debug!("Label not matched.");
                } else {
                    debug!("Not a label.")
                }
            }
            Mode::RecursiveDescent => loop {
                debug!("Setting recursive checkpoint.");
                self.recursive_state = State(Mode::RecursiveDescent, next_node);
                self.transition_based_on_node(next_node);
                match self.classifier.peek() {
                    Some(Structural::Opening(_)) => {
                        self.classifier.next();
                        self.run_state(State(Mode::RecursiveDescent, next_node))
                    }
                    Some(Structural::Closing(_)) => {
                        self.classifier.next();
                        break;
                    }
                    _ => (),
                }
            },
            Mode::DirectDescendant => loop {
                self.transition_based_on_node(next_node);
                match self.classifier.peek() {
                    Some(Structural::Opening(_)) => {
                        self.classifier.next();
                        debug!("Invoking recursive checkpoint.");
                        self.run_state(self.recursive_state);
                        debug!(
                            "Returned from recursive checkpoint back to ({:?}, {:?})",
                            mode,
                            next_node.map(|x| x.debug_description())
                        );
                    }
                    Some(Structural::Closing(_)) => {
                        self.classifier.next();
                        break;
                    }
                    _ => (),
                }
            },
            Mode::Skip => loop {
                debug!("Skipping object...");
                match self.classifier.next() {
                    Some(Structural::Opening(_)) => self.run_state(State(Mode::Skip, next_node)),
                    Some(Structural::Closing(_)) => break,
                    _ => (),
                }
            },
        }
    }

    fn transition_based_on_node(&mut self, node: Option<&'q JsonPathQueryNode>) {
        debug!(
            "Transitioning based on {:?}",
            node.map(|x| x.debug_description())
        );
        match node {
            None => {
                debug!("Hit!");
                self.count += 1;
            }
            Some(JsonPathQueryNode::Root(child)) => {
                if let Some(Structural::Opening(_)) = self.classifier.peek() {
                    self.run_state(State(Mode::Initial, child.as_deref()));
                }
            }
            Some(JsonPathQueryNode::Label(label, child)) => {
                self.run_state(State(Mode::MatchLabel(label), child.as_deref()))
            }
            Some(JsonPathQueryNode::Descendant(child)) => {
                if let Some(Structural::Opening(_)) = self.classifier.peek() {
                    self.classifier.next();
                    self.run_state(State(Mode::RecursiveDescent, Some(child)));
                }
            }
            Some(JsonPathQueryNode::Child(child)) => {
                if let Some(Structural::Opening(_)) = self.classifier.peek() {
                    self.classifier.next();
                    self.run_state(State(Mode::DirectDescendant, Some(child)));
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode<'a> {
    Initial,
    MatchLabel(&'a Label),
    RecursiveDescent,
    DirectDescendant,
    Skip,
}

#[derive(Debug, Clone, Copy)]
struct State<'a>(Mode<'a>, Option<&'a JsonPathQueryNode>);
