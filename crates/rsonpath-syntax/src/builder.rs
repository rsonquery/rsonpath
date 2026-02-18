//! Utility for building a [`JsonPathQuery`](`crate::JsonPathQuery`)
//! programmatically.
//!
//! The entrypoint is the [`JsonPathQueryBuilder`].
//! Consult the structs documentation for details.

use crate::{
    num::JsonInt, str::JsonString, Comparable, ComparisonExpr, ComparisonOp, Index, JsonPathQuery, Literal,
    LogicalExpr, Segment, Selector, Selectors, SingularJsonPathQuery, SingularSegment, Slice, TestExpr,
};

/// Builder for [`JsonPathQuery`] instances.
///
/// # Examples
/// ```
/// # use rsonpath_syntax::{JsonPathQuery, builder::JsonPathQueryBuilder, str::JsonString, Slice};
/// let mut builder = JsonPathQueryBuilder::new();
///     
/// builder.child_name("a")
///     .descendant_name("b")
///     .child_wildcard()
///     .child_name("c")
///     .descendant_wildcard()
///     .child_slice(Slice::build().with_start(3).with_end(-7).with_step(2));
///
/// // Can also use `builder.build()` as a non-consuming version.
/// let query: JsonPathQuery = builder.into();
///
/// assert_eq!(query.to_string(), "$['a']..['b'][*]['c']..[*][3:-7:2]");
/// ```
pub struct JsonPathQueryBuilder {
    segments: Vec<Segment>,
}

/// Builder for a [`Selectors`] collection used by the [`JsonPathQueryBuilder`]
/// to create multiple [`Selector`] instances within a [`Segment`].
pub struct JsonPathSelectorsBuilder {
    selectors: Vec<Selector>,
}

impl JsonPathQueryBuilder {
    /// Initialize an empty builder.
    ///
    /// # Examples
    /// ```
    /// # use rsonpath_syntax::{JsonPathQuery, builder::JsonPathQueryBuilder};
    /// let builder = JsonPathQueryBuilder::new();
    /// let query: JsonPathQuery = builder.into();
    ///
    /// assert!(query.segments().is_empty());
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    /// Add a child segment with selectors defined in the `selectors_builder` function.
    ///
    /// See the documentation of [`JsonPathSelectorsBuilder`] for selector building details.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rsonpath_syntax::{Selector, Index, str::JsonString, num::JsonUInt, builder::JsonPathQueryBuilder};
    /// let mut builder = JsonPathQueryBuilder::new();
    /// builder.child(|x| x.name("abc").index(10).wildcard());
    /// let result = builder.into_query();
    /// assert_eq!(result.segments().len(), 1);
    /// let segment = &result.segments()[0];
    /// assert!(segment.is_child());
    /// assert_eq!(&segment.selectors().as_slice(), &[
    ///     Selector::Name(JsonString::new("abc")),
    ///     Selector::Index(Index::FromStart(JsonUInt::from(10))),
    ///     Selector::Wildcard,
    /// ]);
    /// ```
    #[inline]
    pub fn child<F>(&mut self, selectors_builder: F) -> &mut Self
    where
        F: FnOnce(&mut JsonPathSelectorsBuilder) -> &mut JsonPathSelectorsBuilder,
    {
        let mut builder = JsonPathSelectorsBuilder::new();
        selectors_builder(&mut builder);
        self.segments.push(Segment::Child(builder.build()));
        self
    }

