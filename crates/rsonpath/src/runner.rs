use crate::input::{self, FileOrStdin, ResolvedInputKind};
use crate::{
    args::{InputArg, ResultArg},
    error::report_engine_error,
};
use eyre::{Result, WrapErr};
use log::warn;
use rsonpath_lib::{
    engine::{error::EngineError, main::MainEngine, Compiler, Engine},
    input::{BufferedInput, Input, MmapInput, OwnedBytes},
    query::automaton::Automaton,
    result::{count::CountRecorder, index::IndexRecorder, nodes::NodesRecorder},
};
use std::{
    fs,
    io::{self, Read},
    path::Path,
};

pub struct Runner<'q> {
    pub with_compiled_query: Automaton<'q>,
    pub with_engine: ResolvedEngine,
    pub with_input: ResolvedInput,
    pub with_output: ResolvedOutput,
}

impl<'q> Runner<'q> {
    pub fn run(self) -> Result<()> {
        match self.with_engine {
            ResolvedEngine::Main => {
                let engine = MainEngine::from_compiled_query(self.with_compiled_query);
                self.with_input
                    .run_engine(engine, self.with_output)
                    .wrap_err("Error running the main engine.")
            }
        }
    }
}

pub fn resolve_input<P: AsRef<Path>>(file_path: Option<P>, force_input: Option<&InputArg>) -> Result<ResolvedInput> {
    let file = match file_path {
        Some(path) => FileOrStdin::File(fs::File::open(path).wrap_err("Error reading the provided file.")?),
        None => FileOrStdin::Stdin(io::stdin()),
    };

    let (kind, fallback_kind) = input::decide_input_strategy(&file, force_input)?;

    Ok(ResolvedInput {
        file,
        kind,
        fallback_kind,
    })
}

pub fn resolve_output(result_arg: ResultArg) -> ResolvedOutput {
    match result_arg {
        ResultArg::Bytes => ResolvedOutput::Index,
        ResultArg::Count => ResolvedOutput::Count,
        ResultArg::Nodes => ResolvedOutput::Nodes,
    }
}

pub fn resolve_engine() -> ResolvedEngine {
    ResolvedEngine::Main
}

pub enum ResolvedEngine {
    Main,
}

pub struct ResolvedInput {
    file: FileOrStdin,
    kind: ResolvedInputKind,
    fallback_kind: Option<ResolvedInputKind>,
}

pub enum ResolvedOutput {
    Count,
    Index,
    Nodes,
}

impl ResolvedInput {
    fn run_engine<E: Engine>(self, engine: E, with_output: ResolvedOutput) -> Result<()> {
        match self.kind {
            ResolvedInputKind::Mmap => {
                let mmap_result = unsafe { MmapInput::map_file(&self.file) };

                match mmap_result {
                    Ok(input) => with_output.run_and_output(engine, input),
                    Err(err) => match self.fallback_kind {
                        Some(fallback_kind) => {
                            warn!(
                                "Creating a memory map failed: '{}'. Falling back to a slower input strategy.",
                                err
                            );
                            let new_input = ResolvedInput {
                                kind: fallback_kind,
                                fallback_kind: None,
                                file: self.file,
                            };

                            new_input.run_engine(engine, with_output)
                        }
                        None => Err(err).wrap_err("Creating a memory map failed."),
                    },
                }
            }
            ResolvedInputKind::Owned => {
                let contents = get_contents(self.file)?;
                let input = OwnedBytes::new(&contents)?;
                with_output.run_and_output(engine, input)
            }
            ResolvedInputKind::Buffered => {
                let input = BufferedInput::new(self.file);
                with_output.run_and_output(engine, input)
            }
        }
    }
}

impl ResolvedOutput {
    fn run_and_output<E: Engine, I: Input>(self, engine: E, input: I) -> Result<()> {
        fn run_impl<E: Engine, I: Input>(out: ResolvedOutput, engine: E, input: I) -> Result<(), EngineError> {
            match out {
                ResolvedOutput::Count => {
                    let result = engine.run::<_, CountRecorder>(&input)?;
                    print!("{result}");
                }
                ResolvedOutput::Index => {
                    let result = engine.run::<_, IndexRecorder>(&input)?;
                    print!("{result}");
                }
                ResolvedOutput::Nodes => {
                    let result = engine.run::<_, NodesRecorder>(&input)?;
                    print!("{result}");
                }
            }

            Ok(())
        }

        run_impl(self, engine, input).map_err(|err| report_engine_error(err).wrap_err("Error executing the query."))
    }
}

fn get_contents(mut file: FileOrStdin) -> Result<String> {
    let mut result = String::new();
    file.read_to_string(&mut result).wrap_err("Reading from file failed.")?;
    Ok(result)
}
