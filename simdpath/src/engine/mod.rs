//! Base traits for different implementations of JSONPath execution engines.
//!
//! Defines the [`Runner`] trait that provides different ways of retrieving
//! query results from input bytes. Result types are defined in the [result]
//! module.

pub mod result;

use len_trait::Len;
use result::CountResult;

#[repr(align(4096))]
pub struct Input<T> {
    contents: T,
}

impl<T> std::ops::Deref for Input<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

#[cfg(not(feature = "nosimd"))]
impl<T> Input<T>
where
    T: Extend<char> + Len,
{
    pub fn new(src: T) -> Self {
        #[cfg(not(feature = "nosimd"))]
        {
            use crate::bytes::simd::BLOCK_SIZE;
            let mut contents = src;
            let rem = contents.len() % BLOCK_SIZE;
            let pad = if rem == 0 { 0 } else { BLOCK_SIZE - rem };

            let extension = std::iter::repeat('\0').take(pad);
            contents.extend(extension);

            debug_assert_eq!(contents.len() % BLOCK_SIZE, 0);

            Self { contents }
        }
        #[cfg(feature = "nosimd")]
        {
            Self { src }
        }
    }
}

/// Trait for an engine that can run its query on a given input.
pub trait Runner {
    /// Count the number of values satisfying the query on given input
    /// that can be interpreted as a slice of bytes.
    fn count<T: AsRef<[u8]>>(&self, input: &Input<T>) -> CountResult;
}
