//! Reference implementation of a JSONPath query engine with recursive descent.

#[cfg(feature = "head-skip")]
use super::head_skipping::{CanHeadSkip, HeadSkip};
use crate::classification::quotes::{classify_quoted_sequences, QuoteClassifiedIterator};
use crate::classification::structural::{
    classify_structural_characters, Structural, StructuralIterator,
};
use crate::classification::ClassifierWithSkipping;
use crate::debug;
use crate::engine::error::EngineError;
use crate::engine::{Compiler, Engine, Input};
use crate::query::automaton::{Automaton, State};
use crate::query::error::CompilerError;
use crate::query::{JsonPathQuery, Label};
use crate::result::QueryResult;
use aligners::{alignment, AlignedBytes, AlignedSlice};

/// Recursive implementation of the JSONPath query engine.
pub struct RecursiveEngine<'q> {
    automaton: Automaton<'q>,
}

impl Compiler for RecursiveEngine<'_> {
    type E<'q> = RecursiveEngine<'q>;

    #[must_use = "compiling the query only creates an engine instance that should be used"]
    #[inline(always)]
    fn compile_query(query: &JsonPathQuery) -> Result<RecursiveEngine, CompilerError> {
        let automaton = Automaton::new(query)?;
        debug!("DFA:\n {}", automaton);
        Ok(RecursiveEngine { automaton })
    }
}

