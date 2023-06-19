//! Result types that can be returned by a JSONPath query engine.
use crate::{classification::structural::BracketType, debug, engine::error::EngineError, input::Input};
use std::fmt::{self, Display};

/// Hint given by the engine to a [`QueryResultBuilder`] as to what type of node was matched.
///
/// Since the byte offset given by the engine is not stable and can fall anywhere between
/// the actual value's start and the preceding structural character, this helps a builder quickly
/// advance to the actual value and parse it.
///
/// This is non_exhaustive, since we may add more hints for performance reasons, and the implementations
/// are already forced to handle the [`Any`](`NodeTypeHint::Any`) variant. The behavior for "unknown"
/// hints should be the same as for [`Any`](`NodeTypeHint::Any`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NodeTypeHint {
    /// The value is any atom (number, string, boolean, null).
    Atomic,
    /// The value is an object or an array, and we know what type of bracket to look for.
    Complex(BracketType),
    /// The value is an object or an array, but we do not know which. The builder should seek for a '{' *or* '['.
    AnyComplex,
    /// The value can be any JSON value, we know nothing more.
    Any,
}

/// Result that can be built with some [`QueryResultBuilder`] and returned from a query run.
pub trait QueryResult: Default + Display + PartialEq {
    /// The associated type of the builder.
    type Builder<'i, I: Input + 'i>: QueryResultBuilder<'i, I, Self>;
}

/// A builder for a [`QueryResult`] with access to the underlying input.
pub trait QueryResultBuilder<'i, I: Input, R: QueryResult> {
    /// Create a new, empty builder, with access to the underlying JSON input.
    ///
    /// Implementations should make sure that the result produced from an empty builder
    /// is the same as the empty instance of that result (defined by the [`Default`] trait).
    /// In other words, for any `builder` of result type `R` it should hold that:
    ///
    /// ```ignore
    /// builder.new(input).finish() == R::default()
    /// ```
    fn new(input: &'i I) -> Self;

    /// Report a match of the query. The `index` is guaranteed to be between the first character
    /// of the matched value and the previous structural character.
    ///
    /// When the engine finds a match, it will usually occur at some structural character.
    /// It is guaranteed that `index` points to either:
    /// 1. the first character of the matched value; or
    /// 2. the colon or comma structural character directly preceding the matched value; or
    /// 3. a whitespace character before the matched value, such that the next non-whitespace
    /// character is the first character of the matched value.
    ///
    /// The builder should use the `index` and the provided `hint` to find the start of the
    /// actual value being reported. Note that it is always possible to do so without the hint
    /// (or, equivalently, when [`NodeTypeHint::Any`] is given), but the hint can improve performance.
    /// For example, when the hint is [`NodeTypeHint::Complex`] with the curly bracket type, the
    /// result builder can do a quick direct search for the next '{' character.
    ///
    /// ```json
    /// {
    ///   "match":       42
    ///   //     ^^^^^^^^^
    ///   // any of these characters can be reported for the query $.match
    /// }
    /// ```
    ///
    /// ```json
    /// {
    ///   "match": [42,     30]
    ///   //          ^^^^^^^
    ///   // any of these characters can be reported for the query $.match[1]
    /// }
    /// ```
    ///
    /// # Errors
    /// This function may access the input, which can raise an [`EngineError::InputError`].
    /// More errors can occur if the input JSON is malformed (for example the document abruptly ends),
    /// but they are not guaranteed to be detected or reported.
    fn report(&mut self, index: usize, hint: NodeTypeHint) -> Result<(), EngineError>;

    /// Finish building the result and return it.
    fn finish(self) -> R;
}

/// Result informing on the number of values matching the executed query.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountResult {
    count: u64,
}

impl CountResult {
    /// Number of values matched by the executed query.
    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> u64 {
        self.count
    }
}

impl Display for CountResult {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.count)
    }
}

impl QueryResult for CountResult {
    type Builder<'i, I: Input + 'i> = CountResultBuilder;
}

/// The [`QueryResultBuilder`] for [`CountResult`].
#[must_use]
pub struct CountResultBuilder {
    count: u64,
}

impl<'i, I: Input> QueryResultBuilder<'i, I, CountResult> for CountResultBuilder {
    #[inline(always)]
    fn new(_input: &'i I) -> Self {
        Self { count: 0 }
    }

    #[inline(always)]
    fn report(&mut self, _item: usize, _hint: NodeTypeHint) -> Result<(), EngineError> {
        debug!("Reporting result: {_item}");
        self.count += 1;

        Ok(())
    }

    #[must_use]
    #[inline(always)]
    fn finish(self) -> CountResult {
        CountResult { count: self.count }
    }
}

/// Query result containing all indices of colons that constitute a match.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexResult {
    indices: Vec<usize>,
}

impl IndexResult {
    /// Get indices of colons constituting matches of the query.
    #[must_use]
    #[inline(always)]
    pub fn get(&self) -> &[usize] {
        &self.indices
    }
}

impl From<IndexResult> for Vec<usize> {
    #[inline(always)]
    fn from(result: IndexResult) -> Self {
        result.indices
    }
}

impl Display for IndexResult {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.indices)
    }
}

impl QueryResult for IndexResult {
    type Builder<'i, I: Input + 'i> = IndexResultBuilder<'i, I>;
}

/// The [`QueryResultBuilder`] for [`IndexResult`].
#[must_use]
pub struct IndexResultBuilder<'i, I> {
    input: &'i I,
    indices: Vec<usize>,
}

impl<'i, I: Input> QueryResultBuilder<'i, I, IndexResult> for IndexResultBuilder<'i, I> {
    #[inline(always)]
    fn new(input: &'i I) -> Self {
        Self { input, indices: vec![] }
    }

    #[inline(always)]
    fn report(&mut self, item: usize, hint: NodeTypeHint) -> Result<(), EngineError> {
        debug!("Reporting result: {item} with hint {hint:?}");

        let index = match hint {
            NodeTypeHint::Complex(BracketType::Curly) => self.input.seek_forward(item, [b'{'])?,
            NodeTypeHint::Complex(BracketType::Square) => self.input.seek_forward(item, [b'['])?,
            NodeTypeHint::AnyComplex | NodeTypeHint::Any | NodeTypeHint::Atomic => {
                self.input.seek_non_whitespace_forward(item)?
            }
        }
        .map(|x| x.0);

        match index {
            Some(idx) => {
                self.indices.push(idx);
                Ok(())
            }
            None => Err(EngineError::MissingItem()),
        }
    }

    #[must_use]
    #[inline(always)]
    fn finish(self) -> IndexResult {
        IndexResult { indices: self.indices }
    }
}
