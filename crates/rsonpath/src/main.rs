use args::Args;
use clap::Parser as _;
use color_eyre::{eyre::Result, Help as _};
use error::{report_compiler_error, report_parser_error};
use log::*;
use rsonpath_lib::automaton::Automaton;
use rsonpath_syntax::{JsonPathQuery, ParserBuilder};
use runner::Runner;

mod args;
mod error;
mod input;
mod logger;
mod runner;
mod version;

fn main() -> Result<()> {
    use color_eyre::owo_colors::OwoColorize as _;
    color_eyre::install()?;

    let args = Args::parse();

    logger::configure(args.verbose)?;

    run_with_args(&args).map_err(|err| err.with_note(|| format!("Query string: '{}'.", args.query.dimmed())))
}

fn run_with_args(args: &Args) -> Result<()> {
    let query = parse_query(&args.query)?;
    info!("Preparing query: `{query}`\n");

    let automaton = compile_query(&query)?;
    info!("Automaton: {automaton}");

    if args.compile {
        // Only compilation was requested, so we print the automaton and exit.
        println!("{automaton}");
        debug!("{automaton:?}");
        Ok(())
    } else {
        // Actual query execution.
        let input = runner::resolve_input(
            args.file_path.as_deref(),
            args.json.as_deref(),
            args.force_input.as_ref(),
        )?;
        let engine = runner::resolve_engine();
        let output = runner::resolve_output(args.result);

        Runner {
            with_compiled_query: automaton,
            with_engine: engine,
            with_input: input,
            with_output: output,
        }
        .run()
    }
}

fn parse_query(query_string: &str) -> Result<JsonPathQuery> {
    let mut parser_builder = ParserBuilder::default();
    parser_builder.allow_surrounding_whitespace(true);
    let parser: rsonpath_syntax::Parser = parser_builder.into();
    parser
        .parse(query_string)
        .map_err(|err| report_parser_error(err).wrap_err("Could not parse JSONPath query."))
}

fn compile_query(query: &JsonPathQuery) -> Result<Automaton> {
    Automaton::new(query).map_err(|err| report_compiler_error(query, err).wrap_err("Error compiling the query."))
}
