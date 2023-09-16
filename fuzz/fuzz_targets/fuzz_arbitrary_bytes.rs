#![no_main]

use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::OwnedBytes;
use rsonpath::query::JsonPathQuery;
use std::fmt::Debug;

fuzz_target!(|data: DisplayableBytes| {
    let bytes = OwnedBytes::new(&data.0).expect("error creating input");
    let query = JsonPathQuery::parse("$..*").expect("error when parsing the query");
    let engine = RsonpathEngine::compile_query(&query).expect("error when compiling");
    let mut sink = vec![];

    let _ = engine.matches(&bytes, &mut sink);
});

#[derive(Arbitrary)]
pub struct DisplayableBytes<'a>(&'a [u8]);

impl<'a> Debug for DisplayableBytes<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8_lossy(self.0);
        f.debug_struct("Bytes")
            .field("utf8-lossy", &s)
            .field("raw", &self.0)
            .finish()
    }
}
