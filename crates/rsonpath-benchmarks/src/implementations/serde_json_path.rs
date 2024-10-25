use crate::framework::implementation::Implementation;
use serde_json::Value;
use serde_json_path::{JsonPath, NodeList, ParseError};
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader},
};
use thiserror::Error;

pub struct SerdeJsonPath {}

pub struct SerdeJsonPathResult<'a>(NodeList<'a>);

impl Implementation for SerdeJsonPath {
    type Query = JsonPath;

    type File = Value;

    type Error = SerdeJsonPathError;

    type Result<'a> = SerdeJsonPathResult<'a>;

    fn id() -> &'static str {
        "serde_json_path"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(SerdeJsonPath {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader)?;

        Ok(value)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        JsonPath::parse(query).map_err(SerdeJsonPathError::JsonPathParseError)
    }

    fn run<'a>(&self, query: &Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        Ok(SerdeJsonPathResult(query.query(file)))
    }
}

impl<'a> Display for SerdeJsonPathResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for res in self.0.iter() {
            writeln!(f, "{res}")?;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum SerdeJsonPathError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("error parsing JSON with serde: '{0}'")]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    JsonPathParseError(#[from] ParseError),
}
