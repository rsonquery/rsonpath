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
//! - **slice**,
//! - and **filter**.
//!
//! Descriptions of each segment and selector can be found in the documentation of the
//! relevant type in this crate, while the formal grammar is described in the RFC.
//!
//! ## State of the crate
//!
//! This is an in-development version that supports only name, index, and wildcard selectors.
//! However, these are fully supported, tested, and fuzzed. The planned roadmap is:
//! - support slices
//! - support filters (without functions)
//! - support functions (including type check)
//! - polish the API
//! - 1.0.0 stable release
//!
//! ## Examples
//! To create a query from a query string:
//! ```
//! # use rsonpath_syntax::{JsonPathQuery, Segment, Selectors, Selector, str::JsonString};
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
//! # Ok(())
//! # }
//! ```
//!
//! ## Crate features
//!
//! There are two optional features:
//! - `arbitrary`, which enables a dependency on the [`arbitrary` crate](https://docs.rs/arbitrary/latest/arbitrary/)
//!   to provide [`Arbitrary`](`arbitrary::Arbitrary`) implementations on query types; this is used e.g. for fuzzing.
//! - `color`, which enables a dependency on the [`owo_colors` crate](https://docs.rs/owo-colors/latest/owo_colors/)
//!   to provide colorful [`Display`] representations of [`ParseError`](error::ParseError);
//!   see [`ParseError::colored`](error::ParseError::colored).

#![forbid(unsafe_code)]
// Generic pedantic lints.
#![warn(
    explicit_outlives_requirements,
    semicolon_in_expressions_from_macros,
    unreachable_pub,
    unused_import_braces,
    unused_lifetimes
)]
// Clippy pedantic lints.
#![warn(
    clippy::allow_attributes_without_reason,
    clippy::cargo_common_metadata,
    clippy::cast_lossless,
    clippy::cloned_instead_of_copied,
    clippy::empty_drop,
    clippy::empty_line_after_outer_attr,
    clippy::equatable_if_let,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::fallible_impl_from,
    clippy::flat_map_option,
    clippy::if_then_some_else_none,
    clippy::inconsistent_struct_constructor,
    clippy::large_digit_groups,
    clippy::let_underscore_must_use,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_inline_in_public_items,
    clippy::mod_module_files,
    clippy::must_use_candidate,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::ptr_as_ptr,
    clippy::redundant_closure_for_method_calls,
    clippy::ref_binding_to_reference,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::undocumented_unsafe_blocks,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::use_self
)]
// Panic-free lint.
#![warn(clippy::exit)]
// Panic-free lints (disabled for tests).
#![cfg_attr(not(test), warn(clippy::unwrap_used))]
// IO hygiene, only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(clippy::print_stderr, clippy::print_stdout, clippy::todo)
)]
// Documentation lints, enabled only on --release.
#![cfg_attr(
    not(debug_assertions),
    warn(missing_docs, clippy::missing_errors_doc, clippy::missing_panics_doc,)
)]
#![cfg_attr(not(debug_assertions), warn(rustdoc::missing_crate_level_docs))]
// Docs.rs config.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_logo_url = "https://raw.githubusercontent.com/V0ldek/rsonpath/main/img/rsonquery-logo.svg")]

pub mod builder;
pub mod error;
pub mod num;
mod parser;
pub mod str;

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
    /// Parse a JSONPath query string.
    ///
    /// ## Errors
    /// Fails if the string does not represent a valid JSONPath query
    /// as governed by the [JSONPath RFC specification](https://www.ietf.org/archive/id/draft-ietf-jsonpath-base-21.html).
    ///
    /// Note that leading and trailing whitespace is explicitly disallowed by the spec.
    #[inline]
    pub fn parse(&self, str: &str) -> Result<JsonPathQuery> {
        crate::parser::parse_json_path_query(str, &self.options)
    }
}

/// JSONPath query segment.
///
/// Every query is a sequence of zero or more of segments,
/// each applying one or more selectors to a node and passing it along to the
/// subsequent segments.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Segment {
    /// A child segment contains a sequence of selectors,
    /// each of which selects zero or more children of a node.
    Child(Selectors),
    /// A child segment contains a sequence of selectors,
    /// each of which selects zero or more descendants of a node.
    Descendant(Selectors),
}

// We don't derive this because an empty Vec of Selectors is not a valid representation.
#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for Selectors {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let first = u.arbitrary::<Selector>()?;
        let mut rest = u.arbitrary::<Vec<Selector>>()?;
        rest.push(first);

        Ok(Self::many(rest))
    }
}

/// Collection of one or more [`Selector`] instances.
///
/// Guaranteed to be non-empty.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Selectors {
    inner: Vec<Selector>,
}

/// Each [`Segment`] defines one or more selectors.
/// A selector produces one or more children/descendants of the node it is applied to.
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Selector {
    /// A name selector selects at most one object member value under the key equal to the
    /// selector's [`JsonString`](str::JsonString).
    Name(str::JsonString),
    /// A wildcard selector selects the nodes of all children of an object or array.
    Wildcard,
    /// An index selector matches at most one array element value,
    /// depending on the selector's [`Index`].
    Index(Index),
}

/// Directional index into a JSON array.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Index {
    /// Zero-based index from the start of the array.
    FromStart(num::JsonUInt),
    /// Index from the end of the array.
    ///
    /// `-1` is the last element, `-2` is the second last, etc.
    FromEnd(num::JsonUInt),
}

// We don't derive this because FromEnd(0) is not a valid index.
#[cfg(feature = "arbitrary")]
#[cfg_attr(docsrs, doc(cfg(feature = "arbitrary")))]
impl<'a> arbitrary::Arbitrary<'a> for Index {
    #[inline]
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let num = u.arbitrary::<num::JsonInt>()?;
        Ok(Self::from(num))
    }
}

impl From<num::JsonInt> for Index {
    #[inline]
    fn from(value: num::JsonInt) -> Self {
        if value.as_i64() >= 0 {
            Self::FromStart(value.abs())
        } else {
            Self::FromEnd(value.abs())
        }
    }
}

/// JSONPath query structure represented by a sequence of [`Segments`](Segment).
#[derive(Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct JsonPathQuery {
    segments: Vec<Segment>,
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
    /// let index = Index::FromEnd((-1).into());
    /// assert!(index.is_end_based());
    /// ```
    #[inline(always)]
    #[must_use]
    pub const fn is_end_based(&self) -> bool {
        matches!(self, Self::FromEnd(_))
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
