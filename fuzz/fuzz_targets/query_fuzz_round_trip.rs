//! Fuzz round-tripping - for every valid query parsing its `.to_string()` should give an equivalent query.
#![no_main]

use libfuzzer_sys::fuzz_target;
use rsonpath_lib_fuzz::ArbitraryJsonPathQuery;

fuzz_target!(|data: ArbitraryJsonPathQuery| {
    let str = data.0.to_string();
    match rsonpath_syntax::parse(&str) {
        Ok(query) => assert_eq!(data.0, query, "query string: {str}"),
        Err(_) => panic!("expected parse to succeed"),
    }
});
