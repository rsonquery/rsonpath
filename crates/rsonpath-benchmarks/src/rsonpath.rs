use ouroboros::self_referencing;
use rsonpath::{
    engine::{result::CountResult, Input, Runner},
    query::JsonPathQuery,
    stackless::StacklessRunner,
};
use std::fs;
use thiserror::Error;

use crate::framework::implementation::Implementation;

pub struct Rsonpath {}

#[self_referencing()]
pub struct RsonpathQuery {
    query: JsonPathQuery,
    #[borrows(query)]
    #[not_covariant]
    engine: StacklessRunner<'this>,
}

impl Implementation for Rsonpath {
    type Query = RsonpathQuery;

    type File = Input;

    type Error = RsonpathError;

    fn id() -> &'static str {
        "rsonpath"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(Rsonpath {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let contents = fs::read_to_string(file_path).expect("Reading from file failed.");
        let input = Self::File::new(contents);

        Ok(input)
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        let query = JsonPathQuery::parse(query).unwrap();

        let builder = RsonpathQueryBuilder {
            query,
            engine_builder: |query| StacklessRunner::compile_query(query),
        };

        Ok(builder.build())
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error> {
        Ok(query.with_engine(|engine| engine.run::<CountResult>(file).get() as u64))
    }
}

#[derive(Error, Debug)]
pub enum RsonpathError {
    #[error("something happened")]
    Unknown(),
}
