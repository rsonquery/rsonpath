//! [`Recorder`] implementation for counting the number of matches.
//!
//! This is faster than any recorder that actually examines the values.
use super::*;
use std::cell::Cell;

/// Recorder that keeps only the number of matches, no details.
pub struct CountRecorder {
    count: Cell<MatchCount>,
}

impl CountRecorder {
    pub(crate) fn new() -> Self {
        Self { count: Cell::new(0) }
    }
}

impl From<CountRecorder> for MatchCount {
    #[inline]
    fn from(val: CountRecorder) -> Self {
        val.count.into_inner()
    }
}

impl<B: Deref<Target = [u8]>> InputRecorder<B> for CountRecorder {
    #[inline(always)]
    fn record_block_start(&self, _new_block: B) {
        // Intentionally left empty.
    }
}

impl<B: Deref<Target = [u8]>> Recorder<B> for CountRecorder {
    #[inline]
    fn record_match(&self, _idx: usize, _depth: Depth, _ty: MatchedNodeType) -> Result<(), EngineError> {
        self.count.set(self.count.get() + 1);
        Ok(())
    }

    #[inline]
    fn record_value_terminator(&self, _idx: usize, _depth: Depth) -> Result<(), EngineError> {
        // Intentionally left empty.
        Ok(())
    }
}
