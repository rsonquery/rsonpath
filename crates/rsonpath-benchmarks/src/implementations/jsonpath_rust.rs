use crate::framework::implementation::Implementation;
use jsonpath_rust::{
    parser::{errors::JsonPathError, model::JpQuery, parse_json_path},
    query::{js_path_process, QueryRef},
};
use serde_json::Value;
use std::{
    fmt::Display,
    fs,
    io::{self, BufReader},
};
use thiserror::Error;

pub struct JsonpathRust {}

pub struct JsonpathRustResult<'a>(Vec<QueryRef<'a, Value>>);

impl Implementation for JsonpathRust {
    type Query = JpQuery;

    type File = Value;

    type Error = JsonpathRustError;

    type Result<'a> = JsonpathRustResult<'a>;

    fn id() -> &'static str {
        "jsonpath-rust"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(JsonpathRust {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let file = fs::File::open(file_path)?;
        let reader = BufReader::new(file);
        let value: Value = serde_json::from_reader(reader)?;

        Ok(value)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        parse_json_path(query).map_err(JsonpathRustError::JsonPathInstError)
    }

    fn run<'a>(&self, query: &'a Self::Query, file: &'a Self::File) -> Result<Self::Result<'a>, Self::Error> {
        let results = js_path_process(query, file)?;

        Ok(JsonpathRustResult(results))
    }
}

impl Display for JsonpathRustResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for res in &self.0 {
            writeln!(f, "{}", res.clone().val())?;
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum JsonpathRustError {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("error parsing JSON with serde: '{0}'")]
    SerdeError(#[from] serde_json::Error),
    #[error("error parsing JSONPath query: '{0}'")]
    JsonPathInstError(#[from] JsonPathError),
}
