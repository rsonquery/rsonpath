//! TODO: UPDATE! Classification of structurally significant JSON bytes.
//!
//! Provides the [`Structural`] struct and [`StructuralIterator`] trait
//! that allow effectively iterating over structural characters in a JSON document.
//!
//! Classifying [`Commas`](`Structural::Comma`) and [`Colons`](`Structural::Colon`) is disabled by default.
//! It can be enabled on demand by calling [`StructuralIterator::turn_commas_on`]/[`StructuralIterator::turn_colons_on`].
//!
//! A structural classifier needs ownership over a base [`QuoteClassifiedIterator`](`crate::classification::quotes::QuoteClassifiedIterator`).
//!
//! # Examples
//! ```rust
//! use rsonpath_lib::classification::structural::{Structural, classify_structural_characters};
//! use aligners::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": [{"y": 42}, {}]}""#;
//! let aligned = AlignedBytes::<alignment::Twice<rsonpath_lib::BlockAlignment>>::new_padded(json.as_bytes());
//! let expected = vec![
//!     Structural::Opening(0),
//!     Structural::Opening(6),
//!     Structural::Opening(7),
//!     Structural::Closing(15),
//!     Structural::Opening(18),
//!     Structural::Closing(19),
//!     Structural::Closing(20),
//!     Structural::Closing(21)
//! ];
//! let quote_classifier = rsonpath_lib::classification::quotes::classify_quoted_sequences(&aligned);
//! let actual = classify_structural_characters(quote_classifier).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```
//! ```rust
//! use rsonpath_lib::classification::structural::{Structural, classify_structural_characters};
//! use aligners::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": "[\"\"]"}""#;
//! let aligned = AlignedBytes::<alignment::Twice<rsonpath_lib::BlockAlignment>>::new_padded(json.as_bytes());
//! let expected = vec![
//!     Structural::Opening(0),
//!     Structural::Closing(14)
//! ];
//! let quote_classifier = rsonpath_lib::classification::quotes::classify_quoted_sequences(&aligned);
//! let actual = classify_structural_characters(quote_classifier).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```

use crate::classification::{quotes::QuoteClassifiedIterator, ResumeClassifierState};
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
    /// Represents the comma ',' character.
    Comma(usize),
}
use Structural::*;

impl Structural {
    /// Returns the index of the character in the document,
    /// i.e. which byte it is counting from 0.
    #[inline(always)]
    #[must_use]
    pub fn idx(self) -> usize {
        match self {
            Closing(idx) | Colon(idx) | Opening(idx) | Comma(idx) => idx,
        }
    }

    /// Add a given amount to the structural's index.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath_lib::classification::structural::Structural;
    ///
    /// let structural = Structural::Colon(42);
    /// let offset_structural = structural.offset(10);
    ///
    /// assert_eq!(structural.idx(), 42);
    /// assert_eq!(offset_structural.idx(), 52);
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn offset(self, amount: usize) -> Self {
        match self {
            Closing(idx) => Closing(idx + amount),
            Colon(idx) => Colon(idx + amount),
            Opening(idx) => Opening(idx + amount),
            Comma(idx) => Comma(idx + amount),
        }
    }
}

