use args::{Args, EngineArg, ResultArg};
use clap::Parser;
use color_eyre::{
    eyre::{eyre, Result, WrapErr},
    Help,
};
use error::{report_compiler_error, report_engine_error, report_parser_error};
use log::*;
use rsonpath_lib::{
    engine::{main::MainEngine, recursive::RecursiveEngine, Compiler, Engine},
    input::{Input, MmapInput, OwnedBytes},
    query::{automaton::Automaton, JsonPathQuery},
    result::{CountResult, IndexResult, QueryResult},
};
use std::fs;

mod args;
mod error;
mod logger;
mod version;

fn main() -> Result<()> {
    use color_eyre::owo_colors::OwoColorize;
    color_eyre::install()?;
    let args = Args::parse();

    logger::configure(args.verbose)?;

    run_with_args(&args).map_err(|err| err.with_note(|| format!("Query string: '{}'.", args.query.dimmed())))
}

fn run_with_args(args: &Args) -> Result<()> {
    let query = parse_query(&args.query)?;
    info!("Preparing query: `{query}`\n");

    if args.compile {
        compile(&query)
    } else if args.use_mmap {
        let file = fs::File::open(args.file_path.as_ref().unwrap())?;
        let input = unsafe { MmapInput::map_file(&file) }?;

        match args.result {
            ResultArg::Bytes => run::<IndexResult, _>(&query, &input, args.engine),
            ResultArg::Count => run::<CountResult, _>(&query, &input, args.engine),
        }
    } else {
        let contents = get_contents(args.file_path.as_deref())?;
        let input = OwnedBytes::new(&contents)?;

        match args.result {
            ResultArg::Bytes => run::<IndexResult, _>(&query, &input, args.engine),
            ResultArg::Count => run::<CountResult, _>(&query, &input, args.engine),
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

fn run<R: QueryResult, I: Input>(query: &JsonPathQuery, input: &I, engine: EngineArg) -> Result<()> {
    match engine {
        EngineArg::Main => {
            let result = run_engine::<MainEngine, R, _>(query, input).wrap_err("Error running the main engine.")?;
            println!("{result}");
        }
        EngineArg::Recursive => {
            let result =
                run_engine::<RecursiveEngine, R, _>(query, input).wrap_err("Error running the recursive engine.")?;
            println!("{result}");
        }
        EngineArg::VerifyBoth => {
            let main_result =
                run_engine::<MainEngine, R, _>(query, input).wrap_err("Error running the main engine.")?;
            let recursive_result =
                run_engine::<RecursiveEngine, R, _>(query, input).wrap_err("Error running the recursive engine.")?;

            if recursive_result != main_result {
                return Err(eyre!("Result mismatch!"));
            }

            println!("{main_result}");
        }
    }

    Ok(())
}

fn run_engine<C: Compiler, R: QueryResult, I: Input>(query: &JsonPathQuery, input: &I) -> Result<R> {
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
