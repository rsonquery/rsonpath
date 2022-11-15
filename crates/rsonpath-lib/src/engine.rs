//! Base traits for different implementations of JSONPath execution engines.
//!
//! Defines the [`Runner`] trait that provides different ways of retrieving
//! query results from input bytes. Result types are defined in the [result]
//! module.

pub mod result;

use aligners::{
    alignment::{self},
    AlignedBytes,
};
use cfg_if::cfg_if;
use len_trait::Len;

use self::result::QueryResult;

/// Input into a query engine.
pub struct Input {
    bytes: AlignedBytes<alignment::Page>,
}

impl std::ops::Deref for Input {
    type Target = AlignedBytes<alignment::Page>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl Input {
    /// Transmute a buffer into an input.
    ///
    /// The buffer must know its length, may be extended by auxiliary UTF8 characters
    /// and will be interpreted as a slice of bytes at the end.
    #[inline]
    pub fn new<T: Extend<char> + Len + AsRef<[u8]>>(src: T) -> Self {
        cfg_if! {
            if #[cfg(feature = "simd")] {
                use aligners::alignment::Alignment;
                type A = alignment::Twice::<crate::BlockAlignment>;
                let mut contents = src;
                let rem = contents.len() % A::size();
                let pad = if rem == 0 {
                    0
                } else {
                    A::size() - rem
                };

                let extension = std::iter::repeat('\0').take(pad + A::size());
                contents.extend(extension);

                debug_assert_eq!(contents.len() % A::size(), 0);

                Self {
                    bytes: AlignedBytes::<alignment::Page>::from(contents.as_ref()),
                }
            }
            else {
                Self {
                    bytes: AlignedBytes::<alignment::Page>::from(src.as_ref()),
                }
            }
        }
    }

    /// Transmute a buffer into an input.
    ///
    /// The buffer must know its length, may be extended by auxiliary bytes.
    #[inline]
    pub fn new_bytes<T: Extend<u8> + Len + AsRef<[u8]>>(src: T) -> Self {
        cfg_if! {
            if #[cfg(feature = "simd")] {
                use aligners::alignment::Alignment;
                type A = alignment::Twice::<crate::BlockAlignment>;
                let mut contents = src;
                let rem = contents.len() % A::size();
                let pad = if rem == 0 {
                    0
                } else {
                    A::size() - rem
                };

                let extension = std::iter::repeat(0).take(pad + A::size());
                contents.extend(extension);

                debug_assert_eq!(contents.len() % A::size(), 0);

                Self {
                    bytes: AlignedBytes::<alignment::Page>::from(contents.as_ref()),
                }
            }
            else {
                Self {
                    bytes: AlignedBytes::<alignment::Page>::from(src.as_ref()),
                }
            }
        }
    }
}

/// Trait for an engine that can run its query on a given input.
pub trait Runner {
    /// Compute the [`QueryResult`] on given [`Input`].
    fn run<R: QueryResult>(&self, input: &Input) -> R;
}
