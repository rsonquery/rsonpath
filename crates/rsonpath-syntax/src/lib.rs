//! Complete, fast, and fully spec-compliant JSONPath query parser.
//!
//! The crate exposes the [`JsonPathQuery`] type and the [`parse`](`crate::parse`)
//! function that converts a query string into the AST representation. The parsing
//! complies with the proposed [JSONPath RFC specification](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html).
//!
//! A JSONPath query is a sequence of **segments**, each containing one or more
//! **selectors**. There are two types of segments:
//! - **child** ([`Segment::Child`]), and
//! - **descendant** ([`Segment::Descendant`]);
//!
//! and five different types of selectors:
//! - **name** ([`Selector::Name`]),
//! - **wildcard** ([`Selector::Wildcard`]),
//! - **index** ([`Selector::Index`]),
//! - **slice** ([`Selector::Slice`]),
//! - and **filter** ([`Selector::Filter`]).
//!
//! Descriptions of each segment and selector can be found in the documentation of the
//! relevant type in this crate, while the formal grammar is described in the RFC.
//!
//! ## State of the crate
//!
//! This is an in-development version that does not yet support functions in filter
//! expressions.
//! However, all other constructs are fully supported, tested, and fuzzed. The planned roadmap is:
//! - [x] support slices
//! - [x] support filters (without functions)
//! - [ ] support functions (including type check)
//! - [ ] polish the API
//! - [ ] 1.0.0 stable release
//!
//! ## Examples
//! To create a query from a query string:
//! ```
//! use rsonpath_syntax::prelude::*;
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let query_string = "$..phoneNumbers[*].number";
//! let query = rsonpath_syntax::parse(query_string)?;
//!
//! // Query structure is a linear sequence of segments:
//! // Descendant '..phoneNumbers', child wildcard, child 'number'.
//! assert_eq!(query.segments().len(), 3);
//! assert_eq!(
//!   query.segments()[0],
//!   Segment::Descendant(
//!     Selectors::one(
//!       Selector::Name(
//!         JsonString::new("phoneNumbers")
//! ))));
//! assert_eq!(
//!   query.segments()[1],
//!   Segment::Child(
//!     Selectors::one(
//!       Selector::Wildcard
//! )));
//! assert_eq!(
//!   query.segments()[2],
//!   Segment::Child(
//!     Selectors::one(
//!       Selector::Name(
//!         JsonString::new("number")
//! ))));
//!
//! // Queries stringify to a canonical representation.
//! assert_eq!(query.to_string(), "$..['phoneNumbers'][*]['number']");
//! # Ok(())
//! # }
//! ```
//!
//! Constructing queries programmatically is more ergonomic with the provided builder interface.
//! For example, to construct the same query as above:
//!
//! ```rust
//! use rsonpath_syntax::builder::JsonPathQueryBuilder;
//!
//! let mut query_builder = JsonPathQueryBuilder::new();
//! query_builder
//!   .descendant_name("phoneNumbers")
//!   .child_wildcard()
//!   .child_name("number");
//! let query = query_builder.into_query();
//!
//! assert_eq!(query.to_string(), "$..['phoneNumbers'][*]['number']");
//! ```
//!
//! ## Crate features
//!
//! There is one optional features:
//! - `color`, which enables a dependency on the [`owo_colors` crate](https://docs.rs/owo-colors/latest/owo_colors/)
//!   to provide colorful [`Display`] representations of [`ParseError`](error::ParseError);
//!   see [`ParseError::colored`](error::ParseError::colored).

#![forbid(unsafe_code)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/V0ldek/rsonpath/main/img/rsonquery-logo.svg")]
// Documentation lints, enabled only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(
        missing_docs,
        clippy::cargo_common_metadata,
        clippy::missing_errors_doc,
        clippy::missing_panics_doc,
        clippy::too_long_first_doc_paragraph
    )
)]
#![cfg_attr(not(debug_assertions), warn(rustdoc::missing_crate_level_docs))]
// Panic-free lints (disabled for tests).
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
// IO hygiene, only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(clippy::print_stderr, clippy::print_stdout, clippy::todo)
)]
// Docs.rs config.
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod builder;
pub mod error;
pub mod num;
mod parser;
pub mod prelude;
pub mod str;

/// All characters that are valid whitespace within a JSONPath query.
pub(crate) const JSONPATH_WHITESPACE: [char; 4] = [' ', '\t', '\n', '\r'];
/// All ASCII bytes that are valid whitespace within a JSONPath query.
pub(crate) const JSONPATH_WHITESPACE_BYTES: [u8; 4] = [0x20, 0x09, 0x0A, 0x0D];

use crate::builder::{
    EmptyComparisonExprBuilder, EmptyLogicalExprBuilder, JsonPathQueryBuilder, SingularJsonPathQueryBuilder,
    SliceBuilder,
};
use std::{
    fmt::{self, Display},
    ops::Deref,
};

