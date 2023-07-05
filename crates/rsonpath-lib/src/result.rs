//! Result types that can be returned by a JSONPath query engine.
use crate::classification::structural::Structural;
use std::fmt::Display;

pub mod count;
pub mod empty;
pub mod index;
pub mod nodes;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MatchedNodeType {
    Atomic,
    Complex,
}

/// Result that can be returned from a query run.
pub trait QueryResult: Default + Display + PartialEq {}

pub trait InputRecorder {
    fn record_block_end(&self, new_block: &[u8]);
}

pub trait Recorder: InputRecorder {
    type Result: QueryResult;

    /// Start a new recorder.
    #[must_use]
    fn new() -> Self;

    /// Record a match of the query. The `idx` is guaranteed to be the first character of the matched value.
    fn record_match(&self, idx: usize, ty: MatchedNodeType);

    /// Record an occurrence of a structural character.
    fn record_structural(&self, s: Structural);

    /// Finish building the result and return it.
    #[must_use]
    fn finish(self) -> Self::Result;
}
