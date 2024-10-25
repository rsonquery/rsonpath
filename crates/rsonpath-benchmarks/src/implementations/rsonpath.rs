use crate::framework::implementation::Implementation;
use ouroboros::self_referencing;
use rsonpath::{
    engine::main::MainEngine,
    input::OwnedBytes,
    result::{Match, Sink},
};
use rsonpath::{
    engine::{Compiler, Engine},
    input::MmapInput,
};
use rsonpath_syntax::JsonPathQuery;
use std::{convert::Infallible, fmt::Display, fs, io};
use thiserror::Error;

pub struct Rsonpath {}
pub struct RsonpathCount {}
pub struct RsonpathMmap {}
pub struct RsonpathMmapCount {}

#[self_referencing()]
pub struct RsonpathQuery {
    query: JsonPathQuery,
    #[borrows(query)]
    #[not_covariant]
    engine: MainEngine<'this>,
}

impl Implementation for Rsonpath {
    type Query = RsonpathQuery;

    type File = OwnedBytes<Vec<u8>>;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(Rsonpath {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = OwnedBytes::new(file.into_bytes());

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();

        let rsonpath = RsonpathQuery::try_new(query, |query| {
            MainEngine::compile_query(query).map_err(RsonpathError::CompilerError)
        })?;

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query
            .with_engine(|engine| engine.matches(file, &mut VoidSink))
            .map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathCount {
    type Query = RsonpathQuery;

    type File = OwnedBytes<Vec<u8>>;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath_count"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(RsonpathCount {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = OwnedBytes::new(file.into_bytes());

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();

        let rsonpath = RsonpathQuery::try_new(query, |query| {
            MainEngine::compile_query(query).map_err(RsonpathError::CompilerError)
        })?;

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query
            .with_engine(|engine| engine.count(file))
            .map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathMmap {
    type Query = RsonpathQuery;

    type File = MmapInput;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath_mmap"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(RsonpathMmap {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();

        let rsonpath = RsonpathQuery::try_new(query, |query| {
            MainEngine::compile_query(query).map_err(RsonpathError::CompilerError)
        })?;

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query
            .with_engine(|engine| engine.matches(file, &mut VoidSink))
            .map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathMmapCount {
    type Query = RsonpathQuery;

    type File = MmapInput;

    type Error = RsonpathError;

    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath_mmap_count"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(RsonpathMmapCount {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();

        let rsonpath = RsonpathQuery::try_new(query, |query| {
            MainEngine::compile_query(query).map_err(RsonpathError::CompilerError)
        })?;

        Ok(rsonpath)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query
            .with_engine(|engine| engine.count(file))
            .map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

#[derive(Error, Debug)]
pub enum RsonpathError {
    #[error(transparent)]
    CompilerError(#[from] rsonpath::automaton::error::CompilerError),
    #[error(transparent)]
    EngineError(#[from] rsonpath::engine::error::EngineError),
    #[error(transparent)]
    InputError(#[from] rsonpath::input::error::InputError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("something happened")]
    Unknown(),
}

pub struct MatchDisplay(Vec<Match>);

impl Display for MatchDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for m in &self.0 {
            writeln!(f, "{m}")?
        }

        Ok(())
    }
}

struct VoidSink;

impl<D> Sink<D> for VoidSink {
    type Error = Infallible;

    fn add_match(&mut self, _data: D) -> Result<(), Self::Error> {
        Ok(())
    }
}
