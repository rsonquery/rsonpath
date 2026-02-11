//! Main [`Recorder`] implementation collecting the bytes of all matches.
//!
//! This is the heaviest recorder. It will copy all bytes of all matches into [`Vecs`](`Vec`).
#![allow(
    clippy::expect_used,
    reason = "There is number of invariants that are hard to enforce on the type level, \
    and handling of Depth that should be properly error-handled by the engine, not here. \
    Using `expect` here is idiomatic"
)]

use super::{output_queue::OutputQueue, *};
use crate::{debug, is_json_whitespace};
use std::{
    cell::RefCell,
    fmt::{self, Debug},
    str,
};

/// Recorder that saves full info about a [`Match`].
pub struct NodesRecorder<'s, B, S> {
    internal: RefCell<InternalRecorder<'s, B, S>>,
}

impl<'s, B, S> NodesRecorder<'s, B, S>
where
    B: Deref<Target = [u8]>,
    S: Sink<Match>,
{
    pub(crate) fn build_recorder(sink: &'s mut S, leading_padding_len: usize) -> Self {
        Self {
            internal: RefCell::new(InternalRecorder::new(sink, leading_padding_len)),
        }
    }
}

impl<B, S> InputRecorder<B> for NodesRecorder<'_, B, S>
where
    B: Deref<Target = [u8]>,
    S: Sink<Match>,
{
    #[inline(always)]
    fn record_block_start(&self, new_block: B) {
        self.internal.borrow_mut().record_block(new_block)
    }
}

impl<B, S> Recorder<B> for NodesRecorder<'_, B, S>
where
    B: Deref<Target = [u8]>,
    S: Sink<Match>,
{
    #[inline]
    fn record_match(&self, idx: usize, depth: Depth, ty: MatchedNodeType) -> Result<(), EngineError> {
        debug!("Recording match at {idx}");
        self.internal.borrow_mut().record_match(idx, depth, ty);
        Ok(())
    }

    #[inline]
    fn record_value_terminator(&self, idx: usize, depth: Depth) -> Result<(), EngineError> {
        self.internal
            .borrow_mut()
            .record_value_terminator(idx, depth)
            .map_err(|err| EngineError::SinkError(Box::new(err)))
    }
}

/*
{
    [
        1,
        2,
        [
            3,
            4
        ]
    ],
    [
        5
    ]
}

// Required order:
// [1,2,[3,4]], 1, 2, [3,4], 3, 4, [5], 5

// Finalization order:
// 1, 2, 3, 4, [3,4], [1,2,[3,4]], 5, [5]

1. By default, we assume the common case of no overlapping matches.
In that case we don't have to maintain any stack, the state is simply
a buffer for the current match and information on when to end it.
2. If a new match is registered when there is a match active, it means
they are overlapping and we switch to the second algorithm.

Matches are pushed onto a stack. Every time we finish a match we need to find
the node that is finalized. If we keep all matches on the stack it would take
potentially linear time. In the above example, when [3,4] is finalized,
there is 3 and 4 already finalized *above* on the stack. This leads to a quadratic
blowup if implemented naively (just consider a long list of atoms).

Instead we keep only the active matches on the stack, annotated with the output number
of the match. In a secondary array we keep the finished nodes in the output order.
When popping we can write the node into the array with random-access. Because
the order is maintained, outputting the nodes is easy since we can just look at the
node with the number that should be output next and iterate from there.

This would be potentially wasteful on its own, since we'd always have the secondary array
grow to the total number of matches. We can instead compress the array when it becomes
empty and keep a map between output number and array indices. For example, here's
the state of this algorithm on the above example after the match of "2" is completed.

STACK             | DONE (off. 0) |
                  | Some(2)       |
                  | Some(1)       |
(0, [1,2...)      | None          |

After "4":

STACK             | DONE (off. 0) |
                  | Some(4)       |
                  | Some(3)       |
                  | None          |
                  | Some(2)       |
(3, [3,4])        | Some(1)       |
(0, [1,2,[3,4...) | None          |

Now after the first list gets finalized we can output everything in the array starting from
index 0. Now that the stack is empty we can compress.

STACK             | DONE (off. 5)

Now we push the second list and the 5, finalize the 5.
We write it to array at index 1, since its output order is 6 and the offset from compression
is 5.

STACK             | DONE (off. 5)
                  | Some(5)
(6, [5...)        | None
*/

