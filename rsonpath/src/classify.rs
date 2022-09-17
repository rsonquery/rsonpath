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
//!     Structural::Comma(16),
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

use crate::quotes::QuoteClassifiedIterator;
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
    pub fn idx(self) -> usize {
        match self {
            Closing(idx) => idx,
            Colon(idx) => idx,
            Opening(idx) => idx,
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
            Comma(idx) => Comma(idx + amount),
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
            iter: QuoteClassifiedIterator,
        ) -> impl StructuralIterator {
            SequentialClassifier::new(bytes)
        }
    }
    else if #[cfg(simd = "avx2")] {
        mod avx2;
        use avx2::Avx2Classifier;

        /// Walk through the JSON document represented by `bytes` and iterate over all
        /// occurrences of structural characters in it.
        pub fn classify_structural_characters<'a, I: QuoteClassifiedIterator<'a>>(
            iter: I,
        ) -> impl StructuralIterator<'a> {
            Avx2Classifier::new(iter)
        }
    }
    else {
        compile_error!("Target architecture is not supported by SIMD features of this crate. Disable the default `simd` feature.");
    }
}

#[cfg(test)]
mod tests {
    use aligners::AlignedBytes;

    use crate::classify::Structural;

    use super::classify_structural_characters;

    #[test]
    fn empty_string() {
        let bytes = AlignedBytes::default();
        let result = classify_structural_characters(&bytes);
        let count = result.count();

        assert_eq!(0, count);
    }

    #[test]
    fn json() {
        let json = r#"{"a": [1, 2, 3], "b": "string", "c": {"d": 42, "e": 17}}"#;
        let bytes = AlignedBytes::new_padded(json.as_bytes());
        let expected: &[Structural] = &[
            Structural::Opening(0),
            Structural::Colon(4),
            Structural::Opening(6),
            Structural::Comma(8),
            Structural::Comma(11),
            Structural::Closing(14),
            Structural::Comma(15),
            Structural::Colon(20),
            Structural::Comma(30),
            Structural::Colon(35),
            Structural::Opening(37),
            Structural::Colon(41),
            Structural::Comma(45),
            Structural::Colon(50),
            Structural::Closing(54),
            Structural::Closing(55),
        ];

        let result = classify_structural_characters(&bytes);
        let vec = result.collect::<Vec<_>>();

        assert_eq!(expected, vec);
    }

    #[test]
    fn json_with_escapes() {
        let json = r#"{"a": "Hello, World!", "b": "\"{Hello, [World]!}\""}"#;
        let bytes = AlignedBytes::new_padded(json.as_bytes());
        let expected: &[Structural] = &[
            Structural::Opening(0),
            Structural::Colon(4),
            Structural::Comma(21),
            Structural::Colon(26),
            Structural::Closing(51),
        ];

        let result = classify_structural_characters(&bytes);
        let vec = result.collect::<Vec<_>>();

        assert_eq!(expected, vec);
    }

    #[test]
    fn reverse_exclamation_point() {
        let wtf = "¡";
        let bytes = AlignedBytes::new_padded(wtf.as_bytes());
        let expected: &[Structural] = &[];

        let result = classify_structural_characters(&bytes).collect::<Vec<_>>();

        assert_eq!(expected, result);
    }
}

#[cfg(test)]
mod prop_test {
    use super::{classify_structural_characters, Structural};
    use aligners::AlignedBytes;
    use proptest::{self, collection, prelude::*};

    #[derive(Debug, Clone)]
    enum Token {
        Comma,
        Colon,
        OpeningBrace,
        OpeningBracket,
        ClosingBrace,
        ClosingBracket,
        Garbage(String),
    }

    fn token_strategy() -> impl Strategy<Value = Token> {
        prop_oneof![
            Just(Token::Comma),
            Just(Token::Colon),
            Just(Token::OpeningBrace),
            Just(Token::OpeningBracket),
            Just(Token::ClosingBrace),
            Just(Token::ClosingBracket),
            r#"[^"\\,:{\[\}\]]+"#.prop_map(Token::Garbage)
        ]
    }

    fn tokens_strategy() -> impl Strategy<Value = Vec<Token>> {
        collection::vec(token_strategy(), collection::SizeRange::default())
    }

    fn tokens_into_string(tokens: &[Token]) -> String {
        tokens
            .iter()
            .map(|x| match x {
                Token::Comma => ",",
                Token::Colon => ":",
                Token::OpeningBrace => "{",
                Token::OpeningBracket => "[",
                Token::ClosingBrace => "}",
                Token::ClosingBracket => "]",
                Token::Garbage(string) => string,
            })
            .collect::<String>()
    }

    fn tokens_into_structurals(tokens: &[Token]) -> Vec<Structural> {
        tokens
            .iter()
            .scan(0usize, |idx, x| {
                let expected = match x {
                    Token::Comma => Some(Structural::Comma(*idx)),
                    Token::Colon => Some(Structural::Colon(*idx)),
                    Token::OpeningBrace | Token::OpeningBracket => Some(Structural::Opening(*idx)),
                    Token::ClosingBrace | Token::ClosingBracket => Some(Structural::Closing(*idx)),
                    _ => None,
                };
                match x {
                    Token::Garbage(string) => *idx += string.len(),
                    _ => *idx += 1,
                }
                Some(expected)
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    fn input_string() -> impl Strategy<Value = (String, Vec<Structural>)> {
        tokens_strategy().prop_map(|x| (tokens_into_string(&x), tokens_into_structurals(&x)))
    }

    proptest! {
        #[test]
        fn classifies_correctly((input, expected) in input_string()) {
            let bytes = AlignedBytes::new_padded(input.as_bytes());

            let result = classify_structural_characters(&bytes).collect::<Vec<_>>();

            assert_eq!(expected, result);
        }
    }
}
