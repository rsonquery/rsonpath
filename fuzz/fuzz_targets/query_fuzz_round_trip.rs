#![no_main]

use libfuzzer_sys::fuzz_target;
use rsonpath_syntax::JsonPathQuery;

fuzz_target!(|data: JsonPathQuery| {
    let str = data.to_string();
    let query = rsonpath_syntax::parse(&str).expect("should parse");
    assert_eq!(data, query, "query string: {str}");
});