    /// Add a descendant segment with selectors defined in the `selectors_builder` function.
    ///
    /// See the documentation of [`JsonPathSelectorsBuilder`] for selector building details.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rsonpath_syntax::{Selector, Index, str::JsonString, num::JsonUInt, builder::JsonPathQueryBuilder};
    /// let mut builder = JsonPathQueryBuilder::new();
    /// builder.descendant(|x| x.name("abc").index(10).wildcard());
    /// let result = builder.into_query();
    /// assert_eq!(result.segments().len(), 1);
    /// let segment = &result.segments()[0];
    /// assert!(segment.is_descendant());
    /// assert_eq!(&segment.selectors().as_slice(), &[
    ///     Selector::Name(JsonString::new("abc")),
    ///     Selector::Index(Index::FromStart(JsonUInt::from(10))),
    ///     Selector::Wildcard,
    /// ]);
    /// ```
    #[inline]
    pub fn descendant<F>(&mut self, selectors_builder: F) -> &mut Self
    where
        F: FnOnce(&mut JsonPathSelectorsBuilder) -> &mut JsonPathSelectorsBuilder,
    {
        let mut builder = JsonPathSelectorsBuilder::new();
        selectors_builder(&mut builder);
        self.segments.push(Segment::Descendant(builder.build()));
        self
    }

    /// Add a child segment with a single name selector.
    ///
    /// This is a shorthand for `.child(|x| x.name(name))`.
    #[inline(always)]
    pub fn child_name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.child(|x| x.name(name))
    }

    /// Add a child segment with a single wildcard selector.
    ///
    /// This is a shorthand for `.child(|x| x.wildcard())`.
    #[inline(always)]
    pub fn child_wildcard(&mut self) -> &mut Self {
        self.child(|x| x.wildcard())
    }

    /// Add a child segment with a single index selector.
    ///
    /// This is a shorthand for `.child(|x| x.index(idx))`.
    #[inline(always)]
    pub fn child_index<N: Into<JsonInt>>(&mut self, idx: N) -> &mut Self {
        self.child(|x| x.index(idx))
    }

    /// Add a child segment with a single slice selector.
    ///
    /// This is a shorthand for `.child(|x| x.slice(slice))`.
    #[inline(always)]
    pub fn child_slice(&mut self, slice: impl Into<Slice>) -> &mut Self {
        self.child(|x| x.slice(slice))
    }

    /// Add a child segment with a single filter selector.
    ///
    /// This is a shorthand for `.child(|x| x.filter(filter_builder))`.
    #[inline(always)]
    pub fn child_filter<F>(&mut self, filter_builder: F) -> &mut Self
    where
        F: FnOnce(EmptyLogicalExprBuilder) -> LogicalExprBuilder,
    {
        self.child(|x| x.filter(filter_builder))
    }

    /// Add a descendant segment with a single name selector.
    ///
    /// This is a shorthand for `.descendant(|x| x.name(name))`.
    #[inline(always)]
    pub fn descendant_name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.descendant(|x| x.name(name))
    }

    /// Add a descendant segment with a single name selector.
    ///
    /// This is a shorthand for `.descendant(|x| x.wildcard())`.
    #[inline(always)]
    pub fn descendant_wildcard(&mut self) -> &mut Self {
        self.descendant(|x| x.wildcard())
    }

    /// Add a descendant segment with a single name selector.
    ///
    /// This is a shorthand for `.descendant(|x| x.index(idx))`.
    #[inline(always)]
    pub fn descendant_index<N: Into<JsonInt>>(&mut self, idx: N) -> &mut Self {
        self.descendant(|x| x.index(idx))
    }

    /// Add a descendant segment with a single slice selector.
    ///
    /// This is a shorthand for `.descendant(|x| x.slice(slice))`.
    #[inline(always)]
    pub fn descendant_slice(&mut self, slice: impl Into<Slice>) -> &mut Self {
        self.descendant(|x| x.slice(slice))
    }

    /// Add a descendant segment with a single filter selector.
    ///
    /// This is a shorthand for `.descendant(|x| x.filter(filter_builder))`.
    #[inline(always)]
    pub fn descendant_filter<F>(&mut self, filter_builder: F) -> &mut Self
    where
        F: FnOnce(EmptyLogicalExprBuilder) -> LogicalExprBuilder,
    {
        self.descendant(|x| x.filter(filter_builder))
    }

    /// Produce a [`JsonPathQuery`] from the builder.
    ///
    /// This clones all data in the builder to create the query.
    /// If possible, use [`into_query`](JsonPathQueryBuilder::into_query)
    /// to consume the builder and avoid a copy.
    #[must_use]
    #[inline]
    pub fn to_query(&mut self) -> JsonPathQuery {
        JsonPathQuery {
            segments: self.segments.clone(),
        }
    }

    /// Produce a [`JsonPathQuery`] by consuming this builder.
    ///
    /// To avoid consuming the builder use [`to_query`](JsonPathQueryBuilder::to_query).
    #[must_use]
    #[inline]
    pub fn into_query(self) -> JsonPathQuery {
        JsonPathQuery {
            segments: self.segments,
        }
    }
}

