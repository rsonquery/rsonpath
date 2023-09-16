#![no_main]

use libfuzzer_sys::fuzz_target;
use rsonpath::query::JsonPathQuery;

fuzz_target!(|data: &str| {
    let _ = JsonPathQuery::parse(data);
});