impl Engine for RecursiveEngine<'_> {
    #[inline]
    fn run<R: QueryResult>(&self, input: &Input) -> Result<R, EngineError> {
        if self.automaton.is_empty_query() {
            return empty_query(input);
        }

        let aligned_bytes: &AlignedSlice<alignment::Page> = input;
        let quote_classifier = classify_quoted_sequences(aligned_bytes.relax_alignment());
        let structural_classifier = classify_structural_characters(quote_classifier);
        let mut classifier = ClassifierWithSkipping::new(structural_classifier);

        match classifier.next() {
            Some(Structural::Opening(idx)) => {
                let mut result = R::default();
                let mut execution_ctx = ExecutionContext::new(&self.automaton, input);
                execution_ctx.run(
                    &mut classifier,
                    self.automaton.initial_state(),
                    idx,
                    &mut result,
                )?;
                Ok(result)
            }
            _ => Ok(R::default()),
        }
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

struct ExecutionContext<'q, 'b> {
    automaton: &'b Automaton<'q>,
    bytes: &'b AlignedBytes<alignment::Page>,
}

impl<'q, 'b> ExecutionContext<'q, 'b> {
    pub(crate) fn new(
        automaton: &'b Automaton<'q>,
        bytes: &'b AlignedBytes<alignment::Page>,
    ) -> Self {
        Self { automaton, bytes }
    }

    #[cfg(feature = "head-skip")]
    fn run<'r, Q, I, R>(
        &mut self,
        classifier: &mut ClassifierWithSkipping<'b, Q, I>,
        state: State,
        open_idx: usize,
        result: &'r mut R,
    ) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b>,
        I: StructuralIterator<'b, Q>,
        R: QueryResult,
    {
        let mb_head_skip = HeadSkip::new(self.bytes, self.automaton);

        match mb_head_skip {
            Some(head_skip) => head_skip.run_head_skipping(self, result),
            None => self
                .run_on_subtree(classifier, state, open_idx, result)
                .map(|_| ()),
        }
    }

    #[cfg(not(feature = "head-skip"))]
    fn run<'r, Q, I, R>(
        &mut self,
        classifier: &mut ClassifierWithSkipping<'b, Q, I>,
        state: State,
        open_idx: usize,
        result: &'r mut R,
    ) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b>,
        I: StructuralIterator<'b, Q>,
        R: QueryResult,
    {
        self.run_on_subtree(classifier, state, open_idx, result)
            .map(|_| ())
    }

    fn run_on_subtree<'r, Q, I, R>(
        &mut self,
        classifier: &mut ClassifierWithSkipping<'b, Q, I>,
        state: State,
        open_idx: usize,
        result: &'r mut R,
    ) -> Result<usize, EngineError>
    where
        Q: QuoteClassifiedIterator<'b>,
        I: StructuralIterator<'b, Q>,
        R: QueryResult,
    {
        debug!("Run state {state}");
        let mut next_event = None;
        let mut latest_idx = open_idx;
        let fallback_state = self.automaton[state].fallback_state();
        let is_fallback_accepting = self.automaton.is_accepting(fallback_state);
        let is_list = self.bytes[open_idx] == b'[';
        let needs_commas = is_list && is_fallback_accepting;
        let needs_colons = !is_list && self.automaton.has_transition_to_accepting(state);

        let config_characters = |classifier: &mut ClassifierWithSkipping<'b, Q, I>, idx: usize| {
            if needs_commas {
                classifier.turn_commas_on(idx);
            } else {
                classifier.turn_commas_off();
            }

            if needs_colons {
                classifier.turn_colons_on(idx);
            } else {
                classifier.turn_colons_off();
            }
        };

        config_characters(classifier, open_idx);

        if needs_commas {
            next_event = classifier.next();
            if let Some(Structural::Closing(close_idx)) = next_event {
                for idx in (open_idx + 1)..close_idx {
                    if !self.bytes[idx].is_ascii_whitespace() {
                        debug!("Accepting only item in the list.");
                        result.report(idx);
                        break;
                    }
                }
                return Ok(close_idx);
            }

            debug!("Accepting first item in the list.");
            result.report(open_idx + 1);
        }

        loop {
            if next_event.is_none() {
                next_event = classifier.next();
            }
            debug!("Event: {next_event:?}");
            match next_event {
                Some(Structural::Comma(idx)) => {
                    latest_idx = idx;
                    next_event = classifier.next();
                    let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));

                    if !is_next_opening && is_list && is_fallback_accepting {
                        debug!("Accepting on comma.");
                        result.report(idx);
                    }
                }
                Some(Structural::Colon(idx)) => {
                    debug!(
                        "Colon, label ending with {:?}",
                        std::str::from_utf8(&self.bytes[(if idx < 8 { 0 } else { idx - 8 })..idx])
                            .unwrap_or("[invalid utf8]")
                    );

                    latest_idx = idx;
                    next_event = classifier.next();
                    let is_next_opening = matches!(next_event, Some(Structural::Opening(_)));

                    if !is_next_opening {
                        let mut any_matched = false;

                        for &(label, target) in self.automaton[state].transitions() {
                            if self.automaton.is_accepting(target) && self.is_match(idx, label)? {
                                debug!("Accept {idx}");
                                result.report(idx);
                                any_matched = true;
                                break;
                            }
                        }
                        let fallback_state = self.automaton[state].fallback_state();
                        if !any_matched && self.automaton.is_accepting(fallback_state) {
                            debug!("Value accepted by fallback.");
                            result.report(idx);
                        }
                        #[cfg(feature = "unique-labels")]
                        {
                            let is_next_closing =
                                matches!(next_event, Some(Structural::Closing(_)));
                            if any_matched && !is_next_closing && self.automaton.is_unitary(state) {
                                let opening = if is_list { b'[' } else { b'{' };
                                debug!("Skipping unique state from {}", opening as char);
                                let stop_at = classifier.skip(opening);
                                next_event = Some(Structural::Closing(stop_at));
                            }
                        }
                    }
                }
                Some(Structural::Opening(idx)) => {
                    let mut matched = None;
                    let colon_idx = {
                        let mut colon_idx = idx - 1;
                        while colon_idx > 0 && self.bytes[colon_idx].is_ascii_whitespace() {
                            colon_idx -= 1;
                        }
                        (self.bytes[colon_idx] == b':').then_some(colon_idx)
                    };

                    if let Some(colon_idx) = colon_idx {
                        debug!(
                            "Colon backtracked, label ending with {:?}",
                            std::str::from_utf8(
                                &self.bytes
                                    [(if colon_idx < 8 { 0 } else { colon_idx - 8 })..colon_idx]
                            )
                            .unwrap_or("[invalid utf8]")
                        );
                        for &(label, target) in self.automaton[state].transitions() {
                            if self.is_match(colon_idx, label)? {
                                matched = Some(target);
                                if self.automaton.is_accepting(target) {
                                    debug!("Accept {idx}");
                                    result.report(colon_idx);
                                }
                                break;
                            }
                        }
                    }

                    let end_idx = match matched {
                        Some(target) => self.run_on_subtree(classifier, target, idx, result)?,
                        None => {
                            let fallback = self.automaton[state].fallback_state();
                            debug!("Falling back to {fallback}");

                            if self.automaton.is_accepting(fallback) {
                                debug!("Accept {idx}");
                                result.report(idx);
                            }

                            #[cfg(feature = "tail-skip")]
                            if self.automaton.is_rejecting(fallback_state) {
                                classifier.skip(self.bytes[idx])
                            } else {
                                self.run_on_subtree(classifier, fallback_state, idx, result)?
                            }
                            #[cfg(not(feature = "tail-skip"))]
                            {
                                self.run_on_subtree(classifier, fallback_state, idx, result)?
                            }
                        }
                    };

                    debug!("Return to {state}");
                    next_event = None;
                    latest_idx = end_idx;

                    #[cfg(feature = "unique-labels")]
                    {
                        if matched.is_some() && self.automaton.is_unitary(state) {
                            let opening = if is_list { b'[' } else { b'{' };
                            debug!("Skipping unique state from {}", opening as char);
                            let stop_at = classifier.skip(opening);
                            latest_idx = stop_at;
                            break;
                        }
                    }

                    config_characters(classifier, end_idx);
                }
                Some(Structural::Closing(idx)) => {
                    latest_idx = idx;
                    break;
                }
                None => break,
            }
        }

        Ok(latest_idx)
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

#[cfg(feature = "head-skip")]
impl<'q, 'b> CanHeadSkip<'b> for ExecutionContext<'q, 'b> {
    fn run_on_subtree<'r, R, Q, I>(
        &mut self,
        next_event: Structural,
        state: State,
        classifier: &mut ClassifierWithSkipping<'b, Q, I>,
        result: &'r mut R,
    ) -> Result<(), EngineError>
    where
        Q: QuoteClassifiedIterator<'b>,
        R: QueryResult,
        I: StructuralIterator<'b, Q>,
    {
        self.run_on_subtree(classifier, state, next_event.idx(), result)?;

        Ok(())
    }
}