/// JSONPath query parser.
#[derive(Debug, Clone, Default)]
pub struct Parser {
    options: ParserOptions,
}

/// Configurable builder for a [`Parser`] instance.
#[derive(Debug, Clone, Default)]
pub struct ParserBuilder {
    options: ParserOptions,
}

#[derive(Debug, Clone)]
struct ParserOptions {
    recursion_limit: Option<usize>,
    relaxed_whitespace: bool,
}

impl ParserBuilder {
    /// Create a new instance of the builder with the default settings.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            options: ParserOptions::default(),
        }
    }

    /// Override the default recursion limit in a query.
    /// Defaults to [Parser::RECURSION_LIMIT_DEFAULT].
    ///
    /// JSONPath queries are inherently recursive, since
    /// - [`LogicalExpr`] can be an arbitrarily deep tree of AND/OR operators;
    /// - the [`TestExpr`] in a filter can test arbitrary nested JSONPath queries.
    ///
    /// Our parser implementation is recursive, so an excessively nested query could overflow the stack.
    ///
    /// The limit can be relaxed here, or removed entirely by passing [`None`].
    ///
    /// ## Examples
    /// ```
    /// # use rsonpath_syntax::{JsonPathQuery, Parser, ParserBuilder};
    /// let default_parser = ParserBuilder::new().build();
    /// let no_nesting_parser = ParserBuilder::new()
    ///     .set_recursion_limit(Some(1))
    ///     .build();
    ///
    /// let query = "$[?@[?@]]";
    /// assert!(default_parser.parse(query).is_ok());
    /// assert!(no_nesting_parser.parse(query).is_err());
    /// ```
    #[inline]
    pub fn set_recursion_limit(&mut self, value: Option<usize>) -> &mut Self {
        self.options.recursion_limit = value;
        self
    }

    /// Control whether leading and trailing whitespace is allowed in a query.
    /// Defaults to false.
    ///
    /// The [RFC](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html) grammar
    /// makes leading and trailing whitespace disallowed. The [`Parser`] defaults to this strict handling,
    /// but can be relaxed with this setting.
    ///
    /// ## Examples
    /// ```
    /// # use rsonpath_syntax::{JsonPathQuery, Parser, ParserBuilder};
    /// let default_parser = ParserBuilder::new().build();
    /// let relaxed_parser = ParserBuilder::new()
    ///     .allow_surrounding_whitespace(true)
    ///     .build();
    ///
    /// let query = "  $.leading_whitespace";
    /// assert!(default_parser.parse(query).is_err());
    /// assert!(relaxed_parser.parse(query).is_ok());
    /// ```
    #[inline]
    pub fn allow_surrounding_whitespace(&mut self, value: bool) -> &mut Self {
        self.options.relaxed_whitespace = value;
        self
    }

    /// Build a new instance of a [`Parser`].
    #[inline]
    #[must_use]
    pub fn build(&self) -> Parser {
        Parser {
            options: self.options.clone(),
        }
    }
}

impl ParserOptions {
    fn is_leading_whitespace_allowed(&self) -> bool {
        self.relaxed_whitespace
    }

    fn is_trailing_whitespace_allowed(&self) -> bool {
        self.relaxed_whitespace
    }
}

impl Default for ParserOptions {
    #[inline(always)]
    fn default() -> Self {
        Self {
            recursion_limit: Some(Parser::RECURSION_LIMIT_DEFAULT),
            relaxed_whitespace: false,
        }
    }
}

impl From<ParserBuilder> for Parser {
    #[inline(always)]
    fn from(value: ParserBuilder) -> Self {
        Self { options: value.options }
    }
}

/// Convenience alias for [`Result`](std::result::Result) values returned by this crate.
pub type Result<T> = std::result::Result<T, error::ParseError>;

/// Parse a JSONPath query string using default [`Parser`] configuration.
///
/// ## Errors
/// Fails if the string does not represent a valid JSONPath query
/// as governed by the [JSONPath RFC specification](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html).
///
/// Note that leading and trailing whitespace is explicitly disallowed by the spec.
/// This can be relaxed with a custom [`Parser`] configured with [`ParserBuilder::allow_surrounding_whitespace`].
///
/// # Examples
/// ```
/// # use rsonpath_syntax::parse;
/// let x = "  $.a  ";
/// let err = rsonpath_syntax::parse(x).expect_err("should fail");
/// assert_eq!(err.to_string(),
/// "error: query starting with whitespace
///
///     $.a  
///   ^^ leading whitespace is disallowed
///   (bytes 0-1)
///
///
///error: query ending with whitespace
///
///     $.a  
///        ^^ trailing whitespace is disallowed
///   (bytes 5-6)
///
///
///suggestion: did you mean `$.a` ?
///");
/// ```
#[inline]
pub fn parse(str: &str) -> Result<JsonPathQuery> {
    Parser::default().parse(str)
}

