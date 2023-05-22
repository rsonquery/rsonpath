#![forbid(unsafe_code)]

use clap::{Parser, ValueEnum};
use color_eyre::eyre::{eyre, Result, WrapErr};
use color_eyre::Help;
use log::*;
use rsonpath::{report_compiler_error, report_engine_error, report_parser_error};
use rsonpath_lib::engine::main::MainEngine;
use rsonpath_lib::engine::recursive::RecursiveEngine;
use rsonpath_lib::engine::{Compiler, Engine};
use rsonpath_lib::input::OwnedBytes;
use rsonpath_lib::query::automaton::Automaton;
use rsonpath_lib::query::JsonPathQuery;
use rsonpath_lib::result::{CountResult, IndexResult, QueryResult};
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// JSONPath query to run against the input JSON.
    query: String,
    /// Input JSON file to query.
    ///
    /// If not specified uses the standard input stream.
    file_path: Option<String>,
    /// Include verbose debug information.
    #[clap(short, long)]
    verbose: bool,
    /// Engine to use for evaluating the query.
    #[clap(short, long, value_enum, default_value_t = EngineArg::Main)]
    engine: EngineArg,
    /// Only compile the query and output the automaton, do not run the engine.
    ///
    /// Cannot be used with --engine or FILE_PATH.
    #[clap(short, long)]
    #[arg(conflicts_with = "engine")]
    #[arg(conflicts_with = "file_path")]
    compile: bool,
    /// Result reporting mode.
    #[clap(short, long, value_enum, default_value_t = ResultArg::Bytes)]
    result: ResultArg,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
enum EngineArg {
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
enum ResultArg {
    /// Return a list of all bytes at which a match occurred.
    Bytes,
    /// Return only the number of matches.
    Count,
}

fn main() -> Result<()> {
    use color_eyre::owo_colors::OwoColorize;
    color_eyre::install()?;
    let args = Args::parse();

    configure_logger(args.verbose)?;

    run_with_args(&args).map_err(|err| err.with_note(|| format!("Query string: '{}'.", args.query.dimmed())))
}

fn run_with_args(args: &Args) -> Result<()> {
    let query = parse_query(&args.query)?;
    info!("Preparing query: `{query}`\n");

    if args.compile {
        compile(&query)
    } else {
        let contents = get_contents(args.file_path.as_deref())?;
        let input = OwnedBytes::new(&contents)?;

        match args.result {
            ResultArg::Bytes => run::<IndexResult>(&query, &input, args.engine),
            ResultArg::Count => run::<CountResult>(&query, &input, args.engine),
        }
    }
}

fn compile(query: &JsonPathQuery) -> Result<()> {
    let automaton = Automaton::new(query)
        .map_err(|err| report_compiler_error(query, err).wrap_err("Error compiling the query."))?;
    info!("Automaton: {automaton}");
    println!("{automaton}");
    Ok(())
}

fn run<R: QueryResult>(query: &JsonPathQuery, input: &OwnedBytes, engine: EngineArg) -> Result<()> {
    match engine {
        EngineArg::Main => {
            let result = run_engine::<MainEngine, R>(query, input).wrap_err("Error running the main engine.")?;
            println!("{result}");
        }
        EngineArg::Recursive => {
            let result =
                run_engine::<RecursiveEngine, R>(query, input).wrap_err("Error running the recursive engine.")?;
            println!("{result}");
        }
        EngineArg::VerifyBoth => {
            let main_result = run_engine::<MainEngine, R>(query, input).wrap_err("Error running the main engine.")?;
            let recursive_result =
                run_engine::<RecursiveEngine, R>(query, input).wrap_err("Error running the recursive engine.")?;

            if recursive_result != main_result {
                return Err(eyre!("Result mismatch!"));
            }

            println!("{main_result}");
        }
    }

    Ok(())
}

fn run_engine<C: Compiler, R: QueryResult>(query: &JsonPathQuery, input: &OwnedBytes) -> Result<R> {
    let engine = C::compile_query(query)
        .map_err(|err| report_compiler_error(query, err).wrap_err("Error compiling the query."))?;
    info!("Compilation finished, running...");

    let result = engine
        .run::<_, R>(input)
        .map_err(|err| report_engine_error(err).wrap_err("Error executing the query."))?;
    info!("Result: {result}");

    Ok(result)
}

fn parse_query(query_string: &str) -> Result<JsonPathQuery> {
    JsonPathQuery::parse(query_string)
        .map_err(|err| report_parser_error(query_string, err).wrap_err("Could not parse JSONPath query."))
}

fn get_contents(file_path: Option<&str>) -> Result<String> {
    use std::fs;
    use std::io::{self, Read};
    match file_path {
        Some(path) => fs::read_to_string(path).wrap_err("Reading from file failed."),
        None => {
            let mut result = String::new();
            io::stdin()
                .read_to_string(&mut result)
                .wrap_err("Reading from stdin failed.")?;
            Ok(result)
        }
    }
}

fn configure_logger(verbose: bool) -> Result<()> {
    SimpleLogger::new()
        .with_level(if verbose { LevelFilter::Trace } else { LevelFilter::Warn })
        .init()
        .wrap_err("Logger configuration error.")
}
