//! Base traits for different implementations of JSONPath execution engines.
//!
//! Defines the [`Runner`] trait that provides different ways of retrieving
//! query results from input bytes. Result types are defined in the [result]
//! module.

pub mod result;

use result::CountResult;

/// Trait for an engine that can run its query on a given input.
pub trait Runner {
    /// Count the number of values satisfying the query on given input string
    /// interpreted as a sequence of bytes under UTF8 encoding.
    ///
    /// By default this is equivalent to calling
    /// `runner.count(input.as_bytes())`.
    fn count_utf8(&self, input: &str) -> CountResult {
        self.count(input.as_bytes())
    }

    /// Count the number of values satisfying the query on given input bytes.
    fn count(&self, input: &[u8]) -> CountResult;
}
