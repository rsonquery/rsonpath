//! Empty [`QueryResult`] and [`Recorder`] implementation, mainly for testing purposes.
use super::*;

/// Recorder for [`EmptyResult`].
pub struct EmptyRecorder;

impl<B: Deref<Target = [u8]>> InputRecorder<B> for EmptyRecorder {
    #[inline]
    fn record_block_start(&self, _new_block: B) {
        // Intentionally left empty.
    }
}

impl<B: Deref<Target = [u8]>> Recorder<B> for EmptyRecorder {
    #[inline]
    fn record_match(&self, _idx: usize, _depth: Depth, _ty: MatchedNodeType) -> Result<(), EngineError> {
        // Intentionally left empty.
        Ok(())
    }

    #[inline]
    fn record_value_terminator(&self, _idx: usize, _depth: Depth) -> Result<(), EngineError> {
        // Intentionally left empty.
        Ok(())
    }
}
