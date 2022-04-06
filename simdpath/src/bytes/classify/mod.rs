//! Classification of structurally significant JSON bytes.
//!
//! Provides the [`common::Structural`] struct and [`common::StructuralIterator`] trait
//! that allow effectively iterating over structural characters in a JSON document.
//!
//! # Examples
//! ```rust
//! use simdpath::bytes::{Structural, classify_structural_characters};
//! use align::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": [{"y": 42}, {}]}""#;
//! let aligned = AlignedBytes::<alignment::TwoSimdBlocks>::new_padded(json.as_bytes());
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
//! use simdpath::bytes::{Structural, classify_structural_characters};
//! use align::{alignment, AlignedBytes};
//!
//! let json = r#"{"x": "[\"\"]"}""#;
//! let aligned = AlignedBytes::<alignment::TwoSimdBlocks>::new_padded(json.as_bytes());
//! let expected = vec![
//!     Structural::Opening(0),
//!     Structural::Colon(4),
//!     Structural::Closing(14)
//! ];
//! let actual = classify_structural_characters(&aligned).collect::<Vec<Structural>>();
//! assert_eq!(expected, actual);
//! ```

use align::{alignment, AlignedSlice};
use cfg_if::cfg_if;

mod common;
pub use common::*;

cfg_if! {
    if #[cfg(all(
            any(target_arch = "x86_64", target_arch = "x86"),
            target_feature = "avx2",
            not(feature = "nosimd")
    ))] {
        mod avx2;
    }
}

mod nosimd;

/// Walk through the JSON document represented by `bytes` and iterate over all
/// occurrences of structural characters in it.
#[inline(always)]
pub fn classify_structural_characters(
    bytes: &AlignedSlice<alignment::TwoSimdBlocks>,
) -> impl StructuralIterator {
    cfg_if! {
        if #[cfg(all(
                any(target_arch = "x86_64", target_arch = "x86"),
                target_feature = "avx2",
                not(feature = "nosimd")
        ))] {
            avx2::Avx2Classifier::new(bytes)
        }
        else {
            nosimd::SequentialClassifier::new(bytes)
        }
    }
}
