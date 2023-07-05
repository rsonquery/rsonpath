use super::*;
use std::{
    cell::Cell,
    fmt::{self, Display},
};

/// Result informing on the number of values matching the executed query.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountResult {
    count: u64,
}

impl CountResult {
    /// Number of values matched by the executed query.
    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> u64 {
        self.count
    }
}

impl Display for CountResult {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}

impl QueryResult for CountResult {}

pub struct CountRecorder {
    count: Cell<u64>,
}

impl InputRecorder for CountRecorder {
    #[inline(always)]
    fn record_block_end(&self, _new_block: &[u8]) {
        // Intentionally left empty.
    }
}

impl Recorder for CountRecorder {
    type Result = CountResult;

    #[inline]
    fn new() -> Self {
        Self { count: Cell::new(0) }
    }

    #[inline]
    fn record_match(&self, _idx: usize, _ty: MatchedNodeType) {
        self.count.set(self.count.get() + 1);
    }

    #[inline(always)]
    fn record_structural(&self, _s: Structural) {
        // Intentionally left empty.
    }

    #[inline]
    fn finish(self) -> Self::Result {
        CountResult {
            count: self.count.into_inner(),
        }
    }
}
