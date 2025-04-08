use crate::framework::implementation::Implementation;
use rsonpath::lookup_table::{LookUpTable, LUT};
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
pub struct RsonpathLut {
    cutoff: usize,
}

// Added by Ricardo
impl RsonpathLut {
    pub fn new(cutoff: usize) -> Result<Self, RsonpathError> {
        Ok(RsonpathLut { cutoff })
    }
}

// Added by Ricardo
impl Implementation for RsonpathLut {
    type Query = MainEngine;
    type File = MmapInput;
    type Error = RsonpathError;
    type Result<'a> = &'static str;

    fn id() -> &'static str {
        "rq_lut"
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };
        Ok(input)
    }

    fn compile_query_without_lut(&self, query: &str) -> Result<Self::Query, Self::Error> {
        Err(Self::Error::LutRequiredError)
    }

    fn compile_query(&self, query: &str, file_path: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();
        let mut engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        // Build LUT and add it to the engine object
        if let Ok(lut) = LUT::build(file_path, self.cutoff) {
            engine.add_lut(lut);
        } else {
            return Err(Self::Error::LutRequiredError);
        }

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.matches(file, &mut VoidSink).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl Rsonpath {
    pub fn new() -> Result<Self, RsonpathError> {
        Ok(Rsonpath {})
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

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = OwnedBytes::new(file.into_bytes());

        Ok(input)
    }

    fn compile_query_without_lut(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();
        let engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.matches(file, &mut VoidSink).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl RsonpathCount {
    pub fn new() -> Result<Self, RsonpathError> {
        Ok(RsonpathCount {})
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

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::read_to_string(file_path)?;
        let input = OwnedBytes::new(file.into_bytes());

        Ok(input)
    }

    fn compile_query_without_lut(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();
        let engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.count(file).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl RsonpathMmap {
    pub fn new() -> Result<Self, RsonpathError> {
        Ok(RsonpathMmap {})
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

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };

        Ok(input)
    }

    fn compile_query_without_lut(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = rsonpath_syntax::parse(query).unwrap();
        let engine = MainEngine::compile_query(&query).map_err(RsonpathError::CompilerError)?;

        Ok(engine)
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<Self::Result<'_>, Self::Error> {
        query.matches(file, &mut VoidSink).map_err(RsonpathError::EngineError)?;

        Ok("[not collected]")
    }
}

impl RsonpathMmapCount {
    pub fn new() -> Result<Self, RsonpathError> {
        Ok(RsonpathMmapCount {})
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

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let input = unsafe { MmapInput::map_file(&file)? };

        Ok(input)
    }

    fn compile_query_without_lut(&self, query: &str) -> Result<Self::Query, Self::Error> {
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
    #[error("Query compilation without LUT is not supported")]
    LutRequiredError,
    #[error("Failed to build lookup table: {0}")]
    LutBuildError(#[from] Box<dyn std::error::Error + Send + Sync>),
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