impl Parser {
    /// Default limit on the nesting level of a query.
    ///
    /// This can be overridden by [`ParserBuilder::set_recursion_limit`].
    pub const RECURSION_LIMIT_DEFAULT: usize = 128;

    /// Parse a JSONPath query string.
    ///
    /// ## Errors
    /// Fails if the string does not represent a valid JSONPath query
    /// as governed by the [JSONPath RFC specification](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html).
    ///
    /// Note that leading and trailing whitespace is explicitly disallowed by the spec.
    /// The parser defaults to this strict behavior unless configured with
    /// [`ParserBuilder::allow_surrounding_whitespace`].
    ///
    /// There is a limit on the complexity of the query measured as the depth of nested filter expressions.
    /// This limit defaults to [`RECURSION_LIMIT_DEFAULT`](Self::RECURSION_LIMIT_DEFAULT) and can be overridden
    /// with [`ParserBuilder::set_recursion_limit`].
    #[inline]
    pub fn parse(&self, str: &str) -> Result<JsonPathQuery> {
        crate::parser::parse_with_options(str, &self.options)
    }
}

/// JSONPath query segment.
///
/// Every query is a sequence of zero or more of segments,
/// each applying one or more selectors to a node and passing it along to the
/// subsequent segments.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Segment {
    /// A child segment contains a sequence of selectors,
    /// each of which selects zero or more children of a node.
    Child(Selectors),
    /// A child segment contains a sequence of selectors,
    /// each of which selects zero or more descendants of a node.
    Descendant(Selectors),
}

/// Collection of one or more [`Selector`] instances.
///
/// Guaranteed to be non-empty.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Selectors {
    inner: Vec<Selector>,
}

/// Each [`Segment`] defines one or more selectors.
/// A selector produces one or more children/descendants of the node it is applied to.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Selector {
    /// A name selector selects at most one object member value under the key equal to the
    /// selector's [`JsonString`](str::JsonString).
    Name(str::JsonString),
    /// A wildcard selector selects the nodes of all children of an object or array.
    Wildcard,
    /// An index selector matches at most one array element value,
    /// depending on the selector's [`Index`].
    Index(Index),
    /// A slice selector matches elements from arrays starting at a given index,
    /// ending at a given index, and incrementing with a specified step.
    Slice(Slice),
    /// A filter selector matches members/elements which satisfy the given
    /// [`LogicalExpr`].
    Filter(LogicalExpr),
}

impl From<str::JsonString> for Selector {
    #[inline]
    fn from(value: str::JsonString) -> Self {
        Self::Name(value)
    }
}

impl From<Index> for Selector {
    #[inline]
    fn from(value: Index) -> Self {
        Self::Index(value)
    }
}

impl From<Slice> for Selector {
    #[inline]
    fn from(value: Slice) -> Self {
        Self::Slice(value)
    }
}

impl From<LogicalExpr> for Selector {
    #[inline]
    fn from(value: LogicalExpr) -> Self {
        Self::Filter(value)
    }
}

/// Directional index into a JSON array.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Index {
    /// Zero-based index from the start of the array.
    FromStart(num::JsonUInt),
    /// Index from the end of the array.
    ///
    /// `-1` is the last element, `-2` is the second last, etc.
    FromEnd(num::JsonNonZeroUInt),
}

impl<N: Into<num::JsonInt>> From<N> for Index {
    #[inline]
    fn from(value: N) -> Self {
        let value = value.into();
        if value.as_i64() >= 0 {
            Self::FromStart(value.abs())
        } else {
            Self::FromEnd(value.abs().try_into().expect("checked for zero already"))
        }
    }
}

/// Directional step offset within a JSON array.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Step {
    /// Step forward by a given offset amount.
    Forward(num::JsonUInt),
    /// Step backward by a given offset amount.
    Backward(num::JsonNonZeroUInt),
}

impl From<num::JsonInt> for Step {
    #[inline]
    fn from(value: num::JsonInt) -> Self {
        if value.as_i64() >= 0 {
            Self::Forward(value.abs())
        } else {
            Self::Backward(value.abs().try_into().expect("checked for zero already"))
        }
    }
}

/// Slice selector defining the start and end bounds, as well as the step value and direction.
///
/// The start index is inclusive defaults to `Index::FromStart(0)`.
///
/// The end index is exclusive and optional.
/// If `None`, the end of the slice depends on the step direction:
/// - if going forward, the end is `len` of the array;
/// - if going backward, the end is 0.
///
/// The step defaults to `Step::Forward(1)`. Note that `Step::Forward(0)` is a valid
/// value and is specified to result in an empty slice, regardless of `start` and `end`.
///
/// # Examples
/// ```
/// # use rsonpath_syntax::{Slice, Index, Step, num::JsonUInt};
/// let slice = Slice::default();
/// assert_eq!(slice.start(), Index::FromStart(JsonUInt::ZERO));
/// assert_eq!(slice.end(), None);
/// assert_eq!(slice.step(), Step::Forward(JsonUInt::ONE));
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Slice {
    start: Index,
    end: Option<Index>,
    step: Step,
}

