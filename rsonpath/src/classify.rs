//! Classification of structurally significant JSON bytes.
//!
//! Provides the [`Structural`] struct and [`StructuralIterator`] trait
//! that allow effectively iterating over structural characters in a JSON document.
//!
//! A structural classifier needs ownership over a base [`QuoteClassifiedIterator`](`crate::quotes::QuoteClassifiedIterator`).
//!
//! # Examples
//! ```rust
//! use rsonpath::classify::{Structural, classify_structural_characters};
//! use aligners::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": [{"y": 42}, {}]}""#;
//! let aligned = AlignedBytes::<alignment::Twice<rsonpath::BlockAlignment>>::new_padded(json.as_bytes());
//! let expected = vec![
//!     Structural::Opening(0),
//!     Structural::Colon(4),
//!     Structural::Opening(6),
//!     Structural::Opening(7),
//!     Structural::Colon(11),
//!     Structural::Closing(15),
#![cfg_attr(feature = "commas", doc = "Structural::Comma(16),")]
//!     Structural::Opening(18),
//!     Structural::Closing(19),
//!     Structural::Closing(20),
//!     Structural::Closing(21)
//! ];
//! let quote_classifier = rsonpath::quotes::classify_quoted_sequences(&aligned);
//! let actual = classify_structural_characters(quote_classifier).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```
//! ```rust
//! use rsonpath::classify::{Structural, classify_structural_characters};
//! use aligners::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": "[\"\"]"}""#;
//! let aligned = AlignedBytes::<alignment::Twice<rsonpath::BlockAlignment>>::new_padded(json.as_bytes());
//! let expected = vec![
//!     Structural::Opening(0),
//!     Structural::Colon(4),
//!     Structural::Closing(14)
//! ];
//! let quote_classifier = rsonpath::quotes::classify_quoted_sequences(&aligned);
//! let actual = classify_structural_characters(quote_classifier).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```

use std::marker::PhantomData;

use crate::{
    debug,
    depth::{resume_depth_classification, DepthBlock, DepthIterator, DepthIteratorResumeOutcome},
    quotes::{QuoteClassifiedIterator, ResumeClassifierState},
};
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
    #[cfg(feature = "commas")]
    /// Represents the comma ',' character.
    Comma(usize),
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
            #[cfg(feature = "commas")]
            Comma(idx) => idx,
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
            #[cfg(feature = "commas")]
            Comma(idx) => Comma(idx + amount),
        }
    }
}

pub(crate) struct ClassifierWithSkipping<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    classifier: Option<I>,
    phantom: PhantomData<&'b Q>,
}

impl<'b, Q, I> ClassifierWithSkipping<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    pub(crate) fn new(classifier: I) -> Self {
        Self {
            classifier: Some(classifier),
            phantom: PhantomData,
        }
    }

    pub(crate) fn skip(&mut self, opening: u8) {
        debug!("Skipping");

        let classifier = unsafe { self.classifier.take().unwrap_unchecked() };
        let resume_state = classifier.stop();
        let DepthIteratorResumeOutcome(first_vector, mut depth_classifier) =
            resume_depth_classification(resume_state, opening);

        let mut current_vector = first_vector.or_else(|| depth_classifier.next());
        let mut current_depth = 1;

        'outer: while let Some(ref mut vector) = current_vector {
            vector.add_depth(current_depth);

            debug!("Fetched vector, current depth is {current_depth}");
            debug!("Estimate: {}", vector.estimate_lowest_possible_depth());

            while vector.estimate_lowest_possible_depth() <= 0
                && vector.advance_to_next_depth_decrease()
            {
                if vector.get_depth() == 0 {
                    debug!("Encountered depth 0, breaking.");
                    break 'outer;
                }
            }

            current_depth = vector.depth_at_end();
            current_vector = depth_classifier.next();
        }

        debug!("Skipping complete, resuming structural classification.");
        let resume_state = depth_classifier.stop(current_vector);
        self.classifier = Some(I::resume(resume_state));
    }
    
    pub(crate) fn stop(mut self) -> ResumeClassifierState<'b, Q> {
        unsafe { self.classifier.take().unwrap_unchecked() }.stop()
    }
}

impl<'b, Q, I> std::ops::Deref for ClassifierWithSkipping<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        unsafe { self.classifier.as_ref().unwrap_unchecked() }
    }
}

impl<'b, Q, I> std::ops::DerefMut for ClassifierWithSkipping<'b, Q, I>
where
    Q: QuoteClassifiedIterator<'b>,
    I: StructuralIterator<'b, Q>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.classifier.as_mut().unwrap_unchecked() }
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

    /// Resume classification from a state retrieved by a previous
    /// [`StructuralIterator::stop`] or [`DepthIterator::stop`] invocation.
    fn resume(state: ResumeClassifierState<'a, I>) -> Self;
}

cfg_if! {
    if #[cfg(any(doc, not(feature = "simd")))] {
        mod nosimd;
        use nosimd::*;

        /// Walk through the JSON document represented by `bytes` and iterate over all
        /// occurrences of structural characters in it.
        pub fn classify_structural_characters<'a, I: QuoteClassifiedIterator<'a>>(
            iter: I,
        ) -> impl StructuralIterator<'a, I> {
            SequentialClassifier::new(iter)
        }

        /// Resume classification using a state retrieved from a previously
        /// used classifier via the `stop` function.
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
        pub fn classify_structural_characters<'a, I: QuoteClassifiedIterator<'a>>(
            iter: I,
        ) -> impl StructuralIterator<'a, I> {
            Avx2Classifier::new(iter)
        }

        /// Resume classification using a state retrieved from a previously
        /// used classifier via the `stop` function.
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
    use crate::quotes::classify_quoted_sequences;

    use super::*;
    use aligners::AlignedBytes;

    #[test]
    fn resumption() {
        use Structural::*;

        let json = r#"{"a": [42, 36, { "b": { "c": 1, "d": 2 } }]}"#;
        let bytes = AlignedBytes::new_padded(json.as_bytes());
        let quotes = classify_quoted_sequences(&bytes);

        let mut classifier = classify_structural_characters(quotes);

        assert_eq!(Some(Opening(0)), classifier.next());
        assert_eq!(Some(Colon(4)), classifier.next());
        assert_eq!(Some(Opening(6)), classifier.next());
        #[cfg(feature = "commas")]
        assert_eq!(Some(Comma(9)), classifier.next());
        #[cfg(feature = "commas")]
        assert_eq!(Some(Comma(13)), classifier.next());

        let resume_state = classifier.stop();

        let mut resumed_classifier = resume_structural_classification(resume_state);

        assert_eq!(Some(Opening(15)), resumed_classifier.next());
        assert_eq!(Some(Colon(20)), resumed_classifier.next());
        assert_eq!(Some(Opening(22)), resumed_classifier.next());
        assert_eq!(Some(Colon(27)), resumed_classifier.next());
        #[cfg(feature = "commas")]
        assert_eq!(Some(Comma(30)), resumed_classifier.next());
    }
}
