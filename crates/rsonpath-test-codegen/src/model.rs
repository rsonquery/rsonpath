use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Document {
    pub input: Input,
    pub queries: Vec<Query>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Input {
    pub description: String,
    pub is_compressed: bool,
    pub source: InputSource,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum InputSource {
    #[serde(rename = "large_file")]
    LargeFile(PathBuf),
    #[serde(rename = "json_string")]
    JsonString(String),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Query {
    pub description: String,
    pub query: String,
    pub results: Results,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Results {
    pub count: u64,
    pub bytes: Option<Vec<usize>>,
    pub nodes: Option<Vec<String>>,
}

pub(crate) fn serialize(doc: &Document) -> String {
    toml::to_string(doc).expect("generated toml must be valid")
}

pub(crate) fn deserialize<S: AsRef<str>>(contents: S) -> Document {
    toml::from_str(contents.as_ref()).expect("configuration toml must be valid")
}
