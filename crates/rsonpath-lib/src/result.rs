//! Result types that can be returned by a JSONPath query engine.
use crate::depth::Depth;
use std::{fmt::Display, ops::Deref};

pub mod count;
pub mod empty;
pub mod index;
pub mod nodes;

/// Type of a value being reported to a [`Recorder`].
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MatchedNodeType {
    /// JSON string, number, or literal value.
    Atomic,
    /// JSON object or array.
    Complex,
}

/// Result that can be returned from a query run.
pub trait QueryResult: Default + Display + PartialEq {}

pub trait RecorderSpec {
    type Result: QueryResult;
    type Recorder<B>: Recorder<B, Result = Self::Result>
    where
        B: Deref<Target = [u8]>;

    fn new<B: Deref<Target = [u8]>>() -> Self::Recorder<B>;
}

/// Base trait of any recorder, one that can react to a block of input being processed.
pub trait InputRecorder<B: Deref<Target = [u8]>> {
    /// Record that all processing of a block was started
    ///
    /// The recorder may assume that only matches or terminators with indices pointing to
    /// the block that was last recorded as started are reported.
    fn record_block_start(&self, new_block: B);
}

/// An observer that can build a [`QueryResult`] based on match and structural events coming from the execution engine.
pub trait Recorder<B: Deref<Target = [u8]>>: InputRecorder<B> {
    /// The unique type of a [`QueryResult`] built by this recorder.
    type Result: QueryResult;

    /// Start a new recorder.
    #[must_use]
    fn new() -> Self;

    /// Record a match of the query at a given `depth`.
    /// The `idx` is guaranteed to be the first character of the matched value.
    ///
    /// The type MUST accurately describe the value being matched.
    fn record_match(&self, idx: usize, depth: Depth, ty: MatchedNodeType);

    /// Record a structural character signifying the end of a value at a given `idx`
    /// and with given `depth`.
    fn record_value_terminator(&self, idx: usize, depth: Depth);

    /// Finish building the result and return it.
    #[must_use]
    fn finish(self) -> Self::Result;
}
