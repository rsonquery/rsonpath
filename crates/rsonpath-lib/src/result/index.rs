//! [`QueryResult`] and [`Recorder`] implementation finding the starts of all matches.
//!
//! This is useful if you can provide a separate parsing function that will examine the
//! matches after the search. The result provides starting indices for the parser.
//! If the entire input is available, and you intend to parse the results manually,
//! this search is significantly faster than more involved search techniques.
use super::*;
use std::cell::RefCell;

/// Recorder for [`IndexResult`].
pub struct IndexRecorder<'s, S> {
    sink: RefCell<&'s mut S>,
}

impl<'s, S> IndexRecorder<'s, S> {
    #[inline]
    pub(crate) fn new(sink: &'s mut S) -> Self {
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
