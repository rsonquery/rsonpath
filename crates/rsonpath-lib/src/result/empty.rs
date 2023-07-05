use super::*;
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyResult;

impl Display for EmptyResult {
    #[inline(always)]
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl QueryResult for EmptyResult {}

pub struct EmptyRecorder;

impl InputRecorder for EmptyRecorder {
    #[inline]
    fn record_block_end(&self, _new_block: &[u8]) {
        // Intentionally left empty.
    }
}

impl Recorder for EmptyRecorder {
    type Result = EmptyResult;

    #[inline]
    fn new() -> Self {
        Self
    }

    #[inline]
    fn record_match(&self, _idx: usize, _ty: MatchedNodeType) {
        // Intentionally left empty.
    }

    #[inline]
    fn record_structural(&self, _s: Structural) {
        // Intentionally left empty.
    }

    #[inline]
    fn finish(self) -> Self::Result {
        EmptyResult
    }
}
