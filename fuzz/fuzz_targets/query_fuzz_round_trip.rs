//! Fuzz round-tripping - for every valid query parsing its `.to_string()` should give an equivalent query.
#![no_main]

use libfuzzer_sys::{fuzz_target, Corpus};
use rsonpath_syntax::JsonPathQuery;

fuzz_target!(|data: JsonPathQuery| -> Corpus {
    let str = data.to_string();
    match rsonpath_syntax::parse(&str) {
        Ok(query) => assert_eq!(data, query, "query string: {str}"),
        Err(err) if err.is_nesting_limit_exceeded() => return Corpus::Reject,
        Err(_) => panic!("expected parse to succeed"),
    }
    Corpus::Keep
});
