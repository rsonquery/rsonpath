#![no_main]

use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target};
use rsonpath::engine::{Compiler, Engine, RsonpathEngine};
use rsonpath::input::BorrowedBytes;
use std::fmt::Debug;

fuzz_target!(|data: DisplayableBytes| {
    let bytes = BorrowedBytes::new(data.0);
    let query = rsonpath_syntax::parse("$..*").expect("error when parsing the query");
    let engine = RsonpathEngine::compile_query(&query).expect("error when compiling");
    let mut sink = vec![];

    let _ = engine.matches(&bytes, &mut sink);
});

#[derive(Arbitrary)]
pub struct DisplayableBytes<'a>(&'a [u8]);

impl Debug for DisplayableBytes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf8_lossy(self.0);
        f.debug_struct("Bytes")
            .field("utf8-lossy", &s)
            .field("raw", &self.0)
            .finish()
    }
}
