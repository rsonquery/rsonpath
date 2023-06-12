use crate::version;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(name = "rq", author, version, about)]
#[clap(long_version = version::get_long_version())]
pub struct Args {
    /// JSONPath query to run against the input JSON.
    pub query: String,
    /// Input JSON file to query.
    ///
    /// If not specified uses the standard input stream.
    pub file_path: Option<String>,
    /// Include verbose debug information.
    #[clap(short, long)]
    pub verbose: bool,
    /// TODO: REMOVE
    #[clap(short, long, default_value_t = false)]
    pub use_mmap: bool,
    /// Engine to use for evaluating the query.
    #[clap(short, long, value_enum, default_value_t = EngineArg::Main)]
    pub engine: EngineArg,
    /// Only compile the query and output the automaton, do not run the engine.
    ///
    /// Cannot be used with --engine or FILE_PATH.
    #[clap(short, long)]
    #[arg(conflicts_with = "engine")]
    #[arg(conflicts_with = "file_path")]
    pub compile: bool,
    /// Result reporting mode.
    #[clap(short, long, value_enum, default_value_t = ResultArg::Bytes)]
    pub result: ResultArg,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineArg {
    /// Main SIMD-optimized iterative engine.
    Main,
    /// Alternative recursive engine.
    Recursive,
    /// Use both engines and verify that their outputs match.
    ///
    /// This is for testing purposes only.
    VerifyBoth,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultArg {
    /// Return a list of all bytes at which a match occurred.
    Bytes,
    /// Return only the number of matches.
    Count,
}
