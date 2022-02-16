//! WIP version of [`stack_based`](`crate::stack_based`).
use std::iter::Peekable;

use crate::bytes::align::{alignment, AlignedSlice};
use crate::bytes::{classify_structural_characters, Structural, StructuralIterator};
use crate::engine::{result, Input, Runner};
use crate::query::{JsonPathQuery, JsonPathQueryNode, Label};

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
        }
    }

    pub fn run(mut self) -> usize {
        self.transition_based_on_node(Some(self.query.root()));
        self.count
    }

    fn run_state(&mut self, state: State, next_node: Option<&JsonPathQueryNode>) {
        match state {
            State::Initial => self.transition_based_on_node(next_node),
            State::MatchLabel(label) => {
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
                            self.transition_based_on_node(next_node)
                        }
                    }
                }
            }
            State::RecursiveDescent => loop {
                self.transition_based_on_node(next_node);
                match self.classifier.peek() {
                    Some(Structural::Opening(_)) => {
                        self.classifier.next();
                        self.run_state(State::RecursiveDescent, next_node)
                    }
                    Some(Structural::Closing(_)) => {
                        self.classifier.next();
                        break;
                    }
                    _ => (),
                }
            },
        }
    }

    fn transition_based_on_node(&mut self, node: Option<&JsonPathQueryNode>) {
        match node {
            None => {
                self.count += 1;
            }
            Some(JsonPathQueryNode::Root(child)) => {
                if let Some(Structural::Opening(_)) = self.classifier.peek() {
                    self.run_state(State::Initial, child.as_deref());
                }
            }
            Some(JsonPathQueryNode::Label(label, child)) => {
                self.run_state(State::MatchLabel(label), child.as_deref())
            }
            Some(JsonPathQueryNode::Descendant(child)) => {
                if let Some(Structural::Opening(_)) = self.classifier.peek() {
                    self.classifier.next();
                    self.run_state(State::RecursiveDescent, Some(child));
                }
            }
        }
    }
}

#[derive(Debug)]
enum State<'a> {
    Initial,
    MatchLabel(&'a Label),
    RecursiveDescent,
}
