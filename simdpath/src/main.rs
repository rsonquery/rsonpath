use clap::{App, Arg};
use log::*;
use simdpath_core::engine::runner::Runner;
use simdpath_core::query::{self, *};
use simdpath_stack_based::StackBasedRunner;
use simdpath_stackless::run_simdpath3;
use simple_logger::SimpleLogger;
use std::error::Error;
use std::fs;

const VERBOSE: &str = "verbose";
const FILE: &str = "file";
const QUERY: &str = "query";

fn main() -> Result<(), Box<dyn Error>> {
    let app = configure_app();
    let matches = app.get_matches();
    let file_path = matches.value_of(FILE).unwrap();
    let query_string = matches.value_of(QUERY).unwrap();
    let verbose = matches.is_present(VERBOSE);

    configure_logger(verbose)?;

    let query = parse_query(query_string)?;
    info!("Executing query: {}\n", query);

    let runner = StackBasedRunner::compile_query(&query);
    let contents = fs::read_to_string(&file_path)?;

    let stack_based_count = runner.count(&contents);
    info!("StackBasedRunner count: {}", stack_based_count.count);

    let simdpath_count = run_simdpath3(&contents, "claims", "references", "hash");
    info!("Simdpath count: {}", simdpath_count);

    Ok(())
}

fn parse_query(query_string: &str) -> Result<JsonPathQuery<'_>, String> {
    let query_result = query::parse_json_path_query(query_string);

    match query_result {
        Ok(q) => Ok(q),
        Err(e) => {
            error!("Could not parse JSONPath query: {}", e);
            Err(e)
        }
    }
}

fn configure_app() -> App<'static, 'static> {
    App::new("SIMD Path")
        .version("0.1.0")
        .author("Mateusz Gienieczko <matgienieczko@gmail.com>")
        .about("High-performance JSON Path query engine.")
        .arg(
            Arg::with_name(VERBOSE)
                .short("v")
                .long(VERBOSE)
                .help("If set runs with verbose debug information."),
        )
        .arg(
            Arg::with_name(FILE)
                .short("f")
                .long(FILE)
                .required(true)
                .index(1)
                .takes_value(true)
                .value_name("JSON_FILE")
                .help("Input JSON file to  query."),
        )
        .arg(
            Arg::with_name(QUERY)
                .short("q")
                .long(QUERY)
                .required(true)
                .index(2)
                .takes_value(true)
                .value_name("QUERY")
                .help("JSON Path query to run against the JSON_FILE."),
        )
}

fn configure_logger(verbose: bool) -> Result<(), SetLoggerError> {
    SimpleLogger::new()
        .with_level(if verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Warn
        })
        .init()
}
