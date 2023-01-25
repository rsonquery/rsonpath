use clap::{Parser, ValueEnum};
use color_eyre::eyre::{eyre, Result, WrapErr};
use color_eyre::{Help, SectionExt};
use log::*;
use rsonpath_lib::engine::main::MainEngine;
use rsonpath_lib::engine::recursive::RecursiveEngine;
use rsonpath_lib::engine::result::{CountResult, IndexResult, QueryResult};
use rsonpath_lib::engine::{Engine, Input};
use rsonpath_lib::query::automaton::Automaton;
use rsonpath_lib::query::JsonPathQuery;
use simple_logger::SimpleLogger;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    configure_logger(args.verbose)?;

    let query = parse_query(&args.query)?;
    info!("Preparing query: `{query}`\n");

    if args.compile {
        return compile(&query);
    }

    let mut contents = get_contents(args.file_path.as_deref())?;
    let input = Input::new(&mut contents);

    match args.result {
        ResultArg::Bytes => run::<IndexResult>(&query, &input, args.engine),
        ResultArg::Count => run::<CountResult>(&query, &input, args.engine),
    }
}

fn compile(query: &JsonPathQuery) -> Result<()> {
    let automaton = Automaton::new(query).wrap_err("Error compiling the query.")?;
    info!("Automaton: {automaton}");
    println!("{automaton}");
    Ok(())
}

fn run<R: QueryResult>(query: &JsonPathQuery, input: &Input, engine: EngineArg) -> Result<()> {
    match engine {
        EngineArg::Main => {
            let main_engine =
                MainEngine::compile_query(query).wrap_err("Error compiling the query.")?;
            info!("Compilation finished, running...");

            let main_result = main_engine
                .run::<R>(input)
                .wrap_err("Error in the main engine.")?;
            info!("Main: {main_result}");

            println!("{main_result}");
        }
        EngineArg::Recursive => {
            let recursive_engine =
                RecursiveEngine::compile_query(query).wrap_err("Error compiling the query.")?;
            info!("Compilation finished, running...");

            let recursive_result = recursive_engine
                .run::<R>(input)
                .wrap_err("Error in the recursive engine.")?;
            info!("Recursive: {recursive_result}");

            println!("{recursive_result}");
        }
        EngineArg::VerifyBoth => {
            let main_engine =
                MainEngine::compile_query(query).wrap_err("Error compiling the query.")?;
            let recursive_engine =
                RecursiveEngine::compile_query(query).wrap_err("Error compiling the query.")?;
            info!("Compilation finished, running...");

            let main_result = main_engine
                .run::<R>(input)
                .wrap_err("Error in the main engine.")?;
            info!("Main: {main_result}");

            let recursive_result = recursive_engine
                .run::<R>(input)
                .wrap_err("Error in the recursive engine.")?;
            info!("Recursive: {recursive_result}");

            if recursive_result != main_result {
                return Err(eyre!("Result mismatch!"));
            }

            println!("{recursive_result}");
        }
    }

    Ok(())
}

fn parse_query(query_string: &str) -> Result<JsonPathQuery> {
    use rsonpath_lib::query::error::ParserError;
    match JsonPathQuery::parse(query_string) {
        Ok(query) => Ok(query),
        Err(e) => {
            if let ParserError::SyntaxError { report } = e {
                let mut eyre = Err(eyre!("Could not parse JSONPath query."));
                eyre = eyre.note(format!("for query string {query_string}"));

                for error in report.errors() {
                    use color_eyre::owo_colors::OwoColorize;
                    use std::cmp;
                    const MAX_DISPLAY_LENGTH: usize = 80;

                    let display_start_idx = if error.start_idx > MAX_DISPLAY_LENGTH {
                        error.start_idx - MAX_DISPLAY_LENGTH
                    } else {
                        0
                    };
                    let display_length = cmp::min(
                        error.len + MAX_DISPLAY_LENGTH,
                        query_string.len() - display_start_idx,
                    );
                    let error_slice = &query_string[error.start_idx..error.start_idx + error.len];
                    let slice =
                        &query_string[display_start_idx..display_start_idx + display_length];
                    let error_idx = error.start_idx - display_start_idx;

                    let underline: String = std::iter::repeat(' ')
                        .take(error_idx)
                        .chain(std::iter::repeat('^').take(error.len))
                        .collect();
                    let display_string = format!(
                        "{}\n{}",
                        slice,
                        (underline + " invalid tokens").bright_red()
                    );

                    eyre = eyre.section(display_string.header("Parse error:"));

                    if error.start_idx == 0 {
                        eyre = eyre.suggestion("Queries should start with the root selector `$`.");
                    }

                    if error_slice.contains('$') {
                        eyre = eyre.suggestion("The `$` character is reserved for the root selector and may appear only at the start.");
                    }
                }

                eyre
            } else {
                Err(e).wrap_err("Could not parse JSONPath query.")
            }
        }
    }
}

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

fn configure_logger(verbose: bool) -> Result<()> {
    SimpleLogger::new()
        .with_level(if verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        })
        .init()
        .wrap_err("Logger configuration error.")
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