impl JsonPathSelectorsBuilder {
    fn new() -> Self {
        Self { selectors: vec![] }
    }

    fn build(self) -> Selectors {
        assert!(!self.selectors.is_empty(), "there must be at least one selector");
        Selectors::many(self.selectors)
    }

    /// Add a name selector with a given `name` string to the collection.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rsonpath_syntax::prelude::*;
    /// let mut builder = JsonPathQueryBuilder::new();
    /// builder.child(|x| x.name("book").name("journal"));
    /// let result = builder.into_query();
    /// assert_eq!(result.segments().len(), 1);
    /// let segment = &result.segments()[0];
    /// assert_eq!(segment.selectors().as_slice(), &[
    ///     Selector::Name(JsonString::new("book")),
    ///     Selector::Name(JsonString::new("journal")),
    /// ]);
    /// ```
    #[inline(always)]
    pub fn name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.selectors.push(Selector::Name(name.into()));
        self
    }

    /// Add an index selector based on a given JSON integer.
    ///
    /// The result is a [`Selector::Index`] with an [`Index::FromStart`]
    /// if `idx` converts to a non-negative [`JsonInt`], and [`Index::FromEnd`]
    /// otherwise.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rsonpath_syntax::{prelude::*, num::JsonNonZeroUInt};
    /// let mut builder = JsonPathQueryBuilder::new();
    /// builder.child(|x| x.index(10).index(-20));
    /// let result = builder.into_query();
    /// assert_eq!(result.segments().len(), 1);
    /// let segment = &result.segments()[0];
    /// assert_eq!(segment.selectors().as_slice(), &[
    ///     Selector::Index(Index::FromStart(JsonUInt::from(10))),
    ///     Selector::Index(Index::FromEnd(JsonNonZeroUInt::try_from(20).unwrap())),
    /// ]);
    /// ```
    #[inline(always)]
    pub fn index<N: Into<JsonInt>>(&mut self, idx: N) -> &mut Self {
        let json_int: JsonInt = idx.into();
        self.selectors.push(Selector::Index(Index::from(json_int)));
        self
    }

    /// Add a slice selector based on a given start, end, and step integers.
    ///
    /// The result is a [`Selector::Slice`] with given `start`, `end`, and `step`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use rsonpath_syntax::{prelude::*, num::JsonNonZeroUInt};
    /// let mut builder = JsonPathQueryBuilder::new();
    /// builder.child(|x| x
    ///     .slice(Slice::build().with_start(10).with_end(-20).with_step(5))
    ///     .slice(Slice::build().with_start(-20).with_step(-30)));
    /// let result = builder.into_query();
    ///
    /// assert_eq!(result.segments().len(), 1);
    /// let segment = &result.segments()[0];
    /// let selectors = segment.selectors().as_slice();
    /// match (&selectors[0], &selectors[1]) {
    ///     (Selector::Slice(s1), Selector::Slice(s2)) => {
    ///         assert_eq!(s1.start(), Index::FromStart(10.into()));
    ///         assert_eq!(s1.end(), Some(Index::FromEnd(JsonNonZeroUInt::try_from(20).unwrap())));
    ///         assert_eq!(s1.step(), Step::Forward(5.into()));
    ///         assert_eq!(s2.start(), Index::FromEnd(JsonNonZeroUInt::try_from(20).unwrap()));
    ///         assert_eq!(s2.end(), None);
    ///         assert_eq!(s2.step(), Step::Backward(JsonNonZeroUInt::try_from(30).unwrap()));
    ///     }
    ///     _ => unreachable!()
    /// }
    /// ```
    #[inline(always)]
    pub fn slice(&mut self, slice: impl Into<Slice>) -> &mut Self {
        self.selectors.push(Selector::Slice(slice.into()));
        self
    }

    /// Add a wildcard selector.
    ///
    /// ```rust
    /// # use rsonpath_syntax::prelude::*;
    /// let mut builder = JsonPathQueryBuilder::new();
    /// builder.child(|x| {
    ///     x.filter(|x| {
    ///         x.comparison(|x| {
    ///             x.query_relative(|x| x.name("price"))
    ///              .less_than()
    ///              .literal(JsonInt::from(10))
    ///         })
    ///     })
    /// });
    /// let result = builder.into_query();
    /// assert_eq!(result.segments().len(), 1);
    /// let segment = &result.segments()[0];
    /// assert_eq!(segment.selectors().len(), 1);
    /// let selector = segment.selectors().first();
    ///
    /// let Selector::Filter(LogicalExpr::Comparison(expr)) = selector else {
    ///     panic!("expected comparison filter")
    /// };
    /// let Comparable::RelativeSingularQuery(lhs) = expr.lhs() else {
    ///     panic!("expected lhs to be a relative singular query")
    /// };
    /// let lhs_segments: Vec<_> = lhs.segments().collect();
    /// assert_eq!(&lhs_segments, &[
    ///     &SingularSegment::Name(JsonString::new("price"))
    /// ]);
    /// assert_eq!(expr.op(), ComparisonOp::LessThan);
    /// assert_eq!(expr.rhs(), &Comparable::Literal(JsonInt::from(10).into()));
    ///
    /// ```
    #[inline(always)]
    pub fn wildcard(&mut self) -> &mut Self {
        self.selectors.push(Selector::Wildcard);
        self
    }

    /// Add a filter selector.
    #[inline]
    pub fn filter<F>(&mut self, filter_builder: F) -> &mut Self
    where
        F: FnOnce(EmptyLogicalExprBuilder) -> LogicalExprBuilder,
    {
        let filter = filter_builder(EmptyLogicalExprBuilder);
        let logical_expr = filter.into();
        self.selectors.push(Selector::Filter(logical_expr));
        self
    }
}

