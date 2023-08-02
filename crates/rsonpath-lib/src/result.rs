//! Result types that can be returned by a JSONPath query engine.
use crate::{depth::Depth, engine::error::EngineError};
use std::{convert::Infallible, fmt::Display, io, ops::Deref};

pub mod count;
pub mod empty;
pub mod index;
pub mod nodes;

/// Result of counting query matches.
pub type MatchCount = u64;

/// Representation of the starting index of a match.
pub type MatchIndex = usize;

/// Span of a match &ndash; its start and end index.
///
/// The end index is **inclusive**. For example, the value
/// `true` may have the span of (17, 21), meaning that
/// the first character, 't', occurs at index 17, and the last
/// character, `e` occurs at index 21.
pub struct MatchSpan {
    /// Starting index of the match.
    pub start_idx: MatchIndex,
    /// Last index of the match.
    pub end_idx: MatchIndex,
}

/// Full information of a query match &ndash; its span and the input bytes
/// in that span.
pub struct Match {
    /// JSON contents of the match.
    pub bytes: Vec<u8>,
    /// Span of the match.
    pub span: MatchSpan,
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
    /// may raise an [`io::error::Error`].
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

/// An observer that can build a [`QueryResult`] based on match and structural events coming from the execution engine.
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
