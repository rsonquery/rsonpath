use super::*;
use std::{
    cell::RefCell,
    fmt::{self, Display},
};

/// Query result containing all indices of colons that constitute a match.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexResult {
    indices: Vec<usize>,
}

impl IndexResult {
    /// Get indices of colons constituting matches of the query.
    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> &[usize] {
        &self.indices
    }
}

impl From<IndexResult> for Vec<usize> {
    #[inline(always)]
    fn from(result: IndexResult) -> Self {
        result.indices
    }
}

impl Display for IndexResult {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.indices)
    }
}

impl QueryResult for IndexResult {}

pub struct IndexRecorder {
    indices: RefCell<Vec<usize>>,
}

impl InputRecorder for IndexRecorder {
    #[inline(always)]
    fn record_block_end(&self, _new_block: &[u8]) {
        // Intentionally left empty.
    }
}

impl Recorder for IndexRecorder {
    type Result = IndexResult;

    #[inline]
    fn new() -> Self {
        Self {
            indices: RefCell::new(vec![]),
        }
    }

    #[inline]
    fn record_match(&self, idx: usize, _ty: MatchedNodeType) {
        self.indices.borrow_mut().push(idx);
    }

    #[inline(always)]
    fn record_structural(&self, _s: Structural) {
        // Intentionally left empty.
    }

    #[inline]
    fn finish(self) -> Self::Result {
        IndexResult {
            indices: self.indices.into_inner(),
        }
    }
}