impl Default for JsonPathQueryBuilder {
    /// Return the empty builder.
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl From<JsonPathQueryBuilder> for JsonPathQuery {
    #[inline(always)]
    fn from(value: JsonPathQueryBuilder) -> Self {
        Self {
            segments: value.segments,
        }
    }
}

/// Helper API for programmatically constructing [`Slice`] instances.
///
/// # Examples
/// ```
/// # use rsonpath_syntax::prelude::*;
/// let mut builder = SliceBuilder::new();
///
/// builder.with_start(-3).with_end(1).with_step(-7);
///
/// let slice: Slice = builder.into();
/// assert_eq!(slice.to_string(), "-3:1:-7");
/// ```
pub struct SliceBuilder {
    inner: Slice,
    /// We need to track if start is explicit because the default depends on step sign.
    start_was_explicitly_given: bool,
}

impl SliceBuilder {
    /// Create a new [`Slice`] configuration with default values.
    ///
    /// ```rust
    /// # use rsonpath_syntax::prelude::*;
    /// let slice: Slice = SliceBuilder::new().into();
    /// assert_eq!(Slice::default(), slice);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Slice::default(),
            start_was_explicitly_given: false,
        }
    }

    /// Set the start of the [`Slice`].
    #[inline]
    pub fn with_start<N: Into<JsonInt>>(&mut self, start: N) -> &mut Self {
        self.inner.start = start.into().into();
        self.start_was_explicitly_given = true;
        self
    }

    /// Set the end of the [`Slice`].
    #[inline]
    pub fn with_end<N: Into<JsonInt>>(&mut self, end: N) -> &mut Self {
        self.inner.end = Some(end.into().into());
        self
    }

    /// Set the step of the [`Slice`].
    #[inline]
    pub fn with_step<N: Into<JsonInt>>(&mut self, step: N) -> &mut Self {
        self.inner.step = step.into().into();
        self
    }

    /// Get the configured [`Slice`] instance.
    ///
    /// This does not consume the builder. For a consuming variant use the `Into<Slice>` impl.
    #[inline]
    #[must_use]
    pub fn to_slice(&mut self) -> Slice {
        if !self.start_was_explicitly_given {
            if self.inner.step.is_forward() {
                self.inner.start = Slice::DEFAULT_START_FORWARDS;
            } else {
                self.inner.start = Slice::default_start_backwards();
            }
        }

        self.inner.clone()
    }
}

