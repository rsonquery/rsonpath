use crate::framework::implementation::Implementation;
use rsonpath::lookup_table::LookUpTable;
use rsonpath::{engine::Compiler, input::MmapInput};
use rsonpath::{
    engine::{main::MainEngine, Engine},
    input::OwnedBytes,
    result::{Match, Sink},
};
use std::{convert::Infallible, fmt::Display, fs, io};
use thiserror::Error;

pub struct Rsonpath {}
pub struct RsonpathCount {}
pub struct RsonpathMmap {}
pub struct RsonpathMmapCount {}
pub struct RsonpathWithLut {}

// Added by Ricardo
impl Implementation for RsonpathWithLut {
    type Query = MainEngine;
    type File = (MmapInput, LookUpTable::LookUpTableImpl);
    type Error = RsonpathError;
    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rsonpath_with_lut"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(RsonpathWithLut {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };

        let lut: LookUpTable::LookUpTableImpl = LookUpTable::LookUpTableImpl::build(file_path);

        Ok((input, lut))
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
       // TODO throw error: "This implementation cannot by used without lut"
    }

    fn compile_query_and_build_lut(&self, query: &str, file: &Self::File){
        let query = rsonpath_syntax::parse(query).unwrap();
        let mut engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

      

        engine.add_lut(lut);

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.matches(file, &mut VoidSink).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Implementation for Rsonpath {
    type Query = MainEngine;

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
        let engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.matches(file, &mut VoidSink).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathCount {
    type Query = MainEngine;

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
        let engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.count(file).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathMmap {
    type Query = MainEngine;

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
        let engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.matches(file, &mut VoidSink).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Implementation for RsonpathMmapCount {
    type Query = MainEngine;

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
        let engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.count(file).map_err(RsonpathError::EngineError)?;

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