/// Trait for classifier iterators, i.e. finite iterators of [`Structural`] characters
/// that hold a reference to the JSON document valid for `'a`.
pub trait StructuralIterator<'a, I: QuoteClassifiedIterator<'a>>:
    Iterator<Item = Structural> + 'a
{
    /// Stop classification and return a state object that can be used to resume
    /// a classifier from the place in which the current one was stopped.
    fn stop(self) -> ResumeClassifierState<'a, I>;

    /// Resume classification from a state retrieved by stopping a classifier.
    fn resume(state: ResumeClassifierState<'a, I>) -> Self;

    /// Turn classification of [`Structural::Comma`] characters on.
    ///
    /// The `idx` passed should be the index of the byte in the input
    /// from which commas are to be classified. Passing an `idx` that
    /// does not match the index which the internal [`QuoteClassifiedIterator`]
    /// reached may result in incorrect results.
    fn turn_commas_on(&mut self, idx: usize);

    /// Turn classification of [`Structural::Comma`] characters off.
    fn turn_commas_off(&mut self);

    /// Turn classification of [`Structural::Colon`] characters on.
    ///
    /// The `idx` passed should be the index of the byte in the input
    /// from which commas are to be classified. Passing an `idx` that
    /// does not match the index which the internal [`QuoteClassifiedIterator`]
    /// reached may result in incorrect results.
    fn turn_colons_on(&mut self, idx: usize);

    /// Turn classification of [`Structural::Colon`] characters off.
    fn turn_colons_off(&mut self);
}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        use nosimd::*;

        /// Walk through the JSON document represented by `bytes` and iterate over all
        /// occurrences of structural characters in it.
        #[inline(always)]
        pub fn classify_structural_characters<'a, I: QuoteClassifiedIterator<'a>>(
            iter: I,
        ) -> impl StructuralIterator<'a, I> {
            SequentialClassifier::new(iter)
        }

        /// Resume classification using a state retrieved from a previously
        /// used classifier via the `stop` function.
        #[inline(always)]
        pub fn resume_structural_classification<'a, I: QuoteClassifiedIterator<'a>>(
            state: ResumeClassifierState<'a, I>
        ) -> impl StructuralIterator<'a, I> {
            SequentialClassifier::resume(state)
        }
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        use avx2::Avx2Classifier;

        /// Walk through the JSON document represented by `bytes` and iterate over all
        /// occurrences of structural characters in it.
        #[inline(always)]
        pub fn classify_structural_characters<'a, I: QuoteClassifiedIterator<'a>>(
            iter: I,
        ) -> impl StructuralIterator<'a, I> {
            Avx2Classifier::new(iter)
        }

        /// Resume classification using a state retrieved from a previously
        /// used classifier via the `stop` function.
        #[inline(always)]
        pub fn resume_structural_classification<'a, I: QuoteClassifiedIterator<'a>>(
            state: ResumeClassifierState<'a, I>
        ) -> impl StructuralIterator<'a, I> {
            Avx2Classifier::resume(state)
        }
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}

#[cfg(test)]
mod tests {
    use crate::classification::quotes::classify_quoted_sequences;

    use super::*;
    use aligners::AlignedBytes;

    #[test]
    fn resumption_without_commas_or_colons() {
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let bytes = AlignedBytes::new_padded(json.as_bytes());
        let quotes = classify_quoted_sequences(&bytes);

        let mut classifier = classify_structural_characters(quotes);

        assert_eq!(Some(Opening(0)), classifier.next());
        assert_eq!(Some(Opening(6)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(15)), resumed_classifier.next());
        assert_eq!(Some(Opening(22)), resumed_classifier.next());
    }

    #[test]
    fn resumption_with_commas_but_no_colons() {
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let bytes = AlignedBytes::new_padded(json.as_bytes());
        let quotes = classify_quoted_sequences(&bytes);

        let mut classifier = classify_structural_characters(quotes);
        classifier.turn_commas_on(0);

        assert_eq!(Some(Opening(0)), classifier.next());
        assert_eq!(Some(Opening(6)), classifier.next());
        assert_eq!(Some(Comma(9)), classifier.next());
        assert_eq!(Some(Comma(13)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(15)), resumed_classifier.next());
        assert_eq!(Some(Opening(22)), resumed_classifier.next());
        assert_eq!(Some(Comma(30)), resumed_classifier.next());
    }

    #[test]
    fn resumption_with_colons_but_no_commas() {
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let bytes = AlignedBytes::new_padded(json.as_bytes());
        let quotes = classify_quoted_sequences(&bytes);

        let mut classifier = classify_structural_characters(quotes);
        classifier.turn_colons_on(0);

        assert_eq!(Some(Opening(0)), classifier.next());
        assert_eq!(Some(Colon(4)), classifier.next());
        assert_eq!(Some(Opening(6)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(15)), resumed_classifier.next());
        assert_eq!(Some(Colon(20)), resumed_classifier.next());
        assert_eq!(Some(Opening(22)), resumed_classifier.next());
        assert_eq!(Some(Colon(27)), resumed_classifier.next());
    }

    #[test]
    fn resumption_with_commas_and_colons() {
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let bytes = AlignedBytes::new_padded(json.as_bytes());
        let quotes = classify_quoted_sequences(&bytes);

        let mut classifier = classify_structural_characters(quotes);
        classifier.turn_commas_on(0);
        classifier.turn_colons_on(0);

        assert_eq!(Some(Opening(0)), classifier.next());
        assert_eq!(Some(Colon(4)), classifier.next());
        assert_eq!(Some(Opening(6)), classifier.next());
        assert_eq!(Some(Comma(9)), classifier.next());
        assert_eq!(Some(Comma(13)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(15)), resumed_classifier.next());
        assert_eq!(Some(Colon(20)), resumed_classifier.next());
        assert_eq!(Some(Opening(22)), resumed_classifier.next());
        assert_eq!(Some(Colon(27)), resumed_classifier.next());
        assert_eq!(Some(Comma(30)), resumed_classifier.next());
    }
}
