//! Classification of structurally significant JSON bytes.
//!
//! Provides the [`Structural`] struct and [`StructuralIterator`] trait
//! that allow effectively iterating over structural characters in a JSON document.
//!
//! # Examples
//! ```rust
//! use rsonpath::classify::{Structural, classify_structural_characters};
//! use aligners::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": [{"y": 42}, {}]}""#;
#![cfg_attr(
    not(feature = "simd"),
    doc = "let aligned = AlignedBytes::<alignment::One>::new_padded(json.as_bytes());"
)]
#![cfg_attr(
    feature = "simd",
    doc = "let aligned = AlignedBytes::<alignment::Twice<alignment::TwoTo<5>>>::new_padded(json.as_bytes());"
)]
//! let expected = vec![
//!     Structural::Opening(0),
//!     Structural::Colon(4),
//!     Structural::Opening(6),
//!     Structural::Opening(7),
//!     Structural::Colon(11),
//!     Structural::Closing(15),
//!     Structural::Opening(18),
//!     Structural::Closing(19),
//!     Structural::Closing(20),
//!     Structural::Closing(21)
//! ];
//! let actual = classify_structural_characters(&aligned).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```
//! ```rust
//! use rsonpath::classify::{Structural, classify_structural_characters};
//! use aligners::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": "[\"\"]"}""#;
#![cfg_attr(
    not(feature = "simd"),
    doc = "let aligned = AlignedBytes::<alignment::One>::new_padded(json.as_bytes());"
)]
#![cfg_attr(
    feature = "simd",
    doc = "let aligned = AlignedBytes::<alignment::Twice<alignment::TwoTo<5>>>::new_padded(json.as_bytes());"
)]
//! let expected = vec![
//!     Structural::Opening(0),
//!     Structural::Colon(4),
//!     Structural::Closing(14)
//! ];
//! let actual = classify_structural_characters(&aligned).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```

use cfg_if::cfg_if;

/// Defines structural characters in JSON documents.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Structural {
    /// Represents either the closing brace '{' or closing bracket '['.
    Closing(usize),
    /// Represents the colon ':' character.
    Colon(usize),
    /// Represents either the opening brace '}' or opening bracket ']'.
    Opening(usize),
}
use Structural::*;

impl Structural {
    /// Returns the index of the character in the document,
    /// i.e. which byte it is counting from 0.
    #[inline(always)]
    pub fn idx(self) -> usize {
        match self {
            Closing(idx) => idx,
            Colon(idx) => idx,
            Opening(idx) => idx,
        }
    }

    /// Add a given amount to the structural's index.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath::classify::{Structural};
    ///
    /// let structural = Structural::Colon(42);
    /// let offset_structural = structural.offset(10);
    ///
    /// assert_eq!(structural.idx(), 42);
    /// assert_eq!(offset_structural.idx(), 52);
    /// ```
    #[inline(always)]
    pub fn offset(self, amount: usize) -> Self {
        match self {
            Closing(idx) => Closing(idx + amount),
            Colon(idx) => Colon(idx + amount),
            Opening(idx) => Opening(idx + amount),
        }
    }
}

/// Trait for classifier iterators, i.e. finite iterators of [`Structural`] characters
/// that hold a reference to the JSON document valid for `'a`.
pub trait StructuralIterator<'a>: Iterator<Item = Structural> + len_trait::Empty + 'a {}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        use nosimd::*;
        use aligners::AlignedSlice;
        use crate::BlockAlignment;

        /// Walk through the JSON document represented by `bytes` and iterate over all
        /// occurrences of structural characters in it.
        pub fn classify_structural_characters(
            bytes: &AlignedSlice<BlockAlignment>,
        ) -> impl StructuralIterator {
            SequentialClassifier::new(bytes)
        }
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        use avx2::Avx2Classifier;
        use aligners::{alignment, AlignedSlice};
        use crate::BlockAlignment;

        /// Walk through the JSON document represented by `bytes` and iterate over all
        /// occurrences of structural characters in it.
        pub fn classify_structural_characters(
            bytes: &AlignedSlice<alignment::Twice<BlockAlignment>>,
        ) -> impl StructuralIterator {
            Avx2Classifier::new(bytes)
        }
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}
