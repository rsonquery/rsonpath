//! Utility for building a [`JsonPathQuery`](`crate::JsonPathQuery`)
//! programmatically.
use crate::{num::JsonInt, str::JsonString, Index, JsonPathQuery, Segment, Selector, Selectors};

/// Builder for [`JsonPathQuery`] instances.
///
/// # Examples
/// ```
/// # use rsonpath_syntax::{JsonPathQuery, builder::JsonPathQueryBuilder, str::JsonString};
/// let mut builder = JsonPathQueryBuilder::new();
///     
/// builder.child_name("a")
///     .descendant_name("b")
///     .child_any()
///     .child_name("c")
///     .descendant_any();
///
/// // Can also use `builder.build()` as a non-consuming version.
/// let query: JsonPathQuery = builder.into();
///
/// assert_eq!(format!("{query}"), "$['a']..['b'][*]['c']..[*]");
/// ```
pub struct JsonPathQueryBuilder {
    segments: Vec<Segment>,
}

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

    #[inline]
    pub fn child<F>(&mut self, segment_builder: F) -> &mut Self
    where
        F: FnOnce(&mut JsonPathSelectorsBuilder) -> &mut JsonPathSelectorsBuilder,
    {
        let mut builder = JsonPathSelectorsBuilder::new();
        segment_builder(&mut builder);
        self.segments.push(Segment::Child(builder.build()));
        self
    }

    #[inline]
    pub fn descendant<F>(&mut self, segment_builder: F) -> &mut Self
    where
        F: FnOnce(&mut JsonPathSelectorsBuilder) -> &mut JsonPathSelectorsBuilder,
    {
        let mut builder = JsonPathSelectorsBuilder::new();
        segment_builder(&mut builder);
        self.segments.push(Segment::Descendant(builder.build()));
        self
    }

    #[inline(always)]
    pub fn child_name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.child(|x| x.name(name))
    }

    #[inline(always)]
    pub fn child_any(&mut self) -> &mut Self {
        self.child(|x| x.any())
    }

    #[inline(always)]
    pub fn child_index<N: Into<JsonInt>>(&mut self, idx: N) -> &mut Self {
        self.child(|x| x.index(idx))
    }

    #[inline(always)]
    pub fn descendant_name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.descendant(|x| x.name(name))
    }

    #[inline(always)]
    pub fn descendant_any(&mut self) -> &mut Self {
        self.descendant(|x| x.any())
    }

    #[inline(always)]
    pub fn descendant_index<N: Into<JsonInt>>(&mut self, idx: N) -> &mut Self {
        self.descendant(|x| x.index(idx))
    }

    /// Consume the builder and produce a [`JsonPathQuery`].
    #[must_use]
    #[inline]
    pub fn build(&mut self) -> JsonPathQuery {
        JsonPathQuery {
            segments: self.segments.clone(),
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

    #[inline(always)]
    pub fn name<S: Into<JsonString>>(&mut self, name: S) -> &mut Self {
        self.selectors.push(Selector::Name(name.into()));
        self
    }

    #[inline(always)]
    pub fn index<N: Into<JsonInt>>(&mut self, idx: N) -> &mut Self {
        let json_int: JsonInt = idx.into();
        self.selectors.push(Selector::Index(Index::from(json_int)));
        self
    }

    #[inline(always)]
    pub fn any(&mut self) -> &mut Self {
        self.selectors.push(Selector::Wildcard);
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
