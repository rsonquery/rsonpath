//! Result types that can be returned by a JSONPath query engine.

use std::fmt::{self, Display};

/// Result that can be reported during query execution.
pub trait QueryResult: Default + Display + PartialEq {
    /// Report a match of the query.
    fn report(&mut self, index: usize);
}

/// Result informing on the number of values matching the executed query.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountResult {
    count: usize,
}

impl CountResult {
    /// Number of values matched by the executed query.
    #[inline(always)]
    pub fn get(&self) -> usize {
        self.count
    }
}

impl Display for CountResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}

impl QueryResult for CountResult {
    #[inline(always)]
    fn report(&mut self, _item: usize) {
        self.count += 1;
    }
}

/// Query result containing all indices of colons that constitute a
/// match.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexResult {
    indices: Vec<usize>,
}

impl IndexResult {
    /// Get indices of colons constituting matches of the query.
    #[inline(always)]
    pub fn get(&self) -> &[usize] {
        &self.indices
    }
}

impl From<IndexResult> for Vec<usize> {
    fn from(result: IndexResult) -> Self {
        result.indices
    }
}

impl Display for IndexResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.indices)
    }
}

impl QueryResult for IndexResult {
    #[inline(always)]
    fn report(&mut self, item: usize) {
        self.indices.push(item);
    }
}
