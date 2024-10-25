use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;
use rsonpath_benchmarks::framework::implementation::Implementation;
use rsonpath_benchmarks::implementations::{
    jsonpath_rust::JsonpathRust, rsonpath::RsonpathMmap, rust_jsurfer::JSurfer, serde_json_path::SerdeJsonPath,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    match args.engine {
        ImplArg::Rsonpath => run(RsonpathMmap::new()?, &args.query, &args.file_path),
        ImplArg::JSurfer => run(JSurfer::new()?, &args.query, &args.file_path),
        ImplArg::JsonpathRust => run(JsonpathRust::new()?, &args.query, &args.file_path),
        ImplArg::SerdeJsonPath => run(SerdeJsonPath::new()?, &args.query, &args.file_path),
    }
}

fn run<I: Implementation>(imp: I, query_str: &str, path_str: &str) -> Result<()> {
    let query = imp.compile_query(query_str)?;
    let file = imp.load_file(path_str)?;

    let result = imp.run(&query, &file)?;

    println!("{}", result);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// JSONPath query to run against the input JSON.
    query: String,
    /// Input JSON file to query.
    file_path: String,
    /// JSONPath implementation to use for evaluating the query.
    #[clap(short, long, value_enum)]
    engine: ImplArg,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
enum ImplArg {
    /// Use rsonpath.
    Rsonpath,
    /// Use JSurfer via JNI.
    JSurfer,
    /// Use the jsonpath-rust crate.
    JsonpathRust,
    /// Use the serde_json_path crate.
    SerdeJsonPath,
}
