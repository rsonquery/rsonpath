//! [`Recorder`] implementation finding the starts of all matches.
//! Faster than a full [`NodesRecorder`](super::nodes::NodesRecorder).
use super::*;
use std::cell::RefCell;

/// Recorder that saves only the start indices to the [`Sink`].
pub struct IndexRecorder<'s, S> {
    sink: RefCell<&'s mut S>,
    leading_padding_len: usize,
}

impl<'s, S> IndexRecorder<'s, S> {
    #[inline]
    pub(crate) fn new(sink: &'s mut S, leading_padding_len: usize) -> Self {
        Self {
            sink: RefCell::new(sink),
            leading_padding_len,
        }
    }
}

impl<B: Deref<Target = [u8]>, S> InputRecorder<B> for IndexRecorder<'_, S>
where
    S: Sink<MatchIndex>,
{
    #[inline(always)]
    fn record_block_start(&self, _new_block: B) {
        // Intentionally left empty.
    }
}

impl<B: Deref<Target = [u8]>, S> Recorder<B> for IndexRecorder<'_, S>
where
    S: Sink<MatchIndex>,
{
    #[inline]
    fn record_match(&self, idx: usize, _depth: Depth, _ty: MatchedNodeType) -> Result<(), EngineError> {
        self.sink
            .borrow_mut()
            .add_match(idx - self.leading_padding_len)
            .map_err(|err| EngineError::SinkError(Box::new(err)))
    }

    #[inline]
    fn record_value_terminator(&self, _idx: usize, _depth: Depth) -> Result<(), EngineError> {
        // Intentionally left empty.
        Ok(())
    }
}
