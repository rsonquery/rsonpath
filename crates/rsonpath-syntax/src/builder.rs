//! Utility for building a [`JsonPathQuery`](`crate::JsonPathQuery`)
//! programmatically.

use crate::{
    num::JsonInt, str::JsonString, Comparable, ComparisonExpr, ComparisonOp, Index, JsonPathQuery, Literal,
    LogicalExpr, Segment, Selector, Selectors, SingularJsonPathQuery, SingularSegment, SliceBuilder, TestExpr,
};

/// Builder for [`JsonPathQuery`] instances.
///
/// # Examples
/// ```
/// # use rsonpath_syntax::{JsonPathQuery, builder::JsonPathQueryBuilder, str::JsonString};
/// let mut builder = JsonPathQueryBuilder::new();
///     
/// builder.child_name("a")
///     .descendant_name("b")
///     .child_wildcard()
///     .child_name("c")
///     .descendant_wildcard()
///     .child_slice(|x| x.with_start(3).with_end(-7).with_step(2));
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
    /// This is a shorthand for `.child(|x| x.slice(slice_builder))`.
    #[inline(always)]
    pub fn child_slice<F>(&mut self, slice_builder: F) -> &mut Self
    where
        F: FnOnce(&mut SliceBuilder) -> &mut SliceBuilder,
    {
        self.child(|x| x.slice(slice_builder))
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
    /// This is a shorthand for `.descendant(|x| x.slice(slice_builder))`.
    #[inline(always)]
    pub fn descendant_slice<F>(&mut self, slice_builder: F) -> &mut Self
    where
        F: FnOnce(&mut SliceBuilder) -> &mut SliceBuilder,
    {
        self.descendant(|x| x.slice(slice_builder))
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

    /// Produce a [`JsonPathQuery`] consuming builder.
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
        Selectors::many(self.selectors)
    }

    /// Add a name selector with a given `name` string to the collection.
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
    /// # use rsonpath_syntax::{Selector, Index, num::{JsonNonZeroUInt, JsonUInt}, builder::JsonPathQueryBuilder};
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
    /// # use rsonpath_syntax::{Selector, SliceBuilder, Index, Step, num::{JsonNonZeroUInt, JsonUInt}, builder::JsonPathQueryBuilder};
    /// let mut builder = JsonPathQueryBuilder::new();
    /// builder.child(|x| x
    ///     .slice(|s| s.with_start(10).with_end(-20).with_step(5))
    ///     .slice(|s| s.with_start(-20).with_step(-30)));
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
    pub fn slice<F>(&mut self, slice_builder: F) -> &mut Self
    where
        F: FnOnce(&mut SliceBuilder) -> &mut SliceBuilder,
    {
        let mut slice = SliceBuilder::new();
        slice_builder(&mut slice);
        let slice = slice.into();
        self.selectors.push(Selector::Slice(slice));
        self
    }

    /// Add a wildcard selector.
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

pub struct EmptyLogicalExprBuilder;
pub struct EmptyComparisonBuilder;
pub struct ComparisonWithLhsBuilder {
    lhs: Comparable,
}
pub struct ComparisonWithLhsAndOpBuilder {
    lhs: Comparable,
    op: ComparisonOp,
}

pub struct LogicalExprBuilder {
    current: LogicalExpr,
}

pub struct SingularJsonPathQueryBuilder {
    segments: Vec<SingularSegment>,
}

impl SingularJsonPathQueryBuilder {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self { segments: vec![] }
    }

    #[inline]
    pub fn name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.segments.push(SingularSegment::Name(name.into()));
        self
    }

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
    #[must_use]
    fn from(value: SingularJsonPathQueryBuilder) -> Self {
        Self {
            segments: value.segments,
        }
    }
}

impl EmptyLogicalExprBuilder {
    #[inline]
    #[must_use]
    pub fn comparison<F>(self, cf: F) -> LogicalExprBuilder
    where
        F: FnOnce(EmptyComparisonBuilder) -> ComparisonExpr,
    {
        let comparison = cf(EmptyComparisonBuilder);
        LogicalExprBuilder {
            current: LogicalExpr::Comparison(comparison),
        }
    }

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

impl EmptyComparisonBuilder {
    #[inline]
    #[must_use]
    pub fn literal<L: Into<Literal>>(self, l: L) -> ComparisonWithLhsBuilder {
        ComparisonWithLhsBuilder {
            lhs: Comparable::Literal(l.into()),
        }
    }

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
    #[inline]
    #[must_use]
    pub fn equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::Equal,
        }
    }

    #[inline]
    #[must_use]
    pub fn not_equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::NotEqual,
        }
    }

    #[inline]
    #[must_use]
    pub fn less_than(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::Lesser,
        }
    }

    #[inline]
    #[must_use]
    pub fn lesser_or_equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::LesserOrEqual,
        }
    }

    #[inline]
    #[must_use]
    pub fn greater_than(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::Greater,
        }
    }

    #[inline]
    #[must_use]
    pub fn greater_or_equal_to(self) -> ComparisonWithLhsAndOpBuilder {
        ComparisonWithLhsAndOpBuilder {
            lhs: self.lhs,
            op: ComparisonOp::GreaterOrEqual,
        }
    }
}

impl ComparisonWithLhsAndOpBuilder {
    #[inline]
    pub fn literal<L: Into<Literal>>(self, l: L) -> ComparisonExpr {
        ComparisonExpr {
            lhs: self.lhs,
            op: self.op,
            rhs: Comparable::Literal(l.into()),
        }
    }

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