enum InternalRecorder<'s, B, S> {
    Simple(SimpleRecorder<'s, B, S>),
    Stack(StackRecorder<'s, B, S>),
    Transition,
}

impl<'s, B, S> InternalRecorder<'s, B, S>
where
    B: Deref<Target = [u8]>,
    S: Sink<Match>,
{
    fn new(sink: &'s mut S, leading_padding_len: usize) -> Self {
        Self::Simple(SimpleRecorder::new(sink, leading_padding_len))
    }

    #[inline(always)]
    fn record_block(&mut self, block: B) {
        match self {
            Self::Simple(r) => r.record_block(block),
            Self::Stack(r) => r.record_block(block),
            Self::Transition => unreachable!(),
        }
    }

    #[inline(always)]
    fn record_match(&mut self, idx: usize, depth: Depth, ty: MatchedNodeType) {
        match self {
            Self::Simple(simple) => {
                if !simple.try_record_match(idx, depth, ty) {
                    let simple = match std::mem::replace(self, Self::Transition) {
                        Self::Simple(s) => s,
                        Self::Stack(_) | Self::Transition => unreachable!(),
                    };
                    let mut stack = simple.transform_to_stack();
                    stack.record_match(idx, depth, ty);
                    *self = Self::Stack(stack);
                }
            }
            Self::Stack(stack) => stack.record_match(idx, depth, ty),
            Self::Transition => unreachable!(),
        }
    }

    #[allow(clippy::panic_in_result_fn, reason = "Reaching unreachable is an unrecoverable bug.")]
    #[inline(always)]
    fn record_value_terminator(&mut self, idx: usize, depth: Depth) -> Result<(), EngineError> {
        match self {
            Self::Simple(r) => r.record_value_terminator(idx, depth),
            Self::Stack(r) => r.record_value_terminator(idx, depth),
            Self::Transition => unreachable!(),
        }
    }
}

struct SimpleRecorder<'s, B, S> {
    idx: usize,
    current_block: Option<B>,
    node: Option<SimplePartialNode>,
    sink: &'s mut S,
    leading_padding_len: usize,
}

struct SimplePartialNode {
    start_idx: usize,
    start_depth: Depth,
    buf: Vec<u8>,
    ty: MatchedNodeType,
}

impl<'s, B, S> SimpleRecorder<'s, B, S>
where
    B: Deref<Target = [u8]>,
    S: Sink<Match>,
{
    fn new(sink: &'s mut S, leading_padding_len: usize) -> Self {
        Self {
            idx: 0,
            current_block: None,
            node: None,
            sink,
            leading_padding_len,
        }
    }

    fn record_block(&mut self, block: B) {
        if let Some(finished) = self.current_block.as_ref() {
            if let Some(node) = self.node.as_mut() {
                debug!("Continuing node, idx is {}", self.idx);
                append_block(&mut node.buf, finished, self.idx, node.start_idx)
            }

            self.idx += finished.len();
        }

        self.current_block = Some(block);
        debug!("New block, idx = {}", self.idx);
    }

    fn record_value_terminator(&mut self, idx: usize, depth: Depth) -> Result<(), EngineError> {
        debug!("Value terminator at {idx}, depth {depth}");
        if let Some(node) = self.node.as_ref() {
            if node.start_depth >= depth {
                let mut node = self.node.take().expect("node is Some");
                debug!("Mark node as ended at {}", idx + 1);
                append_final_block(
                    &mut node.buf,
                    self.current_block
                        .as_ref()
                        .ok_or(EngineError::MissingOpeningCharacter())?,
                    self.idx,
                    node.start_idx,
                    idx + 1,
                );
                finalize_node(&mut node.buf, node.ty);

                debug!("Committing and outputting node");
                self.sink
                    .add_match(Match {
                        span_start: node.start_idx - self.leading_padding_len,
                        bytes: node.buf,
                    })
                    .map_err(|err| EngineError::SinkError(Box::new(err)))?;
            }
        }

        Ok(())
    }

    fn try_record_match(&mut self, idx: usize, depth: Depth, ty: MatchedNodeType) -> bool {
        if self.node.is_some() {
            debug!("nested match detected, switching to stack");
            return false;
        }

        let node = SimplePartialNode {
            start_idx: idx,
            start_depth: depth,
            buf: vec![],
            ty,
        };
        self.node = Some(node);

        true
    }

    fn transform_to_stack(self) -> StackRecorder<'s, B, S> {
        match self.node {
            Some(node) => StackRecorder {
                idx: self.idx,
                match_count: 1,
                current_block: self.current_block,
                stack: vec![PartialNode {
                    id: 0,
                    start_idx: node.start_idx,
                    start_depth: node.start_depth,
                    buf: node.buf,
                    ty: node.ty,
                }],
                output_queue: OutputQueue::new(),
                sink: self.sink,
                leading_padding_len: self.leading_padding_len,
            },
            None => StackRecorder {
                idx: self.idx,
                match_count: 0,
                current_block: self.current_block,
                stack: vec![],
                output_queue: OutputQueue::new(),
                sink: self.sink,
                leading_padding_len: self.leading_padding_len,
            },
        }
    }
}

