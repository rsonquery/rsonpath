use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::MmapInput,
    result::MatchWriter,
};
use rsonpath_syntax::JsonPathQuery;
use std::{env, error::Error, fs, io, process::ExitCode};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 3 {
        eprintln!("provide exactly two arguments, query and file path");
        return Ok(ExitCode::FAILURE);
    }

    let query_arg = &args[1];
    let file_path = &args[2];

    let query = JsonPathQuery::parse(query_arg)?;
    let file = fs::File::open(file_path)?;
    let input = unsafe { MmapInput::map_file(&file)? };
    let stdout_lock = io::stdout().lock();
    let mut sink = MatchWriter::from(stdout_lock);

    let engine = RsonpathEngine::compile_query(&query)?;

    engine.approximate_spans(&input, &mut sink)?;

    Ok(ExitCode::SUCCESS)
}
