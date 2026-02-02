#![no_main]

use libfuzzer_sys::{arbitrary::Arbitrary, fuzz_target, Corpus};
use rsonpath::input::BorrowedBytes;
use rsonpath::{
    automaton::error::CompilerError,
    engine::{Compiler, Engine, RsonpathEngine},
};
use rsonpath_lib_fuzz::{ArbitraryJson, ArbitrarySupportedQuery};

#[derive(Debug, Arbitrary)]
struct FuzzData {
    query: ArbitrarySupportedQuery,
    json: ArbitraryJson<2048>,
}

fuzz_target!(|data: FuzzData| -> Corpus {
    let json_string = data.json.to_string();
    let bytes = BorrowedBytes::new(json_string.as_bytes());
    let engine = match RsonpathEngine::compile_query(&data.query.0) {
        Ok(x) => x,
        Err(CompilerError::QueryTooComplex(_)) => return Corpus::Reject,
        Err(err) => panic!("error compiling query: {err}"),
    };
    let mut sink = vec![];

    let _ = engine.matches(&bytes, &mut sink);

    Corpus::Keep
});
