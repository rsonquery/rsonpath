//! [`Recorder`] implementation finding the starts and approximate ends of all matches.
//! Faster than a full [`NodesRecorder`](super::nodes::NodesRecorder), but the span
//! may include trailing whitespace after the actual matched value.
use super::{output_queue::OutputQueue, *};
use std::cell::RefCell;

/// Recorder that finds approximate [`MatchSpans`](`MatchSpan`),
/// possibly including trailing whitespace after the actual match.
pub struct ApproxSpanRecorder<'s, S> {
    internal: RefCell<InternalRecorder<'s, S>>,
}

struct InternalRecorder<'s, S> {
    sink: &'s mut S,
    match_count: usize,
    stack: Vec<PartialNode>,
    output_queue: OutputQueue<MatchSpan>,
}

struct PartialNode {
    id: usize,
    start_idx: MatchIndex,
    start_depth: Depth,
    ty: MatchedNodeType,
}

impl<'s, S> ApproxSpanRecorder<'s, S> {
    #[inline]
    pub(crate) fn new(sink: &'s mut S) -> Self {
        Self {
            internal: RefCell::new(InternalRecorder::new(sink)),
        }
    }
}

impl<'s, B: Deref<Target = [u8]>, S> InputRecorder<B> for ApproxSpanRecorder<'s, S>
where
    S: Sink<MatchSpan>,
{
    #[inline(always)]
    fn record_block_start(&self, _new_block: B) {
        // Intentionally left empty.
    }
}

impl<'s, B: Deref<Target = [u8]>, S> Recorder<B> for ApproxSpanRecorder<'s, S>
where
    S: Sink<MatchSpan>,
{
    #[inline]
    fn record_match(&self, idx: usize, depth: Depth, ty: MatchedNodeType) -> Result<(), EngineError> {
        self.internal.borrow_mut().record_start(idx, depth, ty);
        Ok(())
    }

    #[inline]
    fn record_value_terminator(&self, idx: usize, depth: Depth) -> Result<(), EngineError> {
        self.internal.borrow_mut().record_end(idx, depth)
    }
}

impl<'s, S> InternalRecorder<'s, S> {
    fn new(sink: &'s mut S) -> Self {
        Self {
            sink,
            stack: vec![],
            match_count: 0,
            output_queue: OutputQueue::new(),
        }
    }
}

impl<'s, S> InternalRecorder<'s, S>
where
    S: Sink<MatchSpan>,
{
    fn record_start(&mut self, start_idx: usize, start_depth: Depth, ty: MatchedNodeType) {
        self.stack.push(PartialNode {
            id: self.match_count,
            start_idx,
            start_depth,
            ty,
        });
        self.match_count += 1;
    }

    fn record_end(&mut self, idx: usize, depth: Depth) -> Result<(), EngineError> {
        while let Some(node) = self.stack.last() {
            if node.start_depth >= depth {
                let node = self.stack.pop().expect("last was Some, pop must succeed");
                let end_idx = if node.ty == MatchedNodeType::Complex {
                    idx + 1
                } else {
                    idx
                };
                let span = MatchSpan {
                    start_idx: node.start_idx,
                    len: end_idx - node.start_idx,
                };
                self.output_queue.insert(node.id, span);
            } else {
                break;
            }
        }

        if self.stack.is_empty() {
            self.output_queue
                .output_to(self.sink)
                .map_err(|err| EngineError::SinkError(Box::new(err)))?;
        }

        Ok(())
    }
}
