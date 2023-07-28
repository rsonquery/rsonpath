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

impl RecorderSpec for IndexRecorderSpec {
    type Result = IndexResult;

    type Recorder<B> = IndexRecorder
    where
        B: Deref<Target = [u8]>;

    #[inline(always)]
    fn new<B: Deref<Target = [u8]>>() -> Self::Recorder<B> {
        <IndexRecorder as Recorder<B>>::new()
    }
}

/// Recorder for [`IndexResult`].
pub struct IndexRecorder {
    indices: RefCell<Vec<usize>>,
}

impl<B: Deref<Target = [u8]>> InputRecorder<B> for IndexRecorder {
    #[inline(always)]
    fn record_block_start(&self, _new_block: B) {
        // Intentionally left empty.
    }
}

impl<B: Deref<Target = [u8]>> Recorder<B> for IndexRecorder {
    type Result = IndexResult;

    #[inline]
    fn new() -> Self {
        Self {
            indices: RefCell::new(vec![]),
        }
    }

    #[inline]
    fn record_match(&self, idx: usize, _depth: Depth, _ty: MatchedNodeType) {
        self.indices.borrow_mut().push(idx);
    }

    #[inline]
    fn record_value_terminator(&self, _idx: usize, _depth: Depth) {
        // Intentionally left empty.
    }

    #[inline]
    fn finish(self) -> Self::Result {
        IndexResult {
            indices: self.indices.into_inner(),
        }
    }
}