impl Slice {
    pub(crate) const DEFAULT_START_FORWARDS: Index = Index::FromStart(num::JsonUInt::ZERO);
    /// This is not const because the required NonZeroU64::MIN is from Rust 1.70.
    #[inline(always)]
    pub(crate) fn default_start_backwards() -> Index {
        Index::FromEnd(1.try_into().expect("const 1 is nonzero"))
    }
    pub(crate) const DEFAULT_STEP: Step = Step::Forward(num::JsonUInt::ONE);

    /// Start building a new [`Slice`].
    #[inline(always)]
    #[must_use]
    pub fn build() -> SliceBuilder {
        SliceBuilder::new()
    }

    /// Create a new [`Slice`] from given bounds and step.
    #[inline(always)]
    #[must_use]
    pub fn new(start: Index, end: Option<Index>, step: Step) -> Self {
        Self { start, end, step }
    }

    /// Get the start index of the [`Slice`].
    #[inline(always)]
    #[must_use]
    pub fn start(&self) -> Index {
        self.start
    }

    /// Get the end index of the [`Slice`].
    #[inline(always)]
    #[must_use]
    pub fn end(&self) -> Option<Index> {
        self.end
    }

    /// Get the step of the [`Slice`].
    #[inline(always)]
    #[must_use]
    pub fn step(&self) -> Step {
        self.step
    }
}

impl Default for Slice {
    #[inline]
    fn default() -> Self {
        Self {
            start: Index::FromStart(0.into()),
            end: None,
            step: Step::Forward(1.into()),
        }
    }
}

/// JSON literal value available in comparison expressions of a filter selector.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Literal {
    /// [`JsonString`](str::JsonString) literal.
    String(str::JsonString),
    /// [`JsonNumber`](num::JsonNumber) literal &ndash;
    /// an integer or a floating point value.
    Number(num::JsonNumber),
    /// Boolean JSON value &ndash; `true`` or `false`.
    Bool(bool),
    /// The `null` JSON literal value.
    Null,
}

impl<S> From<S> for Literal
where
    S: Into<str::JsonString>,
{
    #[inline(always)]
    fn from(value: S) -> Self {
        Self::String(value.into())
    }
}

impl From<num::JsonInt> for Literal {
    #[inline(always)]
    fn from(value: num::JsonInt) -> Self {
        Self::Number(num::JsonNumber::Int(value))
    }
}

impl From<num::JsonFloat> for Literal {
    #[inline(always)]
    fn from(value: num::JsonFloat) -> Self {
        Self::Number(num::JsonNumber::Float(value))
    }
}

impl From<num::JsonNumber> for Literal {
    #[inline(always)]
    fn from(value: num::JsonNumber) -> Self {
        Self::Number(value)
    }
}

impl From<bool> for Literal {
    #[inline(always)]
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

/// Logical expression used in a [`Filter`](Selector::Filter) selector.
///
/// Expressions form a tree, where [`Comparison`](LogicalExpr::Comparison)
/// and [`Test`](LogicalExpr::Test) expressions can be leaves, and boolean combinators
/// (OR, AND, NOT) store their children as [`Boxes`](Box).
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LogicalExpr {
    /// Logical disjunction of two child expressions.
    Or(LogicalExprNode, LogicalExprNode),
    /// Logical conjunction of two child expressions.
    And(LogicalExprNode, LogicalExprNode),
    /// Logical negation of a child expression.
    Not(LogicalExprNode),
    /// Comparison expression &ndash; compare single values determined
    /// by query or a literal constant.
    Comparison(ComparisonExpr),
    /// Existence test &ndash; query and see if any matched nodes exist.
    Test(TestExpr),
}

impl LogicalExpr {
    /// Start building a new [`LogicalExpr`].
    #[inline(always)]
    #[must_use]
    pub fn build() -> EmptyLogicalExprBuilder {
        EmptyLogicalExprBuilder
    }

