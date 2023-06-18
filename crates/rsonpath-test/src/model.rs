use serde::Deserialize;

pub struct NamedDocument {
    pub name: String,
    pub document: Document,
}

#[derive(Deserialize)]
pub struct Document {
    pub input: Input,
    pub queries: Vec<Query>,
}

#[derive(Deserialize)]
pub struct Input {
    pub description: String,
    pub json: String,
}

#[derive(Deserialize)]
pub struct Query {
    pub description: String,
    pub query: String,
    pub results: Results,
}

#[derive(Deserialize)]
pub struct Results {
    pub count: u64,
    pub bytes: Vec<usize>,
    pub nodes: Vec<String>,
}
