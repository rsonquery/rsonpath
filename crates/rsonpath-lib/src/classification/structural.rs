//! Classification of structurally significant JSON bytes.
//!
//! Provides the [`Structural`] struct and [`StructuralIterator`] trait
//! that allow effectively iterating over structural characters in a JSON document.
//!
//! Classifying [`Commas`](`Structural::Comma`) and [`Colons`](`Structural::Colon`) is disabled by default.
//! It can be enabled on demand by calling
//! [`StructuralIterator::turn_commas_on`]/[`StructuralIterator::turn_colons_on`].
//! This configuration is persisted across [`stop`](StructuralIterator::stop) and
//! [`resume`](StructuralIterator::resume) calls.
//!
//! A structural classifier needs ownership over a base
//! [`QuoteClassifiedIterator`](`crate::classification::quotes::QuoteClassifiedIterator`).
//!
//! # Examples
//! ```rust
//! use rsonpath_lib::classification::structural::{BracketType, Structural, classify_structural_characters};
//! use rsonpath_lib::input::OwnedBytes;
//!
//! let json = r#"{"x": [{"y": 42}, {}]}""#.to_owned();
//! let aligned = OwnedBytes::try_from(json).unwrap();
//! let expected = vec![
//!     Structural::Opening(BracketType::Curly, 0),
//!     Structural::Opening(BracketType::Square, 6),
//!     Structural::Opening(BracketType::Curly, 7),
//!     Structural::Closing(BracketType::Curly, 15),
//!     Structural::Opening(BracketType::Curly, 18),
//!     Structural::Closing(BracketType::Curly, 19),
//!     Structural::Closing(BracketType::Square, 20),
//!     Structural::Closing(BracketType::Curly, 21)
//! ];
//! let quote_classifier = rsonpath_lib::classification::quotes::classify_quoted_sequences(&aligned);
//! let actual = classify_structural_characters(quote_classifier).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```
//! ```rust
//! use rsonpath_lib::classification::structural::{BracketType, Structural, classify_structural_characters};
//! use rsonpath_lib::classification::quotes::classify_quoted_sequences;
//! use rsonpath_lib::input::OwnedBytes;
//!
//! let json = r#"{"x": "[\"\"]"}""#.to_owned();
//! let aligned = OwnedBytes::try_from(json).unwrap();
//! let expected = vec![
//!     Structural::Opening(BracketType::Curly, 0),
//!     Structural::Closing(BracketType::Curly, 14)
//! ];
//! let quote_classifier = classify_quoted_sequences(&aligned);
//! let actual = classify_structural_characters(quote_classifier).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```
use crate::{
    classification::{quotes::QuoteClassifiedIterator, ResumeClassifierState},
    input::Input,
    BLOCK_SIZE,
};
use cfg_if::cfg_if;

/// Defines the kinds of brackets that can be identified as structural.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum BracketType {
    /// Square brackets, '[' and ']'.
    Square,
    /// Curly braces, '{' and '}'.
    Curly,
}

/// Defines structural characters in JSON documents.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Structural {
    /// Represents the closing square or curly brace, ']' or '}'.
    Closing(BracketType, usize),
    /// Represents the colon ':' character.
    Colon(usize),
    /// Represents the opening square or curly brace, '[' or '{'.
    Opening(BracketType, usize),
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
            Closing(_, idx) | Colon(idx) | Opening(_, idx) | Comma(idx) => idx,
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
            Closing(b, idx) => Closing(b, idx + amount),
            Colon(idx) => Colon(idx + amount),
            Opening(b, idx) => Opening(b, idx + amount),
            Comma(idx) => Comma(idx + amount),
        }
    }

    /// Check if the structural represents a closing character,
    /// i.e. a [`Closing`] with either of the [`BracketType`] variants.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath_lib::classification::structural::{BracketType, Structural};
    ///
    /// let brace = Structural::Closing(BracketType::Curly, 42);
    /// let bracket = Structural::Closing(BracketType::Square, 43);
    /// let neither = Structural::Comma(44);
    ///
    /// assert!(brace.is_closing());
    /// assert!(bracket.is_closing());
    /// assert!(!neither.is_closing());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn is_closing(&self) -> bool {
        matches!(self, Closing(_, _))
    }

    /// Check if the structural represents an opening character,
    /// i.e. an [`Opening`] with either of the [`BracketType`] variants.
    ///
    /// # Examples
    /// ```rust
    /// # use rsonpath_lib::classification::structural::{BracketType, Structural};
    ///
    /// let brace = Structural::Opening(BracketType::Curly, 42);
    /// let bracket = Structural::Opening(BracketType::Square, 43);
    /// let neither = Structural::Comma(44);
    ///
    /// assert!(brace.is_opening());
    /// assert!(bracket.is_opening());
    /// assert!(!neither.is_opening());
    /// ```
    #[inline(always)]
    #[must_use]
    pub fn is_opening(&self) -> bool {
        matches!(self, Opening(_, _))
    }
}

/// Trait for classifier iterators, i.e. finite iterators of [`Structural`] characters
/// that hold a reference to the JSON document valid for `'a`.
pub trait StructuralIterator<'a, I: Input, Q, const N: usize>: Iterator<Item = Structural> + 'a {
    /// Stop classification and return a state object that can be used to resume
    /// a classifier from the place in which the current one was stopped.
    fn stop(self) -> ResumeClassifierState<'a, I, Q, N>;