    fn precedence(&self) -> usize {
        match self {
            Self::Or(_, _) => 2,
            Self::And(_, _) => 3,
            Self::Comparison(_) => 4,
            Self::Not(_) => 5,
            Self::Test(_) => 10,
        }
    }
}

type LogicalExprNode = Box<LogicalExpr>;

/// Existence test based on a relative or absolute [`JsonPathQuery`].
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TestExpr {
    /// Relative test &ndash; query from the selected node.
    Relative(JsonPathQuery),
    /// Absolute test &ndash; query from the document root.
    Absolute(JsonPathQuery),
}

/// Comparison based on two singular values and a comparison operator.
///
/// # Examples
/// ```rust
/// # use rsonpath_syntax::{ComparisonExpr, Comparable, ComparisonOp, Literal, SingularJsonPathQuery};
/// let lhs = Comparable::from(Literal::from("abc"));
/// let rhs = Comparable::RelativeSingularQuery(
///     SingularJsonPathQuery::from_iter(vec![])
/// );
/// let comparison = ComparisonExpr::from_parts(
///     lhs.clone(),
///     ComparisonOp::EqualTo,
///     rhs.clone());
///
/// assert_eq!(&lhs, comparison.lhs());
/// assert_eq!(ComparisonOp::EqualTo, comparison.op());
/// assert_eq!(&rhs, comparison.rhs());
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ComparisonExpr {
    lhs: Comparable,
    op: ComparisonOp,
    rhs: Comparable,
}

impl ComparisonExpr {
    /// Start building a new [`ComparisonExpr`].
    #[inline(always)]
    #[must_use]
    pub fn build() -> EmptyComparisonExprBuilder {
        EmptyComparisonExprBuilder
    }

    /// Get the comparable left-hand side of the comparison operation.
    #[inline]
    #[must_use]
    pub fn lhs(&self) -> &Comparable {
        &self.lhs
    }

    /// Get the comparison operator.
    #[inline]
    #[must_use]
    pub fn op(&self) -> ComparisonOp {
        self.op
    }

    /// Get the comparable right-hand side of the comparison operation.
    #[inline]
    #[must_use]
    pub fn rhs(&self) -> &Comparable {
        &self.rhs
    }

    /// Construct a [`ComparisonExpr`] from its constituent parts.
    #[inline]
    #[must_use]
    pub fn from_parts(lhs: Comparable, op: ComparisonOp, rhs: Comparable) -> Self {
        Self { lhs, op, rhs }
    }
}

/// Comparison operator usable in a [`ComparisonExpr`].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ComparisonOp {
    /// Compares two values for equality; `==`
    EqualTo,
    /// Compares two values for non-equality; `!=`
    NotEqualTo,
    /// Compares whether the lhs is smaller or equal to rhs; '<='
    LesserOrEqualTo,
    /// Compares whether the lhs is bigger or equal to rhs; '>='
    GreaterOrEqualTo,
    /// Compares whether the lhs is smaller than rhs; '<'
    LessThan,
    /// Compares whether the lhs is bigger than rhs; '>'
    GreaterThan,
}

/// One of the sides of a [`ComparisonExpr`], either a constant literal or a singular JSONPath query.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Comparable {
    /// Constant [`Literal`] value.
    Literal(Literal),
    /// Single value queried from the current node.
    RelativeSingularQuery(SingularJsonPathQuery),
    /// Single value queried from the JSON root.
    AbsoluteSingularQuery(SingularJsonPathQuery),
}

impl From<Literal> for Comparable {
    #[inline(always)]
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

/// Singular JSONPath query.
///
/// A singular JSONPath query returns at most one value, and can be used in
/// [`ComparisonExprs`](ComparisonExpr) as any of the comparison sides.
///
/// This is guaranteed syntactically &ndash; only child name and index selectors are allowed
/// in a [`SingularJsonPathQuery`], which naturally matches only the precise specified path,
/// if it exists.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SingularJsonPathQuery {
    segments: Vec<SingularSegment>,
}

impl SingularJsonPathQuery {
    /// Start building a new [`SingularJsonPathQuery`].
    #[inline(always)]
    #[must_use]
    pub fn build() -> SingularJsonPathQueryBuilder {
        SingularJsonPathQueryBuilder::new()
    }

    /// Iterate over the [`SingularSegments`](SingularSegment) of this query.
    #[inline]
    pub fn segments(&self) -> impl Iterator<Item = &'_ SingularSegment> {
        self.segments.iter()
    }
}

/// Segment allowed in a [`SingularJsonPathQuery`].
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SingularSegment {
    /// Child name selector. Equivalent of [`Selector::Name`].
    Name(str::JsonString),
    /// Child index selector. Equivalent of [`Selector::Index`].
    Index(Index),
}

impl FromIterator<SingularSegment> for SingularJsonPathQuery {
    #[inline]
    fn from_iter<T: IntoIterator<Item = SingularSegment>>(iter: T) -> Self {
        Self {
            segments: iter.into_iter().collect(),
        }
    }
}

impl From<SingularSegment> for Segment {
    #[inline]
    fn from(value: SingularSegment) -> Self {
        match value {
            SingularSegment::Name(n) => Self::Child(Selectors::one(Selector::Name(n))),
            SingularSegment::Index(i) => Self::Child(Selectors::one(Selector::Index(i))),
        }
    }
}

/// JSONPath query structure represented by a sequence of [`Segments`](Segment).
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct JsonPathQuery {
    segments: Vec<Segment>,
}

impl FromIterator<Segment> for JsonPathQuery {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Segment>>(iter: T) -> Self {
        Self {
            segments: iter.into_iter().collect(),
        }
    }
}

