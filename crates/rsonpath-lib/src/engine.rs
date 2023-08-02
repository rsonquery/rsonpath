//! Base traits for different implementations of JSONPath execution engines.
//!
//! Defines the [`Engine`] trait that provides different ways of retrieving
//! query results from input bytes, as well as [`Compiler`] which provides
//! a standalone entry point for compiling a [`JsonPathQuery`] into an [`Engine`].
pub mod error;
mod head_skipping;
pub mod main;
mod tail_skipping;
pub use main::MainEngine as RsonpathEngine;

use self::error::EngineError;
use crate::{
    input::Input,
    query::{automaton::Automaton, error::CompilerError, JsonPathQuery},
    result::{Match, MatchCount, MatchIndex, Sink},
};

/// An engine that can run its query on a given input.
pub trait Engine {
    /// Find the number of matches on the given [`Input`].
    ///
    /// The result is equivalent to using [`run`](Engine::run) and counting the matches,
    /// but in general is much more time and memory efficient.
    ///
    /// # Errors
    /// An appropriate [`EngineError`] is returned if the JSON input is malformed
    /// and the syntax error is detected.
    ///
    /// **Please note** that detecting malformed JSONs is not guaranteed.
    /// Some glaring errors like mismatched braces or double quotes are raised,
    /// but in general **the result of an engine run on an invalid JSON is undefined**.
    /// It _is_ guaranteed that the computation terminates and does not panic.
    fn count<I>(&self, input: &I) -> Result<MatchCount, EngineError>
    where
        I: Input;

    /// Find the starting indices of matches on the given [`Input`].
    ///
    /// The result is equivalent to using [`run`](Engine::run) and extracting the
    /// [`Match::span.start_idx`],
    /// but in general is much more time and memory efficient.
    ///
    /// # Errors
    /// An appropriate [`EngineError`] is returned if the JSON input is malformed
    /// and the syntax error is detected.
    ///
    /// **Please note** that detecting malformed JSONs is not guaranteed.
    /// Some glaring errors like mismatched braces or double quotes are raised,
    /// but in general **the result of an engine run on an invalid JSON is undefined**.
    /// It _is_ guaranteed that the computation terminates and does not panic.
    fn indices<I, S>(&self, input: &I, sink: &mut S) -> Result<(), EngineError>
    where
        I: Input,
        S: Sink<MatchIndex>;

    /// Compute a result using the given [`Recorder`] type on given [`Input`].
    ///
    /// # Errors
    /// An appropriate [`EngineError`] is returned if the JSON input is malformed
    /// and the syntax error is detected.
    ///
    /// **Please note** that detecting malformed JSONs is not guaranteed.
    /// Some glaring errors like mismatched braces or double quotes are raised,
    /// but in general **the result of an engine run on an invalid JSON is undefined**.
    /// It _is_ guaranteed that the computation terminates and does not panic.
    fn matches<I, S>(&self, input: &I, sink: &mut S) -> Result<(), EngineError>
    where
        I: Input,
        S: Sink<Match>;
}

/// An engine that can be created by compiling a [`JsonPathQuery`].
pub trait Compiler {
    /// Concrete type of the [`Engines`](`Engine`) created,
    /// parameterized with the lifetime of the input query.
    type E<'q>: Engine + 'q;

    /// Compile a [`JsonPathQuery`] into an [`Engine`].c
    ///
    /// # Errors
    /// An appropriate [`CompilerError`] is returned if the compiler
    /// cannot handle the query.
    fn compile_query(query: &JsonPathQuery) -> Result<Self::E<'_>, CompilerError>;

    /// Turn a compiled [`Automaton`] into an [`Engine`].
    fn from_compiled_query(automaton: Automaton<'_>) -> Self::E<'_>;
}
