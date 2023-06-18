use serde::Deserialize;

#[test]
fn test() {
    let file_path = "jsonpath_com_example.toml";

    let contents = fs::read_to_string(format!("./tests/cases/{}", file_path)).unwrap();

    let case: Case = toml::from_str(&contents).unwrap();

    assert_eq!(case.input.description, "");
}

pub struct NamedCase {
    pub name: String,
    pub case: Case,
}

#[derive(Deserialize)]
pub struct Case {
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