impl From<SliceBuilder> for Slice {
    #[inline]
    fn from(mut value: SliceBuilder) -> Self {
        value.to_slice()
    }
}

impl<'a> From<&'a mut SliceBuilder> for Slice {
    #[inline]
    fn from(value: &'a mut SliceBuilder) -> Self {
        value.to_slice()
    }
}

impl Default for SliceBuilder {
    /// Create a builder configured with default values.
    ///
    /// ```rust
    /// # use rsonpath_syntax::prelude::*;
    /// let slice: Slice = SliceBuilder::default().into();
    /// assert_eq!(Slice::default(), slice);
    /// ```
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Starting point for building a filter [`LogicalExpr`].
pub struct EmptyLogicalExprBuilder;

/// Starting point for building a [`ComparisonExpr`].
pub struct EmptyComparisonExprBuilder;

/// Builder for [`ComparisonExpr`] with the [`lhs`](ComparisonExpr::lhs) already configured.
pub struct ComparisonWithLhsBuilder {
    lhs: Comparable,
}

/// Builder for [`ComparisonExpr`] with the [`lhs`](ComparisonExpr::lhs)
/// and [`op`](ComparisonExpr::op) already configured.
pub struct ComparisonWithLhsAndOpBuilder {
    lhs: Comparable,
    op: ComparisonOp,
}

/// Builder for a [`LogicalExpr`] that can be finalized,
/// or boolean-combined with another [`LogicalExpr`].
///
/// # Examples
/// ```rust
/// # use rsonpath_syntax::prelude::*;
/// let mut builder = JsonPathQueryBuilder::new();
/// builder.child_filter(|fb|
///     fb.test_relative(|qb| qb.child_name("book"))
///     // We could finish here, but we can also chain another expression.
///       .or(|fb2| fb2.test_relative(|qb2| qb2.child_name("journal")))
/// );
///
/// assert_eq!(builder.to_query().to_string(), "$[?@['book'] || @['journal']]");
/// ```
pub struct LogicalExprBuilder {
    current: LogicalExpr,
}

/// Builder for a [`SingularJsonPathQuery`].
pub struct SingularJsonPathQueryBuilder {
    segments: Vec<SingularSegment>,
}

impl SingularJsonPathQueryBuilder {
    /// Create a new, empty builder.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    /// Add a child name segment.
    #[inline]
    pub fn name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.segments.push(SingularSegment::Name(name.into()));
        self
    }

    /// Add a child index segment.
    #[inline]
    pub fn index<N: Into<Index>>(&mut self, n: N) -> &mut Self {
        self.segments.push(SingularSegment::Index(n.into()));
        self
    }
}

impl Default for SingularJsonPathQueryBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl From<SingularJsonPathQueryBuilder> for SingularJsonPathQuery {
    #[inline]
    fn from(value: SingularJsonPathQueryBuilder) -> Self {
        Self {
            segments: value.segments,
        }
    }
}

