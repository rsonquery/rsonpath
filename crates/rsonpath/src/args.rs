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
    /// Only compile the query and output the automaton, do not run the engine.
    ///
    /// Cannot be used with FILE_PATH.
    #[clap(short, long)]
    #[arg(conflicts_with = "file_path")]
    pub compile: bool,
    /// Result reporting mode.
    #[clap(short, long, value_enum, default_value_t = ResultArg::Nodes)]
    pub result: ResultArg,
    /// Bypass automatic resolution of the input strategy and force a specific one.
    ///
    /// This is not recommended in general, since the app automatically picks a strategy
    /// that will result in best performance. It might be useful, however, if the automatic
    /// resolution picks a subpar strategy, or if it is known ahead of time that memory maps
    /// are not available and there is no need for the app to try to create one.
    #[clap(long)]
    pub force_input: Option<InputArg>,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultArg {
    /// Return a list of all bytes at which a match occurred.
    Bytes,
    /// Return only the number of matches.
    Count,
    /// Returns the full text of the matched nodes.
    Nodes,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputArg {
    /// Use a memory map over a file.
    ///
    /// This is preferred by default, if available on a given platform.
    /// Forcing usage of mmap when it is not available will cause an error and will not try a different input.
    Mmap,
    /// Eagerly load all the input into memory before running the query.
    Eager,
    /// Read the input in chunks with a buffer.
    Buffered,
}
