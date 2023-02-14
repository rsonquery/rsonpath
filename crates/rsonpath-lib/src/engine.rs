//! Base traits for different implementations of JSONPath execution engines.
//!
//! Defines the [`Engine`] trait that provides different ways of retrieving
//! query results from input bytes, as well as [`Compiler`] which provides
//! a standalone entry point for compiling a [`JsonPathQuery`] into an [`Engine`].
mod depth;
pub mod error;
#[cfg(feature = "head-skip")]
mod head_skipping;
pub mod main;
pub mod recursive;
#[cfg(feature = "tail-skip")]
mod tail_skipping;

pub use main::MainEngine as RsonpathEngine;

use self::error::EngineError;
use crate::query::{automaton::Automaton, error::CompilerError, JsonPathQuery};
use crate::result::QueryResult;
use aligners::{
    alignment::{self},
    AlignedBytes,
};
use cfg_if::cfg_if;

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
    #[must_use]
    #[inline]
    pub fn new<T: Extend<char> + AsRef<[u8]>>(src: &mut T) -> Self {
        use aligners::alignment::Alignment;
        type A = alignment::Twice<crate::BlockAlignment>;
        let contents = src;
        let rem = contents.as_ref().len() % A::size();
        let pad = if rem == 0 { 0 } else { A::size() - rem };

        let extension = std::iter::repeat('\0').take(pad + A::size());
        contents.extend(extension);

        debug_assert_eq!(contents.as_ref().len() % A::size(), 0);

        Self {
            bytes: AlignedBytes::<alignment::Page>::from(contents.as_ref()),
        }
    }

    /// Transmute a buffer into an input.
    ///
    /// The buffer must know its length, may be extended by auxiliary bytes.
    #[inline]
    pub fn new_bytes<T: Extend<u8> + AsRef<[u8]>>(src: &mut T) -> Self {
        cfg_if! {
            if #[cfg(feature = "simd")] {
                use aligners::alignment::Alignment;
                type A = alignment::Twice::<crate::BlockAlignment>;
                let contents = src;
                let rem = contents.as_ref().len() % A::size();
                let pad = if rem == 0 {
                    0
                } else {
                    A::size() - rem
                };

                let extension = std::iter::repeat(0).take(pad + A::size());
                contents.extend(extension);

                debug_assert_eq!(contents.as_ref().len() % A::size(), 0);

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
pub trait Engine {
    /// Compute the [`QueryResult`] on given [`Input`].
    ///
    /// # Errors
    /// An appropriate [`EngineError`] is returned if the JSON input is malformed
    /// and the syntax error is detected.
    ///
    /// **Please note** that detecting malformed JSONs is not guaranteed.
    /// Some glaring errors like mismatched braces or double quotes are raised,
    /// but in general the result of an engine run on an invalid JSON is undefined.
    /// It _is_ guaranteed that the computation terminates and does not panic.
    fn run<R: QueryResult>(&self, input: &Input) -> Result<R, EngineError>;
}

/// Trait for an engine that can be created by compiling a [`JsonPathQuery`].
pub trait Compiler {
    /// Concrete type of the [`Engines`](`Engine`) created,
    /// parameterized with the lifetime of the input query.
    type E<'q>: Engine + 'q;

    /// Compile a [`JsonPathQuery`] into an [`Engine`].
    ///
    /// # Errors
    /// An appropriate [`CompilerError`] is returned if the compiler
    /// cannot handle the query.
    fn compile_query(query: &JsonPathQuery) -> Result<Self::E<'_>, CompilerError>;

    /// Turn a compiled [`Automaton`] into an [`Engine`].
    fn from_compiled_query(automaton: Automaton<'_>) -> Self::E<'_>;
}
