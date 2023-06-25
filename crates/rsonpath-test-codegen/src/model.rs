use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Document {
    pub input: Input,
    pub queries: Vec<Query>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Input {
    pub description: String,
    pub json: String,
    pub is_compressed: bool,
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
    toml::to_string(doc).expect("toml files must be valid")
}
