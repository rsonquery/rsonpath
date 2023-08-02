//! [`QueryResult`] and [`Recorder`] implementation finding the starts of all matches.
//!
//! This is useful if you can provide a separate parsing function that will examine the
//! matches after the search. The result provides starting indices for the parser.
//! If the entire input is available, and you intend to parse the results manually,
//! this search is significantly faster than more involved search techniques.
use super::*;
use std::{
    cell::RefCell,
    fmt::{self, Display},
};

/// [`QueryResult`] containing all byte indices of starts of matched values.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexResult {
    indices: Vec<usize>,
}

impl IndexResult {
    /// Get starting indices of matches of the query.
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

pub struct IndexRecorderSpec;

/// Recorder for [`IndexResult`].
pub struct IndexRecorder<'s, S> {
    sink: RefCell<&'s mut S>,
}

impl<'s, S> IndexRecorder<'s, S> {
    #[inline]
    pub fn new(sink: &'s mut S) -> Self {
        Self {
            sink: RefCell::new(sink),
        }
    }
}

impl<'s, B: Deref<Target = [u8]>, S> InputRecorder<B> for IndexRecorder<'s, S>
where
    S: Sink<MatchIndex>,
{
    #[inline(always)]
    fn record_block_start(&self, _new_block: B) {
        // Intentionally left empty.
    }
}

impl<'s, B: Deref<Target = [u8]>, S> Recorder<B> for IndexRecorder<'s, S>
where
    S: Sink<MatchIndex>,
{
    #[inline]
    fn record_match(&self, idx: usize, _depth: Depth, _ty: MatchedNodeType) -> Result<(), EngineError> {
        self.sink
            .borrow_mut()
            .add_match(idx)
            .map_err(|err| EngineError::SinkError(Box::new(err)))
    }

    #[inline]
    fn record_value_terminator(&self, _idx: usize, _depth: Depth) -> Result<(), EngineError> {
        // Intentionally left empty.
        Ok(())
    }
}
