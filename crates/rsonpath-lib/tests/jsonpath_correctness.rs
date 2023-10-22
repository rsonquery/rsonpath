mod jsonpath_correctness {
    pub mod arbitrary_json;
}

use crate::jsonpath_correctness::arbitrary_json::{
    crawl, json_and_query, json_string_arbitrary, json_string_ascii, Json, Query,
};
use pretty_assertions::assert_eq;
use proptest::prelude::*;
use rsonpath::{
    engine::{Compiler, Engine, RsonpathEngine},
    input::OwnedBytes,
};

fn run_test(json: Json, query: Query) {
    let expected_results: Vec<_> = crawl(&json).filter(|x| x.matches(&query)).collect();

    let json_string = json.to_string();
    let input = OwnedBytes::new(&json_string).expect("correct input");
    let engine = RsonpathEngine::compile_query(&query.0).expect("compiled engine");
    let mut sink = vec![];

    engine.matches(&input, &mut sink).expect("engine does not crash");

    assert_eq!(expected_results.len(), sink.len(), "different result counts\nfull input: {json}\nfull query: {query}");

    for (ex, ac) in expected_results.iter().zip(sink) {
        let str = std::str::from_utf8(ac.bytes()).unwrap();
        let str2 = ex.value().to_string();
        //let value: serde_json::Value = serde_json::from_str(str).expect(str);

        assert_eq!(str, str2, "raw str: {str}\nraw str2: {str2}\nfull input: {json}\nfull query: {query}");
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1024,
        max_shrink_iters: 1 << 20,
        max_shrink_time: 60_000, // In ms.
        ..Default::default()
    })]
    #[test]
    fn rsonpath_engine_arbitrary_data(input in json_and_query(json_string_arbitrary())) {
        run_test(input.0, input.1)
    }

    #[test]
    fn rsonpath_engine_ascii_data(input in json_and_query(json_string_ascii())) {
        run_test(input.0, input.1)
    }
}
