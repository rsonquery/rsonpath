//! Fuzz round-tripping - for every valid query parsing its `.to_string()` should give an equivalent query.
#![no_main]

use libfuzzer_sys::{fuzz_target, Corpus};
use rsonpath_syntax::JsonPathQuery;

fuzz_target!(|data: JsonPathQuery| -> Corpus {
    if data.nesting_level() > rsonpath_syntax::Parser::RECURSION_LIMIT_DEFAULT {
        return Corpus::Reject;
    }
    let str = data.to_string();
    let query = rsonpath_syntax::parse(&str).expect("should parse");
    assert_eq!(data, query, "query string: {str}");
    Corpus::Keep
});
