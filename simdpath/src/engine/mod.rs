//! Base traits for different implementations of JSONPath execution engines.
//!
//! Defines the [`Runner`] trait that provides different ways of retrieving
//! query results from input bytes. Result types are defined in the [result]
//! module.

pub mod result;

use crate::bytes::align::{alignment, AlignedBytes};
use len_trait::Len;
use result::CountResult;

/// Input into a query engine.
pub struct Input {
    bytes: AlignedBytes<alignment::Page>,
}

impl std::ops::Deref for Input {
    type Target = AlignedBytes<alignment::Page>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl Input {
    /// Transmute a buffer into an input.
    ///
    /// The buffer must know its length, may be extended by auxilliary UTF8 characters
    /// and will be interpreted as a slice of bytes at the end.
    pub fn new<T: Extend<char> + Len + AsRef<[u8]>>(src: T) -> Self {
        #[cfg(not(feature = "nosimd"))]
        {
            use crate::bytes::simd::BLOCK_SIZE;
            let mut contents = src;
            let rem = contents.len() % BLOCK_SIZE;
            let pad = if rem == 0 { 0 } else { BLOCK_SIZE - rem };

            let extension = std::iter::repeat('\0').take(pad + BLOCK_SIZE);
            contents.extend(extension);

            debug_assert_eq!(contents.len() % BLOCK_SIZE, 0);

            Self {
                bytes: AlignedBytes::<alignment::Page>::from(contents.as_ref()),
            }
        }
        #[cfg(feature = "nosimd")]
        {
            Self {
                bytes: AlignedBytes::<alignment::Page>::from(src.as_ref()),
            }
        }
    }
}

/// Trait for an engine that can run its query on a given input.
pub trait Runner {
    /// Count the number of values satisfying the query on given [`Input`].
    fn count(&self, input: &Input) -> CountResult;
}