struct StackRecorder<'s, B, S> {
    idx: usize,
    match_count: usize,
    current_block: Option<B>,
    stack: Vec<PartialNode>,
    output_queue: OutputQueue<Match>,
    sink: &'s mut S,
    leading_padding_len: usize,
}

struct PartialNode {
    id: usize,
    start_idx: usize,
    start_depth: Depth,
    buf: Vec<u8>,
    ty: MatchedNodeType,
}

impl<B, S> StackRecorder<'_, B, S>
where
    B: Deref<Target = [u8]>,
    S: Sink<Match>,
{
    fn record_block(&mut self, block: B) {
        if let Some(finished) = self.current_block.as_ref() {
            for node in &mut self.stack {
                debug!("Continuing node: {node:?}, idx is {}", self.idx);
                append_block(&mut node.buf, finished, self.idx, node.start_idx)
            }

            self.idx += finished.len();
        }

        self.current_block = Some(block);
        debug!("New block, idx = {}", self.idx);
    }

    fn record_match(&mut self, idx: usize, depth: Depth, ty: MatchedNodeType) {
        let node = PartialNode {
            id: self.match_count,
            start_idx: idx,
            start_depth: depth,
            buf: vec![],
            ty,
        };

        debug!("New node {node:?}");
        self.match_count += 1;
        self.stack.push(node);
    }

    #[inline]
    fn record_value_terminator(&mut self, idx: usize, depth: Depth) -> Result<(), EngineError> {
        debug!("Value terminator at {idx}, depth {depth}");
        while let Some(node) = self.stack.last() {
            if node.start_depth >= depth {
                debug!("Mark node {node:?} as ended at {}", idx + 1);
                let mut node = self.stack.pop().expect("last was Some, pop must succeed");
                append_final_block(
                    &mut node.buf,
                    self.current_block
                        .as_ref()
                        .ok_or(EngineError::MissingOpeningCharacter())?,
                    self.idx,
                    node.start_idx,
                    idx + 1,
                );
                finalize_node(&mut node.buf, node.ty);

                debug!("Committing node: {node:?}");
                self.output_queue.insert(
                    node.id,
                    Match {
                        span_start: node.start_idx - self.leading_padding_len,
                        bytes: node.buf,
                    },
                );
            } else {
                break;
            }
        }

        if self.stack.is_empty() {
            debug!("Outputting batch of nodes.");
            self.output_queue
                .output_to(self.sink)
                .map_err(|err| EngineError::SinkError(Box::new(err)))?;
        }

        Ok(())
    }
}

#[inline(always)]
fn append_block(dest: &mut Vec<u8>, src: &[u8], src_start: usize, read_start: usize) {
    if read_start >= src_start + src.len() {
        return;
    }

    let to_extend = if read_start > src_start {
        let in_block_start = read_start - src_start;
        &src[in_block_start..]
    } else {
        src
    };

    dest.extend(to_extend);
}

#[inline(always)]
fn append_final_block(dest: &mut Vec<u8>, src: &[u8], src_start: usize, read_start: usize, read_end: usize) {
    debug!("src_start: {src_start}, read_start: {read_start}, read_end: {read_end}");
    debug_assert!(read_end >= src_start, "block cannot end before it starts");
    let in_block_start = read_start.saturating_sub(src_start);
    let in_block_end = read_end - src_start;

    dest.extend(&src[in_block_start..in_block_end]);
}

#[inline(always)]
fn finalize_node(buf: &mut Vec<u8>, ty: MatchedNodeType) {
    debug!("Finalizing node");

    if ty == MatchedNodeType::Atomic {
        // Atomic nodes are finished when the next structural character is matched.
        // The buffer includes that character and all preceding whitespace.
        // We need to remove it before saving the result.
        if buf.len() <= 1 {
            // This should never happen in a valid JSON, but we also don't want to panic if the file is invalid.
            buf.truncate(0)
        } else {
            let mut i = buf.len() - 2;
            while is_json_whitespace(buf[i]) {
                i -= 1;
            }

            buf.truncate(i + 1);
        }
    }
}

impl Debug for PartialNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PartialNode")
            .field("start_idx", &self.start_idx)
            .field("start_depth", &self.start_depth)
            .field("ty", &self.ty)
            .field("buf", &str::from_utf8(&self.buf).unwrap_or("[invalid utf8]"))
            .finish()
    }
}
