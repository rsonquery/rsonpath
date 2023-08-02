//! Result types that can be returned by a JSONPath query engine.
use crate::{depth::Depth, engine::error::EngineError};
use std::{convert::Infallible, fmt::Display, io, ops::Deref};

pub mod count;
pub mod empty;
pub mod index;
pub mod nodes;

pub type MatchCount = u64;
pub type MatchIndex = usize;

pub struct MatchSpan {
    pub start_idx: MatchIndex,
    pub end_idx: MatchIndex,
}

pub struct Match {
    pub bytes: Vec<u8>,
    pub span: MatchSpan,
}

impl Display for Match {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = String::from_utf8_lossy(&self.bytes);
        write!(f, "{display}")
    }
}

pub trait Sink<D> {
    type Error: std::error::Error + Send + Sync + 'static;

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

/// Result that can be returned from a query run.
pub trait QueryResult: Default + Display + PartialEq {}

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
    fn record_match(&self, idx: usize, depth: Depth, ty: MatchedNodeType) -> Result<(), EngineError>;

    /// Record a structural character signifying the end of a value at a given `idx`
    /// and with given `depth`.
    fn record_value_terminator(&self, idx: usize, depth: Depth) -> Result<(), EngineError>;
}