impl EmptyLogicalExprBuilder {
    /// Start building a [`ComparisonExpr`] logical expression.
    #[inline]
    #[must_use]
    pub fn comparison<F>(self, cf: F) -> LogicalExprBuilder
    where
        F: FnOnce(EmptyComparisonExprBuilder) -> ComparisonExpr,
    {
        let comparison = cf(EmptyComparisonExprBuilder);
        LogicalExprBuilder {
            current: LogicalExpr::Comparison(comparison),
        }
    }

    /// Start building a test query from the root.
    #[inline]
    pub fn test_absolute<F>(self, tf: F) -> LogicalExprBuilder
    where
        F: FnOnce(&mut JsonPathQueryBuilder) -> &mut JsonPathQueryBuilder,
    {
        let mut query = JsonPathQueryBuilder::new();
        tf(&mut query);
        LogicalExprBuilder {
            current: LogicalExpr::Test(TestExpr::Absolute(query.into_query())),
        }
    }

    /// Start building a test query from the current node.
    #[inline]
    pub fn test_relative<F>(self, tf: F) -> LogicalExprBuilder
    where
        F: FnOnce(&mut JsonPathQueryBuilder) -> &mut JsonPathQueryBuilder,
    {
        let mut query = JsonPathQueryBuilder::new();
        tf(&mut query);
        LogicalExprBuilder {
            current: LogicalExpr::Test(TestExpr::Relative(query.into_query())),
        }
    }

    /// Build a logical expression by negating the one created in the inner function.
    #[inline]
    pub fn not<F>(self, tf: F) -> LogicalExprBuilder
    where
        F: FnOnce(Self) -> LogicalExprBuilder,
    {
        let inner = tf(Self).into();
        LogicalExprBuilder {
            current: LogicalExpr::Not(Box::new(inner)),
        }
    }
}

impl EmptyComparisonExprBuilder {
    /// Set the left-hand side of the comparison to a literal.
    #[inline]
    #[must_use]
    pub fn literal<L: Into<Literal>>(self, l: L) -> ComparisonWithLhsBuilder {
        ComparisonWithLhsBuilder {
            lhs: Comparable::Literal(l.into()),
        }
    }

    /// Set the left-hand side of the comparison to a root-based singular query.
    #[inline]
    #[must_use]
    pub fn query_absolute<F>(self, qf: F) -> ComparisonWithLhsBuilder
    where
        F: FnOnce(&mut SingularJsonPathQueryBuilder) -> &mut SingularJsonPathQueryBuilder,
    {
        let mut query = SingularJsonPathQueryBuilder::new();
        qf(&mut query);
        ComparisonWithLhsBuilder {
            lhs: Comparable::AbsoluteSingularQuery(query.into()),
        }
    }

    /// Set the left-hand side of the comparison to a current-node-based singular query.
    #[inline]
    #[must_use]
    pub fn query_relative<F>(self, qf: F) -> ComparisonWithLhsBuilder
    where
        F: FnOnce(&mut SingularJsonPathQueryBuilder) -> &mut SingularJsonPathQueryBuilder,
    {
        let mut query = SingularJsonPathQueryBuilder::new();
        qf(&mut query);
        ComparisonWithLhsBuilder {
            lhs: Comparable::RelativeSingularQuery(query.into()),
        }
    }
}

impl ComparisonWithLhsBuilder {
    /// Use the equality operator `==`.
    #[inline]
    #[must_use]
    pub fn equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::EqualTo,
        }
    }

    /// Use the inequality operator `!=`.
    #[inline]
    #[must_use]
    pub fn not_equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::NotEqualTo,
        }
    }

    /// Use the less-than operator `<`.
    #[inline]
    #[must_use]
    pub fn less_than(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::LessThan,
        }
    }

    /// Use the less-than-or-equal operator `<=`.
    #[inline]
    #[must_use]
    pub fn lesser_or_equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::LesserOrEqualTo,
        }
    }

    /// Use the greater-than operator `>`.
    #[inline]
    #[must_use]
    pub fn greater_than(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::GreaterThan,
        }
    }

    /// Use the greater-than-or-equal operator `>=`.
    #[inline]
    #[must_use]
    pub fn greater_or_equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::GreaterOrEqualTo,
        }
    }
}

