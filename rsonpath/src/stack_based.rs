//! Reference implementation of a JSONPath query engine with recursive descent.

use crate::classify::{classify_structural_characters, Structural, StructuralIterator};
use crate::debug;
use crate::engine::result::QueryResult;
use crate::engine::{Input, Runner};
use crate::query::automaton::Automaton;
use crate::query::{JsonPathQuery, JsonPathQueryNode, Label};
use aligners::{alignment, AlignedBytes, AlignedSlice};
use smallvec::{smallvec, SmallVec};
use std::iter::Peekable;
use std::rc::{Rc, Weak};

pub struct StackBasedRunner<'q> {
    automaton: Automaton<'q>,
}

impl<'q> StackBasedRunner<'q> {
    /// Compile a query into a [`StackBasedRunner`].
    pub fn compile_query(query: &'q JsonPathQuery) -> Self {
        let automaton = Automaton::new(query);

        StackBasedRunner { automaton }
    }
}

impl Runner for StackBasedRunner<'_> {
    fn run<R: QueryResult>(&self, input: &Input) -> R {
        if self.automaton.states().len() == 2 {
            return empty_query(input);
        }

        let aligned_bytes: &AlignedSlice<alignment::Page> = input;
        let mut classifier = classify_structural_characters(aligned_bytes.relax_alignment());
        classifier.next();
        let mut result = R::default();
        let mut execution_ctx =
            ExecutionContext::new(classifier, &self.automaton, input, &mut result);
        execution_ctx.run(0);
        result
    }
}

fn empty_query<R: QueryResult>(bytes: &AlignedBytes<alignment::Page>) -> R {
    let mut block_event_source = classify_structural_characters(bytes.relax_alignment());
    let mut result = R::default();

    if let Some(Structural::Opening(idx)) = block_event_source.next() {
        result.report(idx);
    }

    result
}

struct ExecutionContext<'q, 'b, 'r, I, R>
where
    I: StructuralIterator<'b>,
    R: QueryResult,
{
    classifier: Peekable<I>,
    automaton: &'b Automaton<'q>,
    bytes: &'b [u8],
    #[cfg(debug_assertions)]
    depth: usize,
    result: &'r mut R,
}

impl<'q, 'b, 'r, I, R> ExecutionContext<'q, 'b, 'r, I, R>
where
    I: StructuralIterator<'b>,
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
            classifier: classifier.peekable(),
            automaton,
            bytes,
            depth: 1,
            result,
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
            classifier: classifier.peekable(),
            automaton,
            bytes,
            result,
        }
    }

    pub(crate) fn run(&mut self, state: u8) {
        debug!("Run state {state}, depth {}", self.depth);
        loop {
            match self.classifier.next() {
                Some(Structural::Opening(_)) => {
                    debug!("Opening, falling back");
                    self.increase_depth();
                    self.run(self.automaton.states()[state as usize].fallback_state());
                }
                Some(Structural::Closing(_)) => {
                    debug!("Closing, popping stack");
                    self.decrease_depth();
                    break;
                }
                Some(Structural::Colon(idx)) => {
                    let next_event = self.classifier.peek();
                    let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));
                    for &(label, target) in self.automaton.states()[state as usize].transitions() {
                        if is_next_opening {
                            if self.is_match(idx, label) {
                                debug!("Matched transition to {target}");
                                if target == (self.automaton.states().len() - 2) as u8 {
                                    self.result.report(idx);
                                }
                                self.classifier.next();
                                self.increase_depth();
                                self.run(target);
                            }
                        } else if target == (self.automaton.states().len() - 2) as u8
                            && self.is_match(idx, label)
                        {
                            debug!("Matched transition to acceptance in {target}");
                            self.result.report(idx);
                        }
                    }
                }
                _ => break,
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

    #[inline(always)]
    #[cfg(debug_assertions)]
    fn increase_depth(&mut self) {
        self.depth += 1;
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn increase_depth(&mut self) {}

    #[inline(always)]
    #[cfg(debug_assertions)]
    fn decrease_depth(&mut self) {
        self.depth -= 1;
    }

    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn decrease_depth(&mut self) {}
}