impl JsonPathQuery {
    /// Start building a new [`JsonPathQuery`].
    #[inline(always)]
    #[must_use]
    pub fn build() -> JsonPathQueryBuilder {
        JsonPathQueryBuilder::new()
    }

    fn try_to_singular(self) -> std::result::Result<SingularJsonPathQuery, Self> {
        if self.segments.iter().all(Segment::is_singular) {
            let mut singular_segments = Vec::with_capacity(self.segments.len());
            for segment in self.segments {
                singular_segments.push(segment.into_singular());
            }
            Ok(SingularJsonPathQuery {
                segments: singular_segments,
            })
        } else {
            Err(self)
        }
    }
}

impl JsonPathQuery {
    /// Returns all [`Segments`](Segment) of the query as a slice.
    #[inline(always)]
    #[must_use]
    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }
}

impl Segment {
    /// Returns all [`Selector`] instances of the segment.
    #[inline(always)]
    #[must_use]
    pub fn selectors(&self) -> &Selectors {
        match self {
            Self::Child(s) | Self::Descendant(s) => s,
        }
    }

    /// Check if this is a child segment.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selectors, Segment, Selector};
    /// let segment = Segment::Child(Selectors::one(Selector::Wildcard));
    /// assert!(segment.is_child());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn is_child(&self) -> bool {
        matches!(self, Self::Child(_))
    }

    /// Check if this is a descendant segment.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selectors, Segment, Selector};
    /// let segment = Segment::Descendant(Selectors::one(Selector::Wildcard));
    /// assert!(segment.is_descendant());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn is_descendant(&self) -> bool {
        matches!(self, Self::Descendant(_))
    }

    fn is_singular(&self) -> bool {
        match self {
            Self::Child(s) => s.len() == 1 && s.first().is_singular(),
            Self::Descendant(_) => false,
        }
    }

    fn into_singular(self) -> SingularSegment {
        assert!(self.is_singular(), "forcing a non-singular segment, this is a bug");
        match self {
            Self::Child(mut s) => match s.inner.drain(..).next().expect("is_singular") {
                Selector::Name(n) => SingularSegment::Name(n),
                Selector::Index(i) => SingularSegment::Index(i),
                _ => unreachable!(),
            },
            Self::Descendant(_) => unreachable!(),
        }
    }
}

impl Selectors {
    /// Create a singleton [`Selectors`] instance.
    #[inline(always)]
    #[must_use]
    pub fn one(selector: Selector) -> Self {
        Self { inner: vec![selector] }
    }

    /// Create a [`Selectors`] instance taking ownership of the `vec`.
    ///
    /// ## Panics
    /// If the `vec` is empty.
    ///
    /// ```should_panic
    /// # use rsonpath_syntax::Selectors;
    /// Selectors::many(vec![]);
    /// ```
    #[inline]
    #[must_use]
    pub fn many(vec: Vec<Selector>) -> Self {
        assert!(!vec.is_empty(), "cannot create an empty Selectors collection");
        Self { inner: vec }
    }

    /// Get a reference to the first [`Selector`] in the collection.
    #[inline]
    #[must_use]
    pub fn first(&self) -> &Selector {
        &self.inner[0]
    }

    /// Get the selectors as a non-empty slice.
    #[inline]
    #[must_use]
    pub fn as_slice(&self) -> &[Selector] {
        // Deref magic.
        self
    }
}

impl Selector {
    /// Check if this is a name selector.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selector, str::JsonString};
    /// let selector = Selector::Name(JsonString::new("abc"));
    /// assert!(selector.is_name());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_name(&self) -> bool {
        matches!(self, Self::Name(_))
    }

    /// Check if this is a wildcard selector.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::Selector;
    /// let selector = Selector::Wildcard;
    /// assert!(selector.is_wildcard());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_wildcard(&self) -> bool {
        matches!(self, Self::Wildcard)
    }

    /// Check if this is an index selector.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selector, Index};
    /// let selector = Selector::Index(Index::FromStart(0.into()));
    /// assert!(selector.is_index());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_index(&self) -> bool {
        matches!(self, Self::Index(_))
    }

    /// Check if this is a slice selector.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selector, Slice};
    /// let selector = Selector::Slice(Slice::default());
    /// assert!(selector.is_slice());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_slice(&self) -> bool {
        matches!(self, Self::Slice(_))
    }

    /// Check if this is a filter selector.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{JsonPathQuery, TestExpr, LogicalExpr, Selector, Index};
    /// let selector = Selector::Filter(LogicalExpr::Test(TestExpr::Relative(JsonPathQuery::from_iter(vec![]))));
    /// assert!(selector.is_filter());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_filter(&self) -> bool {
        matches!(self, Self::Filter(_))
    }

    fn is_singular(&self) -> bool {
        matches!(self, Self::Name(_) | Self::Index(_))
    }
}