impl ComparisonWithLhsAndOpBuilder {
    /// Set the right-hand side of the comparison to a literal and finalize the expression.
    #[inline]
    pub fn literal<L: Into<Literal>>(self, l: L) -> ComparisonExpr {
        ComparisonExpr {
            lhs: self.lhs,
            op: self.op,
            rhs: Comparable::Literal(l.into()),
        }
    }

    /// Set the right-hand side of the comparison to a root-based singular query and finalize the expression.
    #[inline]
    #[must_use]
    pub fn query_absolute<F>(self, qf: F) -> ComparisonExpr
    where
        F: FnOnce(&mut SingularJsonPathQueryBuilder) -> &mut SingularJsonPathQueryBuilder,
    {
        let mut query = SingularJsonPathQueryBuilder::new();
        qf(&mut query);
        ComparisonExpr {
            lhs: self.lhs,
            op: self.op,
            rhs: Comparable::AbsoluteSingularQuery(query.into()),
        }
    }

    /// Set the right-hand side of the comparison to a current-node-based singular query and finalize the expression.
    #[inline]
    #[must_use]
    pub fn query_relative<F>(self, qf: F) -> ComparisonExpr
    where
        F: FnOnce(&mut SingularJsonPathQueryBuilder) -> &mut SingularJsonPathQueryBuilder,
    {
        let mut query = SingularJsonPathQueryBuilder::new();
        qf(&mut query);
        ComparisonExpr {
            lhs: self.lhs,
            op: self.op,
            rhs: Comparable::RelativeSingularQuery(query.into()),
        }
    }
}

impl LogicalExprBuilder {
    /// Combine the entire expression built thus far with the one built in the inner function
    /// by using the boolean conjunction operator `&&`.
    #[inline]
    pub fn and<F>(self, f: F) -> Self
    where
        F: FnOnce(EmptyLogicalExprBuilder) -> Self,
    {
        let lhs = Box::new(self.current);
        let rhs = Box::new(f(EmptyLogicalExprBuilder).into());
        Self {
            current: LogicalExpr::And(lhs, rhs),
        }
    }

    /// Combine the entire expression built thus far with the one built in the inner function
    /// by using the boolean disjunction operator `||`.
    #[inline]
    pub fn or<F>(self, f: F) -> Self
    where
        F: FnOnce(EmptyLogicalExprBuilder) -> Self,
    {
        let lhs = Box::new(self.current);
        let rhs = Box::new(f(EmptyLogicalExprBuilder).into());
        Self {
            current: LogicalExpr::Or(lhs, rhs),
        }
    }
}

impl From<LogicalExprBuilder> for LogicalExpr {
    #[inline(always)]
    fn from(value: LogicalExprBuilder) -> Self {
        value.current
    }
}

#[cfg(test)]
mod tests {
    use super::SliceBuilder;
    use crate::{Index, Slice, Step};

    #[test]
    fn slice_builder_default_start_forward() {
        let mut builder = SliceBuilder::new();
        builder.with_end(3).with_step(4);
        let slice: Slice = builder.into();

        assert_eq!(slice.start(), Index::FromStart(0.into()));
        assert_eq!(slice.end(), Some(Index::FromStart(3.into())));
        assert_eq!(slice.step(), Step::Forward(4.into()));
    }

    #[test]
    fn slice_builder_default_start_backward() {
        let mut builder = SliceBuilder::new();
        builder.with_end(3).with_step(-4);
        let slice: Slice = builder.into();

        assert_eq!(slice.start(), Index::FromEnd(1.try_into().unwrap()));
        assert_eq!(slice.end(), Some(Index::FromStart(3.into())));
        assert_eq!(slice.step(), Step::Backward(4.try_into().unwrap()));
    }
}
