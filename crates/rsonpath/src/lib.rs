pub use crate::args::{Args, InputArg, ResultArg};
use crate::error::{report_compiler_error, report_parser_error};
use crate::runner::{BenchmarkRunOutput, Runner};
use clap::Parser;
use color_eyre::{eyre::Result, Help};
use log::*;
use rsonpath_lib::automaton::Automaton;
use rsonpath_syntax::{JsonPathQuery, ParserBuilder};
use web_time::{Duration, Instant};

pub mod args;
mod error;
mod input;
mod logger;
mod runner;
mod version;

pub fn run_main() -> Result<RunOutput> {
    use color_eyre::owo_colors::OwoColorize;

    let args = Args::parse();

    logger::configure(args.verbose)?;

    run_with_args(&args).map_err(|err| err.with_note(|| format!("Query string: '{}'.", args.query.dimmed())))
}

pub fn run_with_args(args: &Args) -> Result<RunOutput> {
    // Benchmark parsing
    let parse_start = Instant::now();
    let query = parse_query(&args.query)?;
    let parse_time = parse_start.elapsed();

    let compile_start = Instant::now();
    let automaton = compile_query(&query)?;
    let compile_time = compile_start.elapsed();

    let out = String::new();
    let err = String::new();

    if args.compile {
        println!("{automaton}");
        debug!("{automaton:?}");
        Ok(RunOutput {
            stdout: out,
            stderr: err,
            benchmark_stats: Some(BenchmarkStats {
                parse_time,
                compile_time,
                run_time: Duration::ZERO,
            }),
        })
    } else {
        let input = runner::resolve_input(
            args.file_path.as_deref(),
            args.json.as_deref(),
            args.force_input.as_ref(),
        )?;
        let engine = runner::resolve_engine();
        let output = runner::resolve_output(args.result);

        let runner = Runner {
            with_compiled_query: automaton,
            with_engine: engine,
            with_input: input,
            with_output: output,
        };

        // Benchmark execution
        let BenchmarkRunOutput {
            mut run_output,
            duration: run_time,
        } = runner.run_with_benchmark()?;

        run_output.benchmark_stats = Some(BenchmarkStats {
            parse_time,
            compile_time,
            run_time,
        });

        Ok(run_output)
    }
}

pub fn parse_query(query_string: &str) -> Result<JsonPathQuery> {
    let mut parser_builder = ParserBuilder::default();
    parser_builder.allow_surrounding_whitespace(true);
    let parser: rsonpath_syntax::Parser = parser_builder.into();
    parser
        .parse(query_string)
        .map_err(|err| report_parser_error(err).wrap_err("Could not parse JSONPath query."))
}

pub fn compile_query(query: &JsonPathQuery) -> Result<Automaton> {
    Automaton::new(query).map_err(|err| report_compiler_error(query, err).wrap_err("Error compiling the query."))
}

pub fn create_args(
    query: String,
    file_path: Option<String>,
    json: Option<String>,
    verbose: bool,
    compile: bool,
    result: ResultArg,
    force_input: Option<InputArg>,
) -> Args {
    Args {
        query,
        file_path,
        json,
        verbose,
        compile,
        result,
        force_input,
    }
}

pub struct BenchmarkStats {
    pub parse_time: Duration,
    pub compile_time: Duration,
    pub run_time: Duration,
}

pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub benchmark_stats: Option<BenchmarkStats>,
}