impl Index {
    /// Check if this is an index counting from the start of an array.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selector, Index};
    /// let index = Index::FromStart(0.into());
    /// assert!(index.is_start_based());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_start_based(&self) -> bool {
        matches!(self, Self::FromStart(_))
    }

    /// Check if this is an index counting from the end of an array.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selector, Index};
    /// let index = Index::FromEnd(1.try_into().unwrap());
    /// assert!(index.is_end_based());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_end_based(&self) -> bool {
        matches!(self, Self::FromEnd(_))
    }
}

impl Step {
    /// Check if this is a step going forward in an array.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selector, Step};
    /// let step = Step::Forward(2.try_into().unwrap());
    /// assert!(step.is_forward());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_forward(&self) -> bool {
        matches!(self, Self::Forward(_))
    }

    /// Check if this is a step going backward in an array.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{Selector, Step};
    /// let step = Step::Backward(2.try_into().unwrap());
    /// assert!(step.is_backward());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_backward(&self) -> bool {
        matches!(self, Self::Backward(_))
    }
}

impl Deref for Selectors {
    type Target = [Selector];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Display for JsonPathQuery {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "$")?;
        for s in &self.segments {
            write!(f, "{s}")?;
        }
        Ok(())
    }
}

impl Display for Segment {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Child(s) => write!(f, "{s}"),
            Self::Descendant(s) => write!(f, "..{s}"),
        }
    }
}

impl Display for Selectors {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}", self.first())?;
        for s in self.inner.iter().skip(1) {
            write!(f, ", {s}")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Display for Selector {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(n) => write!(f, "'{}'", str::escape(n.unquoted(), str::EscapeMode::SingleQuoted)),
            Self::Wildcard => write!(f, "*"),
            Self::Index(idx) => write!(f, "{idx}"),
            Self::Slice(slice) => write!(f, "{slice}"),
            Self::Filter(filter) => write!(f, "?{filter}"),
        }
    }
}

impl Display for Index {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FromStart(idx) => write!(f, "{idx}"),
            Self::FromEnd(idx) => write!(f, "-{idx}"),
        }
    }
}

impl Display for Step {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Forward(idx) => write!(f, "{idx}"),
            Self::Backward(idx) => write!(f, "-{idx}"),
        }
    }
}

impl Display for Slice {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (self.step.is_forward() && self.start != Self::DEFAULT_START_FORWARDS)
            || (self.step.is_backward() && self.start != Self::default_start_backwards())
        {
            write!(f, "{}", self.start)?;
        }
        write!(f, ":")?;
        if let Some(end) = self.end {
            write!(f, "{end}")?;
        }
        if self.step != Self::DEFAULT_STEP {
            write!(f, ":{}", self.step)?;
        }
        Ok(())
    }
}

impl Display for LogicalExpr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Or(lhs, rhs) => {
                if lhs.precedence() <= self.precedence() {
                    write!(f, "({lhs})")?;
                } else {
                    write!(f, "{lhs}")?;
                }
                write!(f, " || ")?;
                if rhs.precedence() < self.precedence() {
                    write!(f, "({rhs})")?;
                } else {
                    write!(f, "{rhs}")?;
                }
                Ok(())
            }
            Self::And(lhs, rhs) => {
                if lhs.precedence() < self.precedence() {
                    write!(f, "({lhs})")?;
                } else {
                    write!(f, "{lhs}")?;
                }
                write!(f, " && ")?;
                if rhs.precedence() <= self.precedence() {
                    write!(f, "({rhs})")?;
                } else {
                    write!(f, "{rhs}")?;
                }
                Ok(())
            }
            Self::Not(expr) => {
                if expr.precedence() <= self.precedence() {
                    write!(f, "!({expr})")
                } else {
                    write!(f, "!{expr}")
                }
            }
            Self::Comparison(expr) => write!(f, "{expr}"),
            Self::Test(test) => write!(f, "{test}"),
        }
    }
}

impl Display for ComparisonExpr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
    }
}

impl Display for TestExpr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Relative(q) => {
                write!(f, "@")?;
                for s in q.segments() {
                    write!(f, "{s}")?;
                }
            }
            Self::Absolute(q) => {
                write!(f, "$")?;
                for s in q.segments() {
                    write!(f, "{s}")?;
                }
            }
        }
        Ok(())
    }
}

impl Display for Comparable {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(lit) => write!(f, "{lit}"),
            Self::RelativeSingularQuery(q) => {
                write!(f, "@")?;
                for s in q.segments() {
                    write!(f, "{s}")?;
                }
                Ok(())
            }
            Self::AbsoluteSingularQuery(q) => {
                write!(f, "$")?;
                for s in q.segments() {
                    write!(f, "{s}")?;
                }
                Ok(())
            }
        }
    }
}

