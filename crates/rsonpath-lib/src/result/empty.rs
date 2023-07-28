//! Empty [`QueryResult`] and [`Recorder`] implementation, mainly for testing purposes.
use super::*;
use std::fmt::{self, Display};

/// Empty [`QueryResult`] that aggregates nothing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmptyResult;

impl Display for EmptyResult {
    #[inline(always)]
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl QueryResult for EmptyResult {}

pub struct EmptyRecorderSpec;

impl RecorderSpec for EmptyRecorderSpec {
    type Result = EmptyResult;

    type Recorder<B> = EmptyRecorder
    where
        B: Deref<Target = [u8]>;

    #[inline(always)]
    fn new<B: Deref<Target = [u8]>>() -> Self::Recorder<B> {
        <EmptyRecorder as Recorder<B>>::new()
    }
}

/// Recorder for [`EmptyResult`].
pub struct EmptyRecorder;

impl<B: Deref<Target = [u8]>> InputRecorder<B> for EmptyRecorder {
    #[inline]
    fn record_block_start(&self, _new_block: B) {
        // Intentionally left empty.
    }
}

impl<B: Deref<Target = [u8]>> Recorder<B> for EmptyRecorder {
    type Result = EmptyResult;

    #[inline]
    fn new() -> Self {
        Self
    }

    #[inline]
    fn record_match(&self, _idx: usize, _depth: Depth, _ty: MatchedNodeType) {
        // Intentionally left empty.
    }

    #[inline]
    fn record_value_terminator(&self, _idx: usize, _depth: Depth) {
        // Intentionally left empty.
    }

    #[inline]
    fn finish(self) -> Self::Result {
        EmptyResult
    }
}
