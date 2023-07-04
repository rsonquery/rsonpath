//! Result types that can be returned by a JSONPath query engine.
use std::fmt::{self, Display};

/// Result that can be returned from a query run.
pub trait QueryResult: Default + Display + PartialEq {}

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

/// The [`QueryResultBuilder`] for [`IndexResult`].
#[must_use]
pub struct IndexResultBuilder<'i, I> {
    input: &'i I,
    indices: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyResult;

impl Display for EmptyResult {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl QueryResult for EmptyResult {}