impl Display for Literal {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{}\"", str::escape(s.unquoted(), str::EscapeMode::DoubleQuoted)),
            Self::Number(n) => write!(f, "{n}"),
            Self::Bool(true) => write!(f, "true"),
            Self::Bool(false) => write!(f, "false"),
            Self::Null => write!(f, "null"),
        }
    }
}

impl Display for ComparisonOp {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EqualTo => write!(f, "=="),
            Self::NotEqualTo => write!(f, "!="),
            Self::LesserOrEqualTo => write!(f, "<="),
            Self::GreaterOrEqualTo => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::GreaterThan => write!(f, ">"),
        }
    }
}

impl Display for SingularJsonPathQuery {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in &self.segments {
            write!(f, "[{s}]")?;
        }
        Ok(())
    }
}

impl Display for SingularSegment {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(n) => write!(f, "['{}']", str::escape(n.unquoted(), str::EscapeMode::SingleQuoted)),
            Self::Index(i) => write!(f, "[{i}]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn leading_whitespace_is_disallowed() {
        let err = parse("  $").expect_err("should fail");
        let display = format!("{err}");
        let expected = r"error: query starting with whitespace

    $
  ^^ leading whitespace is disallowed
  (bytes 0-1)


suggestion: did you mean `$` ?
";
        assert_eq!(display, expected);
    }

    #[test]
    fn trailing_whitespace_is_disallowed() {
        let err = parse("$  ").expect_err("should fail");
        let display = format!("{err}");
        let expected = r"error: query ending with whitespace

  $  
   ^^ trailing whitespace is disallowed
  (bytes 1-2)


suggestion: did you mean `$` ?
";
        assert_eq!(display, expected);
    }

    mod name_selector {
        use super::*;
        use pretty_assertions::assert_eq;
        use test_case::test_case;

        fn parse_single_quoted_name_selector(src: &str) -> Result<JsonPathQuery> {
            let query_string = format!("$['{src}']");
            parse(&query_string)
        }

        #[test_case("", ""; "empty")]
        #[test_case("dog", "dog"; "ascii")]
        #[test_case(r"\\", r"\"; "backslash")]
        #[test_case("unescaped 🔥 fire emoji", "unescaped 🔥 fire emoji"; "unescaped emoji")]
        #[test_case(r"escape \b backspace", "escape \u{0008} backspace"; "BS escape")]
        #[test_case(r"escape \t tab", "escape \t tab"; "HT escape")]
        #[test_case(r"escape \n endln", "escape \n endln"; "LF escape")]
        #[test_case(r"escape \f formfeed", "escape \u{000C} formfeed"; "FF escape")]
        #[test_case(r"escape \r carriage", "escape \r carriage"; "CR escape")]
        #[test_case(r#"escape \' apost"#, r"escape ' apost"; "apostrophe escape")]
        #[test_case(r"escape \/ slash", r"escape / slash"; "slash escape")]
        #[test_case(r"escape \\ backslash", r"escape \ backslash"; "backslash escape")]
        #[test_case(r"escape \u2112 script L", "escape ℒ script L"; "U+2112 Script Capital L escape")]
        #[test_case(r"escape \u211269 script L", "escape ℒ69 script L"; "U+2112 Script Capital L escape followed by digits")]
        #[test_case(r"escape \u21a7 bar down arrow", "escape ↧ bar down arrow"; "U+21a7 Downwards Arrow From Bar (lowercase hex)")]
        #[test_case(r"escape \u21A7 bar down arrow", "escape ↧ bar down arrow"; "U+21A7 Downwards Arrow From Bar (uppercase hex)")]
        #[test_case(r"escape \ud83d\udd25 fire emoji", "escape 🔥 fire emoji"; "U+1F525 fire emoji escape (lowercase hex)")]
        #[test_case(r"escape \uD83D\uDD25 fire emoji", "escape 🔥 fire emoji"; "U+1F525 fire emoji escape (uppercase hex)")]
        fn parse_correct_single_quoted_name(src: &str, expected: &str) {
            let res = parse_single_quoted_name_selector(src).expect("should successfully parse");
            match res.segments().first() {
                Some(Segment::Child(selectors)) => match selectors.first() {
                    Selector::Name(name) => assert_eq!(name.unquoted(), expected),
                    _ => panic!("expected to parse a single name selector, got {res:?}"),
                },
                _ => panic!("expected to parse a single name selector, got {res:?}"),
            }
        }

        #[test]
        fn parse_double_quoted_name_with_escaped_double_quote() {
            let query_string = r#"$["escape \" quote"]"#;
            let res = parse(query_string).expect("should successfully parse");
            match res.segments().first() {
                Some(Segment::Child(selectors)) => match selectors.first() {
                    Selector::Name(name) => assert_eq!(name.unquoted(), "escape \" quote"),
                    _ => panic!("expected to parse a single name selector, got {res:?}"),
                },
                _ => panic!("expected to parse a single name selector, got {res:?}"),
            }
        }
    }
}
