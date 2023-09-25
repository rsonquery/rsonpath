use crate::input::{self, JsonSource, ResolvedInputKind};
use crate::{
    args::{InputArg, ResultArg},
    error::report_engine_error,
};
use eyre::{Result, WrapErr};
use log::warn;
use object_pool::Pool;
use rsonpath_lib::{
    engine::{error::EngineError, main::MainEngine, Compiler, Engine},
    input::{BufferedInput, Input, MmapInput, OwnedBytes},
    query::automaton::Automaton,
    result::{Match, MatchWriter, Sink},
};
use std::{
    fs,
    io::{self, Read},
    path::Path, sync::Arc, convert::Infallible,
};

pub struct Runner<'q, S> {
    pub with_compiled_query: Automaton<'q>,
    pub with_engine: ResolvedEngine,
    pub with_input: ResolvedInput<S>,
    pub with_output: ResolvedOutput,
}

impl<'q, S: AsRef<str>> Runner<'q, S> {
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

pub fn resolve_input<P: AsRef<Path>, S: AsRef<str>>(
    file_path: Option<P>,
    inline_json: Option<S>,
    force_input: Option<&InputArg>,
) -> Result<ResolvedInput<S>> {
    let file = match (file_path, inline_json) {
        (Some(path), None) => JsonSource::File(fs::File::open(path).wrap_err("Error reading the provided file.")?),
        (None, Some(json)) => JsonSource::Inline(json),
        (None, None) => JsonSource::Stdin(io::stdin()),
        (Some(_), Some(_)) => unreachable!("both file_path and json detected"),
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
        ResultArg::Indices => ResolvedOutput::Index,
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

pub struct ResolvedInput<S> {
    file: JsonSource<S>,
    kind: ResolvedInputKind,
    fallback_kind: Option<ResolvedInputKind>,
}

pub enum ResolvedOutput {
    Count,
    Index,
    Nodes,
}

impl<S: AsRef<str>> ResolvedInput<S> {
    fn run_engine<E: Engine>(mut self, engine: E, with_output: ResolvedOutput) -> Result<()> {
        match self.kind {
            ResolvedInputKind::Mmap => {
                let raw_desc = self
                    .file
                    .try_as_raw_desc()
                    .ok_or_else(|| eyre::eyre!("Attempt to create a memory map on inline JSON input."))?;
                let mmap_result = unsafe { MmapInput::map_file(raw_desc) };

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
                let input = match self.file {
                    JsonSource::File(f) => {
                        let contents = get_contents(f)?;
                        OwnedBytes::new(&contents)
                    }
                    JsonSource::Stdin(s) => {
                        let contents = get_contents(s)?;
                        OwnedBytes::new(&contents)
                    }
                    JsonSource::Inline(j) => OwnedBytes::new(&j.as_ref()),
                }?;

                with_output.run_and_output(engine, input)
            }
            ResolvedInputKind::Buffered => {
                let read = self
                    .file
                    .try_as_read()
                    .ok_or_else(|| eyre::eyre!("Attempt to buffer reads on inline JSON input."))?;
                let input = BufferedInput::new(read);
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
                    let result = engine.count(&input)?;
                    print!("{result}");
                }
                ResolvedOutput::Index => {
                    let mut sink = MatchWriter::from(io::stdout().lock());
                    engine.indices(&input, &mut sink)?;
                }
                ResolvedOutput::Nodes => {
                    let (mut s, r) = crossbeam_channel::bounded(4);
                    let pool = Arc::new(Pool::new(5, || Vec::<Match>::with_capacity(BUF_SIZE)));
                    let mut sink = SenderSink::new(s, pool.clone());

                    let writer = std::thread::spawn(move || -> Result<(), io::Error> {
                        use std::io::Write;

                        let mut stdout = io::stdout().lock();

                        while let Ok(mut ms) = r.recv() {
                            for m in ms.drain(..) {
                                writeln!(stdout, "{m}")?;
                            }
                            pool.attach(ms);
                        }

                        Ok(())
                    });

                    engine.matches(&input, &mut sink)?;
                    drop(sink);

                    writer.join().unwrap();
                }
            }

            Ok(())
        }

        run_impl(self, engine, input).map_err(|err| report_engine_error(err).wrap_err("Error executing the query."))
    }
}

fn get_contents<R: Read>(mut stream: R) -> Result<String> {
    let mut result = String::new();
    stream
        .read_to_string(&mut result)
        .wrap_err("Reading from file failed.")?;
    Ok(result)
}

const BUF_SIZE: usize = 8 * 1024;

struct SenderSink<D> {
    s: crossbeam_channel::Sender<Vec<D>>,
    buf: Vec<D>,
    pool: Arc<Pool<Vec<D>>>,
}

impl<D> SenderSink<D> {
    fn new(s: crossbeam_channel::Sender<Vec<D>>, pool: Arc<Pool<Vec<D>>>) -> Self {
        let vec = pool.pull(|| Vec::with_capacity(BUF_SIZE));
        Self {
            s,
            buf: vec.detach().1,
            pool
        }
    }
}

impl<D> Sink<D> for SenderSink<D> where D: Send {
    type Error = Infallible;

    fn add_match(&mut self, data: D) -> std::result::Result<(), Self::Error> {
        self.buf.push(data);

        if self.buf.len() == BUF_SIZE {
            let mut here_buf = self.pool.pull(|| Vec::with_capacity(BUF_SIZE)).detach().1;
            std::mem::swap(&mut here_buf, &mut self.buf);

            self.s.send(here_buf).expect("sender send");
        }

        Ok(())
    }
}

impl<D> Drop for SenderSink<D> {
    fn drop(&mut self) {
        let mut here_buf = vec![];
        std::mem::swap(&mut self.buf, &mut here_buf);
        self.s.send(here_buf).expect("sender send")
    }
}