//! Query run recorder for result reporting.

use crate::result::{QueryResult, EmptyResult};
use crate::classification::structural::Structural;

pub trait InputRecorder {
    fn record_block_start(&self, new_block: &[u8]);
}

pub trait Recorder: InputRecorder {
    type Result: QueryResult;
    
    /// Start a new recorder.
    fn new() -> Self;

    /// Record a match of the query. The `idx` is guaranteed to be the first character of the matched value.
    fn record_match(&self, idx: usize);

    /// Record an occurrence of a structural character.
    fn record_structural(&self, s: Structural, idx: usize);

    /// Finish building the result and return it.
    fn finish(self) -> Self::Result;
}

pub struct EmptyRecorder;

impl InputRecorder for EmptyRecorder {
    #[inline]
    fn record_block_start(&self, _new_block: &[u8]) {
        // Intentionally left empty.
    }
}

impl Recorder for EmptyRecorder {
    type Result = EmptyResult;

    #[inline]
    fn new() -> Self {
        EmptyRecorder
    }

    #[inline]
    fn record_match(&self, _idx: usize) {
        // Intentionally left empty.
    }

    #[inline]
    fn record_structural(&self, s: Structural, idx: usize) {
        // Intentionally left empty.
    }

    #[inline]
    fn finish(self) -> Self::Result {
        EmptyResult
    }
}