    /// Resume classification from a state retrieved by stopping a classifier.
    fn resume(state: ResumeClassifierState<'a, I, Q, N>) -> Self;

    /// Turn classification of [`Structural::Colon`] characters off.
    fn turn_colons_off(&mut self);

    /// Turn classification of [`Structural::Colon`] characters on.
    ///
    /// The `idx` passed should be the index of the byte in the input
    /// from which commas are to be classified. Passing an `idx` that
    /// does not match the index which the internal [`QuoteClassifiedIterator`]
    /// reached may result in incorrect results.
    fn turn_colons_on(&mut self, idx: usize);

    /// Turn classification of [`Structural::Comma`] characters off.
    fn turn_commas_off(&mut self);

    /// Turn classification of [`Structural::Comma`] characters on.
    ///
    /// The `idx` passed should be the index of the byte in the input
    /// from which commas are to be classified. Passing an `idx` that
    /// does not match the index which the internal [`QuoteClassifiedIterator`]
    /// reached may result in incorrect results.
    fn turn_commas_on(&mut self, idx: usize);
}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        type ClassifierImpl<'a, I, Q, const N: usize> = nosimd::SequentialClassifier<'a, I, Q, N>;
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        type ClassifierImpl<'a, I, Q> = avx2::Avx2Classifier<'a, I, Q>;
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}

/// Walk through the JSON document represented by `bytes` and iterate over all
/// occurrences of structural characters in it.
#[inline(always)]
pub fn classify_structural_characters<'a, I: Input + 'a, Q: QuoteClassifiedIterator<'a, I, BLOCK_SIZE>>(
    iter: Q,
) -> impl StructuralIterator<'a, I, Q, BLOCK_SIZE> {
    ClassifierImpl::new(iter)
}

/// Resume classification using a state retrieved from a previously
/// used classifier via the `stop` function.
#[inline(always)]
pub fn resume_structural_classification<'a, I: Input, Q: QuoteClassifiedIterator<'a, I, BLOCK_SIZE>>(
    state: ResumeClassifierState<'a, I, Q, BLOCK_SIZE>,
) -> impl StructuralIterator<'a, I, Q, BLOCK_SIZE> {
    ClassifierImpl::resume(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{classification::quotes::classify_quoted_sequences, input::OwnedBytes};

    #[test]
    fn resumption_without_commas_or_colons() {
        use BracketType::*;
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let json_string = json.to_owned();
        let input = OwnedBytes::new(&json_string).unwrap();
        let quotes = classify_quoted_sequences(&input);

        let mut classifier = classify_structural_characters(quotes);

        assert_eq!(Some(Opening(Curly, 0)), classifier.next());
        assert_eq!(Some(Opening(Square, 6)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(Curly, 15)), resumed_classifier.next());
        assert_eq!(Some(Opening(Curly, 22)), resumed_classifier.next());
    }

    #[test]
    fn resumption_with_commas_but_no_colons() {
        use BracketType::*;
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let json_string = json.to_owned();
        let input = OwnedBytes::new(&json_string).unwrap();
        let quotes = classify_quoted_sequences(&input);

        let mut classifier = classify_structural_characters(quotes);
        classifier.turn_commas_on(0);

        assert_eq!(Some(Opening(Curly, 0)), classifier.next());
        assert_eq!(Some(Opening(Square, 6)), classifier.next());
        assert_eq!(Some(Comma(9)), classifier.next());
        assert_eq!(Some(Comma(13)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(Curly, 15)), resumed_classifier.next());
        assert_eq!(Some(Opening(Curly, 22)), resumed_classifier.next());
        assert_eq!(Some(Comma(30)), resumed_classifier.next());
    }

    #[test]
    fn resumption_with_colons_but_no_commas() {
        use BracketType::*;
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let json_string = json.to_owned();
        let input = OwnedBytes::new(&json_string).unwrap();
        let quotes = classify_quoted_sequences(&input);

        let mut classifier = classify_structural_characters(quotes);
        classifier.turn_colons_on(0);

        assert_eq!(Some(Opening(Curly, 0)), classifier.next());
        assert_eq!(Some(Colon(4)), classifier.next());
        assert_eq!(Some(Opening(Square, 6)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(Curly, 15)), resumed_classifier.next());
        assert_eq!(Some(Colon(20)), resumed_classifier.next());
        assert_eq!(Some(Opening(Curly, 22)), resumed_classifier.next());
        assert_eq!(Some(Colon(27)), resumed_classifier.next());
    }

    #[test]
    fn resumption_with_commas_and_colons() {
        use BracketType::*;
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let json_string = json.to_owned();
        let input = OwnedBytes::new(&json_string).unwrap();
        let quotes = classify_quoted_sequences(&input);

        let mut classifier = classify_structural_characters(quotes);
        classifier.turn_commas_on(0);
        classifier.turn_colons_on(0);

        assert_eq!(Some(Opening(Curly, 0)), classifier.next());
        assert_eq!(Some(Colon(4)), classifier.next());
        assert_eq!(Some(Opening(Square, 6)), classifier.next());
        assert_eq!(Some(Comma(9)), classifier.next());
        assert_eq!(Some(Comma(13)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(Curly, 15)), resumed_classifier.next());
        assert_eq!(Some(Colon(20)), resumed_classifier.next());
        assert_eq!(Some(Opening(Curly, 22)), resumed_classifier.next());
        assert_eq!(Some(Colon(27)), resumed_classifier.next());
        assert_eq!(Some(Comma(30)), resumed_classifier.next());
    }
}
