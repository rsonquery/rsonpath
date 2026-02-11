//! Result types that can be returned by a JSONPath query engine.
use crate::{depth::Depth, engine::error::EngineError};
use std::{convert::Infallible, fmt::Display, io, ops::Deref};

pub mod approx_span;
pub mod count;
pub mod empty;
pub mod index;
pub mod nodes;
mod output_queue;

/// Result of counting query matches.
pub type MatchCount = u64;

/// Representation of the starting index of a match.
pub type MatchIndex = usize;

/// Span of a match &ndash; its start and end index.
///
/// The end index is **exclusive**. For example, the value
/// `true` may have the span of `(17, 21)`, meaning that
/// the first character, 't', occurs at index 17, and the last
/// character, `e` occurs at index 20.
///
/// This is in line with what a `[17..21]` slice in Rust represents.
#[derive(Clone, Copy)]
pub struct MatchSpan {
    /// Starting index of the match.
    start_idx: MatchIndex,
    /// Length of the match
    len: usize,
}

/// Full information of a query match &ndash; its span and the input bytes
/// in that span.
pub struct Match {
    /// JSON contents of the match.
    bytes: Vec<u8>,
    /// Starting index of the match.
    span_start: usize,
}

impl MatchSpan {
    pub(crate) fn from_indices(start_idx: usize, end_idx: usize) -> Self {
        assert!(
            start_idx <= end_idx,
            "start of span {start_idx} is greater than end {end_idx}"
        );
        Self {
            start_idx,
            len: end_idx - start_idx,
        }
    }

    /// Returns the starting index of the match.
    #[inline(always)]
    #[must_use]
    pub fn start_idx(&self) -> usize {
        self.start_idx
    }

    /// Returns the end index of the match.
    #[inline(always)]
    #[must_use]
    pub fn end_idx(&self) -> usize {
        self.start_idx + self.len
    }

    /// Returns the length of the match.
    #[inline(always)]
    #[must_use]
    #[allow(
        clippy::len_without_is_empty,
        reason = "is_empty makes no sense for a match (matches are non-empty)"
    )]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Match {
    pub(crate) fn from_start_and_bytes(span_start: usize, bytes: Vec<u8>) -> Self {
        Self { bytes, span_start }
    }

    /// Returns the JSON contents of the match.
    #[inline(always)]
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Consumes the [`Match`] to take ownership of the underlying JSON bytes.
    #[inline(always)]
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Returns the span of this match in the JSON document:
    /// its starting and ending byte indices.
    #[inline(always)]
    #[must_use]
    pub fn span(&self) -> MatchSpan {
        MatchSpan {
            start_idx: self.span_start,
            len: self.bytes.len(),
        }
    }
}

impl Display for MatchSpan {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start_idx, self.end_idx())
    }
}

impl Display for Match {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = String::from_utf8_lossy(&self.bytes);
        write!(f, "{display}")
    }
}

/// Output sink consuming matches of the type `D`.
pub trait Sink<D> {
    /// Error type that can be raised when consuming a match.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Consume a single match of type `D`.
    ///
    /// # Errors
    /// An error depending on the implementor can be raised.
    /// For example, implementations using an underlying [`io::Write`]
    /// may raise an [`io::Error`].
    fn add_match(&mut self, data: D) -> Result<(), Self::Error>;
}

impl<D> Sink<D> for Vec<D> {
    type Error = Infallible;

    #[inline(always)]
    fn add_match(&mut self, data: D) -> Result<(), Infallible> {
        self.push(data);
        Ok(())
    }
}

/// Empty sink that consumes all matches into the void.
pub struct NullSink;

impl<D> Sink<D> for NullSink {
    type Error = Infallible;

    #[inline(always)]
    fn add_match(&mut self, _data: D) -> Result<(), Infallible> {
        Ok(())
    }
}

/// Thin wrapper over an [`io::Write`] to provide a [`Sink`] impl.
pub struct MatchWriter<W>(W);

impl<W> From<W> for MatchWriter<W>
where
    W: io::Write,
{
    #[inline(always)]
    fn from(value: W) -> Self {
        Self(value)
    }
}

impl<D, W> Sink<D> for MatchWriter<W>
where
    D: Display,
    W: io::Write,
{
    type Error = io::Error;

    #[inline(always)]
    fn add_match(&mut self, data: D) -> Result<(), io::Error> {
        writeln!(self.0, "{data}")
    }
}

/// Type of a value being reported to a [`Recorder`].
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MatchedNodeType {
    /// JSON string, number, or literal value.
    Atomic,
    /// JSON object or array.
    Complex,
}

/// Base trait of any recorder, one that can react to a block of input being processed.
pub trait InputRecorder<B: Deref<Target = [u8]>> {
    /// Record that all processing of a block was started
    ///
    /// The recorder may assume that only matches or terminators with indices pointing to
    /// the block that was last recorded as started are reported.
    fn record_block_start(&self, new_block: B);
}

/// An observer that can determine the query result
/// based on match and structural events coming from the execution engine.
pub trait Recorder<B: Deref<Target = [u8]>>: InputRecorder<B> {
    /// Record a match of the query at a given `depth`.
    /// The `idx` is guaranteed to be the first character of the matched value.
    ///
    /// The type MUST accurately describe the value being matched.
    ///
    /// # Errors
    /// An error can be raised if an output write occurs and the underlying [`Sink`] implementation
    /// returns an error ([`EngineError::SinkError`]).
    fn record_match(&self, idx: usize, depth: Depth, ty: MatchedNodeType) -> Result<(), EngineError>;

    /// Record a structural character signifying the end of a value at a given `idx`
    /// and with given `depth`.
    ///
    /// # Errors
    /// An error can be raised if an output write occurs and the underlying [`Sink`] implementation
    /// returns an error ([`EngineError::SinkError`]), or if the terminator was not expected
    /// ([`EngineError::MissingOpeningCharacter`]).
    fn record_value_terminator(&self, idx: usize, depth: Depth) -> Result<(), EngineError>;
}
